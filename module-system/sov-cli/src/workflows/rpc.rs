use anyhow::Context as _;
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use serde::{de::DeserializeOwned, Serialize};
use sov_modules_core::Context;

use crate::wallet_state::WalletState;

/// Query the current state of the rollup and send transactions
#[derive(clap::Subcommand)]
pub enum RpcWorkFlows {
    /// Sign all transactions from the current batch and submit them to the rollup.
    /// Nonces will be set automatically.
    SubmitBatch,
}

impl RpcWorkFlows {
    pub async fn run<
        C: Context,
        Tx: Serialize + DeserializeOwned + BorshSerialize + BorshDeserialize,
    >(
        self,
        wallet_state: &WalletState<C, Tx>,
    ) -> anyhow::Result<()> {
        match self {
            RpcWorkFlows::SubmitBatch => {
                const RPC_URL: &str = "http://127.0.0.1:8000";

                // TODO: 署名をして、dataにする
                let data = to_vec(&wallet_state.unsent_transactions[0])?;

                let client = HttpClientBuilder::default().build(RPC_URL)?;
                let _res: String = client
                    .request("sequencer_publishBatch", vec![data])
                    .await
                    .context("Unable to publish batch")?;

                Ok(())
            }
        }
    }
}
