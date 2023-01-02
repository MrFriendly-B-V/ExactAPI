use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub exactauth_host: String,
    pub mrauth_host: String,
}