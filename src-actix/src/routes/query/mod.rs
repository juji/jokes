pub mod unstructured;
pub mod structured;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/query/unstructured", web::get().to(unstructured::echo_query_unstructured))
       .route("/query/structured", web::get().to(structured::echo_query_structured));
}
