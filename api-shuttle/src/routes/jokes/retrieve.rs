use actix_web::{get, web, HttpResponse, Responder};
use sqlx::types::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use agitated_chebyshev::lib::providers::manager::JokeManager;
use agitated_chebyshev::lib::providers::types::JokeType;
use agitated_chebyshev::db;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RetrieveJokesParams {
    /// Number of jokes to retrieve (default: 5, max: 20)
    count: Option<usize>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JokeResponse {
    /// List of retrieved jokes
    jokes: Vec<JokeSummary>,
    /// Number of jokes successfully saved to database
    saved_count: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JokeSummary {
    /// Database UUID of the joke
    #[schema(value_type = String)]
    id: Uuid,
    /// Category of the joke (may be null)
    category: Option<String>,
    /// Type of joke: 'single' or 'twopart'
    r#type: String,
    /// Source provider URL
    provider: String,
}

#[utoipa::path(
    get,
    path = "/jokes/retrieve",
    tag = "jokes",
    params(
        ("count" = Option<usize>, Query, description = "Number of jokes to retrieve (default: 100, max: 100)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved and saved jokes", body = JokeResponse),
        (status = 500, description = "Failed to retrieve jokes")
    )
)]
#[get("/retrieve")]
pub async fn retrieve_jokes(
    query: web::Query<RetrieveJokesParams>,
    joke_manager: web::Data<JokeManager>,
) -> impl Responder {
    let count = query.count.unwrap_or(100); // Default to 100 jokes if not specified
    let count = std::cmp::min(count, 100); // Cap at 100 jokes max

    // Get jokes in parallel
    match joke_manager.get_multiple_jokes(count).await {
        Ok(jokes_with_providers) => {
            // Prepare data for batch insert
            let mut values = Vec::new();
            let mut joke_data = Vec::new();
            
            // Start a transaction
            let pool = db::get_pool();
            let mut tx = match pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    eprintln!("Failed to start database transaction: {}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Database error"
                    }));
                }
            };
            
            // Create the VALUES part of the query and collect joke data for response
            for (i, joke_with_provider) in jokes_with_providers.iter().enumerate() {
                let joke = &joke_with_provider.joke;
                let provider = &joke_with_provider.provider;
                
                // Determine the type string
                let joke_type = match joke.r#type {
                    JokeType::Single => "single",
                    JokeType::Twopart => "twopart",
                };
                
                // Store the joke data for later use in the response
                joke_data.push((
                    joke.category.clone(),
                    joke_type.to_string(),
                    provider.clone(),
                ));
                
                // Create placeholders for this joke (e.g., $1, $2, $3...)
                let idx = i * 7 + 1;
                values.push(format!("(${}, ${}, ${}, ${}, ${}, ${}, ${})", 
                    idx, idx+1, idx+2, idx+3, idx+4, idx+5, idx+6));
            }
            
            // If no jokes to insert, return early
            if values.is_empty() {
                return HttpResponse::Ok().json(JokeResponse {
                    jokes: Vec::new(),
                    saved_count: 0,
                });
            }
            
            // Build the complete SQL query
            let values_clause = values.join(", ");
            let sql = format!(
                r#"
                INSERT INTO jokes (external_id, joke, category, type, safe, provider, lang)
                VALUES {}
                ON CONFLICT (external_id, provider)
                DO UPDATE SET 
                    joke = EXCLUDED.joke,
                    category = EXCLUDED.category,
                    type = EXCLUDED.type,
                    safe = EXCLUDED.safe,
                    lang = EXCLUDED.lang,
                    updated_at = CURRENT_TIMESTAMP
                RETURNING id
                "#,
                values_clause
            );
            
            // Create the query builder
            let mut query_builder = sqlx::query_as::<_, (Uuid,)>(&sql);
            
            // Bind all parameters
            for joke_with_provider in &jokes_with_providers {
                let joke = &joke_with_provider.joke;
                let provider = &joke_with_provider.provider;
                
                let joke_type = match joke.r#type {
                    JokeType::Single => "single",
                    JokeType::Twopart => "twopart",
                };
                
                // Extract content for database based on joke type
                let joke_json = if joke_type == "single" {
                    serde_json::json!({
                        "content": joke.joke.content,
                        "setup": null,
                        "punchline": null
                    })
                } else {
                    serde_json::json!({
                        "content": null,
                        "setup": joke.joke.setup,
                        "punchline": joke.joke.punchline
                    })
                };
                
                query_builder = query_builder
                    .bind(joke.id.as_deref())
                    .bind(joke_json)
                    .bind(&joke.category)
                    .bind(joke_type)
                    .bind(joke.safe.unwrap_or(true))
                    .bind(provider)
                    .bind(joke.lang.as_deref().unwrap_or("en"));
            }
            
            // Execute the batch insert and get the ids
            let result = match query_builder.fetch_all(&mut *tx).await {
                Ok(ids) => ids,
                Err(e) => {
                    eprintln!("Failed to execute batch insert: {}", e);
                    let _ = tx.rollback().await;
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Database error"
                    }));
                }
            };
            
            // Commit the transaction
            if let Err(e) = tx.commit().await {
                eprintln!("Failed to commit transaction: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Database error"
                }));
            }
            
            // Create joke summaries for response
            let joke_summaries = result.iter().zip(joke_data.iter())
                .map(|((id,), (category, joke_type, provider))| JokeSummary {
                    id: *id,
                    category: category.clone(),
                    r#type: joke_type.clone(),
                    provider: provider.clone(),
                })
                .collect::<Vec<_>>();
            
            let saved_count = joke_summaries.len();
            HttpResponse::Ok().json(JokeResponse {
                jokes: joke_summaries,
                saved_count,
            })
        },
        Err(e) => {
            eprintln!("Error retrieving jokes: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve jokes"
            }))
        }
    }
}
