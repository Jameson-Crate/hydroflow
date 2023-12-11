use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;

use hydroflow::futures::stream::Stream;
use hydroflow_lang::graph::{
    eliminate_extra_unions_tees, partition_graph, propegate_flow_props, FlatGraphBuilder,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use stageleft::{IntoQuotedOnce, Quoted, QuotedContext};
use syn::parse_quote;

use crate::{HfBuilt, HfStream, RuntimeContext};

pub struct HfBuilder<'a> {
    pub(crate) next_id: RefCell<usize>,
    pub(crate) builders: RefCell<Option<BTreeMap<usize, FlatGraphBuilder>>>,
    _phantom: PhantomData<&'a mut &'a ()>,
}

impl<'a> QuotedContext for HfBuilder<'a> {
    fn create() -> Self {
        HfBuilder::new()
    }
}

impl<'a> HfBuilder<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> HfBuilder<'a> {
        HfBuilder {
            next_id: RefCell::new(0),
            builders: RefCell::new(Some(Default::default())),
            _phantom: PhantomData,
        }
    }

    pub fn build(&self, id: impl Quoted<'a, usize>) -> HfBuilt<'a> {
        let builders = self.builders.borrow_mut().take().unwrap();

        let mut conditioned_tokens = None;
        for (node_id, builder) in builders {
            let (mut flat_graph, _, _) = builder.build();
            eliminate_extra_unions_tees(&mut flat_graph);
            let mut partitioned_graph =
                partition_graph(flat_graph).expect("Failed to partition (cycle detected).");

            let hydroflow_crate = proc_macro_crate::crate_name("hydroflow_plus")
                .expect("hydroflow_plus should be present in `Cargo.toml`");
            let root = match hydroflow_crate {
                proc_macro_crate::FoundCrate::Itself => quote! { hydroflow_plus },
                proc_macro_crate::FoundCrate::Name(name) => {
                    let ident = syn::Ident::new(&name, Span::call_site());
                    quote! { #ident }
                }
            };

            let mut diagnostics = Vec::new();
            // Propgeate flow properties throughout the graph.
            // TODO(mingwei): Should this be done at a flat graph stage instead?
            let _ = propegate_flow_props::propegate_flow_props(
                &mut partitioned_graph,
                &mut diagnostics,
            );

            let tokens = partitioned_graph.as_code(&root, true, quote::quote!(), &mut diagnostics);

            if let Some(conditioned_tokens) = conditioned_tokens.as_mut() {
                *conditioned_tokens = parse_quote! {
                    #conditioned_tokens else if __given_id == #node_id {
                        #tokens
                    }
                };
            } else {
                conditioned_tokens = Some(parse_quote! {
                    if __given_id == #node_id {
                        #tokens
                    }
                });
            }
        }

        let id_spliced = id.splice();
        let conditioned_tokens: TokenStream = conditioned_tokens.unwrap();

        HfBuilt {
            tokens: parse_quote! {
                let __given_id = #id_spliced;
                #conditioned_tokens else {
                    panic!("Invalid node id: {}", __given_id);
                }
            },
            _phantom: PhantomData,
        }
    }

    pub fn runtime_context(&self) -> RuntimeContext<'a> {
        RuntimeContext {
            _phantom: PhantomData,
        }
    }

    pub fn source_stream<T, E: Stream<Item = T> + Unpin>(
        &'a self,
        node_id: usize,
        e: impl Quoted<'a, E>,
    ) -> HfStream<'a, T> {
        let next_id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let ident = syn::Ident::new(&format!("stream_{}", next_id), Span::call_site());
        let e = e.splice();

        self.builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(node_id)
            .or_default()
            .add_statement(parse_quote! {
                #ident = source_stream(#e) -> tee();
            });

        HfStream {
            ident,
            node_id,
            graph: self,
            _phantom: PhantomData,
        }
    }

    pub fn source_iter<T, E: IntoIterator<Item = T>>(
        &'a self,
        node_id: usize,
        e: impl IntoQuotedOnce<'a, E>,
    ) -> HfStream<'a, T> {
        let next_id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let ident = syn::Ident::new(&format!("stream_{}", next_id), Span::call_site());
        let e = e.splice();

        self.builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(node_id)
            .or_default()
            .add_statement(parse_quote! {
                #ident = source_iter(#e) -> tee();
            });

        HfStream {
            ident,
            node_id,
            graph: self,
            _phantom: PhantomData,
        }
    }

    pub fn cycle<T>(&'a self, node_id: usize) -> (HfCycle<'a, T>, HfStream<'a, T>) {
        let next_id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let ident = syn::Ident::new(&format!("stream_{}", next_id), Span::call_site());

        self.builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(node_id)
            .or_default()
            .add_statement(parse_quote! {
                #ident = tee();
            });

        (
            HfCycle {
                ident: ident.clone(),
                node_id,
                graph: self,
                _phantom: PhantomData,
            },
            HfStream {
                ident,
                node_id,
                graph: self,
                _phantom: PhantomData,
            },
        )
    }
}

pub struct HfCycle<'a, T> {
    ident: syn::Ident,
    node_id: usize,
    graph: &'a HfBuilder<'a>,
    _phantom: PhantomData<T>,
}

impl<'a, T> HfCycle<'a, T> {
    pub fn complete(self, stream: &HfStream<'a, T>) {
        let ident = self.ident;
        let stream_ident = stream.ident.clone();

        self.graph
            .builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(self.node_id)
            .or_default()
            .add_statement(parse_quote! {
                #stream_ident -> #ident;
            });
    }
}
