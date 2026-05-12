use thiserror::Error;

#[derive(Debug, Error)]
pub enum VeilError {
    #[error("serialization failed: {0}")]
    Serialization(String),

    #[error("signing failed: {0}")]
    Signing(String),

    #[error("http request failed: {0}")]
    Http(String),

    #[error("invalid intent: {0}")]
    InvalidIntent(String),
}
