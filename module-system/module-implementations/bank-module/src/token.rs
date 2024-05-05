use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sov_modules_core::Context;

use crate::utils::genearte_token_address;

#[derive(Serialize, Deserialize)]
pub struct Token<C: Context> {
    name: String,
    total_supply: u64,
    balance_map: HashMap<C::Address, u64>,
}

impl<C: Context> Token<C> {
    pub(crate) fn create(
        token_name: &str,
        initial_balance: u64,
        minter_address: C::Address,
    ) -> anyhow::Result<(C::Address, Self)> {
        let token_address = genearte_token_address::<C>(token_name);

        let mut balance_map = HashMap::new();

        balance_map.insert(minter_address, initial_balance);

        Ok((
            token_address,
            Token {
                name: token_name.to_string(),
                total_supply: initial_balance,
                balance_map,
            },
        ))
    }
}
