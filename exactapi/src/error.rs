use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use thiserror::Error;
use exact_requests::ExactError;

pub type WebResult<T> = Result<T, Error>;

#[allow(unused)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Authorization error: {0}")]
    AuthClient(#[from] mrauth::Error),
    #[error("Authorization failed")]
    AuthError(#[from] mrauth::actix::AuthError),
    #[error("Exact authorization client error: {0}")]
    ExactAuthClient(#[from] exactauth::Error),
    #[error("Exact API error: {0}")]
    ExactApi(#[from] ExactError),
    #[error("Not found")]
    NotFound,
    #[error("Bad request")]
    BadRequest,
    #[error("Bad request: {0}")]
    BadRequestMsg(String)
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AuthError(e) => e.status_code(),
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest | Self::BadRequestMsg(_) => StatusCode::BAD_REQUEST,
            Self::ExactAuthClient(_) => StatusCode::BAD_GATEWAY,
            Self::AuthClient(e) => match e {
                mrauth::Error::Reqwest(_) => StatusCode::BAD_GATEWAY,
                mrauth::Error::UnknownToken
                | mrauth::Error::MissingScopes => StatusCode::FORBIDDEN,
                mrauth::Error::ProtocolError(_) | mrauth::Error::EncodeDecodeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::ExactApi(e) => match e {
                ExactError::NotFound => StatusCode::NOT_FOUND,
                ExactError::Reqwest(_) => StatusCode::BAD_GATEWAY,
                ExactError::Forbidden => StatusCode::FORBIDDEN,
                ExactError::Ratelimit => StatusCode::TOO_MANY_REQUESTS,
                ExactError::Timeout => StatusCode::BAD_GATEWAY,
                ExactError::ServiceUnavailable => StatusCode::BAD_GATEWAY,
                ExactError::BadRequest => StatusCode::INTERNAL_SERVER_ERROR,
                ExactError::Other(_) => StatusCode::BAD_GATEWAY,
            }
        }

    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::AuthError(e) => e.error_response(),
            _ => HttpResponse::build(self.status_code()).body(self.to_string().into_bytes())
        }
    }
}