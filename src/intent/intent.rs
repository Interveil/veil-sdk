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
