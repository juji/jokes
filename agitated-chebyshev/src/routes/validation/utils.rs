use validator::ValidationErrors;
use super::models::ValidationError;

// Helper function to convert ValidationErrors to our custom format
pub fn format_validation_errors(errors: ValidationErrors) -> Vec<ValidationError> {
  let mut formatted_errors = Vec::new();
  
  for (field, field_errors) in errors.field_errors() {
    for error in field_errors {
      formatted_errors.push(ValidationError {
        field: field.to_string(),
        message: error.message.as_ref()
          .map(|msg| msg.to_string())
          .unwrap_or_else(|| format!("Invalid value for field: {}", field)),
      });
    }
  }
  
  formatted_errors
}
