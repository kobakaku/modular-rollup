use std::fmt::Debug;

pub mod da;
pub mod stf;
pub mod storage;

/// A maker trait for general addresses
pub trait BasicAddress: Debug + serde::de::DeserializeOwned {}

/// An address used inside rollup
pub trait RollupAddress: BasicAddress {}
