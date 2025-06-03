use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Invalid signer: {0}")]
    InvalidSigner(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid request data: {0}")]
    InvalidRequestData(String),
    #[error("URL mismatch: expected {expected}, got {actual}")]
    UrlMismatch { expected: String, actual: String },
    #[error(transparent)]
    FromHexError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    SignatureError(#[from] ethers::core::types::SignatureError),
}
