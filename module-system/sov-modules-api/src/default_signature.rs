use sov_modules_core::{PrivateKey, PublicKey, Signature};

#[derive(Clone)]
pub struct DefaultPublicKey {}

impl PublicKey for DefaultPublicKey {}

#[derive(Clone)]
pub struct DefaultPrivateKey {}

impl PrivateKey for DefaultPrivateKey {}

#[derive(Clone)]
pub struct DefaultSignature {}

impl Signature for DefaultSignature {
    type PublicKey = DefaultPublicKey;

    fn verify(&self, _pub_key: &Self::PublicKey) -> anyhow::Result<()> {
        Ok(())
    }
}
