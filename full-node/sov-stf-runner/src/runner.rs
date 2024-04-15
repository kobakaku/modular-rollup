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
        let listen_address = self.listen_address;
        let _handle = tokio::spawn(async move {
            let server = jsonrpsee::server::ServerBuilder::default()
                .build(listen_address)
                .await
                .unwrap();
            let addr = server.local_addr().unwrap();
            info!("Starting RPC server at {} ", addr);

            let _server_handle = server.start(rpc_module);

            futures::future::pending::<()>().await;
        });
    }

    /// Runs the rollup.
    pub async fn run_in_progress(&self) -> Result<(), anyhow::Error> {
        let height = self.start_height;
        tracing::debug!("Requesting data for height {}", height);
        loop {}
    }
}
