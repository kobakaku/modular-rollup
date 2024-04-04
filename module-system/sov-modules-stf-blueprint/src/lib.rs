mod stf_blueprint;

use rollup_interface::state::stf::StateTransitionFunction;
use sov_modules_core::Context;
pub use stf_blueprint::StfBlueprint;

impl<C> StateTransitionFunction for StfBlueprint<C>
where
    C: Context,
{
    type StateRoot = [u8; 0];

    type GenesisParams = ();

    type PreState = C::Storage;

    type ChangeSet = ();

    fn init_chain(
        &self,
        _pre_state: Self::PreState,
        _genesis_params: Self::GenesisParams,
    ) -> (Self::StateRoot, Self::ChangeSet) {
        ([], ())
    }
}
