mod stf_blueprint;

use rollup_interface::state::stf::StateTransitionFunction;
use stf_blueprint::StfBlueprint;

impl StfBlueprint {}

impl StateTransitionFunction for StfBlueprint {
    type StateRoot = String;

    type GenesisParams = String;

    type PreState = String;

    type ChangeSet = String;

    fn init_chain(
        &self,
        // genesis_state: Self::PreState,
        params: Self::GenesisParams,
    ) -> (Self::StateRoot, Self::ChangeSet) {
        todo!()
    }
}
