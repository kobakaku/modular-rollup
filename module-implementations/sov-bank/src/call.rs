use sov_modules_api::{CallResponse, Module, StateMapAccessor, WorkingSet};

use crate::{Amount, Bank, Token};

#[cfg_attr(
    feature = "native",
    derive(schemars::JsonSchema),
    schemars(bound = "C::Address: ::schemars::JsonSchema", rename = "CallMessage")
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSerialize)]
pub enum CallMessage<C: sov_modules_api::Context> {
    /// Creates a new token with the specified name and initial balance.
    CreateToken {
        /// Random value use to create a unique token address.
        salt: u64,
        /// The name of the new token.
        token_name: String,
        /// The initial balance of the new token.
        initial_balance: Amount,
        /// The address of the account that the new tokens are minted to.
        minter_address: C::Address,
    },
}

impl<C: sov_modules_api::Context> Bank<C> {
    pub(crate) fn init_module(
        &self,
        _config: &<Self as Module>::Config,
        _working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn create_token(
        &self,
        salt: u64,
        token_name: &str,
        initial_balance: Amount,
        minter_address: C::Address,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        let (token_address, token) = Token::<C>::create(
            salt,
            token_name,
            &[(minter_address, initial_balance)],
            context.sender().as_ref(),
            self.tokens.prefix(),
            working_set,
        )?;

        self.tokens.set(&token_address, &token, working_set);
        working_set.add_event(
            "Create Token",
            &format!("A token with token_address {token_address} was created"),
        );
        Ok(CallResponse::default())
    }
}

impl<C: sov_modules_api::Context> Bank<C> {
    pub fn get_balance_of(
        &self,
        user_address: &C::Address,
        token_address: &C::Address,
        working_set: &mut WorkingSet<C>,
    ) -> Option<u64> {
        self.tokens
            .get(token_address, working_set)
            .and_then(|token| token.balances.get(user_address, working_set))
    }

    pub fn get_token_name(
        &self,
        token_address: &C::Address,
        working_set: &mut WorkingSet<C>,
    ) -> Option<String> {
        self.tokens
            .get(token_address, working_set)
            .map(|token| token.name)
    }

    pub fn get_total_supply_of(
        &self,
        token_address: &C::Address,
        working_set: &mut WorkingSet<C>,
    ) -> Option<u64> {
        self.tokens
            .get(token_address, working_set)
            .map(|token| token.total_supply)
    }
}
