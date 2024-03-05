mod runtime_rpc;
mod wallet;

use async_trait::async_trait;
use sov_modules_api::{
    runtime::capabilities::KernelSlotHooks, storage::HierarchicalStorageManager, Context, DaSpec,
    Spec,
};
use sov_modules_stf_blueprint::{Runtime as RuntimeTrait, StfBlueprint};
use sov_rollup_interface::{services::da::DaService, zk::ZkvmHost};
use sov_state::Storage;
use sov_stf_runner::{ProverService, StateTransitionRunner};

#[async_trait]
pub trait RollupBlueprint: Sized + Send + Sync {
    /// Data Availability service.
    type DaService: DaService<Spec = Self::DaSpec, Error = anyhow::Error> + Clone + Send + Sync;
    /// A specification for the types used by a DA layer.
    type DaSpec: DaSpec + Send + Sync;
    /// Data Availability config.
    type DaConfig: Send + Sync;

    /// Host of a zkVM program.
    type Vm: ZkvmHost + Send;

    /// Context for Zero Knowledge environment.
    type ZkContext: Context;
    /// Context for Native environment.
    type NativeContext: Context;

    /// Manager for the native storage lifecycle.
    type StorageManager: HierarchicalStorageManager<
        Self::DaSpec,
        NativeStorage = <Self::NativeContext as Spec>::Storage,
        NativeChangeSet = <Self::NativeContext as Spec>::Storage,
    >;

    /// Runtime for the Zero Knowledge environment.
    type ZkRuntime: RuntimeTrait<Self::ZkContext, Self::DaSpec> + Default;
    /// Runtime for the Native environment.
    type NativeRuntime: RuntimeTrait<Self::NativeContext, Self::DaSpec> + Default + Send + Sync;

    /// The kernel for the native environment.
    type NativeKernel: KernelSlotHooks<Self::NativeContext, Self::DaSpec> + Default + Send + Sync;
    /// The kernel for the Zero Knowledge environment.
    type ZkKernel: KernelSlotHooks<Self::ZkContext, Self::DaSpec> + Default;

    /// Prover service.
    type ProverService: ProverService<
        StateRoot = <<Self::NativeContext as Spec>::Storage as Storage>::Root,
        Witness = <<Self::NativeContext as Spec>::Storage as Storage>::Witness,
        DaService = Self::DaService,
    >;

    async fn create_new_rollup() -> anyhow::Result<Rollup<Self>> {
        Ok(Rollup::<Self> {})
    }
}

/// Dependencies needed to run the rollup.
pub struct Rollup<S: RollupBlueprint> {
    // /// The State Transition Runner.
    // pub runner: StateTransitionRunner<
    //     StfBlueprint<S::NativeContext, S::DaSpec, S::Vm, S::NativeRuntime, S::NativeKernel>,
    //     S::StorageManager,
    //     S::DaService,
    //     S::Vm,
    //     S::ProverService,
    // >,
    // /// Rpc methods for the rollup.
    // pub rpc_methods: jsonrpsee::RpcModule<()>,
}
