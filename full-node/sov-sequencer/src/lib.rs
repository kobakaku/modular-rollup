pub mod batch_builder;
pub mod rpc;

use batch_builder::FiFoBatchBuilder;
use jsonrpsee::core::RpcResult;
use rpc::SequencerRpcServer;

// Single data structure that manages mempool and batch producting.
pub struct Sequencer {
    batch_builder: FiFoBatchBuilder,
}

impl Sequencer {
    pub fn new(batch_builder: FiFoBatchBuilder) -> Self {
        Self { batch_builder }
    }

    fn submit_tx(&self) -> anyhow::Result<()> {
        tracing::info!("Submit batch request has been received!");
        Ok(())
    }

    fn accept_tx(&self) -> anyhow::Result<()> {
        tracing::info!("Accepting tx: 0x......");
        Ok(())
    }
}

impl SequencerRpcServer for Sequencer {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"sequencer_publish_batch","id":1}' http://127.0.0.1:12345 | jq
    fn publish_batch(&self) -> RpcResult<()> {
        self.accept_tx().unwrap();
        self.submit_tx().unwrap();
        Ok(())
    }
}
