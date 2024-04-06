use jsonrpsee::{core::RpcResult, proc_macros::rpc, RpcModule};

use crate::{batch_builder::FiFoBatchBuilder, Sequencer};

#[rpc(server, namespace = "sequencer")]
pub trait SequencerRpc {
    #[method(name = "publish_batch")]
    fn publish_batch(&self) -> RpcResult<()>;
}

pub fn get_sequencer_rpc(batch_builder: FiFoBatchBuilder) -> RpcModule<()> {
    let mut module = RpcModule::new(());
    let sequencer = Sequencer { batch_builder };
    module.merge(Sequencer::into_rpc(sequencer)).unwrap();
    module
}
