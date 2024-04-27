use clap::Parser;
use sov_cli::{
    wallet_state::WalletState,
    workflows::{rpc::RpcWorkFlows, transaction::TransactionWorkFlows},
};

use crate::RollupBlueprint;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    workflow: Workflows,
}

#[derive(clap::Subcommand)]
enum Workflows {
    #[clap(subcommand)]
    Transaction(TransactionWorkFlows),
    #[clap(subcommand)]
    Rpc(RpcWorkFlows),
}

const WALLET_STATE_DIR: &str = "wallet_state.json";

pub trait WalletBlueprint: RollupBlueprint
where
    <Self as RollupBlueprint>::NativeContext: serde::de::DeserializeOwned,
{
    fn run_wallet() -> anyhow::Result<()> {
        let wallet_state =
            WalletState::<<Self as RollupBlueprint>::NativeContext>::load(WALLET_STATE_DIR)?;

        println!("current wallet state:{:?}", wallet_state);

        let invocation = Cli::parse();

        match invocation.workflow {
            Workflows::Transaction(_) => todo!(),
            Workflows::Rpc(_) => todo!(),
        }
    }
}
