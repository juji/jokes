use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::env;

mod api_doc;
mod routes;

use agitated_chebyshev::db;
use api_doc::ApiDoc;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    // set database URL
    let database_url = secrets.get("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database pool from connection string
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize database connection with Shuttle's managed PostgreSQL
    db::init(pool).await.expect("Failed to initialize database");

    // Run database migrations
    db::migrate().await.expect("Failed to run migrations");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.configure(routes::configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            );
    };

    Ok(config.into())
}
