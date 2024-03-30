mod runtime_rpc;
mod wallet;

pub use runtime_rpc::register_rpc;

use async_trait::async_trait;

use sov_db::ledger_db::LedgerDB;
use sov_stf_runner::{InitVariant, RollupConfig, StateTransitionRunner};

#[async_trait]
pub trait RollupBlueprint: Sized + Send + Sync {
    /// Creates instance of a LedgerDB.
    fn create_ledger_db(&self, rollup_config: &RollupConfig) -> LedgerDB {
        LedgerDB::open_ledger_db(&rollup_config.storage.path).expect("Ledger DB failed to open")
    }

    /// Creates a new Rollup
    async fn create_new_rollup(&self, rollup_config: RollupConfig) -> anyhow::Result<Rollup> {
        let ledger_db = self.create_ledger_db(&rollup_config);
        // TODO: はじめてRollupを起動したかどうかを判定する
        let init_valiant = InitVariant::Genesis {
            block_header: "todo".to_string(),
        };
        let runner = StateTransitionRunner::new(rollup_config.runner, ledger_db, init_valiant)?;
        let rpc_methods = self.create_rpc_methods()?;
        Ok(Rollup {
            runner,
            rpc_methods,
        })
    }

    /// Creates RPC methods for the rollup
    fn create_rpc_methods(&self) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error>;
}

/// Dependencies needed to run the rollup.
pub struct Rollup {
    /// The State Transition Runner.
    pub runner: StateTransitionRunner,
    /// Rpc methods for the rollup.
    pub rpc_methods: jsonrpsee::RpcModule<()>,
}

impl Rollup {
    /// Runs the rollup.
    pub async fn run(self) -> Result<(), anyhow::Error> {
        self.run_and_report_rpc_port().await
    }

    /// Runs the rollup. Reports rpc port to the caller using the provided channel.
    async fn run_and_report_rpc_port(self) -> Result<(), anyhow::Error> {
        let runner = self.runner;
        runner.start_rpc_server(self.rpc_methods).await;
        runner.run_in_progress()?;
        Ok(())
    }
}
