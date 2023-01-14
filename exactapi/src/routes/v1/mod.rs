use std::fmt::Debug;
use actix_web::web;
use actix_web::web::ServiceConfig;
use exact_filter::Filter;
use crate::routable::Routable;

mod account;
mod contact;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/v1")
            .configure(account::Router::configure)
            .configure(contact::Router::configure)
        );
    }
}
fn set_filter<T: ToString + Debug>(existing_filter: Option<Filter<T>>, new_filter: Filter<T>, and_mode: bool) -> Filter<T> {
    match existing_filter {
        Some(x) if and_mode => x.join_and(&new_filter),
        Some(x) => x.join_or(&new_filter),
        None => new_filter
    }
}