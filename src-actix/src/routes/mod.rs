pub mod hello;
pub mod method;
pub mod query;
pub mod root;
pub mod upload;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .configure(root::configure)
    .configure(hello::configure)
    .configure(query::configure)
    .configure(method::configure)
    .configure(upload::configure);
}
