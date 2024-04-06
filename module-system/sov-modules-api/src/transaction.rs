use sov_modules_core::Context;

#[derive(Clone)]
pub struct Transaction<C: Context> {
    signature: C::Signature,
    pub_key: C::PublicKey,
    chain_id: u64,
    nonce: u64,
}
