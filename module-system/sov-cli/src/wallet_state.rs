use sov_modules_api::transaction::UnsentTransaction;
use sov_modules_core::Context;

/// A struct representing the current state of the CLI wallet
pub struct WalletState<C: Context> {
    pub unsent_trasactions: Vec<UnsentTransaction<C>>,
    pub addressed: AddressList<C>,
}

pub struct AddressList<C: Context> {
    /// The active address is stored first in this array.
    addresses: Vec<AddressEntry<C>>,
}

/// An entry in the address list
pub struct AddressEntry<C: Context> {
    /// The address
    pub address: C::Address,
    /// The public key associated with the address
    pub pub_key: C::PublicKey,
}
