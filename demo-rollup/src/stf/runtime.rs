use std::io::Cursor;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use sov_bank_module::{BankCallMessage, BankModule};
use sov_modules_core::{Context, DispatchCall, Module};
use sov_modules_stf_blueprint::RuntimeTrait;

/// Defines the modules inside this runtime.
#[derive(Serialize, Deserialize)]
pub struct Runtime<C: Context> {
    bank: BankModule<C>,
}

impl<C: Context> Default for Runtime<C> {
    fn default() -> Self {
        Self {
            bank: BankModule::default(),
        }
    }
}

impl<C: Context> RuntimeTrait for Runtime<C> {}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum RuntimeCall<C: Context> {
    Bank(BankCallMessage<C>),
}

impl<C: Context> DispatchCall for Runtime<C> {
    type Context = C;

    type Decodable = RuntimeCall<C>;

    fn decode_call(serialized_message: &[u8]) -> anyhow::Result<Self::Decodable> {
        let mut data = Cursor::new(serialized_message);
        let msg = Self::Decodable::deserialize_reader(&mut data)?;
        Ok(msg)
    }

    fn dispatch_call(
        &mut self,
        message: Self::Decodable,
    ) -> anyhow::Result<sov_modules_core::CallResponse> {
        match message {
            RuntimeCall::Bank(msg) => BankModule::<Self::Context>::call(&mut self.bank, msg),
        }
    }
}
