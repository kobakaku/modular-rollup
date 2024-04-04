use async_trait::async_trait;

use sov_modules_api::default_context::DefaultContext;
use sov_modules_rollup_blueprint::{register_rpc, RollupBlueprint};
use sov_prover_storage_manager::ProverStorageManager;
use sov_stf_runner::RollupConfig;

pub struct MockRollup {}

#[async_trait]
impl RollupBlueprint for MockRollup {
    type StorageManager = ProverStorageManager;
    type NativeContext = DefaultContext;

    fn create_storage_manager(
        &self,
        _rollup_config: &RollupConfig,
    ) -> anyhow::Result<Self::StorageManager> {
        Ok(Self::StorageManager::new())
    }

    fn create_rpc_methods(&self) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
        register_rpc()
    }
}
