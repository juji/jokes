use actix_web::{web, HttpResponse, Result};
use utoipa;

/// Root endpoint - Hello World
#[utoipa::path(
  get,
  path = "/",
  responses(
    (status = 200, description = "Hello World message", body = String)
  ),
  tag = "root"
)]
pub async fn hello() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().body("Hello, World!"))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(hello));
}
