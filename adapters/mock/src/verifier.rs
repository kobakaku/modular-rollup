use rollup_interface::state::da::{DaSpec, DaVerifier};

use crate::types::{MockBlob, MockBlockHeader, MockHash};

pub struct MockDaSpec;

impl DaSpec for MockDaSpec {
    type BlockHash = MockHash;

    type BlockHeader = MockBlockHeader;

    type BlobTransaction = MockBlob;

    type Address = String;
}

pub struct MockDaVerifier;

impl DaVerifier for MockDaVerifier {
    type Spec = MockDaSpec;

    fn new() -> Self {
        todo!()
    }

    fn verify() -> anyhow::Result<()> {
        todo!()
    }
}
