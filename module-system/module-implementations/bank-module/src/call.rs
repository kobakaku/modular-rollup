use serde::{Deserialize, Serialize};
use sov_modules_core::Context;

#[derive(Serialize, Deserialize)]
pub enum CallMessage<C: Context> {
    CreateToken {
        /// The name of the new token
        token_name: String,
        /// The initial balance of the new token
        initial_balance: u64,
        /// Authorized minter list
        minter_address: C::Address,
    },
}
