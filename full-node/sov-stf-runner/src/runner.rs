use jsonrpsee::server::{RpcModule, Server};
use std::net::{IpAddr, SocketAddr};
use tracing::info;

use rollup_interface::{
    services::da::DaService,
    state::{da::DaSpec, stf::StateTransitionFunction, storage::HierarchicalStorageManager},
};
use sov_db::ledger_db::LedgerDB;

use crate::config::RunnerConfig;

pub struct StateTransitionRunner<Stf, Sm, Da>
where
    Stf: StateTransitionFunction,
    Sm: HierarchicalStorageManager,
    Da: DaService,
{
    start_height: u64,
    listen_address: SocketAddr,
    ledger_db: LedgerDB,
    stf: Stf,
    storage_manager: Sm,
    da_service: Da,
    state_root: Stf::StateRoot,
}

pub enum InitVariant<Stf: StateTransitionFunction, Da: DaSpec> {
    Initialized(Stf::StateRoot),
    Genesis {
        block_header: Da::BlockHeader,
        genesis_params: Stf::GenesisParams,
    },
}

impl<Stf, Sm, Da> StateTransitionRunner<Stf, Sm, Da>
where
    Stf: StateTransitionFunction<PreState = Sm::NativeStorage>,
    Sm: HierarchicalStorageManager,
    Da: DaService,
{
    pub fn new(
        runner_config: RunnerConfig,
        ledger_db: LedgerDB,
        init_variant: InitVariant<Stf, Da::Spec>,
        stf: Stf,
        storage_manager: Sm,
        da_service: Da,
    ) -> Result<Self, anyhow::Error> {
        let RunnerConfig {
            start_height,
            rpc_config,
        } = runner_config;

        let prev_state_root = match init_variant {
            InitVariant::Initialized(state_root) => {
                info!("Chain is already initialized. Skipping initialization.");
                state_root
            }
            InitVariant::Genesis {
                block_header,
                genesis_params,
            } => {
                info!(
                    "No history detected. Initializing chain on block_header={:?}",
                    block_header
                );
                let storage = storage_manager.create_storage_on()?;
                let (genesis_root, _initialized_storage) = stf.init_chain(storage, genesis_params);
                storage_manager.save_change_set()?;
                storage_manager.finalize()?;
                info!(
                    "Chain initialization is done. Genesis root: 0x{}",
                    hex::encode(genesis_root.as_ref()),
                );
                genesis_root
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
            da_service,
            state_root: prev_state_root,
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
