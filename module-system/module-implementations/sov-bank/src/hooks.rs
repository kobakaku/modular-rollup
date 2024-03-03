use core::str::FromStr;

use sov_modules_api::{hooks::TxHooks, macros::config_constant, GasUnit};

use crate::Bank;

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
        hook: &Self::PreArg,
    ) -> anyhow::Result<Self::PreResult> {
        let BankTxHook { sender, sequencer } = hook;

        // Charge the base tx gas cost
        let gas_fixed_cost = tx.gas_fixed_cost();
        if working_set.charge_gas(&gas_fixed_cost).is_err() {
            let amount = gas_fixed_cost.value(working_set.gas_price());
            anyhow::bail!(
                "Transaction sender doesn't have enough funds to pay for the transaction base cost. Amount: {}",
                amount
            );
        }

        let amount = tx.gas_limit().saturating_add(tx.gas_tip());
        if amount > 0 {
            let token_address = C::Address::from_str(GAS_TOKEN_ADDRESS)
                .map_err(|_| anyhow::anyhow!("Failed to parse gas token address"))?;

            let from = sender;
            let to = sequencer;
            let coins = crate::Coins {
                amount,
                token_address,
            };

            self.transfer_from(from, to, coins, working_set)?;
        }

        Ok(())
    }

    fn post_dispatch_tx_hook(
        &self,
        _tx: &sov_modules_api::transaction::Transaction<Self::Context>,
        ctx: &Self::Context,
        working_set: &mut sov_modules_api::WorkingSet<Self::Context>,
    ) -> anyhow::Result<()> {
        let amount = working_set.gas_remaining_funds();

        if amount > 0 {
            let token_address = C::Address::from_str(GAS_TOKEN_ADDRESS)
                .map_err(|_| anyhow::anyhow!("Failed to parse gas token address"))?;
            let from = ctx.sequencer();
            let to = ctx.sender();
            let coins = crate::Coins {
                amount,
                token_address,
            };

            self.transfer_from(from, to, coins, working_set)?;
        }
        Ok(())
    }
}
