pub trait HierarchicalStorageManager {
    type NativeStorage;

    /// Creates storage based on given Da block header.
    fn create_storage_on(&self) -> anyhow::Result<Self::NativeStorage>;

    /// Snapshots that points directly to finalized storage.
    /// Won't be saved if somehow 'saved'
    fn create_finalized_storage(&self) -> anyhow::Result<Self::NativeStorage>;

    /// Adds [`Self::NativeChangeSet`] to the storage
    fn save_change_set(&self) -> anyhow::Result<()>;

    /// Finalizes snapshot on given block header
    fn finalize(&self) -> anyhow::Result<()>;
}
