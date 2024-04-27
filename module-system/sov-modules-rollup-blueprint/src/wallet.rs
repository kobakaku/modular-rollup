use clap::Parser;
use sov_cli::{workflows::rpc::RpcWorkFlows, workflows::transaction::TransactionWorkFlows};

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

pub trait WalletBlueprint: RollupBlueprint {
    fn run_wallet() -> anyhow::Result<()> {
        // TODO: walletの値をjsonの形で取得する

        let invocation = Cli::parse();

        match invocation.workflow {
            Workflows::Transaction(_) => todo!(),
            Workflows::Rpc(_) => todo!(),
        }
    }
}
