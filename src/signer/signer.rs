use crate::error::VeilError;

/// Wallet abstraction. Any wallet can integrate with Veil SDK
/// by implementing this trait.
pub trait Signer {
    /// Return the signer's public key as raw bytes (32 bytes for Ed25519).
    fn public_key(&self) -> Vec<u8>;

    /// Sign the given message bytes. Returns Ed25519 signature (64 bytes).
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, VeilError>;
}
