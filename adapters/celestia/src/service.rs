use crate::{
    types::{CelestiaBlock, CelestiaBlockHeader},
    verifier::{CelestiaDaSpec, CelestiaDaVerifier},
    CelestiaDaConfig,
};
use async_trait::async_trait;
use celestia_rpc::prelude::*;
use celestia_types::{blob::SubmitOptions, nmt::Namespace, Blob};
use jsonrpsee::{http_client::HttpClient, ws_client::HeaderMap};
use rollup_interface::services::da::DaService;

#[derive(Clone)]
pub struct CelestiaDaService {
    client: HttpClient,
    namespace: Namespace,
}

impl CelestiaDaService {
    pub fn new(config: &CelestiaDaConfig) -> Self {
        let client = {
            let mut headers = HeaderMap::new();
            headers.append(
                "Authorization",
                format!("Bearer {}", config.rpc_auth_token).parse().unwrap(),
            );

            jsonrpsee::http_client::HttpClientBuilder::default()
                .set_headers(headers)
                .build(&config.rpc_address)
        }
        .expect("Client initialization is valid");

        Self {
            client,
            // TODO: 適切なnamespaceを考える
            namespace: Namespace::const_v0([0, 0, 115, 111, 118, 45, 116, 101, 115, 116]),
        }
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
        let header: celestia_types::ExtendedHeader = client.header_get_by_height(height).await?;
        tracing::debug!("Fetching header at height: {}", height);

        Ok(CelestiaBlock::new(CelestiaBlockHeader::new(
            // TODO: 正しいhash値をいれる
            [0; 32],
            header.height().into(),
        )))
    }

    async fn get_last_finalized_block_header(
        &self,
    ) -> anyhow::Result<<Self::Spec as rollup_interface::state::da::DaSpec>::BlockHeader> {
        // Tendermint has instant finality, so head block is the finalized one
        let header = self.client.header_network_head().await?;
        Ok(CelestiaBlockHeader::from(header))
    }

    async fn send_transaction(&self, blob: &[u8]) -> anyhow::Result<()> {
        tracing::debug!("Sending {} bytes of raw data to Celestia.", blob.len());

        let blob = Blob::new(self.namespace, blob.to_vec())?;
        let height = self
            .client
            .blob_submit(
                &[blob],
                SubmitOptions {
                    // TODO: Blobをsubmitする際に適切なOptionを考慮する
                    fee: None,
                    gas_limit: None,
                },
            )
            .await?;

        tracing::info!(
            "Blob has been submitted to Celestia. block-height={}",
            height
        );

        Ok(())
    }
}
