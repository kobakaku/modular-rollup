mod stf_blueprint;

use rollup_interface::state::stf::StateTransitionFunction;
pub use stf_blueprint::StfBlueprint;

impl StfBlueprint {}

impl StateTransitionFunction for StfBlueprint {
    type StateRoot = String;

    type GenesisParams = String;

    type PreState = String;

    type ChangeSet = String;

    fn init_chain(
        &self,
        // genesis_state: Self::PreState,
        _params: Self::GenesisParams,
    ) -> (Self::StateRoot, Self::ChangeSet) {
        let genesis_hash = "TODO".to_string();
        let pre_state = "TODO".to_string();
        (genesis_hash, pre_state)
    }
}
