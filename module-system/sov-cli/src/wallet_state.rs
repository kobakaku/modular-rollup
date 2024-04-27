use std::{fs, path::Path};

use serde::de::DeserializeOwned;
use serde::Deserialize;
use sov_modules_api::transaction::UnsentTransaction;
use sov_modules_core::Context;

/// A struct representing the current state of the CLI wallet
#[derive(Debug, Deserialize)]
pub struct WalletState<C: Context> {
    pub unsent_transactions: Vec<UnsentTransaction<C>>,
    pub addresses: Vec<AddressEntry<C>>,
}

impl<C: Context> Default for WalletState<C> {
    fn default() -> Self {
        Self {
            unsent_transactions: Vec::new(),
            addresses: Vec::new(),
        }
    }
}

impl<C: Context + DeserializeOwned> WalletState<C> {
    /// Read the wallet state from the given path
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
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
    pub fn write() -> () {}
}

// #[derive(Debug, Deserialize)]
// pub struct AddressList<C: Context> {
//     /// The active address is stored first in this array.
//     addresses: Vec<AddressEntry<C>>,
// }

/// An entry in the address list
#[derive(Debug, Deserialize)]
pub struct AddressEntry<C: Context> {
    /// The address
    pub address: C::Address,
    /// The public key associated with the address
    pub pub_key: C::PublicKey,
}
