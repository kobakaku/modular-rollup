mod runtime_rpc;
mod wallet;

use async_trait::async_trait;

#[async_trait]
pub trait RollupBlueprint: Sized + Send + Sync {
    async fn create_new_rollup(self) -> anyhow::Result<Rollup> {
        let runner = StateTransitionRunner::new();
        Ok(Rollup { runner })
    }
}

/// Dependencies needed to run the rollup.
pub struct Rollup {
    pub runner: StateTransitionRunner,
}

impl Rollup {
    pub fn run(self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

pub struct StateTransitionRunner {
    pub start_height: u64,
}

impl StateTransitionRunner {
    pub fn new() -> Self {
        StateTransitionRunner { start_height: 1 }
    }
}
