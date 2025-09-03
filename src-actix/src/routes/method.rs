use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct ResourceData {
    id: Option<u32>,
    name: String,
    description: Option<String>,
}

// POST - Create a new resource
async fn create_resource(data: web::Json<ResourceData>) -> Result<HttpResponse> {
    let response = json!({
        "method": "POST",
        "action": "create",
        "data": *data,
        "message": "Resource created successfully"
    });
    
    Ok(HttpResponse::Created().json(response))
}

// PUT - Update an existing resource
async fn update_resource(
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
async fn patch_resource(
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
async fn delete_resource(path: web::Path<u32>) -> Result<HttpResponse> {
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
async fn get_resource(path: web::Path<u32>) -> Result<HttpResponse> {
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
    cfg.route("/method", web::post().to(create_resource))
       .route("/method/{id}", web::get().to(get_resource))
       .route("/method/{id}", web::put().to(update_resource))
       .route("/method/{id}", web::patch().to(patch_resource))
       .route("/method/{id}", web::delete().to(delete_resource));
    // Note: HEAD requests to /method/{id} will automatically be handled
    // by calling get_resource and stripping the response body
}
