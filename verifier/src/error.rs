use thiserror::Error;

/// Error codes for verifier operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerifierErrorCode {
    // Application-specific errors (1000s)
    InvalidSignature = 1001,
    InvalidSigner = 1002,
    MissingField = 1003,
    InvalidRequestData = 1004,
    UrlMismatch = 1005,

    // External library errors (2000s)
    FromHexError = 2001,
    SerdeJsonError = 2002,
    SignatureError = 2003,
}

impl VerifierErrorCode {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Error, Debug)]
pub enum VerifierError {
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

impl VerifierError {
    /// Get the error code for this error variant
    pub fn code(&self) -> VerifierErrorCode {
        match self {
            VerifierError::InvalidSignature(_) => VerifierErrorCode::InvalidSignature,
            VerifierError::InvalidSigner(_) => VerifierErrorCode::InvalidSigner,
            VerifierError::MissingField(_) => VerifierErrorCode::MissingField,
            VerifierError::InvalidRequestData(_) => VerifierErrorCode::InvalidRequestData,
            VerifierError::UrlMismatch { .. } => VerifierErrorCode::UrlMismatch,
            VerifierError::FromHexError(_) => VerifierErrorCode::FromHexError,
            VerifierError::SerdeJsonError(_) => VerifierErrorCode::SerdeJsonError,
            VerifierError::SignatureError(_) => VerifierErrorCode::SignatureError,
        }
    }
}
