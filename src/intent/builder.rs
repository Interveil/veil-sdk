use crate::chains::solana::payloads::{TransferSolPayload, TransferSplPayload};
use crate::intent::intent::Intent;
use crate::types::{Chain, Nonce};
use std::time::{SystemTime, UNIX_EPOCH};

impl Intent {
    /// Create a TransferSol intent in one line.
    /// Auto-sets: version = 1, chain = Solana, action = "transfer_sol",
    /// nonce = current timestamp millis, expires_at = now + 300s
    pub fn transfer_sol(to: String, lamports: u64) -> Self {
        let now = current_timestamp_millis();
        let payload = TransferSolPayload { to, lamports };
        Self {
            version: 1,
            chain: Chain::Solana,
            action: "transfer_sol".to_string(),
            payload: serde_json::to_value(&payload).expect("payload serialization infallible"),
            nonce: now,
            expires_at: now + 300_000,
        }
    }

    /// Create a TransferSpl intent in one line.
    /// Auto-sets: version = 1, chain = Solana, action = "transfer_spl",
    /// nonce = current timestamp millis, expires_at = now + 300s
    pub fn transfer_spl(to: String, amount: u64, mint: String) -> Self {
        let now = current_timestamp_millis();
        let payload = TransferSplPayload {
            to,
            amount,
            mint,
        };
        Self {
            version: 1,
            chain: Chain::Solana,
            action: "transfer_spl".to_string(),
            payload: serde_json::to_value(&payload).expect("payload serialization infallible"),
            nonce: now,
            expires_at: now + 300_000,
        }
    }
}

/// Returns current UTC timestamp in milliseconds.
/// Used as nonce source. Not cryptographically random, but sufficient
/// for Phase 1 replay protection (unique per intent creation moment).
fn current_timestamp_millis() -> Nonce {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis() as u64
}
