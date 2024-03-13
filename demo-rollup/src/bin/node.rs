use tracing::info;

use demo_rollup::mock_rollup::MockRollup;
use sov_modules_rollup_blueprint::{Rollup, RollupBlueprint};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();
    let rollup = new_rollup().await?;

    info!(rollup.runner.start_height);

    rollup.run()
}

async fn new_rollup() -> Result<Rollup, anyhow::Error> {
    let mock_rollup = MockRollup {};
    mock_rollup.create_new_rollup().await
}
