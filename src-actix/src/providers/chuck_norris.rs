use crate::providers::types::{Joke, JokeContent, JokeProvider, JokeType};
use async_trait::async_trait;
use reqwest::Client;

pub struct ChuckNorrisProvider {
    client: Client,
    categories: Vec<String>,
}

impl ChuckNorrisProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            categories: vec![
                "animal".to_string(),
                "career".to_string(),
                "celebrity".to_string(),
                "dev".to_string(),
                "explicit".to_string(),
                "fashion".to_string(),
                "food".to_string(),
                "history".to_string(),
                "money".to_string(),
                "movie".to_string(),
                "music".to_string(),
                "political".to_string(),
                "religion".to_string(),
                "science".to_string(),
                "sport".to_string(),
                "travel".to_string(),
            ],
        }
    }
}

#[async_trait]
impl JokeProvider for ChuckNorrisProvider {
    fn name(&self) -> &str {
        "Chuck Norris Jokes API"
    }

    fn base_url(&self) -> &str {
        "https://api.chucknorris.io"
    }

    async fn get_random_joke(&self) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/jokes/random", self.base_url());
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        Ok(Joke {
            id: data["id"].as_str().map(|s| s.to_string()),
            joke: JokeContent {
                content: data["value"].as_str().map(|s| s.to_string()),
                setup: None,
                punchline: None,
            },
            category: data["categories"].as_array()
                .and_then(|arr| arr.first())
                .and_then(|cat| cat.as_str())
                .map(|s| s.to_lowercase())
                .or_else(|| Some("uncategorized".to_string())),
            r#type: JokeType::Single,
            safe: None,
            lang: None,
        })
    }

    async fn get_joke_by_category(&self, category: &str) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let valid_category = if self.categories.contains(&category.to_lowercase()) {
            Some(category.to_lowercase())
        } else {
            None
        };

        if let Some(cat) = valid_category {
            let url = format!("{}/jokes/random?category={}", self.base_url(), cat);
            let response = self.client.get(&url).send().await?;
            let data: serde_json::Value = response.json().await?;

            Ok(Joke {
                id: data["id"].as_str().map(|s| s.to_string()),
                joke: JokeContent {
                    content: data["value"].as_str().map(|s| s.to_string()),
                    setup: None,
                    punchline: None,
                },
                category: data["categories"].as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|cat| cat.as_str())
                    .map(|s| s.to_lowercase())
                    .or_else(|| Some(cat)),
                r#type: JokeType::Single,
                safe: None,
                lang: None,
            })
        } else {
            self.get_random_joke().await
        }
    }

    fn get_supported_categories(&self) -> Vec<String> {
        self.categories.clone()
    }
}