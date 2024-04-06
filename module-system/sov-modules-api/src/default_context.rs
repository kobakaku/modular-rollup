use sov_modules_core::{Context, Spec};
use sov_state::ProverStorage;

use crate::default_signature::{DefaultPrivateKey, DefaultPublicKey, DefaultSignature};

pub struct DefaultContext {}

impl Spec for DefaultContext {
    type Storage = ProverStorage;
    type PublicKey = DefaultPublicKey;
    type PrivateKey = DefaultPrivateKey;
    type Signature = DefaultSignature;
}

impl Context for DefaultContext {}
