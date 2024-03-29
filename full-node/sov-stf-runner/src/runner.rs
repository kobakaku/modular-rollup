use jsonrpsee::server::{RpcModule, Server};
use std::net::{IpAddr, SocketAddr};
use tracing::info;

use sov_db::ledger_db::LedgerDB;

use crate::config::RunnerConfig;

pub struct StateTransitionRunner {
    pub start_height: u64,
    pub listen_address: SocketAddr,
    pub ledger_db: LedgerDB,
}

impl StateTransitionRunner {
    pub fn new(runner_config: RunnerConfig, ledger_db: LedgerDB) -> Result<Self, anyhow::Error> {
        let RunnerConfig {
            start_height,
            rpc_config,
        } = runner_config;
        let listen_address = SocketAddr::new(
            rpc_config.bind_host.parse::<IpAddr>()?,
            rpc_config.bind_port,
        );
        Ok(Self {
            start_height,
            listen_address,
            ledger_db,
        })
    }

    /// Starts a RPC server
    pub async fn start_rpc_server(&self, rpc_module: RpcModule<()>) {
        let server_addr = Self::run_server(rpc_module, self.listen_address)
            .await
            .unwrap();
        info!("Starting RPC server at {} ", &server_addr);
        futures::future::pending().await
    }

    async fn run_server(
        rpc_module: RpcModule<()>,
        listen_address: SocketAddr,
    ) -> Result<SocketAddr, anyhow::Error> {
        let server = Server::builder().build(listen_address).await?;

        let addr = server.local_addr()?;
        let handle = server.start(rpc_module);
        tokio::spawn(handle.stopped());

        Ok(addr)
    }

    /// Runs the rollup.
    pub fn run_in_progress(self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
