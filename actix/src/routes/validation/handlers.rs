use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Result};
use futures_util::TryStreamExt as _;
use serde_json::json;
use validator::Validate;

use super::models::*;
use super::utils::format_validation_errors;

/// Validate JSON data
#[utoipa::path(
    post,
    path = "/validate/json",
    request_body = JsonValidationRequest,
    responses(
        (status = 200, description = "Validation successful", body = ValidationResponse),
        (status = 400, description = "Validation failed", body = ValidationResponse)
    ),
    tag = "validation"
)]
pub async fn validate_json(data: web::Json<JsonValidationRequest>) -> Result<HttpResponse> {
    match data.validate() {
        Ok(_) => {
            let response = ValidationResponse {
                success: true,
                message: "JSON validation successful".to_string(),
                data: Some(serde_json::to_value(&*data).unwrap()),
                errors: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(errors) => {
            let formatted_errors = format_validation_errors(errors);
            let response = ValidationResponse {
                success: false,
                message: "JSON validation failed".to_string(),
                data: None,
                errors: Some(formatted_errors),
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
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
        (status = 200, description = "Validation successful", body = ValidationResponse),
        (status = 400, description = "Validation failed", body = ValidationResponse)
    ),
    tag = "validation"
)]
pub async fn validate_form(data: web::Form<FormValidationRequest>) -> Result<HttpResponse> {
    match data.validate() {
        Ok(_) => {
            let response = ValidationResponse {
                success: true,
                message: "Form validation successful".to_string(),
                data: Some(serde_json::to_value(&*data).unwrap()),
                errors: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(errors) => {
            let formatted_errors = format_validation_errors(errors);
            let response = ValidationResponse {
                success: false,
                message: "Form validation failed".to_string(),
                data: None,
                errors: Some(formatted_errors),
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

/// Validate multipart upload data
#[utoipa::path(
    post,
    path = "/validate/upload",
    responses(
        (status = 200, description = "Upload validation successful", body = ValidationResponse),
        (status = 400, description = "Upload validation failed", body = ValidationResponse)
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
                errors.push(ValidationError {
                    field: field_name.clone(),
                    message: "File size must be less than 5MB".to_string(),
                });
                file_validation_status = "invalid".to_string();
            }

            // File type validation for images
            if let Some(ref ct) = content_type {
                if field_name == "image" && !ct.starts_with("image/") {
                    errors.push(ValidationError {
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
        errors.push(ValidationError {
            field: "form".to_string(),
            message: "Required fields: title, category".to_string(),
        });
    }

    let response_data = json!({
        "form_data": form_data,
        "files": files,
    });

    if errors.is_empty() {
        let response = ValidationResponse {
            success: true,
            message: "Upload validation successful".to_string(),
            data: Some(response_data),
            errors: None,
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = ValidationResponse {
            success: false,
            message: "Upload validation failed".to_string(),
            data: Some(response_data),
            errors: Some(errors),
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

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