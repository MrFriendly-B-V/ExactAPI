use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::routable::Routable;

mod account;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/v1")
            .configure(account::Router::configure)
        );
    }
}