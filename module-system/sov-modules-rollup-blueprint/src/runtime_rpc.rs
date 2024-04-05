use jsonrpsee::server::RpcModule;

use my_module::get_my_module_rpc_method;

pub fn register_rpc() -> anyhow::Result<jsonrpsee::RpcModule<()>> {
    let mut module = RpcModule::new(());

    // Module RPC.
    let my_module_rpc_method = get_my_module_rpc_method();
    module.merge(my_module_rpc_method).unwrap();

    // Sequencer RPC.
    let sequencer_rpc_method = sov_sequencer::rpc::get_sequencer_rpc();
    module.merge(sequencer_rpc_method).unwrap();

    Ok(module)
}
