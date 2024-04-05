use jsonrpsee::{core::RpcResult, proc_macros::rpc, RpcModule};

use crate::Sequencer;

#[rpc(server, namespace = "sequencer")]
pub trait SequencerRpc {
    #[method(name = "publish_batch")]
    fn publish_batch(&self) -> RpcResult<()>;
}

pub fn get_sequencer_rpc() -> RpcModule<()> {
    let mut module = RpcModule::new(());
    module.merge(Sequencer.into_rpc()).unwrap();
    module
}
