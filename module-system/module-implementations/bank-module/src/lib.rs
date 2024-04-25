use jsonrpsee::core::RpcResult;
use rpc::BankModuleRpcServer;

pub use rpc::get_bank_module_rpc_method;

mod rpc;

pub struct BankModule {}

impl BankModuleRpcServer for BankModule {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"BankModule_bank_method","id":1}' http://127.0.0.1:8000 | jq
    fn bank_method(&self) -> RpcResult<String> {
        Ok("called bank_method in bankModule".to_string())
    }
}
