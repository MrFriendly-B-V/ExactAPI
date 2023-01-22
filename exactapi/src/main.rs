use std::io;
use std::time::Duration;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use exactauth::ExactAuthClient;
use moka::future::Cache;
use mrauth::MrAuthClient;
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use proto::Account;
use crate::config::Config;
use crate::routable::Routable;

mod config;
mod error;
mod routes;
mod exact_api_conversion;
mod routable;

pub type AuthData = web::Data<MrAuthClient>;
pub type ExactAuthData = web::Data<ExactAuthClient>;

#[cfg(not(debug_assertions))]
const BIND_PORT: u16 = 8080;
#[cfg(debug_assertions)]
const BIND_PORT: u16 = 8082;

const EXACT_CACHE_SIZE: u64 = 10_000;
const EXACT_CACHE_TTL: Duration = Duration::from_secs(86_400);

type AccountCache = web::Data<Cache<String, Account>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    setup_tracing();
    info!("Starting server");

    let config: Config = envy::from_env()
        .expect("Reading configuration");

    let mrauth_client = MrAuthClient::new(
        &format!("MrFriendly ExactAPI {}", env!("CARGO_PKG_VERSION")),
        config.mrauth_host.clone(),
    );

    let exactauth_client = ExactAuthClient::new(
        config.exactauth_host.clone(),
        &format!("MrFriendly ExactAPI {}", env!("CARGO_PKG_VERSION")),
    ).expect("Configuring ExactAuth client");

    let account_cache = Cache::builder()
        .max_capacity(EXACT_CACHE_SIZE)
        .time_to_idle(EXACT_CACHE_TTL)
        .build();

    HttpServer::new(move || App::new()
        .wrap(TracingLogger::<NoiselessRootSpanBuilder>::new())
        .wrap(Cors::permissive())
        .app_data(AuthData::new(mrauth_client.clone()))
        .app_data(ExactAuthData::new(exactauth_client.clone()))
        .app_data(AccountCache::new(account_cache.clone()))
        .configure(routes::Router::configure)
    ).bind(&format!("0.0.0.0:{BIND_PORT}"))?.run().await
}

fn setup_tracing() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO")
    }

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(layer().compact())
        .init()
}