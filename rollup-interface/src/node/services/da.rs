use crate::state::da::{DaSpec, DaVerifier};

/// The DaService is the local side of an RPC connection talking to a node of the DA layer.
/// The DaService has two responsibilities - facing data from the DA layer, transforming the
/// data into a representation that can be efficiently verified in circuit.
pub trait DaService {
    /// A handle to the types used by the DA layer.
    type Spec: DaSpec;

    /// The verifier for the DA layer.
    type Verifier: DaVerifier;

    /// The DA lauyer block.
    type Block: SlotData;

    /// Fetch the block at the given height.
    fn get_block_at(&self, height: u64) -> anyhow::Result<Self::Block>;

    /// Fetch the block header of the last finalized block.
    /// If there's no finalized block yet, it should return an error.
    fn get_last_finalized_block_header(
        &self,
    ) -> anyhow::Result<<Self::Spec as DaSpec>::BlockHeader>;

    /// Send a transaction directly to the DA layer.
    /// Returns nothing if the transaction wa successfully sent.
    fn send_transaction(&self, blob: &[u8]) -> anyhow::Result<()>;
}

/// `SlotData` is the subset of the DA layer block which is stored in the rollup's database.
pub trait SlotData {
    type BlockHeader;

    fn hash(&self) -> [u8; 32];

    fn header(&self) -> &Self::BlockHeader;
}
