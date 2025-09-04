use actix_multipart::Multipart;
use actix_web::{HttpResponse, Result};
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};

// Upload validation request (for form fields in multipart)
#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UploadValidationRequest {
  #[validate(length(min = 2, max = 50, message = "Title must be between 2 and 50 characters"))]
  pub title: String,
  
  #[validate(length(max = 200, message = "Description must be less than 200 characters"))]
  pub description: Option<String>,
  
  #[validate(custom(function = "validate_file_category", message = "Category must be one of: image, document, video, audio"))]
  pub category: String,
}

// Response structures
#[derive(Serialize, ToSchema)]
pub struct UploadValidationResponse {
  pub success: bool,
  pub message: String,
  pub data: Option<serde_json::Value>,
  pub errors: Option<Vec<UploadValidationError>>,
}

#[derive(Serialize, ToSchema)]
pub struct UploadValidationError {
  pub field: String,
  pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct FileValidationInfo {
  pub filename: String,
  pub size: usize,
  pub content_type: Option<String>,
  pub validation_status: String,
}

// Custom validator for file category
fn validate_file_category(category: &str) -> Result<(), ValidationError> {
  let valid_categories = ["image", "document", "video", "audio"];
  if valid_categories.contains(&category) {
    Ok(())
  } else {
    Err(ValidationError::new("invalid_category"))
  }
}

// Helper function to format validation errors
fn format_validation_errors(errors: ValidationErrors) -> Vec<UploadValidationError> {
  let mut formatted_errors = Vec::new();
  
  for (field, field_errors) in errors.field_errors() {
    for error in field_errors {
      formatted_errors.push(UploadValidationError {
        field: field.to_string(),
        message: error.message.as_ref()
          .map(|msg| msg.to_string())
          .unwrap_or_else(|| format!("Invalid value for field: {}", field)),
      });
    }
  }
  
  formatted_errors
}

/// Validate multipart upload data
#[utoipa::path(
  post,
  path = "/validate/upload",
  responses(
    (status = 200, description = "Upload validation successful", body = UploadValidationResponse),
    (status = 400, description = "Upload validation failed", body = UploadValidationResponse)
  ),
  tag = "validation"
)]
pub async fn validate_upload(mut payload: Multipart) -> Result<HttpResponse> {
  let mut form_data = std::collections::HashMap::new();
  let mut files = Vec::new();
  let mut errors = Vec::new();

  // Parse multipart data
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
      let content_type = field.content_type().map(|ct| ct.to_string());

      // Read file data to get size
      while let Some(chunk) = field.try_next().await? {
        file_size += chunk.len();
      }

      // Validate file
      let mut file_validation_status = "valid".to_string();
      
      // File size validation (max 5MB)
      if file_size > 5 * 1024 * 1024 {
        errors.push(UploadValidationError {
          field: field_name.clone(),
          message: "File size must be less than 5MB".to_string(),
        });
        file_validation_status = "invalid".to_string();
      }

      // File type validation for images
      if let Some(ref ct) = content_type {
        if field_name == "image" && !ct.starts_with("image/") {
          errors.push(UploadValidationError {
            field: field_name.clone(),
            message: "File must be an image".to_string(),
          });
          file_validation_status = "invalid".to_string();
        }
      }

      files.push(FileValidationInfo {
        filename,
        size: file_size,
        content_type,
        validation_status: file_validation_status,
      });
    } else {
      // This is a regular form field
      let mut field_data = Vec::new();
      while let Some(chunk) = field.try_next().await? {
        field_data.extend_from_slice(&chunk);
      }
      let field_value = String::from_utf8_lossy(&field_data).to_string();
      form_data.insert(field_name, field_value);
    }
  }

  // Validate form fields if we have the required fields
  if let (Some(title), Some(category)) = (form_data.get("title"), form_data.get("category")) {
    let upload_request = UploadValidationRequest {
      title: title.clone(),
      description: form_data.get("description").cloned(),
      category: category.clone(),
    };

    if let Err(validation_errors) = upload_request.validate() {
      errors.extend(format_validation_errors(validation_errors));
    }
  } else {
    errors.push(UploadValidationError {
      field: "form".to_string(),
      message: "Required fields: title, category".to_string(),
    });
  }

  let response_data = json!({
    "form_data": form_data,
    "files": files,
  });

  if errors.is_empty() {
    let response = UploadValidationResponse {
      success: true,
      message: "Upload validation successful".to_string(),
      data: Some(response_data),
      errors: None,
    };
    Ok(HttpResponse::Ok().json(response))
  } else {
    let response = UploadValidationResponse {
      success: false,
      message: "Upload validation failed".to_string(),
      data: Some(response_data),
      errors: Some(errors),
    };
    Ok(HttpResponse::BadRequest().json(response))
  }
}
