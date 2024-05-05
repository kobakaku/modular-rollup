use std::collections::HashMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sov_modules_core::Context;

use crate::utils::genearte_token_address;

#[derive(Clone, Serialize, Deserialize)]
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

    pub(crate) fn mint(&mut self, amount: u64, minter_address: C::Address) -> anyhow::Result<()> {
        let to_amount = self
            .balance_map
            .get(&minter_address)
            .unwrap()
            .checked_add(amount)
            .ok_or(anyhow!("Account balance overflow."))?;

        self.balance_map.insert(minter_address, to_amount);

        self.total_supply = self
            .total_supply
            .checked_add(amount)
            .ok_or(anyhow!("Total supply overflow."))?;

        Ok(())
    }

    pub(crate) fn transfer(
        &mut self,
        amount: u64,
        from_address: C::Address,
        to_address: C::Address,
    ) -> anyhow::Result<()> {
        let from_amount = self
            .balance_map
            .get(&from_address)
            .unwrap()
            .checked_sub(amount)
            .ok_or(anyhow!("Account balance overflow."))?;
        self.balance_map.insert(from_address, from_amount);

        let to_amount = self
            .balance_map
            .get(&to_address)
            .unwrap()
            .checked_add(amount)
            .ok_or(anyhow!("Account balance overflow."))?;
        self.balance_map.insert(to_address, to_amount);

        Ok(())
    }

    pub(crate) fn burn(&mut self, amount: u64, burner_address: C::Address) -> anyhow::Result<()> {
        let amount = self
            .balance_map
            .get(&burner_address)
            .unwrap()
            .checked_sub(amount)
            .ok_or(anyhow!("Account balance overflow."))?;
        self.balance_map.insert(burner_address, amount);

        self.total_supply = self
            .total_supply
            .checked_sub(amount)
            .ok_or(anyhow!("Total supply overflow."))?;

        Ok(())
    }
}
