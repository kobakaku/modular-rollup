use std::{collections::VecDeque, io::Cursor};

use anyhow::{bail, Context as _};
use borsh::BorshDeserialize;
use rollup_interface::services::batch_builder::BatchBuilder;
use sov_modules_api::transaction::Transaction;
use sov_modules_core::Context;

#[derive(Clone)]
pub struct PooledTransaction<C: Context> {
    /// Raw transaction bytes.
    raw: Vec<u8>,
    /// Deserialized transaction.
    tx: Transaction<C>,
}

/// BatchBuilder that creates batches of transaction in the order of FIFO (First-In First-Out)
pub struct FiFoBatchBuilder<C: Context> {
    mempool: VecDeque<PooledTransaction<C>>,
    current_storage: C::Storage,
    mempool_max_txs_count: usize,
}

impl<C: Context> FiFoBatchBuilder<C> {
    pub fn new(current_storage: C::Storage, mempool_max_txs_count: usize) -> Self {
        Self {
            mempool: VecDeque::new(),
            current_storage,
            mempool_max_txs_count,
        }
    }
}

impl<C: Context> BatchBuilder for FiFoBatchBuilder<C> {
    /// Attempt to add transaction to the mempool.
    ///
    /// The transaction is discarded if:
    ///  -  mempool is full
    ///  -  transaction is invalid (deserializeation, verification or decoding of the runtime message failed)
    fn accept_tx(&mut self, raw: Vec<u8>) -> anyhow::Result<()> {
        if self.mempool.len() >= self.mempool_max_txs_count {
            anyhow::bail!("Mempool is full.")
        }

        // Deserialize
        let mut data = Cursor::new(&raw);
        let tx = Transaction::<C>::deserialize_reader(&mut data)
            .context("Failed to deserialize transaction.")?;

        // Verify
        tx.verify().context("Failed to verify transaction.")?;

        // Decode (メッセージをデコードするときに必要？)

        // Add the tx to mempool
        self.mempool.push_back(PooledTransaction { raw, tx });
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
