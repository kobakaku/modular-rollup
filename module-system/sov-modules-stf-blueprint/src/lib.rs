mod stf_blueprint;

use rollup_interface::state::stf::StateTransitionFunction;
use sov_modules_core::{Context, Storage};
pub use stf_blueprint::StfBlueprint;

impl<C> StateTransitionFunction for StfBlueprint<C>
where
    C: Context,
{
    type StateRoot = <C::Storage as Storage>::Root;

    type GenesisParams = ();

    type PreState = C::Storage;

    type ChangeSet = ();

    fn init_chain(
        &self,
        pre_state: Self::PreState,
        _genesis_params: Self::GenesisParams,
    ) -> (Self::StateRoot, Self::ChangeSet) {
        let state_root = pre_state.get_root_hash().unwrap();
        (state_root, ())
    }
}
