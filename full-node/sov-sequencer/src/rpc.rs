use jsonrpsee::{types::ErrorObjectOwned, RpcModule};
use rollup_interface::services::{batch_builder::BatchBuilder, da::DaService};
use sov_modules_api::utils::to_jsonrpsee_error_object;

use crate::Sequencer;

const SEQUENCER_RPC_ERROR: &str = "SEQUENCER_RPC_ERROR";

fn register_txs_rpc_methods<B, D>(
    rpc: &mut RpcModule<Sequencer<B, D>>,
) -> Result<(), jsonrpsee::core::Error>
where
    B: BatchBuilder + Send + Sync + 'static,
    D: DaService + Send,
{
    rpc.register_async_method("sequencer_publishBatch", |params, sequencer| async move {
        let mut params_iter = params.sequence();
        while let Some(tx) = params_iter.optional_next::<Vec<u8>>()? {
            sequencer
                .accept_tx(tx)
                .await
                .map_err(|e| to_jsonrpsee_error_object(SEQUENCER_RPC_ERROR, e))?;
        }
        let blob_len = sequencer
            .submit_batch()
            .await
            .map_err(|e| to_jsonrpsee_error_object(SEQUENCER_RPC_ERROR, e))?;

        Ok::<String, ErrorObjectOwned>(format!("Submitted {} transactions", blob_len))
    })?;

    Ok(())
}

pub fn get_sequencer_rpc<B: BatchBuilder + Send + Sync + 'static, D: DaService>(
    batch_builder: B,
    da_service: D,
) -> RpcModule<Sequencer<B, D>> {
    let sequencer = Sequencer::new(batch_builder, da_service);
    let mut module = RpcModule::new(sequencer);
    register_txs_rpc_methods(&mut module).expect("Failed to register sequencer RPC methods");
    module
}
