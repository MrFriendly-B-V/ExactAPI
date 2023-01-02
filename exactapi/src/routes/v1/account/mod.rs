use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::routable::Routable;

mod list;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/account")
            .route("/list", web::get().to(list::list))
        );
    }
}