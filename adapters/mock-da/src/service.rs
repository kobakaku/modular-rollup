use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockWriteGuard};

use crate::types::MockHash;
use crate::types::{MockBlob, MockBlockHeader};
use crate::{
    types::MockBlock,
    verifier::{MockDaSpec, MockDaVerifier},
};
use anyhow::anyhow;
use async_trait::async_trait;
use rollup_interface::services::da::DaService;
use rollup_interface::services::da::SlotData;
use rollup_interface::state::da::BlockHeaderTrait;
use tokio::sync::broadcast;
use tracing::span;

const GENESIS_HEADER: MockBlockHeader = MockBlockHeader {
    prev_hash: MockHash([0; 32]),
    hash: MockHash([1; 32]),
    height: 0,
};

#[derive(Clone)]
pub struct MockDaService {
    sender_address: [u8; 32],
    blocks: Arc<RwLock<VecDeque<MockBlock>>>,
    /// Used for calculating correct finality from state of `blocks`
    finalized_header_sender: broadcast::Sender<MockBlockHeader>,
}

impl MockDaService {
    pub fn new(sender_address: [u8; 32]) -> Self {
        let (tx, rx1) = broadcast::channel(10);
        tokio::spawn(async move {
            let mut rx = rx1;
            while let Ok(header) = rx.recv().await {
                tracing::debug!("Finalized MockHeader: {:?}", header);
            }
        });
        Self {
            sender_address,
            blocks: Arc::new(RwLock::new(VecDeque::new())),
            finalized_header_sender: tx,
        }
    }

    fn add_blob(
        &self,
        blob: &[u8],
        blocks: &mut RwLockWriteGuard<'_, VecDeque<MockBlock>>,
    ) -> anyhow::Result<()> {
        let (prev_block_hash, height) = match blocks.iter().last().map(|b| b.header()) {
            Some(block_header) => (block_header.hash(), block_header.height()),
            None => (GENESIS_HEADER.hash(), GENESIS_HEADER.height()),
        };

        let header = MockBlockHeader {
            prev_hash: prev_block_hash,
            // TODO: blobデータから有効なハッシュを作成する。
            hash: MockHash([123; 32]),
            height,
        };
        let blob = MockBlob::new();
        let block = MockBlock {
            header,
            blobs: vec![blob],
        };
        blocks.push_back(block);

        let next_index_to_fialize = blocks.len() - 1;
        let next_finalized_header = blocks[next_index_to_fialize].header().clone();
        self.finalized_header_sender
            .send(next_finalized_header)
            .unwrap();

        Ok(())
    }
}

#[async_trait]
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
        let blocks = self
            .blocks
            .read()
            .map_err(|e| anyhow!("Failed to lock blocks: {}", e))?;
        if blocks.len() < 1 {
            return Ok(GENESIS_HEADER);
        }

        let index = blocks.len() - 1;
        Ok(blocks[index].header().clone())
    }

    async fn send_transaction(&self, blob: &[u8]) -> anyhow::Result<()> {
        let mut blocks = self.blocks.write().unwrap();
        self.add_blob(blob, &mut blocks)?;
        Ok(())
    }
}
