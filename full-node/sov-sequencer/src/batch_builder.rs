use std::{collections::VecDeque, io::Cursor};

use anyhow::{bail, Context as _};
use borsh::BorshDeserialize;
use rollup_interface::services::batch_builder::BatchBuilder;
use sov_modules_api::transaction::Transaction;
use sov_modules_core::{Context, DispatchCall};

#[derive(Clone)]
pub struct PooledTransaction<C: Context, DC: DispatchCall<Context = C>> {
    /// Raw transaction bytes.
    raw: Vec<u8>,
    /// Deserialized transaction.
    tx: Transaction<C>,
    /// The decoded runtime message.
    msg: DC::Decodable,
}

/// BatchBuilder that creates batches of transaction in the order of FIFO (First-In First-Out)
pub struct FiFoBatchBuilder<C: Context, DC: DispatchCall<Context = C>> {
    mempool: VecDeque<PooledTransaction<C, DC>>,
    current_storage: C::Storage,
    mempool_max_txs_count: usize,
}

impl<C: Context, DC: DispatchCall<Context = C>> FiFoBatchBuilder<C, DC> {
    pub fn new(current_storage: C::Storage, mempool_max_txs_count: usize) -> Self {
        Self {
            mempool: VecDeque::new(),
            current_storage,
            mempool_max_txs_count,
        }
    }
}

impl<C: Context, DC: DispatchCall<Context = C>> BatchBuilder for FiFoBatchBuilder<C, DC> {
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

        // Decode
        // let msg: DC::Decodable = decode(tx.runtime_msg);

        // Add the tx to mempool
        self.mempool.push_back(PooledTransaction {
            raw,
            tx,
            msg: todo!(),
        });
        Ok(())
    }

    /// Builds a new batch of valid transactions in order they were added to mempool.
    fn get_next_blob(&mut self) -> anyhow::Result<Vec<Vec<u8>>> {
        let mut txs = Vec::new();

        while let Some(pooled_tx) = self.mempool.pop_front() {
            let msg = pooled_tx.tx.runtime_msg;

            tracing::info!("Transaction has been included in the batch");
            txs.push(pooled_tx.raw);
        }

        if txs.is_empty() {
            bail!("No valid transations are available");
        }

        Ok(txs)
    }
}
