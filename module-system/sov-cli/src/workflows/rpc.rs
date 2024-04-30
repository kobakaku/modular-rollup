use std::{mem, path::Path};

use anyhow::Context as _;
use borsh::{BorshDeserialize, BorshSerialize};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use serde::{de::DeserializeOwned, Serialize};
use sov_modules_api::transaction::Transaction;
use sov_modules_core::Context;

use crate::{wallet_state::WalletState, workflows::key::load_priv_key};

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
        wallet_state: &mut WalletState<C, Tx>,
    ) -> anyhow::Result<()> {
        match self {
            RpcWorkFlows::SubmitBatch => {
                const RPC_URL: &str = "http://127.0.0.1:8000";

                let address_entry = &wallet_state.addresses[0].clone();
                let priv_key = load_priv_key::<C>(&address_entry.path)?;

                let data = mem::take(&mut wallet_state.unsent_transactions)
                    .into_iter()
                    .enumerate()
                    .map(|(offset, tx)| {
                        Transaction::<C>::new_signed_tx(
                            &priv_key,
                            tx.try_to_vec().unwrap(),
                            tx.chain_id,
                            offset as u64,
                        )
                        .try_to_vec()
                        .unwrap()
                    })
                    .collect::<Vec<_>>();

                let client = HttpClientBuilder::default().build(RPC_URL)?;
                let _res: String = client
                    .request("sequencer_publishBatch", data)
                    .await
                    .context("Unable to publish batch")?;

                Ok(())
            }
        }
    }
}
