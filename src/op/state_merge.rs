use crate::hide::{Hide, Delta, Cumul};
use crate::lattice::{LatticeRepr, Merge, Convert};

use super::{Op, OpDelta, OpCumul};

pub struct StateMerge<PrecOp, OLr>
where
    PrecOp: OpDelta,
    PrecOp::OLatRepr: Convert<OLr>,
    OLr: LatticeRepr + Merge<PrecOp::OLatRepr> + 'static,
{
    _phantom: std::marker::PhantomData<(PrecOp, OLr)>,
}

impl<PrecOp, OLr> Op for StateMerge<PrecOp, OLr>
where
    PrecOp: OpDelta,
    PrecOp::OLatRepr: Convert<OLr>,
    OLr: LatticeRepr + Merge<PrecOp::OLatRepr> + 'static,
{
    type ILatRepr = PrecOp::ILatRepr;
    type OLatRepr = OLr;

    type State = (PrecOp::State, Hide<'static, Cumul, OLr>);
}

impl<PrecOp, OLr> OpDelta for StateMerge<PrecOp, OLr>
where
    PrecOp: OpDelta,
    PrecOp::OLatRepr: Convert<OLr> + 'static,
    <PrecOp::OLatRepr as LatticeRepr>::Repr: 'static,
    OLr: LatticeRepr + Merge<PrecOp::OLatRepr> + 'static,

{
    fn get_delta<'h>(state: &'h mut Self::State, input: Hide<'h, Delta, Self::ILatRepr>)
        -> Hide<'h, Delta, Self::OLatRepr>
    {
        let (prec_state, self_state) = state;
        let item = PrecOp::get_delta(prec_state, input);
        Merge::merge_hide(self_state, item.clone());
        return Convert::convert_hide(item);
    }
}

impl<PrecOp, OLr> OpCumul for StateMerge<PrecOp, OLr>
where
    PrecOp: OpDelta,
    PrecOp::OLatRepr: Convert<OLr>,
    OLr: LatticeRepr + Merge<PrecOp::OLatRepr> + 'static,
{
    fn get_value<'a>(state: &'a mut Self::State)
        -> Hide<'a, Cumul, Self::OLatRepr>
    {
        let (_prec_state, self_state) = state;
        return self_state.clone();
    }
}

