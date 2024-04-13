/// An interface for storing and retrieving values in the storage.
pub trait Storage: Clone + Send + Sync {
    /// A cryptographic commitment to the content of this storage
    type Root: AsRef<[u8]>;

    /// Get the root hash of the tree.
    fn get_root_hash(&self) -> anyhow::Result<Self::Root>;
}
