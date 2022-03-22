use super::{
    BaseSurface, PullSurface, PushSurface, PushSurfaceReversed, TrackPullDependencies,
    TrackPushDependencies,
};

use crate::builder::build::pull_filter::FilterPullBuild;
use crate::builder::build::push_filter::FilterPushBuild;

pub struct FilterSurface<Prev, Func>
where
    Prev: BaseSurface,
{
    prev: Prev,
    func: Func,
}
impl<Prev, Func> FilterSurface<Prev, Func>
where
    Prev: BaseSurface,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    pub fn new(prev: Prev, func: Func) -> Self {
        Self { prev, func }
    }
}

impl<Prev, Func> BaseSurface for FilterSurface<Prev, Func>
where
    Prev: BaseSurface,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    type ItemOut = Prev::ItemOut;
}

impl<Prev, Func> PullSurface for FilterSurface<Prev, Func>
where
    Prev: PullSurface,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    type InputHandoffs = Prev::InputHandoffs;
    type Build = FilterPullBuild<Prev::Build, Func>;

    fn into_parts(self) -> (Self::InputHandoffs, Self::Build) {
        let (connect, build) = self.prev.into_parts();
        let build = FilterPullBuild::new(build, self.func);
        (connect, build)
    }
}
impl<Prev, Func> TrackPullDependencies for FilterSurface<Prev, Func>
where
    Prev: PullSurface + TrackPullDependencies,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    fn insert_dep(&self, e: &mut super::DirectedEdgeSet) -> u16 {
        let my_id = e.add_node("Filter".to_string());
        let prev_id = self.prev.insert_dep(e);
        e.add_edge((prev_id, my_id));
        my_id
    }
}

impl<Prev, Func> PushSurface for FilterSurface<Prev, Func>
where
    Prev: PushSurface,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    type Output<Next> = Prev::Output<FilterPushSurfaceReversed<Next, Func>>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>;

    fn push_to<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    {
        self.prev
            .push_to(FilterPushSurfaceReversed::new(next, self.func))
    }
}
impl<Prev, Func> TrackPushDependencies for FilterSurface<Prev, Func>
where
    Prev: PushSurface + TrackPushDependencies,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    fn insert_dep(&self, e: &mut super::DirectedEdgeSet) -> u16 {
        let my_id = e.add_node("Filter".to_string());
        let prev_id = self.prev.insert_dep(e);
        e.add_edge((prev_id, my_id));
        my_id
    }
}

pub struct FilterPushSurfaceReversed<Next, Func>
where
    Next: PushSurfaceReversed,
{
    next: Next,
    func: Func,
}
impl<Next, Func> FilterPushSurfaceReversed<Next, Func>
where
    Next: PushSurfaceReversed,
    Func: FnMut(&Next::ItemIn) -> bool,
{
    pub fn new(next: Next, func: Func) -> Self {
        Self { next, func }
    }
}
impl<Next, Func> TrackPushDependencies for FilterPushSurfaceReversed<Next, Func>
where
    Next: PushSurfaceReversed + TrackPushDependencies,
    Func: FnMut(&Next::ItemIn) -> bool,
{
    fn insert_dep(&self, e: &mut super::DirectedEdgeSet) -> u16 {
        let my_id = e.add_node("Filter".to_string());
        let next_id = self.next.insert_dep(e);
        e.add_edge((my_id, next_id));
        my_id
    }
}

impl<Next, Func> PushSurfaceReversed for FilterPushSurfaceReversed<Next, Func>
where
    Next: PushSurfaceReversed,
    Func: FnMut(&Next::ItemIn) -> bool,
{
    type ItemIn = Next::ItemIn;

    type OutputHandoffs = Next::OutputHandoffs;
    type Build = FilterPushBuild<Next::Build, Func>;

    fn into_parts(self) -> (Self::OutputHandoffs, Self::Build) {
        let (connect, build) = self.next.into_parts();
        let build = FilterPushBuild::new(build, self.func);
        (connect, build)
    }
}
