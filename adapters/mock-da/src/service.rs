use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use rollup_interface::services::da::DaService;
use rollup_interface::services::da::SlotData;

use crate::{
    types::MockBlock,
    verifier::{MockDaSpec, MockDaVerifier},
};

pub struct MockDaService {
    sender_address: [u8; 32],
    blocks: Arc<Mutex<Vec<MockBlock>>>,
}

impl MockDaService {
    pub fn new(sender_address: [u8; 32]) -> Self {
        Self {
            sender_address,
            blocks: Arc::new(Default::default()),
        }
    }
}

impl DaService for MockDaService {
    type Spec = MockDaSpec;

    type Verifier = MockDaVerifier;

    type Block = MockBlock;

    fn get_block_at(&self, height: u64) -> anyhow::Result<Self::Block> {
        todo!()
    }

    fn get_last_finalized_block_header(
        &self,
    ) -> anyhow::Result<<Self::Spec as rollup_interface::state::da::DaSpec>::BlockHeader> {
        let mut blocks = self
            .blocks
            .lock()
            .map_err(|e| anyhow!("Failed to lock blocks: {}", e))?;
        Ok(blocks[0].header().clone())
    }

    fn send_transaction(&self, blob: &[u8]) -> anyhow::Result<()> {
        todo!()
    }
}
