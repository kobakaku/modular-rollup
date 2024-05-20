use std::marker::PhantomData;

use sov_modules_core::Context;

/// An implementation of the
/// [`StateTransitionFunction`](sov_rollup_interface::stf::StateTransitionFunction)
pub struct StfBlueprint<C: Context> {
    phantom_context: PhantomData<C>,
}

impl<C> StfBlueprint<C>
where
    C: Context,
{
    /// [`StfBlueprint`] constructor.
    pub fn new() -> Self {
        Self {
            phantom_context: PhantomData,
        }
    }
}
