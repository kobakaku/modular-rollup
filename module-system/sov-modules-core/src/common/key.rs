/// PublicKey used in the Module System.
pub trait PublicKey {}

/// PrivateKey used in the Module System.
pub trait PrivateKey {}

/// Signature used in the Module System.
pub trait Signature {
    /// The public key associated with the key pair of the signature;
    type PublicKey;

    fn verify(&self, pub_key: &Self::PublicKey) -> anyhow::Result<()>;
}
