pub mod rpc;

use jsonrpsee::core::RpcResult;
use rpc::SequencerRpcServer;

pub struct Sequencer;

impl Sequencer {
    pub fn new() -> Self {
        Self {}
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
