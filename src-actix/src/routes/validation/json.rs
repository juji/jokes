use actix_web::{web, HttpResponse, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};

// Regex for password validation
lazy_static::lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).+$").unwrap();
}

// JSON validation request model
#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct JsonValidationRequest {
  #[validate(length(min = 2, max = 50, message = "Name must be between 2 and 50 characters"))]
  pub name: String,
  
  #[validate(email(message = "Invalid email format"))]
  pub email: String,
  
  #[validate(range(min = 18, max = 120, message = "Age must be between 18 and 120"))]
  pub age: u8,
  
  #[validate(url(message = "Invalid URL format"))]
  pub website: Option<String>,
  
  #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
  #[validate(custom(function = "validate_password"))]
  pub password: String,
  
  #[validate(must_match(other = "password", message = "Passwords do not match"))]
  pub confirm_password: String,
}

// Response structures
#[derive(Serialize, ToSchema)]
pub struct JsonValidationResponse {
  pub success: bool,
  pub message: String,
  pub data: Option<serde_json::Value>,
  pub errors: Option<Vec<JsonValidationError>>,
}

#[derive(Serialize, ToSchema)]
pub struct JsonValidationError {
  pub field: String,
  pub message: String,
}

// Custom validator for password strength
fn validate_password(password: &str) -> Result<(), ValidationError> {
  if PASSWORD_REGEX.is_match(password) {
    Ok(())
  } else {
    Err(ValidationError::new("password_too_weak"))
  }
}

// Helper function to format validation errors
fn format_validation_errors(errors: ValidationErrors) -> Vec<JsonValidationError> {
  let mut formatted_errors = Vec::new();
  
  for (field, field_errors) in errors.field_errors() {
    for error in field_errors {
      formatted_errors.push(JsonValidationError {
        field: field.to_string(),
        message: error.message.as_ref()
          .map(|msg| msg.to_string())
          .unwrap_or_else(|| format!("Invalid value for field: {}", field)),
      });
    }
  }
  
  formatted_errors
}

/// Validate JSON data
#[utoipa::path(
  post,
  path = "/validate/json",
  request_body = JsonValidationRequest,
  responses(
    (status = 200, description = "Validation successful", body = JsonValidationResponse),
    (status = 400, description = "Validation failed", body = JsonValidationResponse)
  ),
  tag = "validation"
)]
pub async fn validate_json(data: web::Json<JsonValidationRequest>) -> Result<HttpResponse> {
  match data.validate() {
    Ok(_) => {
      let response = JsonValidationResponse {
        success: true,
        message: "JSON validation successful".to_string(),
        data: Some(serde_json::to_value(&*data).unwrap()),
        errors: None,
      };
      Ok(HttpResponse::Ok().json(response))
    }
    Err(errors) => {
      let formatted_errors = format_validation_errors(errors);
      let response = JsonValidationResponse {
        success: false,
        message: "JSON validation failed".to_string(),
        data: None,
        errors: Some(formatted_errors),
      };
      Ok(HttpResponse::BadRequest().json(response))
    }
  }
}
