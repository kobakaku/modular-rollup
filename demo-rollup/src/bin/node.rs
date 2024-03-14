use anyhow::Context;
use sov_stf_runner::{from_toml_path, RollupConfig};
use tracing::info;

use demo_rollup::mock_rollup::MockRollup;
use sov_modules_rollup_blueprint::{Rollup, RollupBlueprint};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();
    let rollup = new_rollup("./rollup_config.toml").await?;

    info!(
        "Starting Mock Rollup with start_height: {}, listen_address: {}",
        rollup.runner.start_height, rollup.runner.listen_address,
    );

    rollup.run().await
}

async fn new_rollup(rollup_config_path: &str) -> Result<Rollup, anyhow::Error> {
    let rollup_config: RollupConfig =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;
    let mock_rollup = MockRollup {};
    mock_rollup.create_new_rollup(rollup_config).await
}
