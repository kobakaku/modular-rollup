use std::{fmt::Debug, hash::Hash};

pub mod da;
pub mod stf;
pub mod storage;

/// A maker trait for general addresses
pub trait BasicAddress:
    Debug
    + Clone
    + Send
    + Sync
    + Eq
    + Hash
    + serde::Serialize
    + serde::de::DeserializeOwned
    + borsh::BorshSerialize
    + borsh::de::BorshDeserialize
{
}

/// An address used inside rollup
pub trait RollupAddress: BasicAddress + From<[u8; 32]> {}
