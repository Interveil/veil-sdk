// TODO: Implement Ethereum intent validation (address format, chain ID, etc.)

use crate::error::VeilError;
use crate::intent::intent::Intent;

pub fn validate_intent(_intent: &Intent) -> Result<(), VeilError> {
    Err(VeilError::InvalidIntent(
        "Ethereum not yet supported".to_string(),
    ))
}
