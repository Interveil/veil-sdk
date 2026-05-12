use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentPayload {
    TransferSol {
        to: String,
        lamports: u64,
    },
}
