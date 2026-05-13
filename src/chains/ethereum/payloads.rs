// TODO: Implement Ethereum payload types (e.g., TransferEthPayload, TransferErc20Payload)
// Phase 2 will add:
//   pub struct TransferEthPayload { pub to: String, pub value: U256 }
//   pub struct TransferErc20Payload { pub to: String, pub amount: U256, pub contract: String }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferEthPayload {
    pub to: String,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferErc20Payload {
    pub to: String,
    pub amount: u64,
    pub contract: String,
}
