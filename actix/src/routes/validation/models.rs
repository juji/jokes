use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// JSON validation request
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
    #[validate(custom(function = "super::validators::validate_password"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,
}

// Form validation request
#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct FormValidationRequest {
    #[validate(length(min = 2, max = 30, message = "First name must be between 2 and 30 characters"))]
    pub first_name: String,

    #[validate(length(min = 2, max = 30, message = "Last name must be between 2 and 30 characters"))]
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 10, max = 15, message = "Phone must be between 10 and 15 characters"))]
    #[validate(custom(function = "super::validators::validate_phone"))]
    pub phone: String,

    #[validate(length(min = 10, max = 500, message = "Message must be between 10 and 500 characters"))]
    pub message: String,
}

// Upload validation request (for form fields in multipart)
#[derive(Deserialize, Serialize, Validate, ToSchema)]
pub struct UploadValidationRequest {
    #[validate(length(min = 2, max = 50, message = "Title must be between 2 and 50 characters"))]
    pub title: String,

    #[validate(length(max = 200, message = "Description must be less than 200 characters"))]
    pub description: Option<String>,

    #[validate(custom(function = "super::validators::validate_file_category", message = "Category must be one of: image, document, video, audio"))]
    pub category: String,
}

// Response structures
#[derive(Serialize, ToSchema)]
pub struct ValidationResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub errors: Option<Vec<ValidationError>>,
}

#[derive(Serialize, ToSchema)]
pub struct ValidationError {
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