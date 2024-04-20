pub mod batch_builder;
pub mod rpc;

use anyhow::anyhow;
use rollup_interface::services::{batch_builder::BatchBuilder, da::DaService};
use tokio::sync::Mutex;

// Single data structure that manages mempool and batch producting.
pub struct Sequencer<B: BatchBuilder, T: DaService> {
    batch_builder: Mutex<B>,
    da_service: T,
}

impl<B: BatchBuilder, D: DaService> Sequencer<B, D> {
    pub fn new(batch_builder: B, da_service: D) -> Self {
        Self {
            batch_builder: Mutex::new(batch_builder),
            da_service,
        }
    }

    async fn submit_batch(&self) -> anyhow::Result<usize> {
        tracing::info!("Submit batch request has been received!");
        let mut batch_builder = self.batch_builder.lock().await;
        let blob = batch_builder.get_next_blob()?;
        let blob_len = blob.len();
        let blob = borsh::to_vec(&blob)?;

        match self.da_service.send_transaction(&blob).await {
            Ok(_) => Ok(blob_len),
            Err(e) => Err(anyhow!("Failed to submit batch: {:?}", e)),
        }
    }

    async fn accept_tx(&self, tx: Vec<u8>) -> anyhow::Result<()> {
        tracing::info!("Accepting tx: 0x{}", hex::encode(&tx));
        let mut batch_builder = self.batch_builder.lock().await;
        batch_builder.accept_tx(tx)?;
        Ok(())
    }
}
