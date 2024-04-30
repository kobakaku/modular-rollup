use crate::wallet_state::WalletState;
use serde::{de::DeserializeOwned, Serialize};
use sov_modules_core::Context;

/// Generate, Clean and List the key
#[derive(clap::Subcommand)]
pub enum KeyWorkFlows {
    /// Generate a new key pair
    Generate,
    /// Clean a key from the wallet
    Clean,
    /// List the keys in this wallet
    List,
}

impl KeyWorkFlows {
    /// Run the transaction workflow
    pub fn run<C: Context, Tx: Serialize + DeserializeOwned>(
        self,
        wallet_state: &mut WalletState<C, Tx>,
    ) -> anyhow::Result<()> {
        match self {
            KeyWorkFlows::Generate => generate_key(),
            KeyWorkFlows::Clean => Ok(()),
            KeyWorkFlows::List => {
                println!("{}", serde_json::to_string_pretty(&wallet_state.addresses)?);
                Ok(())
            }
        }
    }
}

/// Generate a new key
fn generate_key() -> anyhow::Result<()> {
    todo!()
}
