use std::fmt::Debug;

use celestia_types::DataAvailabilityHeader;
use rollup_interface::{services::da::SlotData, state::da::BlockHeaderTrait};
use serde::{Deserialize, Serialize};
use tendermint::block::Header;

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
        match self.header.hash() {
            tendermint::Hash::Sha256(h) => h,
            tendermint::Hash::None => {
                unreachable!("tendermint::Hash::None should not be reachable")
            }
        }
    }

    fn header(&self) -> &Self::BlockHeader {
        &self.header
    }
}

// 正しいHeader情報を定義する
#[derive(Debug, Clone)]
pub struct CelestiaBlockHeader {
    pub dah: DataAvailabilityHeader,
    pub header: Header,
    pub prev_hash: Option<[u8; 32]>,
}

impl CelestiaBlockHeader {
    pub fn new(dah: DataAvailabilityHeader, header: Header) -> Self {
        Self {
            dah,
            header,
            prev_hash: None,
        }
    }
}

impl From<celestia_types::ExtendedHeader> for CelestiaBlockHeader {
    fn from(extended_header: celestia_types::ExtendedHeader) -> Self {
        CelestiaBlockHeader::new(extended_header.dah, extended_header.header)
    }
}

impl BlockHeaderTrait for CelestiaBlockHeader {
    type Hash = tendermint::Hash;

    fn prev_hash(&self) -> Self::Hash {
        // TODO: 正しい値をハッシュを計算する
        self.header.hash()
    }

    fn hash(&self) -> Self::Hash {
        self.header.hash()
    }

    fn height(&self) -> u64 {
        self.header.height.value()
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
