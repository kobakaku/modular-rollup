use crate::{storage::Storage, PrivateKey, PublicKey, Signature};

/// The `Spec` trait configures certain key primitives to be used by a paticular instance of a rollup.
/// `Spec` is almost always implemented on a Context object.
pub trait Spec {
    /// Authenticated state storage used by the rollup. Typically some variant of a merkle-patricia trie.
    type Storage: Storage;

    /// The public key used for digital signatures.
    type PublicKey: PublicKey;

    /// The private key used for digital signatures.
    type PrivateKey: PrivateKey;

    /// The digital signature schema used by the rollup.
    type Signature: Signature;
}

/// A context contains information which is passed to modules during transaction is executed.
pub trait Context: Spec {}
