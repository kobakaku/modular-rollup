use jsonrpsee::server::RpcModule;

use my_module::get_my_module_rpc_method;

pub fn register_rpc() -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
    let mut module = RpcModule::new(());
    let my_module_rpc_method = get_my_module_rpc_method();
    module.merge(my_module_rpc_method).unwrap();
    Ok(module)
}
