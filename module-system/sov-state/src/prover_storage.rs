use sov_modules_core::Storage;

pub struct ProverStorage {}

impl Storage for ProverStorage {}
impl NativeStorage for ProverStorage {}
pub trait NativeStorage: Storage {}
