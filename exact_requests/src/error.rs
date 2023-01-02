use thiserror::Error;

pub type ExactResult<T> = Result<T, ExactError>;

#[derive(Debug, Error)]
pub enum ExactError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Bad Request")]
    BadRequest,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not found")]
    NotFound,
    #[error("Timeout")]
    Timeout,
    #[error("Rate limit")]
    Ratelimit,
    #[error("Service unavailable")]
    ServiceUnavailable,
    #[error("Other error with status: {0}")]
    Other(u16)
}