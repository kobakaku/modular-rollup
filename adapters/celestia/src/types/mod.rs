// use std::fmt::{write, Display};

use std::fmt::{Debug, Formatter};

use rollup_interface::{services::da::SlotData, state::da::BlockHeaderTrait};

#[derive(Clone)]
pub struct CelestiaBlock {
    pub header: CelestiaBlockHeader,
    pub blobs: Vec<CelestiaBlob>,
}

impl SlotData for CelestiaBlock {
    type BlockHeader = CelestiaBlockHeader;

    fn hash(&self) -> [u8; 32] {
        self.header.hash.0
    }

    fn header(&self) -> &Self::BlockHeader {
        &self.header
    }
}

#[derive(Clone, Copy)]
pub struct CelestiaHash(pub [u8; 32]);

impl Debug for CelestiaHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl From<CelestiaHash> for [u8; 32] {
    fn from(value: CelestiaHash) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct CelestiaBlockHeader {
    pub prev_hash: CelestiaHash,
    pub hash: CelestiaHash,
    pub height: u64,
}

impl BlockHeaderTrait for CelestiaBlockHeader {
    type Hash = CelestiaHash;

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

#[derive(Clone)]
pub struct CelestiaBlob {}

impl CelestiaBlob {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
