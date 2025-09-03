use actix_web::{web, HttpResponse, Result};

async fn hello_name(path: web::Path<String>) -> Result<HttpResponse> {
    let name = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("Hello, {}!", name)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/hello/{name}", web::get().to(hello_name));
}
