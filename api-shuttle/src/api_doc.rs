use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  info(
    title = "Actix Jokes API",
    version = "1.0.0",
    description = "A demonstration API built with Actix Web showcasing various HTTP methods and endpoints"
  ),
  paths(
    crate::routes::root::hello,
    crate::routes::hello::hello_name,
    crate::routes::jokes::retrieve::retrieve_jokes,
    crate::routes::method::create_resource,
    crate::routes::method::get_resource,
    crate::routes::method::update_resource,
    crate::routes::method::patch_resource,
    crate::routes::method::delete_resource,
    crate::routes::query::structured::echo_query_structured,
    crate::routes::query::unstructured::echo_query_unstructured,
    crate::routes::upload::upload_info,
    crate::routes::upload::upload_single_file,
    crate::routes::upload::upload_multiple_files,
    crate::routes::upload::upload_with_metadata,
    crate::routes::upload::upload_base64,
    crate::routes::validation::info::validation_info,
    crate::routes::validation::json::validate_json,
    crate::routes::validation::form::validate_form,
    crate::routes::validation::upload::validate_upload,
  ),
  components(
    schemas(
      crate::routes::method::ResourceData,
      crate::routes::jokes::retrieve::RetrieveJokesParams,
      crate::routes::jokes::retrieve::JokeResponse,
      crate::routes::jokes::retrieve::JokeSummary,
      crate::routes::query::structured::SearchQuery,
      crate::routes::upload::Base64UploadRequest,
      crate::routes::upload::UploadResponse,
      crate::routes::upload::FileInfo,
      crate::routes::validation::json::JsonValidationRequest,
      crate::routes::validation::json::JsonValidationResponse,
      crate::routes::validation::json::JsonValidationError,
      crate::routes::validation::form::FormValidationRequest,
      crate::routes::validation::form::FormValidationResponse,
      crate::routes::validation::form::FormValidationError,
      crate::routes::validation::upload::UploadValidationRequest,
      crate::routes::validation::upload::UploadValidationResponse,
      crate::routes::validation::upload::UploadValidationError,
      crate::routes::validation::upload::FileValidationInfo,
    )
  ),
  tags(
    (name = "root", description = "Root endpoint"),
    (name = "greetings", description = "Greeting endpoints"),
    (name = "jokes", description = "Joke retrieval and management endpoints"),
    (name = "method", description = "HTTP method demonstrations"),
    (name = "query", description = "Query parameter handling"),
    (name = "upload", description = "File upload demonstrations"),
    (name = "validation", description = "Data validation endpoints")
  )
)]
pub struct ApiDoc;
