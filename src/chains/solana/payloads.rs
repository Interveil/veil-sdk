use crate::chains::ChainValidator;
use crate::chains::solana::constants::{BASE58_ALPHABET, MAX_ADDRESS_LEN};
use crate::error::VeilError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferSolPayload {
    pub to: String,
    pub lamports: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferSplPayload {
    pub to: String,
    pub amount: u64,
    pub mint: String,
}

impl TransferSolPayload {
    pub fn validate(&self) -> Result<(), VeilError> {
        if self.lamports == 0 {
            return Err(VeilError::InvalidIntent(
                "lamports must be greater than 0".to_string(),
            ));
        }
        if self.to.is_empty() {
            return Err(VeilError::InvalidIntent(
                "recipient address must not be empty".to_string(),
            ));
        }
        if self.to.len() > MAX_ADDRESS_LEN {
            return Err(VeilError::InvalidIntent(
                "recipient address too long (max 44 chars)".to_string(),
            ));
        }
        if !self.to.bytes().all(|b| BASE58_ALPHABET.contains(&b)) {
            return Err(VeilError::InvalidIntent(
                "recipient address contains invalid characters".to_string(),
            ));
        }
        Ok(())
    }
}

impl TransferSplPayload {
    pub fn validate(&self) -> Result<(), VeilError> {
        if self.amount == 0 {
            return Err(VeilError::InvalidIntent(
                "amount must be greater than 0".to_string(),
            ));
        }
        if self.to.is_empty() {
            return Err(VeilError::InvalidIntent(
                "recipient address must not be empty".to_string(),
            ));
        }
        if self.to.len() > MAX_ADDRESS_LEN {
            return Err(VeilError::InvalidIntent(
                "recipient address too long (max 44 chars)".to_string(),
            ));
        }
        if !self.to.bytes().all(|b| BASE58_ALPHABET.contains(&b)) {
            return Err(VeilError::InvalidIntent(
                "recipient address contains invalid characters".to_string(),
            ));
        }
        if self.mint.is_empty() {
            return Err(VeilError::InvalidIntent("mint address must not be empty".to_string()));
        }
        if self.mint.len() > MAX_ADDRESS_LEN {
            return Err(VeilError::InvalidIntent(
                "mint address too long (max 44 chars)".to_string(),
            ));
        }
        if !self.mint.bytes().all(|b| BASE58_ALPHABET.contains(&b)) {
            return Err(VeilError::InvalidIntent(
                "mint address contains invalid characters".to_string(),
            ));
        }
        Ok(())
    }
}

pub struct SolanaAddressValidator;

impl ChainValidator for SolanaAddressValidator {
    fn validate_address(address: &str) -> Result<(), VeilError> {
        if address.is_empty() {
            return Err(VeilError::InvalidIntent("address must not be empty".to_string()));
        }
        if address.len() > MAX_ADDRESS_LEN {
            return Err(VeilError::InvalidIntent(
                "address too long (max 44 chars)".to_string(),
            ));
        }
        if !address.bytes().all(|b| BASE58_ALPHABET.contains(&b)) {
            return Err(VeilError::InvalidIntent(
                "address contains invalid base58 characters".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_sol_payload_roundtrip() {
        let payload = TransferSolPayload {
            to: "11111111111111111111111111111111".to_string(),
            lamports: 1_000_000_000,
        };
        let json = serde_json::to_value(&payload).unwrap();
        let restored: TransferSolPayload = serde_json::from_value(json).unwrap();
        assert_eq!(restored, payload);
    }

    #[test]
    fn transfer_spl_payload_roundtrip() {
        let payload = TransferSplPayload {
            to: "11111111111111111111111111111111".to_string(),
            amount: 500,
            mint: "So11111111111111111111111111111111111111112".to_string(),
        };
        let json = serde_json::to_value(&payload).unwrap();
        let restored: TransferSplPayload = serde_json::from_value(json).unwrap();
        assert_eq!(restored, payload);
    }

    #[test]
    fn validate_sol_rejects_zero_lamports() {
        let payload = TransferSolPayload {
            to: "11111111111111111111111111111111".to_string(),
            lamports: 0,
        };
        assert!(payload.validate().is_err());
    }

    #[test]
    fn validate_sol_rejects_empty_to() {
        let payload = TransferSolPayload {
            to: String::new(),
            lamports: 100,
        };
        assert!(payload.validate().is_err());
    }

    #[test]
    fn validate_sol_rejects_long_to() {
        let payload = TransferSolPayload {
            to: "a".repeat(45),
            lamports: 100,
        };
        assert!(payload.validate().is_err());
    }

    #[test]
    fn validate_sol_rejects_invalid_base58() {
        let payload = TransferSolPayload {
            to: "0OIl".to_string(),
            lamports: 100,
        };
        assert!(payload.validate().is_err());
    }

    #[test]
    fn validate_sol_accepts_valid() {
        let payload = TransferSolPayload {
            to: "11111111111111111111111111111111".to_string(),
            lamports: 500_000_000,
        };
        assert!(payload.validate().is_ok());
    }

    #[test]
    fn validate_spl_rejects_zero_amount() {
        let payload = TransferSplPayload {
            to: "11111111111111111111111111111111".to_string(),
            amount: 0,
            mint: "So11111111111111111111111111111111111111112".to_string(),
        };
        assert!(payload.validate().is_err());
    }

    #[test]
    fn validate_spl_rejects_empty_mint() {
        let payload = TransferSplPayload {
            to: "11111111111111111111111111111111".to_string(),
            amount: 100,
            mint: String::new(),
        };
        assert!(payload.validate().is_err());
    }

    #[test]
    fn validate_spl_accepts_valid() {
        let payload = TransferSplPayload {
            to: "11111111111111111111111111111111".to_string(),
            amount: 500,
            mint: "So11111111111111111111111111111111111111112".to_string(),
        };
        assert!(payload.validate().is_ok());
    }

    #[test]
    fn solana_address_validator_accepts_valid() {
        let result =
            SolanaAddressValidator::validate_address("11111111111111111111111111111111");
        assert!(result.is_ok());
    }

    #[test]
    fn solana_address_validator_rejects_invalid_chars() {
        let result = SolanaAddressValidator::validate_address("0OIl");
        assert!(result.is_err());
    }

    #[test]
    fn solana_address_validator_rejects_empty() {
        let result = SolanaAddressValidator::validate_address("");
        assert!(result.is_err());
    }

    #[test]
    fn solana_address_validator_rejects_too_long() {
        let result = SolanaAddressValidator::validate_address(&"a".repeat(45));
        assert!(result.is_err());
    }
}
