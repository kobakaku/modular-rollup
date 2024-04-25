use jsonrpsee::server::RpcModule;

use rollup_interface::services::da::DaService;
use sov_bank_module::get_bank_module_rpc_method;
use sov_modules_core::Context;
use sov_sequencer::batch_builder::FiFoBatchBuilder;

pub fn register_rpc<C: Context, D>(
    storage: &C::Storage,
    da_service: &D,
) -> anyhow::Result<jsonrpsee::RpcModule<()>>
where
    D: DaService + Clone,
{
    let mut module = RpcModule::new(());

    // Module RPC.
    let bank_module_rpc_method = get_bank_module_rpc_method();
    module.merge(bank_module_rpc_method).unwrap();

    // Sequencer RPC.
    {
        // TODO: Cをいれたくない
        let batch_builder = FiFoBatchBuilder::<C>::new(storage.clone(), u32::MAX as usize);
        let sequencer_rpc_method =
            sov_sequencer::rpc::get_sequencer_rpc(batch_builder, da_service.clone());
        module.merge(sequencer_rpc_method).unwrap();
    }

    Ok(module)
}
