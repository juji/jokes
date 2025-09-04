use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Actix API",
        version = "1.0.0",
        description = "A demonstration API built with Actix Web showcasing various HTTP methods and endpoints"
    ),
    paths(
        crate::routes::root::hello,
        crate::routes::hello::hello_name,
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
        crate::routes::validation::handlers::validation_info,
        crate::routes::validation::handlers::validate_json,
        crate::routes::validation::handlers::validate_form,
        crate::routes::validation::handlers::validate_upload,
    ),
    components(
        schemas(
            crate::routes::method::ResourceData,
            crate::routes::query::structured::SearchQuery,
            crate::routes::upload::Base64UploadRequest,
            crate::routes::upload::UploadResponse,
            crate::routes::upload::FileInfo,
            crate::routes::validation::models::JsonValidationRequest,
            crate::routes::validation::models::FormValidationRequest,
            crate::routes::validation::models::UploadValidationRequest,
            crate::routes::validation::models::ValidationResponse,
            crate::routes::validation::models::ValidationError,
            crate::routes::validation::models::FileValidationInfo,
        )
    ),
    tags(
        (name = "root", description = "Root endpoint"),
        (name = "greetings", description = "Greeting endpoints"),
        (name = "method", description = "HTTP method demonstrations"),
        (name = "query", description = "Query parameter handling"),
        (name = "upload", description = "File upload demonstrations"),
        (name = "validation", description = "Data validation endpoints")
    )
)]
pub struct ApiDoc;