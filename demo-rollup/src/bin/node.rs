use anyhow::Context;
use tracing::info;

use demo_rollup::mock_rollup::MockRollup;
use sov_modules_rollup_blueprint::{Rollup, RollupBlueprint};
use sov_stf_runner::{from_toml_path, RollupConfig};

const CONFIG_PATH: &'static str = "./rollup_config.toml";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let rollup = new_rollup(CONFIG_PATH).await?;

    info!("Starting Mock Rollup with config: {}", CONFIG_PATH);

    rollup.run().await
}

async fn new_rollup(rollup_config_path: &str) -> Result<Rollup<MockRollup>, anyhow::Error> {
    let rollup_config: RollupConfig =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;
    let mock_rollup = MockRollup {};
    mock_rollup.create_new_rollup(rollup_config).await
}
