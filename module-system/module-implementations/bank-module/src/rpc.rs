use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::de::DeserializeOwned;
use sov_modules_core::Context;

use crate::BankModule;

#[rpc(server, namespace = "bankModule")]
pub trait BankModuleRpc<C: Context> {
    #[method(name = "health")]
    fn health(&self) -> RpcResult<String>;
    #[method(name = "balance_of")]
    fn balance_of(
        &self,
        token_address: C::Address,
        user_address: C::Address,
    ) -> RpcResult<Option<u64>>;
    #[method(name = "supply_of")]
    fn supply_of(&self, token_address: C::Address) -> RpcResult<Option<u64>>;
}

impl<C: Context + Send + Sync> BankModuleRpcServer<C> for BankModule<C> {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"bankModule_health","id":1}' http://127.0.0.1:8000 | jq
    fn health(&self) -> RpcResult<String> {
        Ok("called health method in bankModule".to_string())
    }
    fn balance_of(
        &self,
        token_address: C::Address,
        user_address: C::Address,
    ) -> RpcResult<Option<u64>> {
        let balance = self.get_balance_of(token_address, user_address);
        Ok(balance)
    }
    fn supply_of(&self, token_address: C::Address) -> RpcResult<Option<u64>> {
        let total_supply = self.get_total_supply_of(token_address);
        Ok(total_supply)
    }
}

pub fn get_bank_module_rpc_method<C: Context + Send + Sync + DeserializeOwned>(
) -> jsonrpsee::RpcModule<()> {
    let mut module = jsonrpsee::RpcModule::new(());
    let bank_module = BankModule::<C>::default();
    module.merge(BankModule::into_rpc(bank_module)).unwrap();
    module
}
