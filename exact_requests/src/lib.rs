
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod error;
mod endpoints;

pub use error::*;
pub use endpoints::*;

const BASE_URI: &str = "https://start.exactonline.nl";

pub struct Api {
    token: String,
}

impl Api {
    pub fn new(token: String) -> Self {
        Self {
            token,
        }
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> ExactResult<Vec<T>> {
        let response = client()
            .get(get_path(path))
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .send()
            .await?;
        let response = check_status(response)?;
        let payload: ExactResponse<T> = response
            .json()
            .await?;
        Ok(payload.d.results)
    }

    #[allow(unused)]
    async fn post_empty<T: Serialize>(&self, path: &str, body: T) -> ExactResult<()> {
        let response = client()
            .post(get_path(path))
            .bearer_auth(&self.token)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&body)
            .send()
            .await?;
        check_status(response)?;

        Ok(())
    }

    #[allow(unused)]
    async fn post<T: Serialize, R: DeserializeOwned>(&self, path: &str, body: T) -> ExactResult<Vec<R>> {
        let response = client()
            .post(get_path(path))
            .bearer_auth(&self.token)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&body)
            .send()
            .await?;
        let response = check_status(response)?;
        let payload: ExactResponse<R> = response
            .json()
            .await?;
        Ok(payload.d.results)
    }
}

fn check_status(response: Response) -> ExactResult<Response> {
    if response.status().is_success() {
        return Ok(response)
    }

    match response.status().as_u16() {
        400 => Err(ExactError::BadRequest),
        403 => Err(ExactError::Forbidden),
        404 => Err(ExactError::NotFound),
        408 => Err(ExactError::Timeout),
        429 => Err(ExactError::Ratelimit),
        503 => Err(ExactError::ServiceUnavailable),
        v @ _ => Err(ExactError::Other(v))
    }
}

fn client() -> Client {
    Client::builder()
        .user_agent(format!("MrFriendly ExactAPI v{}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap()
}

fn get_path(path: &str) -> String {
    format!("{BASE_URI}{path}")
}

#[derive(Deserialize)]
pub struct ExactResponse<T> {
    d: ResponseData<T>,
}

#[derive(Deserialize)]
pub struct ResponseData<T> {
    results: Vec<T>,
}