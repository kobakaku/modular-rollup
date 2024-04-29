use std::fmt::Debug;

pub mod da;
pub mod stf;
pub mod storage;

/// A maker trait for general addresses
pub trait BasicAddress:
    Debug
    + Send
    + Sync
    + serde::Serialize
    + serde::de::DeserializeOwned
    + borsh::BorshSerialize
    + borsh::de::BorshDeserialize
{
}

/// An address used inside rollup
pub trait RollupAddress: BasicAddress {}
