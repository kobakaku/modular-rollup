use jsonrpsee::core::RpcResult;
use rpc::MyModuleRpcServer;

mod rpc;
pub use rpc::get_my_module_rpc_method;

pub struct MyModule;

impl MyModuleRpcServer for MyModule {
    // $ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"myModule_my_method","id":1}' http://127.0.0.1:12345 | jq
    fn my_method(&self) -> RpcResult<String> {
        Ok("called my_method in myModule".to_string())
    }
}
