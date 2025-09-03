use regex::Regex;
use validator::ValidationError;

// Regex patterns for validation
lazy_static::lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).+$").unwrap();
    static ref PHONE_REGEX: Regex = Regex::new(r"^[\d\s\-\(\)]+$").unwrap();
}

// Custom validator for password strength
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
  if PASSWORD_REGEX.is_match(password) {
    Ok(())
  } else {
    Err(ValidationError::new("password_too_weak"))
  }
}

// Custom validator for phone format
pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
  if PHONE_REGEX.is_match(phone) {
    Ok(())
  } else {
    Err(ValidationError::new("invalid_phone_format"))
  }
}

// Custom validator for file category
pub fn validate_file_category(category: &str) -> Result<(), ValidationError> {
  let valid_categories = ["image", "document", "video", "audio"];
  if valid_categories.contains(&category) {
    Ok(())
  } else {
    Err(ValidationError::new("invalid_category"))
  }
}
