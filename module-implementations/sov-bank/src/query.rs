use jsonrpsee::core::RpcResult;
use sov_modules_api::macros::rpc_gen;
use sov_modules_api::WorkingSet;

use crate::{Amount, Bank};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BalanceResponse {
    pub amount: Option<Amount>,
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SupplyResponse {
    pub amount: Option<Amount>,
}

#[rpc_gen(client, server, namespace = "bank")]
impl<C: sov_modules_api::Context> Bank<C> {
    #[rpc_method(name = "balanceOf")]
    pub fn balance_of(
        &self,
        user_address: C::Address,
        token_address: C::Address,
        working_set: &mut WorkingSet<C>,
    ) -> RpcResult<BalanceResponse> {
        Ok(BalanceResponse {
            amount: self.get_balance_of(&user_address, &token_address, working_set),
        })
    }

    #[rpc_method(name = "supplyOf")]
    pub fn supply_of(
        &self,
        token_address: C::Address,
        working_set: &mut WorkingSet<C>,
    ) -> RpcResult<SupplyResponse> {
        Ok(SupplyResponse {
            amount: self.get_total_supply_of(&token_address, working_set),
        })
    }
}
