use rollup_interface::node::services::batch_builder::BatchBuilder;
use sov_modules_api::transaction::Transaction;
use sov_modules_core::Context;

pub struct PooledTransaction<C: Context> {
    /// Raw transaction bytes.
    raw: Vec<u8>,
    /// Deserialized transaction.
    transaction: Transaction<C>,
}

/// BatchBuilder that creates batches of transaction in the order of FIFO (First-In First-Out)
pub struct FiFoBatchBuilder {}

impl FiFoBatchBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl BatchBuilder for FiFoBatchBuilder {
    fn accept_tx() -> anyhow::Result<()> {
        Ok(())
    }

    fn get_next_blob() -> anyhow::Result<()> {
        Ok(())
    }
}
