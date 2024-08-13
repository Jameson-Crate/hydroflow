use std::cell::RefCell;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use hydroflow::bytes::Bytes;
use hydroflow::futures::Sink;
use hydroflow_lang::parse::Pipeline;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stageleft::{q, IntoQuotedMut, Quoted};
use syn::parse_quote;

use crate::builder::FlowLeaves;
use crate::ir::{DebugInstantiate, HfPlusLeaf, HfPlusNode, HfPlusSource};
use crate::location::{CanSend, Location, LocationKind};
use crate::Cluster;

/// Marks the stream as being asynchronous, which means the presence
/// of all elements is directly influenced by the runtime's batching
/// behavior. Aggregation operations are not permitted on streams
/// with this tag because the developer has not explicitly specified
/// if they want to aggregate over the entire stream or just the
/// current batch.
pub struct Async {}

/// Marks the stream as being windowed, which means the developer has
/// opted-into either a batched or persistent windowing semantics.
/// Aggregation operations are permitted on streams with this tag.
pub struct Windowed {}

/// An infinite stream of elements of type `T`.
///
/// Type Parameters:
/// - `'a`: the lifetime of the final Hydroflow graph, which constraints
///   which values can be captured in closures passed to operators
/// - `T`: the type of elements in the stream
/// - `W`: the windowing semantics of the stream, which is either [`Async`]
///    or [`Windowed`]
/// - `N`: the type of the node that the stream is materialized on
pub struct Stream<'a, T, W, N: Location> {
    location_kind: LocationKind,

    ir_leaves: FlowLeaves<'a>,
    pub(crate) ir_node: RefCell<HfPlusNode<'a>>,

    _phantom: PhantomData<(&'a mut &'a (), T, N, W)>,
}

impl<'a, T, W, N: Location> Stream<'a, T, W, N> {
    pub(crate) fn new(
        location_kind: LocationKind,
        ir_leaves: FlowLeaves<'a>,
        ir_node: HfPlusNode<'a>,
    ) -> Self {
        Stream {
            location_kind,
            ir_leaves,
            ir_node: RefCell::new(ir_node),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: Clone, W, N: Location> Clone for Stream<'a, T, W, N> {
    fn clone(&self) -> Self {
        if !matches!(self.ir_node.borrow().deref(), HfPlusNode::Tee { .. }) {
            let orig_ir_node = self.ir_node.replace(HfPlusNode::Placeholder);
            *self.ir_node.borrow_mut() = HfPlusNode::Tee {
                inner: Rc::new(RefCell::new(orig_ir_node)),
            };
        }

        Stream::new(
            self.location_kind,
            self.ir_leaves.clone(),
            self.ir_node.borrow().clone(),
        )
    }
}

impl<'a, T, W, N: Location> Stream<'a, T, W, N> {
    pub fn map<U, F: Fn(T) -> U + 'a>(self, f: impl IntoQuotedMut<'a, F>) -> Stream<'a, U, W, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Map {
                f: f.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn flat_map<U, I: IntoIterator<Item = U>, F: Fn(T) -> I + 'a>(
        self,
        f: impl IntoQuotedMut<'a, F>,
    ) -> Stream<'a, U, W, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::FlatMap {
                f: f.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn enumerate(self) -> Stream<'a, (usize, T), W, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Enumerate(Box::new(self.ir_node.into_inner())),
        )
    }

    pub fn inspect<F: Fn(&T) + 'a>(self, f: impl IntoQuotedMut<'a, F>) -> Stream<'a, T, W, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Inspect {
                f: f.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn filter<F: Fn(&T) -> bool + 'a>(
        self,
        f: impl IntoQuotedMut<'a, F>,
    ) -> Stream<'a, T, W, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Filter {
                f: f.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn filter_map<U, F: Fn(T) -> Option<U> + 'a>(
        self,
        f: impl IntoQuotedMut<'a, F>,
    ) -> Stream<'a, U, W, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::FilterMap {
                f: f.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn cross_singleton<O>(self, other: Stream<'a, O, Windowed, N>) -> Stream<'a, (T, O), W, N>
    where
        O: Clone,
    {
        if self.location_kind != other.location_kind {
            panic!("cross_singleton must be called on streams on the same node");
        }

        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::CrossSingleton(
                Box::new(self.ir_node.into_inner()),
                Box::new(other.ir_node.into_inner()),
            ),
        )
    }

    /// Allow this stream through if the other stream has elements, otherwise the output is empty.
    pub fn continue_if<U>(self, signal: Stream<'a, U, Windowed, N>) -> Stream<'a, T, W, N> {
        self.cross_singleton(signal.map(q!(|_u| ())))
            .map(q!(|(d, _signal)| d))
    }

    /// Allow this stream through if the other stream is empty, otherwise the output is empty.
    pub fn continue_unless<U>(self, other: Stream<'a, U, Windowed, N>) -> Stream<'a, T, W, N> {
        self.continue_if(other.count().filter(q!(|c| *c == 0)))
    }

    // TODO(shadaj): should allow for differing windows, using strongest one
    pub fn cross_product<O>(self, other: Stream<'a, O, W, N>) -> Stream<'a, (T, O), W, N>
    where
        T: Clone,
        O: Clone,
    {
        if self.location_kind != other.location_kind {
            panic!("cross_product must be called on streams on the same node");
        }

        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::CrossProduct(
                Box::new(self.ir_node.into_inner()),
                Box::new(other.ir_node.into_inner()),
            ),
        )
    }

    pub fn union(self, other: Stream<'a, T, W, N>) -> Stream<'a, T, W, N> {
        if self.location_kind != other.location_kind {
            panic!("union must be called on streams on the same node");
        }

        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Union(
                Box::new(self.ir_node.into_inner()),
                Box::new(other.ir_node.into_inner()),
            ),
        )
    }

    pub fn for_each<F: Fn(T) + 'a>(self, f: impl IntoQuotedMut<'a, F>) {
        self.ir_leaves.borrow_mut().as_mut().expect("Attempted to add a leaf to a flow that has already been finalized. No leaves can be added after the flow has been compiled.").push(HfPlusLeaf::ForEach {
            input: Box::new(self.ir_node.into_inner()),
            f: f.splice().into(),
        });
    }

    pub fn dest_sink<S: Unpin + Sink<T> + 'a>(self, sink: impl Quoted<'a, S>) {
        self.ir_leaves.borrow_mut().as_mut().expect("Attempted to add a leaf to a flow that has already been finalized. No leaves can be added after the flow has been compiled.").push(HfPlusLeaf::DestSink {
            sink: sink.splice().into(),
            input: Box::new(self.ir_node.into_inner()),
        });
    }

    pub fn all_ticks(self) -> Stream<'a, T, Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Persist(Box::new(self.ir_node.into_inner())),
        )
    }

    pub fn assume_windowed(self) -> Stream<'a, T, Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            self.ir_node.into_inner(),
        )
    }
}

impl<'a, T, N: Location> Stream<'a, T, Async, N> {
    pub fn tick_batch(self) -> Stream<'a, T, Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            self.ir_node.into_inner(),
        )
    }
}

impl<'a, T, N: Location> Stream<'a, T, Windowed, N> {
    pub fn fold<A, I: Fn() -> A + 'a, C: Fn(&mut A, T)>(
        self,
        init: impl IntoQuotedMut<'a, I>,
        comb: impl IntoQuotedMut<'a, C>,
    ) -> Stream<'a, A, Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Fold {
                init: init.splice().into(),
                acc: comb.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn reduce<C: Fn(&mut T, T) + 'a>(
        self,
        comb: impl IntoQuotedMut<'a, C>,
    ) -> Stream<'a, T, Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Reduce {
                f: comb.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn sort(self) -> Stream<'a, T, Windowed, N>
    where
        T: Ord,
    {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Sort(Box::new(self.ir_node.into_inner())),
        )
    }

    pub fn count(self) -> Stream<'a, usize, Windowed, N> {
        self.fold(q!(|| 0usize), q!(|count, _| *count += 1))
    }

    pub fn delta(self) -> Stream<'a, T, Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Delta(Box::new(self.ir_node.into_inner())),
        )
    }

    pub fn defer_tick(self) -> Stream<'a, T, Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::DeferTick(Box::new(self.ir_node.into_inner())),
        )
    }

    pub fn unique(self) -> Stream<'a, T, Windowed, N>
    where
        T: Eq + Hash,
    {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Unique(Box::new(self.ir_node.into_inner())),
        )
    }

    pub fn filter_not_in(self, other: Stream<'a, T, Windowed, N>) -> Stream<'a, T, Windowed, N>
    where
        T: Eq + Hash,
    {
        if self.location_kind != other.location_kind {
            panic!("union must be called on streams on the same node");
        }

        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Difference(
                Box::new(self.ir_node.into_inner()),
                Box::new(other.ir_node.into_inner()),
            ),
        )
    }

    pub fn sample_every(
        self,
        duration: impl Quoted<'a, std::time::Duration> + Copy + 'a,
    ) -> Stream<'a, T, Windowed, N> {
        let interval = duration.splice();

        let samples = Stream::<'a, hydroflow::tokio::time::Instant, Windowed, N>::new(
            self.location_kind,
            self.ir_leaves.clone(),
            HfPlusNode::Source {
                source: HfPlusSource::Interval(interval.into()),
                location_kind: self.location_kind,
            },
        );

        self.continue_if(samples)
    }
}

impl<'a, T: Clone, W, N: Location> Stream<'a, &T, W, N> {
    pub fn cloned(self) -> Stream<'a, T, W, N> {
        self.map(q!(|d| d.clone()))
    }
}

impl<'a, K, V1, W, N: Location> Stream<'a, (K, V1), W, N> {
    // TODO(shadaj): figure out window semantics
    pub fn join<W2, V2>(self, n: Stream<'a, (K, V2), W2, N>) -> Stream<'a, (K, (V1, V2)), W, N>
    where
        K: Eq + Hash,
    {
        if self.location_kind != n.location_kind {
            panic!("join must be called on streams on the same node");
        }

        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::Join(
                Box::new(self.ir_node.into_inner()),
                Box::new(n.ir_node.into_inner()),
            ),
        )
    }

    pub fn anti_join<W2>(self, n: Stream<'a, K, W2, N>) -> Stream<'a, (K, V1), W, N>
    where
        K: Eq + Hash,
    {
        if self.location_kind != n.location_kind {
            panic!("anti_join must be called on streams on the same node");
        }

        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::AntiJoin(
                Box::new(self.ir_node.into_inner()),
                Box::new(n.ir_node.into_inner()),
            ),
        )
    }
}

impl<'a, K: Eq + Hash, V, N: Location> Stream<'a, (K, V), Windowed, N> {
    pub fn fold_keyed<A, I: Fn() -> A + 'a, C: Fn(&mut A, V) + 'a>(
        self,
        init: impl IntoQuotedMut<'a, I>,
        comb: impl IntoQuotedMut<'a, C>,
    ) -> Stream<'a, (K, A), Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::FoldKeyed {
                init: init.splice().into(),
                acc: comb.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn reduce_keyed<F: Fn(&mut V, V) + 'a>(
        self,
        comb: impl IntoQuotedMut<'a, F>,
    ) -> Stream<'a, (K, V), Windowed, N> {
        Stream::new(
            self.location_kind,
            self.ir_leaves,
            HfPlusNode::ReduceKeyed {
                f: comb.splice().into(),
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }
}

fn get_this_crate() -> TokenStream {
    let hydroflow_crate = proc_macro_crate::crate_name("hydroflow_plus")
        .expect("hydroflow_plus should be present in `Cargo.toml`");
    match hydroflow_crate {
        proc_macro_crate::FoundCrate::Itself => quote! { hydroflow_plus },
        proc_macro_crate::FoundCrate::Name(name) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            quote! { #ident }
        }
    }
}

fn serialize_bincode<T: Serialize>(is_demux: bool) -> Pipeline {
    let root = get_this_crate();

    let t_type: syn::Type = stageleft::quote_type::<T>();

    if is_demux {
        parse_quote! {
            map(|(id, data)| {
                (id, #root::runtime_support::bincode::serialize::<#t_type>(&data).unwrap().into())
            })
        }
    } else {
        parse_quote! {
            map(|data| {
                #root::runtime_support::bincode::serialize::<#t_type>(&data).unwrap().into()
            })
        }
    }
}

fn deserialize_bincode<T: DeserializeOwned>(tagged: bool) -> Pipeline {
    let root = get_this_crate();

    let t_type: syn::Type = stageleft::quote_type::<T>();

    if tagged {
        parse_quote! {
            map(|res| {
                let (id, b) = res.unwrap();
                (id, #root::runtime_support::bincode::deserialize::<#t_type>(&b).unwrap())
            })
        }
    } else {
        parse_quote! {
            map(|res| {
                #root::runtime_support::bincode::deserialize::<#t_type>(&res.unwrap()).unwrap()
            })
        }
    }
}

impl<'a, T, W, N: Location> Stream<'a, T, W, N> {
    pub fn send_bincode<N2: Location, CoreType>(
        self,
        other: &N2,
    ) -> Stream<'a, N::Out<CoreType>, Async, N2>
    where
        N: CanSend<N2, In<CoreType> = T>,
        CoreType: Serialize + DeserializeOwned,
    {
        let serialize_pipeline = Some(serialize_bincode::<CoreType>(N::is_demux()));

        let deserialize_pipeline = Some(deserialize_bincode::<CoreType>(N::is_tagged()));

        Stream::new(
            other.location_kind(),
            self.ir_leaves,
            HfPlusNode::Network {
                from_location: self.location_kind,
                to_location: other.location_kind(),
                serialize_pipeline,
                instantiate_fn: DebugInstantiate::Building(),
                deserialize_pipeline,
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn send_bytes<N2: Location>(self, other: &N2) -> Stream<'a, N::Out<Bytes>, Async, N2>
    where
        N: CanSend<N2, In<Bytes> = T>,
    {
        Stream::new(
            other.location_kind(),
            self.ir_leaves,
            HfPlusNode::Network {
                from_location: self.location_kind,
                to_location: other.location_kind(),
                serialize_pipeline: None,
                instantiate_fn: DebugInstantiate::Building(),
                deserialize_pipeline: if N::is_tagged() {
                    Some(parse_quote!(map(|(id, b)| (id, b.unwrap().freeze()))))
                } else {
                    Some(parse_quote!(map(|b| b.unwrap().freeze())))
                },
                input: Box::new(self.ir_node.into_inner()),
            },
        )
    }

    pub fn send_bincode_interleaved<N2: Location, Tag, CoreType>(
        self,
        other: &N2,
    ) -> Stream<'a, CoreType, Async, N2>
    where
        N: CanSend<N2, In<CoreType> = T, Out<CoreType> = (Tag, CoreType)>,
        CoreType: Serialize + DeserializeOwned,
    {
        self.send_bincode::<N2, CoreType>(other).map(q!(|(_, b)| b))
    }

    pub fn send_bytes_interleaved<N2: Location, Tag, V>(
        self,
        other: &N2,
    ) -> Stream<'a, Bytes, Async, N2>
    where
        N: CanSend<N2, In<Bytes> = T, Out<Bytes> = (Tag, Bytes)>,
    {
        self.send_bytes::<N2>(other).map(q!(|(_, b)| b))
    }

    pub fn broadcast_bincode<C2>(
        self,
        other: &Cluster<'a, C2>,
    ) -> Stream<'a, N::Out<T>, Async, Cluster<'a, C2>>
    where
        N: CanSend<Cluster<'a, C2>, In<T> = (u32, T)>,
        T: Clone + Serialize + DeserializeOwned,
    {
        let ids = other.ids();

        self.flat_map(q!(|b| ids.iter().map(move |id| (
            ::std::clone::Clone::clone(id),
            ::std::clone::Clone::clone(&b)
        ))))
        .send_bincode(other)
    }

    pub fn broadcast_bincode_interleaved<C2, Tag>(
        self,
        other: &Cluster<'a, C2>,
    ) -> Stream<'a, T, Async, Cluster<'a, C2>>
    where
        N: CanSend<Cluster<'a, C2>, In<T> = (u32, T), Out<T> = (Tag, T)> + 'a,
        T: Clone + Serialize + DeserializeOwned,
    {
        self.broadcast_bincode(other).map(q!(|(_, b)| b))
    }

    pub fn broadcast_bytes<C2>(
        self,
        other: &Cluster<'a, C2>,
    ) -> Stream<'a, N::Out<Bytes>, Async, Cluster<'a, C2>>
    where
        N: CanSend<Cluster<'a, C2>, In<Bytes> = (u32, T)> + 'a,
        T: Clone,
    {
        let ids = other.ids();

        self.flat_map(q!(|b| ids.iter().map(move |id| (
            ::std::clone::Clone::clone(id),
            ::std::clone::Clone::clone(&b)
        ))))
        .send_bytes(other)
    }

    pub fn broadcast_bytes_interleaved<C2, Tag>(
        self,
        other: &Cluster<'a, C2>,
    ) -> Stream<'a, Bytes, Async, Cluster<'a, C2>>
    where
        N: CanSend<Cluster<'a, C2>, In<Bytes> = (u32, T), Out<Bytes> = (Tag, Bytes)> + 'a,
        T: Clone,
    {
        self.broadcast_bytes(other).map(q!(|(_, b)| b))
    }
}
