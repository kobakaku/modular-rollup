use rollup_interface::state::RollupAddress;

pub struct Address {
    addr: [u8; 32],
}

impl RollupAddress for Address {}
