mod call;
mod genesis;
mod hooks;
#[cfg(feature = "native")]
mod query;
mod token;
mod utils;

pub use call::*;
pub use genesis::*;
pub use hooks::BankTxHook;
#[cfg(feature = "native")]
pub use query::*;
use token::Token;
pub use token::{Amount, Coins};
pub use utils::*;

use sov_modules_api::{Error, GasUnit, Module, ModuleInfo, WorkingSet};

pub struct BankGasConfig<GU: GasUnit> {
    /// Gas price multiplier for the create token operation
    create_token: GU,

    /// Gas price multiplier for the transfer operation
    transfer: GU,

    /// Gas price multiplier for the burn operation
    burn: GU,

    /// Gas price multiplier for the mint operation
    mint: GU,

    /// Gas price multiplier for the freeze operation
    freeze: GU,
}

#[cfg_attr(feature = "native", derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo)]
pub struct Bank<C: sov_modules_api::Context> {
    #[address]
    address: C::Address,

    #[gas]
    gas: BankGasConfig<C::GasUnit>,

    #[state]
    tokens: sov_modules_api::StateMap<C::Address, Token<C>>,
}

impl<C: sov_modules_api::Context> Module for Bank<C> {
    type Context = C;
    type Config = BankConfig<C>;
    type CallMessage = CallMessage<C>;
    type Event = ();

    fn genesis(&self, config: &Self::Config, working_set: &mut WorkingSet<C>) -> Result<(), Error> {
        Ok(self.init_module(config, working_set)?)
    }

    fn call(
        &self,
        msg: Self::CallMessage,
        context: &Self::Context,
        working_set: &mut WorkingSet<C>,
    ) -> Result<sov_modules_api::CallResponse, Error> {
        let call_result = match msg {
            CallMessage::CreateToken {
                salt,
                token_name,
                initial_balance,
                minter_address,
            } => {
                self.charge_gas(working_set, &self.gas.create_token)?;
                self.create_token(
                    salt,
                    &token_name,
                    initial_balance,
                    minter_address,
                    context,
                    working_set,
                )
            }
        };
        Ok(call_result?)
    }
}
