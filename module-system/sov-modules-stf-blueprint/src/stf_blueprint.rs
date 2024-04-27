use std::marker::PhantomData;

use sov_modules_core::Context;

use crate::RuntimeTrait;

/// An implementation of the
/// [`StateTransitionFunction`](sov_rollup_interface::stf::StateTransitionFunction)
pub struct StfBlueprint<C: Context, Rt: RuntimeTrait> {
    runtime: Rt,
    phantom_context: PhantomData<C>,
}

impl<C, RT> StfBlueprint<C, RT>
where
    C: Context,
    RT: RuntimeTrait,
{
    /// [`StfBlueprint`] constructor.
    pub fn new() -> Self {
        Self {
            runtime: RT::default(),
            phantom_context: PhantomData,
        }
    }
}
