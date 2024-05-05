use anyhow::anyhow;
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
    Mint {
        token_address: C::Address,
        amount: u64,
        minter_address: C::Address,
    },
    Transfer {
        token_address: C::Address,
        amount: u64,
        from_address: C::Address,
        to_address: C::Address,
    },
    Burn {
        token_address: C::Address,
        amount: u64,
        burner_address: C::Address,
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
        tracing::info!(
            "Create token(token_address: {}).",
            serde_json::to_string_pretty(&token_address)?
        );

        self.tokens.insert(token_address, token);

        Ok(CallResponse::default())
    }

    pub(crate) fn mint_token(
        &mut self,
        token_address: C::Address,
        amount: u64,
        minter_address: C::Address,
    ) -> anyhow::Result<CallResponse> {
        tracing::info!(
            "Mint token(token_address: {}).",
            serde_json::to_string_pretty(&token_address)?
        );
        let mut token = self
            .tokens
            .get(&token_address)
            .ok_or(anyhow!("Failed to get the token."))?
            .clone();

        token.mint(amount, minter_address)?;
        self.tokens.insert(token_address, token);

        Ok(CallResponse::default())
    }

    pub(crate) fn transfer_token(
        &mut self,
        token_address: C::Address,
        amount: u64,
        from_address: C::Address,
        to_address: C::Address,
    ) -> anyhow::Result<CallResponse> {
        tracing::debug!(
            "Transfer token(token_address: {}).",
            serde_json::to_string_pretty(&token_address)?
        );
        let mut token = self
            .tokens
            .get(&token_address)
            .ok_or(anyhow!("Failed to get the token."))?
            .clone();

        token.transfer(amount, from_address, to_address)?;
        self.tokens.insert(token_address, token);

        Ok(CallResponse::default())
    }

    pub(crate) fn burn_token(
        &mut self,
        token_address: C::Address,
        amount: u64,
        burner_address: C::Address,
    ) -> anyhow::Result<CallResponse> {
        tracing::debug!(
            "Burn token(token_address: {}).",
            serde_json::to_string_pretty(&token_address)?
        );
        let mut token = self
            .tokens
            .get(&token_address)
            .ok_or(anyhow!("Failed to get the token."))?
            .clone();

        token.burn(amount, burner_address)?;
        self.tokens.insert(token_address, token);

        Ok(CallResponse::default())
    }

    pub(crate) fn get_balance_of(
        &self,
        token_address: C::Address,
        user_address: C::Address,
    ) -> Option<u64> {
        self.tokens
            .get(&token_address)
            .and_then(|token| token.balance_map.get(&user_address))
            .cloned()
    }

    pub(crate) fn get_total_supply_of(&self, token_address: C::Address) -> Option<u64> {
        self.tokens
            .get(&token_address)
            .map(|token| token.total_supply)
    }
}
