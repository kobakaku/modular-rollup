use jsonrpsee::{core::RpcResult, proc_macros::rpc, RpcModule};
use sov_modules_core::Context;

use crate::{batch_builder::FiFoBatchBuilder, Sequencer};

#[rpc(server, namespace = "sequencer")]
pub trait SequencerRpc {
    #[method(name = "publish_batch")]
    fn publish_batch(&self) -> RpcResult<()>;
}

pub fn get_sequencer_rpc<C: Context>(batch_builder: FiFoBatchBuilder<C>) -> RpcModule<()> {
    let sequencer = Sequencer::new(batch_builder);
    let mut module = RpcModule::new(());
    module.merge(Sequencer::into_rpc(sequencer)).unwrap();
    module
}
