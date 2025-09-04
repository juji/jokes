use actix_web::{web, HttpResponse, Result};
use utoipa;

/// Personalized hello endpoint
#[utoipa::path(
    get,
    path = "/hello/{name}",
    responses(
        (status = 200, description = "Personalized hello message", body = String)
    ),
    params(
        ("name" = String, Path, description = "Name to greet")
    ),
    tag = "greetings"
)]
pub async fn hello_name(path: web::Path<String>) -> Result<HttpResponse> {
    let name = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("Hello, {}!", name)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/hello/{name}", web::get().to(hello_name));
}