use actix_web::{App, HttpServer};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_doc;
mod routes;

use api_doc::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load environment variables from .env file if it exists
  dotenvy::dotenv().ok();

  // Get host and port from environment variables with defaults
  let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
  let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
  let bind_address = format!("{}:{}", host, port);

  println!("Starting Actix Web server at http://{}", bind_address);
  println!("Swagger UI available at http://{}/swagger-ui/", bind_address);

  HttpServer::new(|| {
    App::new()
      .configure(routes::configure_routes)
      .service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
      )
  })
    .bind(&bind_address)?
    .run()
    .await
}
