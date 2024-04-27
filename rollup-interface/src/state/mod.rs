pub mod da;
pub mod stf;
pub mod storage;

/// A maker trait for general addresses
pub trait BasicAddress {}

/// An address used inside rollup
pub trait RollupAddress: BasicAddress {}
