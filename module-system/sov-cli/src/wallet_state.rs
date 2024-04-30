use std::path::PathBuf;
use std::{fs, path::Path};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sov_modules_api::transaction::UnsentTransaction;
use sov_modules_core::Context;

/// A struct representing the current state of the CLI wallet
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "Tx: Serialize + DeserializeOwned, C::Address: Serialize")]
pub struct WalletState<C: Context, Tx> {
    pub unsent_transactions: Vec<UnsentTransaction<Tx>>,
    pub addresses: Vec<AddressEntry<C>>,
}

impl<C: Context, Tx: Serialize + DeserializeOwned> Default for WalletState<C, Tx> {
    fn default() -> Self {
        Self {
            unsent_transactions: Vec::new(),
            addresses: Vec::new(),
        }
    }
}

impl<C: Context + DeserializeOwned, Tx: Serialize + DeserializeOwned> WalletState<C, Tx> {
    /// Read the wallet state from the given path
    pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let data = fs::read(path)?;
            let state = serde_json::from_slice(data.as_slice())?;
            Ok(state)
        } else {
            Ok(Default::default())
        }
    }

    /// Write the wallet state to the given path
    pub fn write<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}

/// A struct representing private key and associated address
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateKeyAndAddress<C: Context> {
    pub private_key: C::PrivateKey,
    /// Address associated from the private key
    pub address: C::Address,
}

impl<C: Context> PrivateKeyAndAddress<C> {
    // TODO: 鍵を生成できるようにする
    // pub fn generate() -> Self {}
}

// #[derive(Debug, Deserialize)]
// pub struct AddressList<C: Context> {
//     /// The active address is stored first in this array.
//     addresses: Vec<AddressEntry<C>>,
// }

/// An entry in the address list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressEntry<C: Context> {
    /// The address
    pub address: C::Address,
    /// The public key associated with the address
    pub pub_key: C::PublicKey,
    /// The path of the private key on disk.
    pub path: PathBuf,
}
