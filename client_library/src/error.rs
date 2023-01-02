use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExactApiError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Auth error: {0}")]
    Auth(#[from] mrauth::Error),
    #[error("Serialization error: {0}")]
    SerdeQs(#[from] serde_qs::Error),
    #[error("Protobuf decoding error: {0}")]
    Decode(#[from] reqwest_protobuf::DecodeError),
}