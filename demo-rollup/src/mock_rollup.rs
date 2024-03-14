use async_trait::async_trait;
use sov_modules_rollup_blueprint::{register_rpc, RollupBlueprint};

pub struct MockRollup {}

#[async_trait]
impl RollupBlueprint for MockRollup {
    fn create_rpc_methods(self) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
        register_rpc()
    }
}
