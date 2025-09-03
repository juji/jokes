use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokeContent {
    pub content: Option<String>,
    pub setup: Option<String>,
    pub punchline: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Joke {
    pub id: Option<String>,
    pub joke: JokeContent,
    pub category: Option<String>,
    pub r#type: JokeType,
    pub safe: Option<bool>,
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JokeType {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "twopart")]
    Twopart,
}

#[derive(Debug, Clone)]
pub struct JokeApiResponse {
    pub data: serde_json::Value,
}

#[async_trait::async_trait]
pub trait JokeProvider: Send + Sync {
    fn name(&self) -> &str;
    fn base_url(&self) -> &str;
    async fn get_random_joke(&self) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_joke_by_category(&self, _category: &str) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        // Default implementation falls back to random joke
        self.get_random_joke().await
    }
    fn get_supported_categories(&self) -> Vec<String> {
        vec![]
    }
}