use crate::state::da::DaSpec;

pub trait DaService {
    type Spec: DaSpec;
}
