use anyhow::bail;
use sov_modules_api::{StateMap, StateMapAccessor, WorkingSet};
use sov_state::Prefix;

use crate::prefix_from_address_with_parent;

pub(crate) type Amount = u64;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub(crate) struct Token<C: sov_modules_api::Context> {
    pub(crate) name: String,
    pub(crate) total_supply: u64,
    pub(crate) balances: sov_modules_api::StateMap<C::Address, Amount>,
}

impl<C: sov_modules_api::Context> Token<C> {
    pub(crate) fn create(
        salt: u64,
        token_name: &str,
        address_and_balances: &[(C::Address, u64)],
        sender: &[u8],
        parent_prefix: &Prefix,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<(C::Address, Self)> {
        let token_address = super::get_token_address::<C>(token_name, sender, salt);
        let token_prefix = prefix_from_address_with_parent::<C>(parent_prefix, &token_address);
        let mut balances = StateMap::new(token_prefix);

        let mut total_supply: Option<u64> = Some(0);
        for (address, balance) in address_and_balances.iter() {
            balances.set(address, balance, working_set);
            total_supply = total_supply.and_then(|ts| ts.checked_add(*balance));
        }

        let total_supply = match total_supply {
            Some(total_supply) => total_supply,
            None => bail!("Total supply overflow"),
        };

        let token = Token::<C> {
            name: token_name.to_string(),
            total_supply,
            balances,
        };
        Ok((token_address, token))
    }
}
