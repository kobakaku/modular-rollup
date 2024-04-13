use sov_modules_core::Storage;

#[derive(Clone)]
pub struct ProverStorage {}

impl Storage for ProverStorage {
    type Root = jmt::RootHash;

    fn get_root_hash(&self) -> anyhow::Result<Self::Root> {
        // TODO: 正しい値を返す
        Ok(jmt::RootHash([01; 32]))
    }
}
