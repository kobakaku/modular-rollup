/// PublicKey used in the Module System.
pub trait PublicKey: Clone + Send + Sync {}

/// PrivateKey used in the Module System.
pub trait PrivateKey: Clone + Send + Sync {}

/// Signature used in the Module System.
pub trait Signature: Clone + Send + Sync {
    /// The public key associated with the key pair of the signature;
    type PublicKey;

    fn verify(&self, pub_key: &Self::PublicKey) -> anyhow::Result<()>;
}
