use crate::error::VeilError;

/// Wallet abstraction. Any wallet can integrate with Veil SDK
/// by implementing this trait.
pub trait IntentSigner {
    /// Return the signer's public key.
    fn public_key(&self) -> String;

    /// Sign the given message bytes.
    fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, VeilError>;
}
