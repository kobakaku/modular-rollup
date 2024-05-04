use bech32::{FromBase32, ToBase32, Variant};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use rollup_interface::state::{BasicAddress, RollupAddress};

#[derive(Debug, Clone, PartialEq, Eq, Hash, BorshSerialize, BorshDeserialize)]
pub struct Address {
    addr: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressBech32(String);

impl From<[u8; 32]> for Address {
    fn from(addr: [u8; 32]) -> Self {
        Self { addr }
    }
}

impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serde::Serialize::serialize(&AddressBech32::from(self), serializer)
        } else {
            serde::Serialize::serialize(&self.addr, serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let bech32_addr: AddressBech32 = serde::Deserialize::deserialize(deserializer)?;
            Ok(Address::from(bech32_addr.to_byte_array()))
        } else {
            let addr: [u8; 32] = serde::Deserialize::deserialize(deserializer)?;
            Ok(Address { addr })
        }
    }
}

impl BasicAddress for Address {}
impl RollupAddress for Address {}

impl AddressBech32 {
    pub fn get_string(&self) -> &str {
        &self.0
    }

    fn to_byte_array(&self) -> [u8; 32] {
        let (_, vec, _) = split_address_bech32(self.get_string()).unwrap();
        if vec.len() != 32 {
            panic!("Invalid length {}, should be 32", vec.len())
        }

        let mut addr_bytes: [u8; 32] = [0u8; 32];
        addr_bytes.copy_from_slice(&vec);

        addr_bytes
    }
}

impl From<&Address> for AddressBech32 {
    fn from(value: &Address) -> Self {
        let string = to_address_bech32(&value.addr, HRP).unwrap();
        AddressBech32(string)
    }
}

/// Human Readable Part.
const HRP: &str = "sov";

fn to_address_bech32(addr: &[u8], hrp: &str) -> Result<String, bech32::Error> {
    let data = addr.to_base32();
    let bech32_addr = bech32::encode(hrp, data, bech32::Variant::Bech32)?;
    Ok(bech32_addr)
}

fn split_address_bech32(bech32_addr: &str) -> Result<(String, Vec<u8>, Variant), bech32::Error> {
    let (hrp, data, variant) = bech32::decode(bech32_addr).unwrap();
    let vec = Vec::<u8>::from_base32(&data)?;
    Ok((hrp, vec, variant))
}
