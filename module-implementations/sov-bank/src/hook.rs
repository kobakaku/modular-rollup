use sov_modules_api::{hooks::TxHooks, macros::config_constant};

pub struct BankTxHook<C: sov_modules_api::Context> {
    pub sender: C::Address,
    pub sequencer: C::Address,
}

#[config_constant]
const GAS_TOKEN_ADDRESS: &'static str;

impl<C: sov_modules_api::Context> TxHooks for Bank<C> {
    type Context = C;
    type PreArg = BankTxHook<C>;
    type PreResult = ();

    fn pre_dispatch_tx_hook(
        &self,
        tx: &sov_modules_api::transaction::Transaction<Self::Context>,
        working_set: &mut sov_modules_api::WorkingSet<Self::Context>,
        arg: &Self::PreArg,
    ) -> anyhow::Result<Self::PreResult> {
        Ok(())
    }

    fn post_dispatch_tx_hook(
        &self,
        tx: &sov_modules_api::transaction::Transaction<Self::Context>,
        ctx: &Self::Context,
        working_set: &mut sov_modules_api::WorkingSet<Self::Context>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
