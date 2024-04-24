use rollup_interface::state::da::{DaSpec, DaVerifier};

use crate::types::{CelestiaBlob, CelestiaBlockHeader, CelestiaHash};

pub struct CelestiaDaSpec;

impl DaSpec for CelestiaDaSpec {
    type BlockHash = CelestiaHash;

    type BlockHeader = CelestiaBlockHeader;

    type BlobTransaction = CelestiaBlob;

    type Address = String;
}

pub struct CelestiaDaVerifier;

impl DaVerifier for CelestiaDaVerifier {
    type Spec = CelestiaDaSpec;

    fn new() -> Self {
        todo!()
    }

    fn verify() -> anyhow::Result<()> {
        todo!()
    }
}
