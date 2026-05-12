use crate::intent::intent::Intent;
use crate::intent::payload::IntentPayload;
use crate::types::{Chain, Nonce};
use std::time::{SystemTime, UNIX_EPOCH};

impl Intent {
    /// Create a TransferSol intent in one line.
    /// Auto-sets: version = 1, chain = Solana, nonce = current timestamp millis
    pub fn transfer_sol(to: String, lamports: u64) -> Self {
        Self {
            version: 1,
            chain: Chain::Solana,
            nonce: current_timestamp_millis(),
            payload: IntentPayload::TransferSol { to, lamports },
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
