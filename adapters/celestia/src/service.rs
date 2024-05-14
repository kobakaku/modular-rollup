use crate::{
    types::CelestiaBlock,
    verifier::{CelestiaDaSpec, CelestiaDaVerifier},
    CelestiaDaConfig,
};
use async_trait::async_trait;
use jsonrpsee::http_client::HttpClient;
use rollup_interface::services::da::DaService;

#[derive(Clone)]
pub struct CelestiaDaService {
    client: HttpClient,
}

impl CelestiaDaService {
    pub fn new(config: &CelestiaDaConfig) -> Self {
        let client = jsonrpsee::http_client::HttpClientBuilder::default()
            .build(&config.rpc_address)
            .unwrap();
        Self { client }
    }
}

#[async_trait]
impl DaService for CelestiaDaService {
    type Spec = CelestiaDaSpec;

    type Verifier = CelestiaDaVerifier;

    type Block = CelestiaBlock;

    /// Gets block at given height
    async fn get_block_at(&self, height: u64) -> anyhow::Result<Self::Block> {
        if height == 0 {
            anyhow::bail!("The lowest queryable block should be > 0.")
        }

        let client = self.client.clone();

        // TODO: RPCを通じてCelestiaからHeaderを取得する
        tracing::debug!("Fetching header at height: {}", height);

        todo!()
    }

    async fn get_last_finalized_block_header(
        &self,
    ) -> anyhow::Result<<Self::Spec as rollup_interface::state::da::DaSpec>::BlockHeader> {
        // Tendermint has instant finality, so head block is the finalized one

        // TODO: 一番最後のblockのheaderを取得する
        todo!()
    }

    async fn send_transaction(&self, blob: &[u8]) -> anyhow::Result<()> {
        tracing::debug!("Sending {} bytes of raw data to Celestia.", blob.len());

        // TODO: Blobをsubmitする

        Ok(())
    }
}
