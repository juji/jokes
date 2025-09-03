use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;
use std::collections::HashMap;

pub async fn echo_query_unstructured(req: HttpRequest) -> Result<HttpResponse> {
  // Using web::Query with HashMap for completely dynamic parameters
  let query: web::Query<HashMap<String, String>> =
    web::Query::from_query(req.query_string()).unwrap_or_else(|_| web::Query(HashMap::new()));

  let response = json!({
      "query_string": req.query_string(),
      "parameters": *query
  });

  Ok(HttpResponse::Ok().json(response))
}
