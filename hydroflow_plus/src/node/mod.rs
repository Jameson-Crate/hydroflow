use std::cell::RefCell;
use std::io;
use std::marker::PhantomData;

use hydroflow::bytes::BytesMut;
use hydroflow::futures::stream::Stream;
use hydroflow_lang::parse::Pipeline;
use proc_macro2::Span;
use stageleft::Quoted;
use syn::parse_quote;

use crate::builder::Builders;
use crate::{HfBuilder, HfCycle, HfStream};

mod graphs;
pub use graphs::*;

pub trait HfDeploy<'a> {
    type Node: HfNode<'a>;
}

pub trait HfNetworkedDeploy<'a>: HfDeploy<'a, Node = Self::NetworkedNode> {
    type NetworkedNode: HfNode<'a, Port = Self::Port> + HfSendTo<'a, Self::NetworkedNode>;
    type Port;
}

impl<'a, T: HfDeploy<'a, Node = N>, N: HfSendTo<'a, N>> HfNetworkedDeploy<'a> for T {
    type NetworkedNode = N;
    type Port = N::Port;
}

pub trait HfNodeBuilder<'a, D: HfDeploy<'a> + ?Sized> {
    fn build(&mut self, id: usize, builder: &'a HfBuilder<'a, D>) -> D::Node;
}

pub trait HfSendTo<'a, O: HfNode<'a>>: HfNode<'a> {
    fn send_to(&self, other: &O, source_port: &Self::Port, recipient_port: &O::Port);
}

pub trait HfNode<'a>: Clone {
    type Port;

    fn id(&self) -> usize;
    fn graph_builder(&self) -> (&'a RefCell<usize>, &'a Builders);
    fn next_port(&self) -> Self::Port;
    fn gen_source_statement(&self, port: &Self::Port) -> Pipeline;
    fn gen_sink_statement(&self, port: &Self::Port) -> Pipeline;

    fn source_stream<T, E: Stream<Item = T> + Unpin>(
        &self,
        e: impl Quoted<'a, E>,
    ) -> HfStream<'a, T, Self> {
        let (next_id_cell, builders) = self.graph_builder();

        let next_id = {
            let mut next_id = next_id_cell.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let ident = syn::Ident::new(&format!("stream_{}", next_id), Span::call_site());
        let e = e.splice();

        builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(self.id())
            .or_default()
            .add_statement(parse_quote! {
                #ident = source_stream(#e) -> tee();
            });

        HfStream {
            ident,
            node: self.clone(),
            next_id: next_id_cell,
            builders,
            _phantom: PhantomData,
        }
    }

    fn source_external(&self) -> (Self::Port, HfStream<'a, Result<BytesMut, io::Error>, Self>) {
        let (next_id_cell, builders) = self.graph_builder();

        let next_id = {
            let mut next_id = next_id_cell.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let ident = syn::Ident::new(&format!("stream_{}", next_id), Span::call_site());
        let port = self.next_port();
        let source_pipeline = self.gen_source_statement(&port);

        builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(self.id())
            .or_default()
            .add_statement(parse_quote! {
                #ident = #source_pipeline -> tee();
            });

        (
            port,
            HfStream {
                ident,
                node: self.clone(),
                next_id: next_id_cell,
                builders,
                _phantom: PhantomData,
            },
        )
    }

    fn source_iter<T, E: IntoIterator<Item = T>>(
        &self,
        e: impl Quoted<'a, E>,
    ) -> HfStream<'a, T, Self> {
        let (next_id_cell, builders) = self.graph_builder();

        let next_id = {
            let mut next_id = next_id_cell.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let ident = syn::Ident::new(&format!("stream_{}", next_id), Span::call_site());
        let e = e.splice();

        builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(self.id())
            .or_default()
            .add_statement(parse_quote! {
                #ident = source_iter(#e) -> tee();
            });

        HfStream {
            ident,
            node: self.clone(),
            next_id: next_id_cell,
            builders,
            _phantom: PhantomData,
        }
    }

    fn cycle<T>(&self) -> (HfCycle<'a, T, Self>, HfStream<'a, T, Self>) {
        let (next_id_cell, builders) = self.graph_builder();

        let next_id = {
            let mut next_id = next_id_cell.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let ident = syn::Ident::new(&format!("stream_{}", next_id), Span::call_site());

        builders
            .borrow_mut()
            .as_mut()
            .unwrap()
            .entry(self.id())
            .or_default()
            .add_statement(parse_quote! {
                #ident = tee();
            });

        (
            HfCycle {
                ident: ident.clone(),
                node: self.clone(),
                builders,
                _phantom: PhantomData,
            },
            HfStream {
                ident,
                node: self.clone(),
                next_id: next_id_cell,
                builders,
                _phantom: PhantomData,
            },
        )
    }
}
