use rollup_interface::state::{BasicAddress, RollupAddress};

#[derive(Debug)]
pub struct Address {
    addr: [u8; 32],
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
