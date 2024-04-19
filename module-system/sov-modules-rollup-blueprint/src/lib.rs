mod runtime_rpc;
mod wallet;

pub use runtime_rpc::register_rpc;

use async_trait::async_trait;

use rollup_interface::{services::da::DaService, state::storage::HierarchicalStorageManager};
use sov_db::ledger_db::LedgerDB;
use sov_modules_core::{Context, Spec, Storage};
use sov_modules_stf_blueprint::StfBlueprint;
use sov_stf_runner::{InitVariant, RollupConfig, StateTransitionRunner};

#[async_trait]
pub trait RollupBlueprint: Sized + Send + Sync {
    type StorageManager: HierarchicalStorageManager<
        NativeStorage = <Self::NativeContext as Spec>::Storage,
    >;
    type NativeContext: Context;
    type DaService: DaService;

    /// Creates instance of a LedgerDB.
    fn create_ledger_db(&self, rollup_config: &RollupConfig) -> LedgerDB {
        LedgerDB::open_ledger_db(&rollup_config.storage.path).expect("Ledger DB failed to open")
    }

    fn create_da_service(&self, rollup_config: &RollupConfig) -> Self::DaService;

    fn create_storage_manager(
        &self,
        rollup_config: &RollupConfig,
    ) -> anyhow::Result<Self::StorageManager>;

    /// Creates a new Rollup
    async fn create_new_rollup(&self, rollup_config: RollupConfig) -> anyhow::Result<Rollup<Self>> {
        let da_service = self.create_da_service(&rollup_config);

        let last_finalized_block_header = da_service.get_last_finalized_block_header()?;

        let ledger_db = self.create_ledger_db(&rollup_config);

        let storage_manager = self.create_storage_manager(&rollup_config)?;
        let prover_storage = storage_manager.create_finalized_storage()?;

        let prev_root = ledger_db
            .get_head_slot()?
            .map(|(_, _)| prover_storage.get_root_hash())
            .transpose()?;

        let init_valiant = match prev_root {
            Some(state_root) => InitVariant::Initialized(state_root),
            None => InitVariant::Genesis {
                block_header: last_finalized_block_header,
                genesis_params: (),
            },
        };

        let stf = StfBlueprint::new();

        let rpc_methods = self.create_rpc_methods(&prover_storage, &da_service)?;

        let runner = StateTransitionRunner::new(
            rollup_config.runner,
            ledger_db,
            init_valiant,
            stf,
            storage_manager,
            da_service,
        )?;

        Ok(Rollup {
            runner,
            rpc_methods,
        })
    }

    /// Creates RPC methods for the rollup
    fn create_rpc_methods(
        &self,
        storage: &<Self::NativeContext as Spec>::Storage,
        da_service: &Self::DaService,
    ) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error>;
}

/// Dependencies needed to run the rollup.
pub struct Rollup<S: RollupBlueprint> {
    /// The State Transition Runner.
    pub runner:
        StateTransitionRunner<StfBlueprint<S::NativeContext>, S::StorageManager, S::DaService>,
    /// Rpc methods for the rollup.
    pub rpc_methods: jsonrpsee::RpcModule<()>,
}

impl<S: RollupBlueprint> Rollup<S> {
    /// Runs the rollup.
    pub async fn run(self) -> Result<(), anyhow::Error> {
        self.run_and_report_rpc_port().await
    }

    /// Runs the rollup. Reports rpc port to the caller using the provided channel.
    async fn run_and_report_rpc_port(self) -> Result<(), anyhow::Error> {
        let runner = self.runner;
        runner.start_rpc_server(self.rpc_methods).await;
        runner.run_in_progress().await?;
        Ok(())
    }
}
