pub mod batch_builder;
pub mod rpc;

use batch_builder::FiFoBatchBuilder;
use jsonrpsee::core::RpcResult;
use rpc::SequencerRpcServer;
use sov_modules_core::Context;

// Single data structure that manages mempool and batch producting.
pub struct Sequencer<C: Context> {
    batch_builder: FiFoBatchBuilder<C>,
}

impl<C: Context> Sequencer<C> {
    pub fn new(batch_builder: FiFoBatchBuilder<C>) -> Self {
        Self { batch_builder }
    }

    fn submit_batch(&self) -> anyhow::Result<()> {
        tracing::info!("Submit batch request has been received!");
        Ok(())
    }

    fn accept_tx(&self) -> anyhow::Result<()> {
        tracing::info!("Accepting tx: 0x......");
        Ok(())
    }
}

impl<C: Context> SequencerRpcServer for Sequencer<C> {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"sequencer_publish_batch","id":1}' http://127.0.0.1:12345 | jq
    fn publish_batch(&self) -> RpcResult<()> {
        self.accept_tx().unwrap();
        self.submit_batch().unwrap();
        Ok(())
    }
}
