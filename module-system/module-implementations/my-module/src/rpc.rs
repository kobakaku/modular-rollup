use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::MyModule;

#[rpc(server, namespace = "myModule")]
pub trait MyModuleRpc {
    #[method(name = "my_method")]
    fn my_method(&self) -> RpcResult<String>;
}

pub fn get_my_module_rpc_method() -> jsonrpsee::RpcModule<()> {
    let mut module = jsonrpsee::RpcModule::new(());
    let my_module = MyModule {};
    module.merge(MyModule::into_rpc(my_module)).unwrap();
    module
}
