pub mod form;
pub mod info;
pub mod json;
pub mod upload;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/validate/info", web::get().to(info::validation_info))
    .route("/validate/json", web::post().to(json::validate_json))
    .route("/validate/form", web::post().to(form::validate_form))
    .route("/validate/upload", web::post().to(upload::validate_upload));
}