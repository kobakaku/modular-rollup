/// State transition function defines business logic that responsible for changing state.
pub trait StateTransitionFunction {
    /// Root hash of state merkle tree
    type StateRoot;

    /// The initial params of the rollup.
    type GenesisParams;

    /// State of the rollup before transition.
    type PreState;

    /// State of the rollup after transition.
    type ChangeSet;

    /// Perform one-time initialization for the genesis block and
    /// returns the resulting root hash and changeset.
    /// If the init chain fails we panic.
    fn init_chain(
        &self,
        // genesis_state: Self::PreState,
        params: Self::GenesisParams,
    ) -> (Self::StateRoot, Self::ChangeSet);
}
