use sov_modules_core::{PrivateKey, PublicKey, Signature};

pub struct DefaultPublicKey {}

impl PublicKey for DefaultPublicKey {}

pub struct DefaultPrivateKey {}

impl PrivateKey for DefaultPrivateKey {}

pub struct DefaultSignature {}

impl Signature for DefaultSignature {
    type PublicKey = DefaultPublicKey;

    fn verify(&self, _pub_key: &Self::PublicKey) -> anyhow::Result<()> {
        Ok(())
    }
}
