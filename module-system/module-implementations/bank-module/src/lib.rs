use std::marker::PhantomData;

use jsonrpsee::core::RpcResult;
use rpc::BankModuleRpcServer;

pub use call::CallMessage as BankCallMessage;
pub use rpc::get_bank_module_rpc_method;
use serde::{Deserialize, Serialize};
use sov_modules_core::{CallResponse, Context, Module};

mod call;
mod rpc;

#[derive(Serialize, Deserialize)]
pub struct BankModule<C: Context> {
    phantom: PhantomData<C>,
}

impl<C: Context> Module for BankModule<C> {
    type Context = C;
    type CallMessage = BankCallMessage<C>;

    fn call(&self, msg: Self::CallMessage) -> anyhow::Result<sov_modules_core::CallResponse> {
        tracing::debug!("{:?}", msg);
        Ok(CallResponse {})
    }
}

impl<C: Context> Default for BankModule<C> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<C: Context + Send + Sync> BankModuleRpcServer for BankModule<C> {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"BankModule_bank_method","id":1}' http://127.0.0.1:8000 | jq
    fn bank_method(&self) -> RpcResult<String> {
        Ok("called bank_method in bankModule".to_string())
    }
}
