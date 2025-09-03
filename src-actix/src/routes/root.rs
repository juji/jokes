use actix_web::{web, HttpResponse, Result};

async fn hello() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().body("Hello, World!"))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(hello));
}
