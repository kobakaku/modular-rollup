use sov_modules_api::{CallResponse, Module, WorkingSet};

use crate::Bank;

#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSerialize)]
pub enum CallMessage<C: sov_modules_api::Context> {
    CreateToken { address: C::Address },
}

impl<C: sov_modules_api::Context> Bank<C> {
    pub(crate) fn init_module(
        &self,
        _config: &<Self as Module>::Config,
        _working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn create_token(
        &self,
        _context: &C,
        _working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<CallResponse> {
        Ok(CallResponse::default())
    }
}
