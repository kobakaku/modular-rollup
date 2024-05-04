use sha2::Digest;

use sov_modules_core::Context;

pub(crate) fn genearte_token_address<C: Context>(token_name: &str) -> C::Address {
    let mut hasher = sha2::Sha256::new();
    hasher.update(token_name);
    let result: [u8; 32] = hasher.finalize().into();
    C::Address::from(result)
}
