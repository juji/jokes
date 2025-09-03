use actix_web::{App, HttpServer};
use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();
    
    // Get host and port from environment variables with defaults
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);
    
    println!("Starting Actix Web server at http://{}", bind_address);
    
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
