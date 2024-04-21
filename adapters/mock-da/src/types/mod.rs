// use std::fmt::{write, Display};

use std::fmt::{Debug, Formatter};

use rollup_interface::{services::da::SlotData, state::da::BlockHeaderTrait};

pub struct MockBlock {
    pub header: MockBlockHeader,
    pub blobs: Vec<MockBlob>,
}

impl SlotData for MockBlock {
    type BlockHeader = MockBlockHeader;

    fn hash(&self) -> [u8; 32] {
        self.header.hash.0
    }

    fn header(&self) -> &Self::BlockHeader {
        &self.header
    }
}

#[derive(Clone, Copy)]
pub struct MockHash(pub [u8; 32]);

impl Debug for MockHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

#[derive(Debug, Clone)]
pub struct MockBlockHeader {
    pub prev_hash: MockHash,
    pub hash: MockHash,
    pub height: u64,
}

impl BlockHeaderTrait for MockBlockHeader {
    type Hash = MockHash;

    fn prev_hash(&self) -> Self::Hash {
        self.prev_hash
    }

    fn hash(&self) -> Self::Hash {
        self.hash
    }

    fn height(&self) -> u64 {
        self.height
    }
}

pub struct MockBlob {}

impl MockBlob {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
