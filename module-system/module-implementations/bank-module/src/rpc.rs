use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sov_modules_core::Context;

use crate::BankModule;

#[rpc(server, namespace = "bankModule")]
pub trait BankModuleRpc {
    #[method(name = "bank_method")]
    fn bank_method(&self) -> RpcResult<String>;
}

pub fn get_bank_module_rpc_method<C: Context + Send + Sync>() -> jsonrpsee::RpcModule<()> {
    let mut module = jsonrpsee::RpcModule::new(());
    let bank_module = BankModule::<C>::default();
    module.merge(BankModule::into_rpc(bank_module)).unwrap();
    module
}
