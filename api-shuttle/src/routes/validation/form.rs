use actix_web::{web, HttpResponse, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};

// Regex for phone validation
lazy_static::lazy_static! {
    static ref PHONE_REGEX: Regex = Regex::new(r"^[\d\s\-\(\)]+$").unwrap();
}

// Form validation request model
#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct FormValidationRequest {
  #[validate(length(min = 2, max = 30, message = "First name must be between 2 and 30 characters"))]
  pub first_name: String,
  
  #[validate(length(min = 2, max = 30, message = "Last name must be between 2 and 30 characters"))]
  pub last_name: String,
  
  #[validate(email(message = "Invalid email format"))]
  pub email: String,
  
  #[validate(length(min = 10, max = 15, message = "Phone must be between 10 and 15 characters"))]
  #[validate(custom(function = "validate_phone"))]
  pub phone: String,
  
  #[validate(length(min = 10, max = 500, message = "Message must be between 10 and 500 characters"))]
  pub message: String,
}

// Response structures
#[derive(Serialize, ToSchema)]
pub struct FormValidationResponse {
  pub success: bool,
  pub message: String,
  pub data: Option<serde_json::Value>,
  pub errors: Option<Vec<FormValidationError>>,
}

#[derive(Serialize, ToSchema)]
pub struct FormValidationError {
  pub field: String,
  pub message: String,
}

// Custom validator for phone format
fn validate_phone(phone: &str) -> Result<(), ValidationError> {
  if PHONE_REGEX.is_match(phone) {
    Ok(())
  } else {
    Err(ValidationError::new("invalid_phone_format"))
  }
}

// Helper function to format validation errors
fn format_validation_errors(errors: ValidationErrors) -> Vec<FormValidationError> {
  let mut formatted_errors = Vec::new();
  
  for (field, field_errors) in errors.field_errors() {
    for error in field_errors {
      formatted_errors.push(FormValidationError {
        field: field.to_string(),
        message: error.message.as_ref()
          .map(|msg| msg.to_string())
          .unwrap_or_else(|| format!("Invalid value for field: {}", field)),
      });
    }
  }
  
  formatted_errors
}

/// Validate form data
#[utoipa::path(
  post,
  path = "/validate/form",
  request_body(
    content = FormValidationRequest,
    content_type = "application/x-www-form-urlencoded"
  ),
  responses(
    (status = 200, description = "Validation successful", body = FormValidationResponse),
    (status = 400, description = "Validation failed", body = FormValidationResponse)
  ),
  tag = "validation"
)]
pub async fn validate_form(data: web::Form<FormValidationRequest>) -> Result<HttpResponse> {
  match data.validate() {
    Ok(_) => {
      let response = FormValidationResponse {
        success: true,
        message: "Form validation successful".to_string(),
        data: Some(serde_json::to_value(&*data).unwrap()),
        errors: None,
      };
      Ok(HttpResponse::Ok().json(response))
    }
    Err(errors) => {
      let formatted_errors = format_validation_errors(errors);
      let response = FormValidationResponse {
        success: false,
        message: "Form validation failed".to_string(),
        data: None,
        errors: Some(formatted_errors),
      };
      Ok(HttpResponse::BadRequest().json(response))
    }
  }
}
