use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Result};
use base64::{engine::general_purpose, Engine as _};
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

// Structs for POST request bodies
#[derive(Deserialize, Serialize, ToSchema)]
pub struct Base64UploadRequest {
  pub filename: String,
  pub data: String, // base64 encoded data
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct UploadResponse {
  pub message: String,
  pub files: Vec<FileInfo>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub form_data: Option<std::collections::HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub total_files: Option<usize>,
}

#[derive(Serialize, ToSchema)]
pub struct FileInfo {
  pub filename: String,
  pub size: usize,
  pub status: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub field_name: Option<String>,
}

// Single file upload handler - just echo info, don't save
/// Upload a single file
#[utoipa::path(
  post,
  path = "/upload/single",
  responses(
    (status = 200, description = "Single file upload processed", body = UploadResponse)
  ),
  tag = "upload"
)]
pub async fn upload_single_file(mut payload: Multipart) -> Result<HttpResponse> {
  let mut uploaded_files = Vec::new();

  while let Some(mut field) = payload.try_next().await? {
    let filename = field
      .content_disposition()
      .and_then(|cd| cd.get_filename())
      .map(|f| f.to_string());

    if let Some(filename) = filename {
      let mut file_size = 0;

      // Read field data to calculate size but don't save
      while let Some(chunk) = field.try_next().await? {
        file_size += chunk.len();
      }

      uploaded_files.push(FileInfo {
        filename,
        size: file_size,
        status: "received (not saved)".to_string(),
        field_name: None,
      });
    }
  }

  let response = UploadResponse {
    message: "Single file upload processed".to_string(),
    files: uploaded_files,
    form_data: None,
    total_files: None,
  };

  Ok(HttpResponse::Ok().json(response))
}

// Multiple files upload handler - just echo info, don't save
/// Upload multiple files
#[utoipa::path(
  post,
  path = "/upload/multiple",
  responses(
    (status = 200, description = "Multiple files upload processed", body = UploadResponse)
  ),
  tag = "upload"
)]
pub async fn upload_multiple_files(mut payload: Multipart) -> Result<HttpResponse> {
  let mut uploaded_files = Vec::new();
  let mut field_count = 0;

  while let Some(mut field) = payload.try_next().await? {
    field_count += 1;
    let filename = field
      .content_disposition()
      .and_then(|cd| cd.get_filename())
      .map(|f| f.to_string());

    if let Some(filename) = filename {
      let mut file_size = 0;

      // Read field data to calculate size but don't save
      while let Some(chunk) = field.try_next().await? {
        file_size += chunk.len();
      }

      uploaded_files.push(FileInfo {
        filename,
        size: file_size,
        status: "received (not saved)".to_string(),
        field_name: None,
      });
    }
  }

  let response = UploadResponse {
    message: "Multiple files upload processed".to_string(),
    files: uploaded_files,
    form_data: None,
    total_files: Some(field_count),
  };

  Ok(HttpResponse::Ok().json(response))
}

// Upload with metadata - echo info, don't save
/// Upload files with metadata
#[utoipa::path(
  post,
  path = "/upload/metadata",
  responses(
    (status = 200, description = "Upload with metadata processed", body = UploadResponse)
  ),
  tag = "upload"
)]
pub async fn upload_with_metadata(mut payload: Multipart) -> Result<HttpResponse> {
  let mut uploaded_files = Vec::new();
  let mut form_data = std::collections::HashMap::new();

  while let Some(mut field) = payload.try_next().await? {
    let field_name = field
      .content_disposition()
      .and_then(|cd| cd.get_name())
      .unwrap_or("unknown")
      .to_string();

    let filename = field
      .content_disposition()
      .and_then(|cd| cd.get_filename())
      .map(|f| f.to_string());

    if let Some(filename) = filename {
      // This is a file field
      let mut file_size = 0;

      // Read field data to calculate size but don't save
      while let Some(chunk) = field.try_next().await? {
        file_size += chunk.len();
      }

      uploaded_files.push(FileInfo {
        filename,
        size: file_size,
        status: "received (not saved)".to_string(),
        field_name: Some(field_name),
      });
    } else {
      // This is a regular form field
      let mut field_data = Vec::new();
      while let Some(chunk) = field.try_next().await? {
        field_data.extend_from_slice(&chunk);
      }
      let field_value = String::from_utf8_lossy(&field_data);
      form_data.insert(field_name, field_value.to_string());
    }
  }

  let response = UploadResponse {
    message: "Upload with metadata processed".to_string(),
    files: uploaded_files,
    form_data: Some(form_data),
    total_files: None,
  };

  Ok(HttpResponse::Ok().json(response))
}

// Base64 upload handler - just echo info, don't save
/// Upload base64 encoded file
#[utoipa::path(
  post,
  path = "/upload/base64",
  request_body = Base64UploadRequest,
  responses(
    (status = 200, description = "Base64 upload processed", body = UploadResponse),
    (status = 400, description = "Invalid base64 data")
  ),
  tag = "upload"
)]
pub async fn upload_base64(data: web::Json<Base64UploadRequest>) -> Result<HttpResponse> {
  // Decode base64 to calculate size but don't save
  match general_purpose::STANDARD.decode(&data.data) {
    Ok(decoded_data) => {
      let file_info = FileInfo {
        filename: data.filename.clone(),
        size: decoded_data.len(),
        status: "received (not saved)".to_string(),
        field_name: None,
      };

      let response = UploadResponse {
        message: "Base64 upload processed".to_string(),
        files: vec![file_info],
        form_data: None,
        total_files: None,
      };

      Ok(HttpResponse::Ok().json(response))
    }
    Err(_) => {
      let error_response = json!({
          "error": "Invalid base64 data",
          "status": "failed"
      });
      Ok(HttpResponse::BadRequest().json(error_response))
    }
  }
}

// Upload info/status endpoint
/// Get upload endpoint information
#[utoipa::path(
  get,
  path = "/upload/info",
  responses(
    (status = 200, description = "Upload endpoints information")
  ),
  tag = "upload"
)]
pub async fn upload_info() -> Result<HttpResponse> {
  let response = json!({
      "endpoints": {
          "/upload/single": "POST - Upload a single file (multipart/form-data) - echoes filename and size",
          "/upload/multiple": "POST - Upload multiple files (multipart/form-data) - echoes filenames and sizes",
          "/upload/metadata": "POST - Upload files with form data (multipart/form-data) - echoes files and form fields",
          "/upload/base64": "POST - Upload base64 encoded file (application/json) - echoes filename and decoded size",
          "/upload/info": "GET - This endpoint information"
      },
      "note": "All uploads are processed but NOT saved to disk - only metadata is returned",
      "supported_methods": ["POST", "GET"],
      "supported_formats": "All file types"
  });

  Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/upload/info", web::get().to(upload_info))
    .route("/upload/single", web::post().to(upload_single_file))
    .route("/upload/multiple", web::post().to(upload_multiple_files))
    .route("/upload/metadata", web::post().to(upload_with_metadata))
    .route("/upload/base64", web::post().to(upload_base64));
}
