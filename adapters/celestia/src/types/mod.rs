// use std::fmt::{write, Display};

use std::fmt::Debug;

use rollup_interface::{services::da::SlotData, state::da::BlockHeaderTrait};
use serde::{Deserialize, Serialize};

mod address;

#[derive(Clone)]
pub struct CelestiaBlock {
    pub header: CelestiaBlockHeader,
}

impl CelestiaBlock {
    pub fn new(header: CelestiaBlockHeader) -> Self {
        Self { header }
    }
}

impl SlotData for CelestiaBlock {
    type BlockHeader = CelestiaBlockHeader;

    fn hash(&self) -> [u8; 32] {
        self.header.hash
    }

    fn header(&self) -> &Self::BlockHeader {
        &self.header
    }
}

// #[derive(Clone, Copy, Default)]
// pub struct CelestiaHash(pub [u8; 32]);

// impl Debug for CelestiaHash {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "0x{}", hex::encode(self.0))
//     }
// }

// impl From<CelestiaHash> for [u8; 32] {
//     fn from(value: CelestiaHash) -> Self {
//         value.0
//     }
// }

// 正しいHeader情報を定義する
#[derive(Debug, Clone, Default)]
pub struct CelestiaBlockHeader {
    pub hash: [u8; 32],
    pub height: u64,
}

impl CelestiaBlockHeader {
    pub fn new(hash: [u8; 32], height: u64) -> Self {
        Self { hash, height }
    }
}

impl From<celestia_types::ExtendedHeader> for CelestiaBlockHeader {
    fn from(_extended_header: celestia_types::ExtendedHeader) -> Self {
        // TODO: 正しい値を返す
        CelestiaBlockHeader::new([0; 32], 0)
    }
}

impl BlockHeaderTrait for CelestiaBlockHeader {
    type Hash = [u8; 32];

    fn prev_hash(&self) -> Self::Hash {
        // TODO: 正しい値を取得する
        [0; 32]
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

impl CelestiaBlob {}

#[derive(Serialize, Deserialize)]
pub struct CelestiaDaConfig {
    /// The JWT used to authenticate with the Celestia RPC server
    pub rpc_auth_token: String,
    /// The address of the Celestia RPC server
    pub rpc_address: String,
}
