use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub(crate) const DEPLOYER: [u8; 32] = [0; 32];

#[derive(Debug, serde::Serialize, serde::Deserialize)]

pub struct BankConfig<C: sov_modules_api::Context> {
    pub tokens: Vec<TokenConfig<C>>,
}

/// [`TokenConfig`] specifies a configuration used when generating a token for the bank
/// module.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(bound = "C::Address: Serialize + DeserializeOwned")]
pub struct TokenConfig<C: sov_modules_api::Context> {
    /// The name of the token.
    pub token_name: String,
    /// A vector of tuples containing the initial addresses and balances (as u64)
    pub address_and_balances: Vec<(C::Address, u64)>,
    /// A salt used to encrypt the token address.
    pub salt: u64,
}
