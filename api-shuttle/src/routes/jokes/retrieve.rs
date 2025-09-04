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
            // Use a HashMap to deduplicate jokes by external_id and provider
            use std::collections::HashMap;
            let mut joke_map: HashMap<(Option<String>, String), (usize, serde_json::Value, Option<String>, String, bool, String)> = HashMap::new();
            let mut joke_data = Vec::new();
            
            // Deduplicate jokes based on external_id and provider
            for (i, joke_with_provider) in jokes_with_providers.iter().enumerate() {
                let joke = &joke_with_provider.joke;
                let provider = &joke_with_provider.provider;
                
                // Generate a key for deduplication (external_id, provider)
                let key = (joke.id.clone(), provider.clone());
                
                // Determine the type string
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
                
                // Store data for this joke - we'll only keep one entry per (external_id, provider)
                joke_map.insert(
                    key,
                    (
                        i,  // Original index
                        joke_json, 
                        joke.category.clone(),
                        joke_type.to_string(),
                        joke.safe.unwrap_or(true),
                        joke.lang.as_deref().unwrap_or("en").to_string()
                    )
                );
                
                // Store the joke data for later use in the response
                joke_data.push((
                    joke.category.clone(),
                    joke_type.to_string(),
                    provider.clone(),
                ));
            }
            
            // If no jokes to insert, return early
            if joke_map.is_empty() {
                return HttpResponse::Ok().json(JokeResponse {
                    jokes: Vec::new(),
                    saved_count: 0,
                });
            }
            
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
            
            // Create values part and bind parameters
            let mut values = Vec::new();
            let mut params = Vec::new();
            
            // Process deduplicated jokes
            for (idx, ((external_id, provider), (_, joke_json, category, joke_type, safe, lang))) in joke_map.iter().enumerate() {
                // Create placeholders for this joke
                let param_idx = idx * 7 + 1;
                values.push(format!("(${}, ${}, ${}, ${}, ${}, ${}, ${})",
                    param_idx, param_idx+1, param_idx+2, param_idx+3, param_idx+4, param_idx+5, param_idx+6
                ));
                
                // Store parameters in order
                params.push((
                    external_id.clone(),
                    provider.clone(),
                    joke_json.clone(),
                    category.clone(),
                    joke_type.clone(),
                    *safe,
                    lang.clone()
                ));
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
            for param in &params {
                let (ref external_id, ref provider, ref joke_json, ref category, ref joke_type, safe, ref lang) = *param;
                query_builder = query_builder
                    .bind(external_id.as_deref())
                    .bind(joke_json)
                    .bind(category)
                    .bind(joke_type)
                    .bind(safe)
                    .bind(provider)
                    .bind(lang);
            }
            
            // Execute the batch insert and get the ids
            let result = match query_builder.fetch_all(&mut *tx).await {
                Ok(ids) => ids,
                Err(e) => {
                    eprintln!("Failed to execute batch insert: {}", e);
                    let _ = tx.rollback().await;
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("Database error: {}", e)
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
