use sov_modules_core::{Context, Spec};
use sov_state::ProverStorage;

pub struct DefaultContext {}

impl Spec for DefaultContext {
    type Storage = ProverStorage;
}

impl Context for DefaultContext {}
