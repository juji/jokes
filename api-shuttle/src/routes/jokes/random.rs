use actix_web::{get, HttpResponse, Responder};
use sqlx::types::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use agitated_chebyshev::db;

#[derive(Debug, Serialize, ToSchema)]
pub struct RandomJokeResponse {
    /// The joke retrieved from the database
    joke: JokeDetail,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JokeDetail {
    /// Database UUID of the joke
    #[schema(value_type = String)]
    id: Uuid,
    /// Category of the joke (may be null)
    category: Option<String>,
    /// Type of joke: 'single' or 'twopart'
    r#type: String,
    /// Joke content
    content: JokeContent,
    /// Whether the joke is considered safe/SFW
    safe: bool,
    /// Language code
    lang: String,
    /// Source provider URL
    provider: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JokeContent {
    /// Content for single-line jokes
    content: Option<String>,
    /// Setup part for two-part jokes
    setup: Option<String>,
    /// Punchline for two-part jokes
    punchline: Option<String>,
}

#[utoipa::path(
    get,
    path = "/jokes/random",
    tag = "jokes",
    responses(
        (status = 200, description = "Successfully retrieved a random joke", body = RandomJokeResponse),
        (status = 404, description = "No jokes found in the database"),
        (status = 500, description = "Database error")
    )
)]
#[get("/random")]
pub async fn random_joke() -> impl Responder {
    // Get a random joke from the database
    match sqlx::query_as::<_, (Uuid, Option<String>, serde_json::Value, Option<String>, String, bool, String, String)>(
        r#"
        SELECT id, external_id, joke, category, type, safe, lang, provider
        FROM jokes
        ORDER BY RANDOM()
        LIMIT 1
        "#
    )
    .fetch_optional(db::get_pool())
    .await {
        Ok(Some((id, _, joke_json, category, joke_type, safe, lang, provider))) => {
            // Parse joke content from JSON
            let joke_content: JokeContent = match serde_json::from_value(joke_json) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Failed to parse joke content: {}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to parse joke content"
                    }));
                }
            };
            
            let joke_detail = JokeDetail {
                id,
                category,
                r#type: joke_type,
                content: joke_content,
                safe,
                lang,
                provider,
            };
            
            HttpResponse::Ok().json(RandomJokeResponse {
                joke: joke_detail,
            })
        },
        Ok(None) => {
            // No jokes found in the database
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "No jokes found in the database"
            }))
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}
