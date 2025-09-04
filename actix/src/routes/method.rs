use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ResourceData {
    pub id: Option<u32>,
    pub name: String,
    pub description: Option<String>,
}

// POST - Create a new resource
#[utoipa::path(
    post,
    path = "/method",
    tag = "method",
    request_body = ResourceData,
    responses(
        (status = 201, description = "Resource created successfully"),
    )
)]
pub async fn create_resource(data: web::Json<ResourceData>) -> Result<HttpResponse> {
    let response = json!({
        "method": "POST",
        "action": "create",
        "data": *data,
        "message": "Resource created successfully"
    });

    Ok(HttpResponse::Created().json(response))
}

// PUT - Update an existing resource
#[utoipa::path(
    put,
    path = "/method/{id}",
    tag = "method",
    request_body = ResourceData,
    params(
        ("id" = u32, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Resource updated successfully"),
    )
)]
pub async fn update_resource(
    path: web::Path<u32>,
    data: web::Json<ResourceData>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    let response = json!({
        "method": "PUT",
        "action": "update",
        "id": id,
        "data": *data,
        "message": format!("Resource {} updated successfully", id)
    });

    Ok(HttpResponse::Ok().json(response))
}

// PATCH - Partially update an existing resource
#[utoipa::path(
    patch,
    path = "/method/{id}",
    tag = "method",
    params(
        ("id" = u32, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Resource partially updated successfully"),
    )
)]
pub async fn patch_resource(
    path: web::Path<u32>,
    data: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    let response = json!({
        "method": "PATCH",
        "action": "partial_update",
        "id": id,
        "partial_data": *data,
        "message": format!("Resource {} partially updated successfully", id)
    });

    Ok(HttpResponse::Ok().json(response))
}

// DELETE - Delete a resource
#[utoipa::path(
    delete,
    path = "/method/{id}",
    tag = "method",
    params(
        ("id" = u32, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Resource deleted successfully"),
    )
)]
pub async fn delete_resource(path: web::Path<u32>) -> Result<HttpResponse> {
    let id = path.into_inner();

    let response = json!({
        "method": "DELETE",
        "action": "delete",
        "id": id,
        "message": format!("Resource {} deleted successfully", id)
    });

    Ok(HttpResponse::Ok().json(response))
}

// GET - For comparison (retrieve a resource)
// Note: HEAD requests will automatically be handled by Actix Web
// by calling this GET handler and stripping the response body
#[utoipa::path(
    get,
    path = "/method/{id}",
    tag = "method",
    params(
        ("id" = u32, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Resource retrieved successfully"),
        (status = 404, description = "Resource not found")
    )
)]
pub async fn get_resource(path: web::Path<u32>) -> Result<HttpResponse> {
    let id = path.into_inner();

    let response = json!({
        "method": "GET",
        "action": "retrieve",
        "id": id,
        "data": {
            "id": id,
            "name": format!("Resource {}", id),
            "description": "This is a sample resource"
        }
    });

    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/method", web::post().to(create_resource))
        .route("/method/{id}", web::get().to(get_resource))
        .route("/method/{id}", web::put().to(update_resource))
        .route("/method/{id}", web::patch().to(patch_resource))
        .route("/method/{id}", web::delete().to(delete_resource));
    // Note: HEAD requests to /method/{id} will automatically be handled
    // by calling get_resource and stripping the response body
}