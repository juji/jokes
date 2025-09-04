use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::env;

mod api_doc;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    println!("Starting Actix Web server on http://localhost:{}", port);
    println!("Swagger UI available at http://localhost:{}/swagger-ui/", port);

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", api_doc::ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}