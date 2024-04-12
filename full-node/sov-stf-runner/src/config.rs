use serde::{de::DeserializeOwned, Deserialize};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RollupConfig {
    pub runner: RunnerConfig,
    pub storage: StorageConfig,
    /// Da configuration.
    pub da: DaServiceConfig,
    /// Prover Service configration.
    pub prover_service: ProverServiceConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RunnerConfig {
    /// DA start height.
    pub start_height: u64,
    /// RPC configuration.
    pub rpc_config: RpcConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RpcConfig {
    /// RPC host.
    pub bind_host: String,
    /// RPC port.
    pub bind_port: u16,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StorageConfig {
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DaServiceConfig {
    pub sender_address: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ProverServiceConfig {}

/// Reads toml file as a specific type.
pub fn from_toml_path<P: AsRef<Path> + std::fmt::Display, R: DeserializeOwned>(
    path: P,
) -> anyhow::Result<R> {
    let mut contents = String::new();
    {
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;
    }

    let result: R = toml::from_str(&contents)?;

    Ok(result)
}
