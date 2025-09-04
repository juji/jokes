use super::types::{Joke, JokeContent, JokeProvider, JokeType};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

pub struct Sv443JokeProvider {
    client: Client,
    categories: Vec<String>,
}

impl Sv443JokeProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            categories: vec![
                "programming".to_string(),
                "miscellaneous".to_string(),
                "dark".to_string(),
                "pun".to_string(),
                "spooky".to_string(),
                "christmas".to_string(),
            ],
        }
    }
}

#[async_trait]
impl JokeProvider for Sv443JokeProvider {
    fn name(&self) -> &str {
        "Sv443 JokeAPI"
    }

    fn base_url(&self) -> &str {
        "https://sv443.net/jokeapi/v2"
    }

    async fn get_random_joke(&self) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/joke/Any?safe-mode&type=single,twopart", self.base_url());
        let response = self.client.get(&url).send().await?;
        let data: Value = response.json().await?;
        Ok(self.normalize_joke(data))
    }

    async fn get_joke_by_category(&self, category: &str) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let valid_category = if self.categories.contains(&category.to_lowercase()) {
            category
        } else {
            "Any"
        };
        let url = format!("{}/joke/{}?safe-mode&type=single,twopart", self.base_url(), valid_category);
        let response = self.client.get(&url).send().await?;
        let data: Value = response.json().await?;
        Ok(self.normalize_joke(data))
    }

    fn get_supported_categories(&self) -> Vec<String> {
        self.categories.clone()
    }
}

impl Sv443JokeProvider {
    fn normalize_joke(&self, data: Value) -> Joke {
        if data["type"] == "single" {
            Joke {
                id: data["id"].as_u64().map(|id| id.to_string()),
                joke: JokeContent {
                    content: data["joke"].as_str().map(|s| s.to_string()),
                    setup: None,
                    punchline: None,
                },
                category: data["category"].as_str().map(|s| s.to_lowercase()),
                r#type: JokeType::Single,
                safe: data["safe"].as_bool(),
                lang: data["lang"].as_str().map(|s| s.to_string()),
            }
        } else {
            Joke {
                id: data["id"].as_u64().map(|id| id.to_string()),
                joke: JokeContent {
                    content: None,
                    setup: data["setup"].as_str().map(|s| s.to_string()),
                    punchline: data["delivery"].as_str().map(|s| s.to_string()),
                },
                category: data["category"].as_str().map(|s| s.to_lowercase()),
                r#type: JokeType::Twopart,
                safe: data["safe"].as_bool(),
                lang: data["lang"].as_str().map(|s| s.to_string()),
            }
        }
    }
}