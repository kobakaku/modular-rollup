use std::marker::PhantomData;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use sov_bank_module::BankCallMessage;
use sov_modules_core::{Context, DispatchCall};
use sov_modules_stf_blueprint::RuntimeTrait;

/// Defines the modules inside this runtime.
#[derive(Serialize, Deserialize)]
pub struct Runtime<C: Context> {
    phantom: PhantomData<C>,
}

impl<C: Context> Default for Runtime<C> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
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
}
