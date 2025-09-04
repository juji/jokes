use super::types::{Joke, JokeContent, JokeProvider, JokeType};
use async_trait::async_trait;
use reqwest::Client;

pub struct OfficialJokeProvider {
    client: Client,
    categories: Vec<String>,
}

impl OfficialJokeProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            categories: vec![
                "general".to_string(),
                "programming".to_string(),
                "knock-knock".to_string(),
                "dad".to_string(),
            ],
        }
    }
}

#[async_trait]
impl JokeProvider for OfficialJokeProvider {
    fn name(&self) -> &str {
        "Official Joke API"
    }

    fn base_url(&self) -> &str {
        "https://official-joke-api.appspot.com"
    }

    async fn get_random_joke(&self) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/random_joke", self.base_url());
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        Ok(Joke {
            id: data["id"].as_u64().map(|id| id.to_string()),
            joke: JokeContent {
                content: None,
                setup: data["setup"].as_str().map(|s| s.to_string()),
                punchline: data["punchline"].as_str().map(|s| s.to_string()),
            },
            category: data["type"].as_str().map(|s| s.to_lowercase()),
            r#type: JokeType::Twopart,
            safe: None,
            lang: None,
        })
    }

    async fn get_joke_by_category(&self, category: &str) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let valid_category = if self.categories.contains(&category.to_lowercase()) {
            category.to_lowercase()
        } else {
            "general".to_string()
        };

        let url = format!("{}/jokes/{}/random", self.base_url(), valid_category);
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        // API returns an array, so take the first joke
        let joke_data = if data.is_array() {
            data.as_array().and_then(|arr| arr.first()).unwrap_or(&data)
        } else {
            &data
        };

        Ok(Joke {
            id: joke_data["id"].as_u64().map(|id| id.to_string()),
            joke: JokeContent {
                content: None,
                setup: joke_data["setup"].as_str().map(|s| s.to_string()),
                punchline: joke_data["punchline"].as_str().map(|s| s.to_string()),
            },
            category: joke_data["type"].as_str().map(|s| s.to_lowercase()),
            r#type: JokeType::Twopart,
            safe: None,
            lang: None,
        })
    }

    fn get_supported_categories(&self) -> Vec<String> {
        self.categories.clone()
    }
}