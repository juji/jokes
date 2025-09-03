use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

// Example of structured query parameters (if you know the expected fields)
#[derive(Deserialize, Serialize)]
pub struct SearchQuery {
  pub q: Option<String>,
  pub page: Option<u32>,
  pub limit: Option<u32>,
  pub sort: Option<String>,
}

pub async fn echo_query_structured(query: web::Query<SearchQuery>) -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(&*query))
}
