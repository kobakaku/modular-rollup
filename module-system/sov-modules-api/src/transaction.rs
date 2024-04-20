use sov_modules_core::Context;

#[derive(Clone, Debug, borsh::BorshDeserialize)]
pub struct Transaction<C: Context> {
    signature: C::Signature,
    pub_key: C::PublicKey,
    chain_id: u64,
    nonce: u64,
}

impl<C: Context> Transaction<C> {
    pub fn verify(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
