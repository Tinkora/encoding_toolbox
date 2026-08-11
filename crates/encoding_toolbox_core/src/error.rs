use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("input is not valid {algorithm}")]
    InvalidEncoding { algorithm: &'static str },
    #[error("input exceeds the {max_bytes}-byte limit")]
    InputTooLarge { max_bytes: usize },
    #[error("HMAC key must not be empty")]
    EmptyHmacKey,
}

impl Error {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedAlgorithm(_) => "UNSUPPORTED_ALGORITHM",
            Self::InvalidEncoding { .. } => "INVALID_ENCODING",
            Self::InputTooLarge { .. } => "INPUT_TOO_LARGE",
            Self::EmptyHmacKey => "EMPTY_HMAC_KEY",
        }
    }
}
