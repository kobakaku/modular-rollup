use jsonrpsee::core::RpcResult;
use sov_modules_api::macros::rpc_gen;

use crate::Bank;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BalanceResponse {}

#[rpc_gen(client, server, namespace = "bank")]
impl<C: sov_modules_api::Context> Bank<C> {
    #[rpc_method(name = "getToken")]
    pub fn get_token(&self) -> RpcResult<BalanceResponse> {
        Ok(BalanceResponse {})
    }
}
