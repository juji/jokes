pub mod root;
pub mod hello;
pub mod query;
pub mod method;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(root::configure)
       .configure(hello::configure)
       .configure(query::configure)
       .configure(method::configure);
}
