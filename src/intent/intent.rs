use crate::error::VeilError;
use crate::types::{Chain, Nonce};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    pub version: u8,
    pub chain: Chain,
    pub action: String,
    pub payload: serde_json::Value,
    pub nonce: Nonce,
    pub expires_at: u64,
}

impl Intent {
    /// Validate intent before signing.
    /// Returns Ok(()) if valid, Err(VeilError::InvalidIntent) otherwise.
    ///
    /// Performs chain-agnostic checks, then delegates to
    /// chain-specific validators.
    pub fn validate(&self) -> Result<(), VeilError> {
        if self.action.is_empty() {
            return Err(VeilError::InvalidIntent(
                "action must not be empty".to_string(),
            ));
        }
        if self.expires_at == 0 {
            return Err(VeilError::InvalidIntent(
                "expires_at must be greater than 0".to_string(),
            ));
        }
        if self.payload.is_null() {
            return Err(VeilError::InvalidIntent(
                "payload must not be null".to_string(),
            ));
        }
        match self.chain {
            Chain::Solana => crate::chains::solana::validate::validate_intent(self),
            Chain::Ethereum => crate::chains::ethereum::validate::validate_intent(self),
        }
    }
}
