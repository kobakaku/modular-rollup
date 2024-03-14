mod runtime_rpc;
mod wallet;

use async_trait::async_trait;

use sov_stf_runner::{RollupConfig, StateTransitionRunner};

#[async_trait]
pub trait RollupBlueprint: Sized + Send + Sync {
    async fn create_new_rollup(self, rollup_config: RollupConfig) -> anyhow::Result<Rollup> {
        let runner = StateTransitionRunner::new(rollup_config.runner)?;
        Ok(Rollup { runner })
    }
}

/// Dependencies needed to run the rollup.
pub struct Rollup {
    pub runner: StateTransitionRunner,
}

impl Rollup {
    /// Runs the rollup.
    pub async fn run(self) -> Result<(), anyhow::Error> {
        self.run_and_report_rpc_port().await
    }

    /// Runs the rollup. Reports rpc port to the caller using the provided channel.
    async fn run_and_report_rpc_port(self) -> Result<(), anyhow::Error> {
        let mut runner = self.runner;
        runner.start_rpc_server().await;
        runner.run_in_progress()?;
        Ok(())
    }
}
