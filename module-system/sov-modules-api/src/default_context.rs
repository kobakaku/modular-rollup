use sov_modules_core::{Address, Context, Spec};
use sov_state::ProverStorage;

use crate::default_signature::{DefaultPrivateKey, DefaultPublicKey, DefaultSignature};

#[derive(Clone, Debug)]
pub struct DefaultContext {}

impl Spec for DefaultContext {
    type Address = Address;
    type Storage = ProverStorage;
    type PublicKey = DefaultPublicKey;
    type PrivateKey = DefaultPrivateKey;
    type Signature = DefaultSignature;
}

impl Context for DefaultContext {}
