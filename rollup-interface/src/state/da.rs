use std::fmt::Debug;

/// A specification for the types used by the DA layer.
pub trait DaSpec {
    /// The hash of the DA layer block.
    type BlockHash;

    /// The block header type used by the DA layer.
    type BlockHeader: Debug;

    /// The transaction used by the DA layer.
    type BlobTransaction;

    type Address;
}

/// A `DaVerifier` implements the logic required to create a zk proof that some data has been processed.
pub trait DaVerifier {
    type Spec: DaSpec;

    fn new() -> Self;

    fn verify() -> anyhow::Result<()>;
}
