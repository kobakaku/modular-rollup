mod call;
mod genesis;
mod query;

// use call::*;
use genesis::*;

use crate::call::CallMessage;

use sov_modules_api::{Error, Module, ModuleInfo, WorkingSet};

#[derive(ModuleInfo)]
pub struct Bank<C: sov_modules_api::Context> {
    #[address]
    address: C::Address,
}

impl<C: sov_modules_api::Context> Module for Bank<C> {
    type Context = C;
    type Config = BankConfig;
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
            CallMessage::CreateToken { address: _ } => self.create_token(context, working_set),
        };
        Ok(call_result?)
    }
}
