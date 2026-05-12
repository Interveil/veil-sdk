use crate::error::VeilError;
use crate::intent::intent::Intent;

impl Intent {
    pub fn to_bytes(&self) -> Result<Vec<u8>, VeilError> {
        bincode::serialize(self).map_err(|e| VeilError::Serialization(e.to_string()))
    }
}
