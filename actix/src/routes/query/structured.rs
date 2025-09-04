use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Example of structured query parameters (if you know the expected fields)
#[derive(Deserialize, Serialize, ToSchema)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
}

/// Structured query parameter endpoint
#[utoipa::path(
    get,
    path = "/query/structured",
    responses(
        (status = 200, description = "Echo structured query parameters", body = SearchQuery)
    ),
    params(
        ("q" = Option<String>, Query, description = "Search query string"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page"),
        ("sort" = Option<String>, Query, description = "Sort field")
    ),
    tag = "query"
)]
pub async fn echo_query_structured(query: web::Query<SearchQuery>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(&*query))
}