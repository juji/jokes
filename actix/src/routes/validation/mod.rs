pub mod handlers;
pub mod models;
pub mod utils;
pub mod validators;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/validate/info", web::get().to(handlers::validation_info))
        .route("/validate/json", web::post().to(handlers::validate_json))
        .route("/validate/form", web::post().to(handlers::validate_form))
        .route("/validate/upload", web::post().to(handlers::validate_upload));
}