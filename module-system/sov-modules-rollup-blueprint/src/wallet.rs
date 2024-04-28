use clap::Parser;
use sov_cli::{
    wallet_state::WalletState,
    workflows::{rpc::RpcWorkFlows, transaction::TransactionWorkFlows},
};
use sov_modules_core::DispatchCall;

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

pub trait WalletBlueprint: RollupBlueprint {
    fn run_wallet() -> anyhow::Result<()>
    where
        <Self as RollupBlueprint>::NativeContext: serde::Serialize + serde::de::DeserializeOwned,
        <<Self as RollupBlueprint>::NativeRuntime as DispatchCall>::Decodable:
            serde::Serialize + serde::de::DeserializeOwned,
    {
        let mut wallet_state = WalletState::<
            <Self as RollupBlueprint>::NativeContext,
            <<Self as RollupBlueprint>::NativeRuntime as DispatchCall>::Decodable,
        >::read(WALLET_STATE_DIR)?;

        let cli = Cli::parse();

        match cli.workflow {
            Workflows::Transaction(inner) => inner.run(&mut wallet_state)?,
            Workflows::Rpc(_) => todo!(),
        }

        wallet_state.write(WALLET_STATE_DIR)
    }
}
