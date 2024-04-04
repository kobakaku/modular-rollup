use rollup_interface::state::storage::HierarchicalStorageManager;
use sov_state::ProverStorage;

pub struct ProverStorageManager {}
impl HierarchicalStorageManager for ProverStorageManager {
    type NativeStorage = ProverStorage;

    fn create_storage_on(&self) -> anyhow::Result<Self::NativeStorage> {
        Ok(ProverStorage {})
    }

    fn create_finalized_storage(&self) -> anyhow::Result<Self::NativeStorage> {
        todo!()
    }

    fn save_change_set(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn finalize(&self) -> anyhow::Result<()> {
        todo!()
    }
}
