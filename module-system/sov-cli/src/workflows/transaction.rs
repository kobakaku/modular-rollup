use crate::wallet_state::WalletState;
use serde::{de::DeserializeOwned, Serialize};
use sov_modules_core::Context;

/// Clean and List the transactions
#[derive(clap::Subcommand)]
pub enum TransactionWorkFlows {
    /// Clean the currenct batch of transactions
    Clean,
    /// List the currenct batch of transactions
    List,
}

impl TransactionWorkFlows {
    /// Run the transaction workflow
    pub fn run<C: Context, Tx: Serialize + DeserializeOwned>(
        self,
        wallet_state: &mut WalletState<C, Tx>,
    ) -> anyhow::Result<()> {
        match self {
            TransactionWorkFlows::Clean => {
                wallet_state.unsent_transactions.clear();
                Ok(())
            }
            TransactionWorkFlows::List => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&wallet_state.unsent_transactions)?
                );
                Ok(())
            }
        }
    }
}
