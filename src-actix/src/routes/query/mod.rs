pub mod structured;
pub mod unstructured;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg
    .route(
      "/query/unstructured",
      web::get().to(unstructured::echo_query_unstructured),
    )
    .route(
      "/query/structured",
      web::get().to(structured::echo_query_structured),
    );
}
