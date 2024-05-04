use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use sov_modules_core::{CallResponse, Context};

use crate::{token::Token, BankModule};

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
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

impl<C: Context> BankModule<C> {
    pub(crate) fn create_token(
        &mut self,
        token_name: &str,
        initial_balance: u64,
        minter_address: C::Address,
    ) -> anyhow::Result<CallResponse> {
        let (token_address, token) =
            Token::<C>::create(token_name, initial_balance, minter_address)?;

        self.tokens.insert(token_address, token);

        Ok(CallResponse::default())
    }
}
