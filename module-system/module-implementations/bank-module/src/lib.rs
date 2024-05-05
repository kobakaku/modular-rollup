use std::collections::HashMap;

use jsonrpsee::core::RpcResult;
use rpc::BankModuleRpcServer;

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

impl<C: Context + Send + Sync> BankModuleRpcServer for BankModule<C> {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"BankModule_bank_method","id":1}' http://127.0.0.1:8000 | jq
    fn bank_method(&self) -> RpcResult<String> {
        Ok("called bank_method in bankModule".to_string())
    }
}
