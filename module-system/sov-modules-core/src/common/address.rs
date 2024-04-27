use rollup_interface::state::{BasicAddress, RollupAddress};

#[derive(Debug)]
pub struct Address {
    addr: [u8; 32],
}

impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&self.addr, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let addr = <[u8; 32] as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Address { addr })
    }
}

impl BasicAddress for Address {}
impl RollupAddress for Address {}
