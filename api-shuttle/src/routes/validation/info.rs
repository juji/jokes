use actix_web::{HttpResponse, Result};
use serde_json::json;

/// Get validation endpoint information
#[utoipa::path(
  get,
  path = "/validate/info",
  responses(
    (status = 200, description = "Validation endpoints information")
  ),
  tag = "validation"
)]
pub async fn validation_info() -> Result<HttpResponse> {
  let response = json!({
    "endpoints": {
      "/validate/json": {
        "method": "POST",
        "content_type": "application/json",
        "description": "Validate JSON data with user registration fields",
        "required_fields": ["name", "email", "age", "password", "confirm_password"],
        "optional_fields": ["website"],
        "validations": {
          "name": "2-50 characters",
          "email": "valid email format",
          "age": "18-120 years",
          "website": "valid URL (optional)",
          "password": "min 8 chars, must contain uppercase, lowercase, and number",
          "confirm_password": "must match password"
        }
      },
      "/validate/form": {
        "method": "POST",
        "content_type": "application/x-www-form-urlencoded",
        "description": "Validate form data with contact form fields",
        "required_fields": ["first_name", "last_name", "email", "phone", "message"],
        "validations": {
          "first_name": "2-30 characters",
          "last_name": "2-30 characters", 
          "email": "valid email format",
          "phone": "10-15 chars, numbers/spaces/dashes/parentheses only",
          "message": "10-500 characters"
        }
      },
      "/validate/upload": {
        "method": "POST",
        "content_type": "multipart/form-data",
        "description": "Validate file upload with metadata",
        "required_fields": ["title", "category"],
        "optional_fields": ["description", "file"],
        "validations": {
          "title": "2-50 characters",
          "description": "max 200 characters (optional)",
          "category": "must be one of: image, document, video, audio",
          "file": "max 5MB, image files must have image/* content-type"
        }
      }
    },
    "common_validation_rules": {
      "email": "RFC 5322 compliant email address",
      "url": "Valid HTTP/HTTPS URL",
      "password_strength": "Minimum 8 characters with uppercase, lowercase, and number",
      "file_size": "Maximum 5MB per file",
      "string_length": "Enforced min/max character limits"
    }
  });

  Ok(HttpResponse::Ok().json(response))
}
