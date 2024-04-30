use sov_modules_core::{PrivateKey, PublicKey, Signature};

#[derive(
    Clone,
    Debug,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct DefaultPublicKey(String);

impl PublicKey for DefaultPublicKey {}

#[derive(Clone, Debug, borsh::BorshDeserialize, serde::Deserialize, serde::Serialize)]
pub struct DefaultPrivateKey(String);

impl PrivateKey for DefaultPrivateKey {
    type PublicKey = DefaultPublicKey;

    type Signature = DefaultSignature;

    fn pub_key(&self) -> Self::PublicKey {
        DefaultPublicKey("TODO".to_string())
    }

    fn sign(&self, _msg: &[u8]) -> Self::Signature {
        DefaultSignature("TODO".to_string())
    }
}

#[derive(
    Clone,
    Debug,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct DefaultSignature(String);

impl Signature for DefaultSignature {
    type PublicKey = DefaultPublicKey;

    fn verify(&self, _pub_key: &Self::PublicKey) -> anyhow::Result<()> {
        Ok(())
    }
}
