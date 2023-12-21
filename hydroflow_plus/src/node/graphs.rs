use std::cell::RefCell;

use stageleft::{Quoted, RuntimeData};

use super::{HfCluster, HfNode, LocalDeploy, NodeBuilder};
use crate::builder::Builders;
use crate::GraphBuilder;

pub struct SingleGraph {}

impl<'a> LocalDeploy<'a> for SingleGraph {
    type Node = SingleNode<'a>;
    type Cluster = SingleNode<'a>;
    type Meta = ();
    type RuntimeID = ();
}

impl<'a> NodeBuilder<'a, SingleGraph> for () {
    fn build(
        &self,
        _id: usize,
        builder: &'a GraphBuilder<'a, SingleGraph>,
        _meta: &mut (),
    ) -> SingleNode<'a> {
        SingleNode { builder }
    }
}

#[derive(Clone)]
pub struct SingleNode<'a> {
    builder: &'a GraphBuilder<'a, SingleGraph>,
}

impl<'a> HfNode<'a> for SingleNode<'a> {
    type Port = ();
    type Meta = ();

    fn id(&self) -> usize {
        0
    }

    fn graph_builder(&self) -> (&'a RefCell<usize>, &'a Builders) {
        (&self.builder.next_id, &self.builder.builders)
    }

    fn next_port(&self) {
        panic!();
    }

    fn update_meta(&mut self, _meta: &Self::Meta) {}
}

impl<'a> HfCluster<'a> for SingleNode<'a> {
    fn ids(&self) -> impl Quoted<'a, &'a Vec<u32>> + Copy + 'a {
        panic!();
        #[allow(unreachable_code)]
        RuntimeData::new("")
    }
}

pub struct MultiGraph {}

impl<'a> LocalDeploy<'a> for MultiGraph {
    type Node = MultiNode<'a>;
    type Cluster = MultiNode<'a>;
    type Meta = ();
    type RuntimeID = usize;
}

impl<'a> NodeBuilder<'a, MultiGraph> for () {
    fn build(
        &self,
        id: usize,
        builder: &'a GraphBuilder<'a, MultiGraph>,
        _meta: &mut (),
    ) -> MultiNode<'a> {
        MultiNode { builder, id }
    }
}

#[derive(Clone)]
pub struct MultiNode<'a> {
    builder: &'a GraphBuilder<'a, MultiGraph>,
    id: usize,
}

impl<'a> HfNode<'a> for MultiNode<'a> {
    type Port = ();
    type Meta = ();

    fn id(&self) -> usize {
        self.id
    }

    fn graph_builder(&self) -> (&'a RefCell<usize>, &'a Builders) {
        (&self.builder.next_id, &self.builder.builders)
    }

    fn next_port(&self) {
        panic!();
    }

    fn update_meta(&mut self, _meta: &Self::Meta) {}
}

impl<'a> HfCluster<'a> for MultiNode<'a> {
    fn ids(&self) -> impl Quoted<'a, &'a Vec<u32>> + Copy + 'a {
        panic!();
        #[allow(unreachable_code)]
        RuntimeData::new("")
    }
}
