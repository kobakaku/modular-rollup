use async_trait::async_trait;

use sov_mock::MockDaService;
use sov_modules_api::default_context::DefaultContext;
use sov_modules_core::Spec;
use sov_modules_rollup_blueprint::{register_rpc, RollupBlueprint, WalletBlueprint};
use sov_prover_storage_manager::ProverStorageManager;
use sov_stf_runner::RollupConfig;

pub struct MockRollup {}

#[async_trait]
impl RollupBlueprint for MockRollup {
    type StorageManager = ProverStorageManager;
    type NativeContext = DefaultContext;
    type DaService = MockDaService;

    fn create_storage_manager(
        &self,
        _rollup_config: &RollupConfig,
    ) -> anyhow::Result<Self::StorageManager> {
        Ok(Self::StorageManager::new())
    }

    fn create_da_service(&self, rollup_config: &RollupConfig) -> Self::DaService {
        Self::DaService::new(rollup_config.da.sender_address)
    }

    fn create_rpc_methods(
        &self,
        storage: &<Self::NativeContext as Spec>::Storage,
        da_service: &Self::DaService,
    ) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
        register_rpc::<Self::NativeContext, Self::DaService>(storage, da_service)
    }
}

impl WalletBlueprint for MockRollup {}
