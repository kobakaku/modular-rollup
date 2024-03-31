use jsonrpsee::server::{RpcModule, Server};
use std::net::{IpAddr, SocketAddr};
use tracing::info;

use rollup_interface::state::stf::StateTransitionFunction;
use sov_db::ledger_db::LedgerDB;

use crate::config::RunnerConfig;

pub struct StateTransitionRunner<Stf>
where
    Stf: StateTransitionFunction,
{
    pub start_height: u64,
    pub listen_address: SocketAddr,
    pub ledger_db: LedgerDB,
    pub stf: Stf,
}

pub enum InitVariant<Stf: StateTransitionFunction> {
    Initialized,
    Genesis {
        block_header: String,
        genesis_params: Stf::GenesisParams,
    },
}

impl<Stf> StateTransitionRunner<Stf>
where
    Stf: StateTransitionFunction,
{
    pub fn new(
        runner_config: RunnerConfig,
        ledger_db: LedgerDB,
        init_variant: InitVariant<Stf>,
        stf: Stf,
    ) -> Result<Self, anyhow::Error> {
        let RunnerConfig {
            start_height,
            rpc_config,
        } = runner_config;
        let _ = match init_variant {
            InitVariant::Initialized => {}
            InitVariant::Genesis {
                block_header: _,
                genesis_params,
            } => {
                let (_genesis_hash, _pre_state) = stf.init_chain(genesis_params);
            }
        };
        let listen_address = SocketAddr::new(
            rpc_config.bind_host.parse::<IpAddr>()?,
            rpc_config.bind_port,
        );
        Ok(Self {
            start_height,
            listen_address,
            ledger_db,
            stf,
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
