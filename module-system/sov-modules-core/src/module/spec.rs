use crate::storage::Storage;

pub trait Spec {
    type Storage: Storage;
}

pub trait Context: Spec {}
