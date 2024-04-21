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

/// A block header, typically used in the context of an underlying DA blockchain.
pub trait BlockHeaderTrait {
    /// Each block header must have a unique canonical hash.
    type Hash;

    /// Each block header must contain the hash of the previous block.
    fn prev_hash(&self) -> Self::Hash;

    /// Hash the type to get the digest.
    fn hash(&self) -> Self::Hash;

    /// The current header height.
    fn height(&self) -> u64;
}
