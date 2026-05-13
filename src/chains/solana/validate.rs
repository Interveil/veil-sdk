use crate::chains::solana::payloads::{TransferSolPayload, TransferSplPayload};
use crate::error::VeilError;
use crate::intent::intent::Intent;

pub fn validate_intent(intent: &Intent) -> Result<(), VeilError> {
    match intent.action.as_str() {
        "transfer_sol" => {
            let payload: TransferSolPayload = serde_json::from_value(intent.payload.clone())
                .map_err(|e| VeilError::InvalidIntent(format!("invalid payload: {}", e)))?;
            payload.validate()
        }
        "transfer_spl" => {
            let payload: TransferSplPayload = serde_json::from_value(intent.payload.clone())
                .map_err(|e| VeilError::InvalidIntent(format!("invalid payload: {}", e)))?;
            payload.validate()
        }
        _ => Err(VeilError::InvalidIntent(format!(
            "unknown action: {}",
            intent.action
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Chain;

    fn make_intent(action: &str, payload: serde_json::Value) -> Intent {
        Intent {
            version: 1,
            chain: Chain::Solana,
            action: action.to_string(),
            payload,
            nonce: 42,
            expires_at: 999,
        }
    }

    #[test]
    fn validate_rejects_unknown_action() {
        let intent = make_intent("unknown_action", serde_json::json!({}));
        assert!(validate_intent(&intent).is_err());
    }

    #[test]
    fn validate_sol_rejects_zero_lamports() {
        let intent = make_intent(
            "transfer_sol",
            serde_json::json!({"to": "11111111111111111111111111111111", "lamports": 0}),
        );
        assert!(validate_intent(&intent).is_err());
    }

    #[test]
    fn validate_sol_accepts_valid() {
        let intent = make_intent(
            "transfer_sol",
            serde_json::json!({"to": "11111111111111111111111111111111", "lamports": 500_000_000}),
        );
        assert!(validate_intent(&intent).is_ok());
    }

    #[test]
    fn validate_spl_accepts_valid() {
        let intent = make_intent(
            "transfer_spl",
            serde_json::json!({
                "to": "11111111111111111111111111111111",
                "amount": 500,
                "mint": "So11111111111111111111111111111111111111112"
            }),
        );
        assert!(validate_intent(&intent).is_ok());
    }
}
