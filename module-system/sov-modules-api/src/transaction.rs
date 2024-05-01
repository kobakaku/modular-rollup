use borsh::{BorshDeserialize, BorshSerialize};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use sov_modules_core::Context;
use sov_modules_core::PrivateKey;

const EXTEND_MESSAGE_LEN: usize = 2 * core::mem::size_of::<u64>();

#[derive(Clone, Debug, Deserialize, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct Transaction<C: Context> {
    pub signature: C::Signature,
    pub pub_key: C::PublicKey,
    pub runtime_msg: Vec<u8>,
    pub chain_id: u64,
    pub nonce: u64,
}

impl<C: Context> Transaction<C> {
    /// New transaction.
    pub fn new(
        signature: C::Signature,
        pub_key: C::PublicKey,
        runtime_msg: Vec<u8>,
        chain_id: u64,
        nonce: u64,
    ) -> Self {
        Self {
            signature,
            pub_key,
            runtime_msg,
            chain_id,
            nonce,
        }
    }
    /// New signed transaction.
    pub fn new_signed_tx(
        priv_key: &C::PrivateKey,
        mut message: Vec<u8>,
        chain_id: u64,
        nonce: u64,
    ) -> Self {
        // Since we own the message already, try to add the serialized nonce in-place.
        // This lets us avoid a copy if the message vec has at least 8 bytes of extra capacity.
        let len = message.len();

        // resize to avoid potential multiple realloc
        message.resize_with(len + EXTEND_MESSAGE_LEN, Default::default);

        // Not extend_from_slice
        message[len..len + 8].copy_from_slice(&chain_id.to_be_bytes());
        message[len + 8..len + 16].copy_from_slice(&nonce.to_be_bytes());

        let pub_key = priv_key.pub_key();
        let signature = priv_key.sign(&message);

        // Don't forget to truncate the message back to its original length!
        // message.truncate(len);

        Self {
            signature,
            pub_key,
            runtime_msg: message,
            chain_id,
            nonce,
        }
    }
}

/// A unsent transaction with the required data to be submitted to the DA layer
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
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
