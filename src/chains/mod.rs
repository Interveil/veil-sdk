pub mod ethereum;
pub mod solana;

use crate::error::VeilError;

pub trait ChainValidator {
    fn validate_address(address: &str) -> Result<(), VeilError>;
}
