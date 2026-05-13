use crate::error::VeilError;

/// Wallet abstraction. Any wallet can integrate with Veil SDK
/// by implementing this trait.
pub trait Signer {
    /// Return the signer's public key as raw bytes.
    fn public_key(&self) -> Vec<u8>;

    /// Sign the given message bytes.
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, VeilError>;
}
