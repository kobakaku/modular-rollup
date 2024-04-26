use demo_rollup::mock_rollup::MockRollup;
use sov_modules_rollup_blueprint::WalletBlueprint;

fn main() -> anyhow::Result<()> {
    MockRollup::run_wallet()
}
