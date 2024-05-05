use std::collections::HashMap;

pub use call::CallMessage as BankCallMessage;
pub use rpc::get_bank_module_rpc_method;
use serde::{Deserialize, Serialize};
use sov_modules_core::{Context, Module};
use token::Token;

mod call;
mod rpc;
mod token;
mod utils;

#[derive(Serialize, Deserialize)]
pub struct BankModule<C: Context> {
    pub tokens: HashMap<C::Address, Token<C>>,
}

impl<C: Context> Module for BankModule<C> {
    type Context = C;
    type CallMessage = BankCallMessage<C>;

    fn call(&mut self, msg: Self::CallMessage) -> anyhow::Result<sov_modules_core::CallResponse> {
        match msg {
            BankCallMessage::CreateToken {
                token_name,
                initial_balance,
                minter_address,
            } => self.create_token(&token_name, initial_balance, minter_address),
            BankCallMessage::Mint {
                token_address,
                amount,
                minter_address,
            } => self.mint_token(token_address, amount, minter_address),
            BankCallMessage::Transfer {
                token_address,
                amount,
                from_address,
                to_address,
            } => self.transfer_token(token_address, amount, from_address, to_address),
            BankCallMessage::Burn {
                token_address,
                amount,
                burner_address,
            } => self.burn_token(token_address, amount, burner_address),
        }
    }
}

impl<C: Context> Default for BankModule<C> {
    fn default() -> Self {
        Self {
            tokens: HashMap::default(),
        }
    }
}
