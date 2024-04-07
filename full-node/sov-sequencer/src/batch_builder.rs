use std::collections::VecDeque;

use anyhow::bail;
use rollup_interface::services::batch_builder::BatchBuilder;
use sov_modules_api::transaction::Transaction;
use sov_modules_core::Context;

#[derive(Clone)]
pub struct PooledTransaction<C: Context> {
    /// Raw transaction bytes.
    raw: Vec<u8>,
    /// Deserialized transaction.
    transaction: Transaction<C>,
}

/// BatchBuilder that creates batches of transaction in the order of FIFO (First-In First-Out)
pub struct FiFoBatchBuilder<C: Context> {
    mempool: VecDeque<PooledTransaction<C>>,
    current_storage: C::Storage,
}

impl<C: Context> FiFoBatchBuilder<C> {
    pub fn new(current_storage: C::Storage) -> Self {
        Self {
            mempool: VecDeque::new(),
            current_storage,
        }
    }
}

impl<C: Context> BatchBuilder for FiFoBatchBuilder<C> {
    /// Attempt to add transaction to the mempool.
    fn accept_tx() -> anyhow::Result<()> {
        Ok(())
    }

    /// Builds a new batch of valid transactions in order they were added to mempool.
    fn get_next_blob(&mut self) -> anyhow::Result<Vec<Vec<u8>>> {
        let mut txs = Vec::new();

        while let Some(pooled_tx) = self.mempool.pop_front() {
            tracing::info!("Transaction has been included in the batch");
            txs.push(pooled_tx.raw);
        }

        if txs.is_empty() {
            bail!("No valid transations are available");
        }

        Ok(txs)
    }
}
