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
