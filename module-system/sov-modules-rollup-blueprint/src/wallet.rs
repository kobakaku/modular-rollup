use crate::RollupBlueprint;

pub trait WalletBlueprint: RollupBlueprint {
    fn run_wallet() -> anyhow::Result<()> {
        todo!()
    }
}
