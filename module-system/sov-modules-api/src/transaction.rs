use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sov_modules_core::Context;

#[derive(Clone, Debug, Deserialize, borsh::BorshDeserialize)]
pub struct Transaction<C: Context> {
    signature: C::Signature,
    pub_key: C::PublicKey,
    chain_id: u64,
    nonce: u64,
}

/// A unsent transaction with the required data to be submitted to the DA layer
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "Tx: Serialize + DeserializeOwned")]
pub struct UnsentTransaction<Tx> {
    /// The underlying transaction
    pub tx: Tx,
    /// The ID of the target chain
    pub chain_id: u64,
}

impl<C: Context> Transaction<C> {
    pub fn verify(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
