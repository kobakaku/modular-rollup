use demo_rollup::{initialize_logging, mock_rollup::MockRollup};
use sov_modules_rollup_blueprint::WalletBlueprint;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logging();
    MockRollup::run_wallet().await
}
