use crate::error::VeilError;
use crate::intent::payload::IntentPayload;
use crate::types::{Chain, Nonce};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intent {
    pub version: u8,
    pub chain: Chain,
    pub nonce: Nonce,
    pub payload: IntentPayload,
}

impl Intent {
    /// Validate intent payload before signing.
    /// Returns Ok(()) if valid, Err(VeilError::InvalidIntent) otherwise.
    ///
    /// Phase 1 rules:
    /// - lamports must be > 0
    /// - recipient `to` must not be empty
    /// - recipient `to` must be <= 44 characters (max base58 Solana address length)
    pub fn validate(&self) -> Result<(), VeilError> {
        match &self.payload {
            IntentPayload::TransferSol { to, lamports } => {
                if *lamports == 0 {
                    return Err(VeilError::InvalidIntent(
                        "lamports must be greater than 0".to_string(),
                    ));
                }
                if to.is_empty() {
                    return Err(VeilError::InvalidIntent(
                        "recipient address must not be empty".to_string(),
                    ));
                }
                if to.len() > 44 {
                    return Err(VeilError::InvalidIntent(
                        "recipient address too long (max 44 chars)".to_string(),
                    ));
                }
                Ok(())
            }
        }
    }
}
