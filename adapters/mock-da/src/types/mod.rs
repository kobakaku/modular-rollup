// use std::fmt::{write, Display};

use rollup_interface::services::da::SlotData;

pub struct MockBlock {
    pub header: MockBlockHeader,
    pub blob: Vec<MockBlob>,
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

#[derive(Debug, Clone)]
pub struct MockHash([u8; 32]);

impl core::fmt::Display for MockHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

#[derive(Debug, Clone)]
pub struct MockBlockHeader {
    prev_hash: MockHash,
    hash: MockHash,
    height: u64,
}

// impl Display for MockBlockHeader {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{{ height: {}, prev_hash: {}, next_hash:{} }}",
//             self.height, self.prev_hash, self.hash
//         )
//     }
// }

pub struct MockBlob {}
