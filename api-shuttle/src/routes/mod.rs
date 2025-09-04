pub mod jokes;
pub mod root;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .configure(root::configure)
    .configure(jokes::configure);
}
