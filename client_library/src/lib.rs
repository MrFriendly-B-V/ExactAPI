use reqwest::Client;

mod error;
mod account;
pub use error::*;

#[derive(Clone)]
pub struct ExactApiClient {
    base_url: String,
    client: Client,
}

impl ExactApiClient {
    pub fn new(base_url: String, user_agent: &str) -> reqwest::Result<Self> {
        let client = Client::builder()
            .user_agent(user_agent)
            .build()?;
        Ok(Self {
            base_url,
            client
        })
    }

    pub fn get_url(&self, path: &str) -> String {
        format!("{}{path}", &self.base_url)
    }
}