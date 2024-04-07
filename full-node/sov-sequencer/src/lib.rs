pub mod batch_builder;
pub mod rpc;

use anyhow::anyhow;
use jsonrpsee::core::RpcResult;
use rollup_interface::services::batch_builder::BatchBuilder;
use rpc::SequencerRpcServer;
use sov_modules_api::utils::to_jsonrpsee_error_object;
use std::sync::Mutex;

const SEQUENCER_RPC_ERROR: &str = "SEQUENCER_RPC_ERROR";

// Single data structure that manages mempool and batch producting.
pub struct Sequencer<B: BatchBuilder> {
    batch_builder: Mutex<B>,
}

impl<B: BatchBuilder> Sequencer<B> {
    pub fn new(batch_builder: B) -> Self {
        Self {
            batch_builder: Mutex::new(batch_builder),
        }
    }

    fn submit_batch(&self) -> anyhow::Result<usize> {
        tracing::info!("Submit batch request has been received!");
        let mut batch_builder = self
            .batch_builder
            .lock()
            .map_err(|e| anyhow!("Failed to lock mempool: {}", e))?;
        let txs = batch_builder.get_next_blob()?;
        let txs_len = txs.len();

        // TODO: トランザクション送信処理を追加する

        Ok(txs_len)
    }

    fn accept_tx(&self) -> anyhow::Result<()> {
        tracing::info!("Accepting tx: 0x......");
        Ok(())
    }
}

impl<B: BatchBuilder> SequencerRpcServer for Sequencer<B> {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"sequencer_publish_batch","id":1}' http://127.0.0.1:12345 | jq
    fn publish_batch(&self) -> RpcResult<()> {
        self.accept_tx()
            .map_err(|e| to_jsonrpsee_error_object(SEQUENCER_RPC_ERROR, e))?;
        self.submit_batch()
            .map_err(|e| to_jsonrpsee_error_object(SEQUENCER_RPC_ERROR, e))?;
        Ok(())
    }
}
