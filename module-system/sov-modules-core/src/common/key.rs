use std::fmt::Debug;

use borsh::BorshDeserialize;
use serde::{de::DeserializeOwned, Serialize};

/// PublicKey used in the Module System.
pub trait PublicKey:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + BorshDeserialize
{
}

/// PrivateKey used in the Module System.
pub trait PrivateKey: Clone + Debug + Send + Sync + Serialize + BorshDeserialize {}

/// Signature used in the Module System.
pub trait Signature:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + BorshDeserialize
{
    /// The public key associated with the key pair of the signature;
    type PublicKey;

    fn verify(&self, pub_key: &Self::PublicKey) -> anyhow::Result<()>;
}
