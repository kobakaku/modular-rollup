use demo_rollup::{celestia_rollup::CelestiaRollup, initialize_logging};
use sov_modules_rollup_blueprint::WalletBlueprint;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logging();
    CelestiaRollup::run_wallet().await
}
