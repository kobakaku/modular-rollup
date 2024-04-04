use jsonrpsee::server::{RpcModule, Server};
use std::net::{IpAddr, SocketAddr};
use tracing::info;

use rollup_interface::state::{stf::StateTransitionFunction, storage::HierarchicalStorageManager};
use sov_db::ledger_db::LedgerDB;

use crate::config::RunnerConfig;

pub struct StateTransitionRunner<Stf, Sm>
where
    Stf: StateTransitionFunction,
    Sm: HierarchicalStorageManager,
{
    pub start_height: u64,
    pub listen_address: SocketAddr,
    pub ledger_db: LedgerDB,
    pub stf: Stf,
    pub storage_manager: Sm,
}

pub enum InitVariant<Stf: StateTransitionFunction> {
    Initialized,
    Genesis {
        block_header: String,
        genesis_params: Stf::GenesisParams,
    },
}

impl<Stf, Sm> StateTransitionRunner<Stf, Sm>
where
    Stf: StateTransitionFunction<PreState = Sm::NativeStorage>,
    Sm: HierarchicalStorageManager,
{
    pub fn new(
        runner_config: RunnerConfig,
        ledger_db: LedgerDB,
        init_variant: InitVariant<Stf>,
        stf: Stf,
        storage_manager: Sm,
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
                let storage = storage_manager.create_storage_on()?;
                let (_gemesis_root, _initialized_storage) = stf.init_chain(storage, genesis_params);
                storage_manager.save_change_set()?;
                storage_manager.finalize()?;
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
            storage_manager,
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
