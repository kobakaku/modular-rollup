use std::fmt::Debug;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{de::DeserializeOwned, Serialize};

/// PublicKey used in the Module System.
pub trait PublicKey:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + BorshSerialize + BorshDeserialize
{
}

/// PrivateKey used in the Module System.
pub trait PrivateKey:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + BorshDeserialize
{
    /// The public key associated with the key pair.
    type PublicKey;

    /// The signature associated with the key pair.
    type Signature;

    /// Returns the public key associated with this private key.
    fn pub_key(&self) -> Self::PublicKey;

    /// Sign the provided message.
    fn sign(&self, msg: &[u8]) -> Self::Signature;
}

/// Signature used in the Module System.
pub trait Signature:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + BorshSerialize + BorshDeserialize
{
    /// The public key associated with the key pair of the signature;
    type PublicKey;

    fn verify(&self, pub_key: &Self::PublicKey) -> anyhow::Result<()>;
}
