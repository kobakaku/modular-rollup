use async_trait::async_trait;

use sov_modules_rollup_blueprint::{register_rpc, RollupBlueprint};
use sov_stf_runner::RollupConfig;

pub struct MockRollup {}

#[async_trait]
impl RollupBlueprint for MockRollup {
    type StorageManage;
    type NativeContext;

    fn create_storage_manager(
        &self,
        rollup_config: &RollupConfig,
    ) -> anyhow::Result<Self::StorageManager> {
    }

    fn create_rpc_methods(&self) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
        register_rpc()
    }
}
