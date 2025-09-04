use super::types::{Joke, JokeContent, JokeProvider, JokeType};
use async_trait::async_trait;
use reqwest::Client;

pub struct DadJokesProvider {
    client: Client,
}

impl DadJokesProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl JokeProvider for DadJokesProvider {
    fn name(&self) -> &str {
        "icanhazdadjoke"
    }

    fn base_url(&self) -> &str {
        "https://icanhazdadjoke.com"
    }

    async fn get_random_joke(&self) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.client
            .get(self.base_url())
            .header("Accept", "application/json")
            .header("User-Agent", "Jokes App (https://github.com/yourapp)")
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;

        Ok(Joke {
            id: data["id"].as_str().map(|s| s.to_string()),
            joke: JokeContent {
                content: data["joke"].as_str().map(|s| s.to_string()),
                setup: None,
                punchline: None,
            },
            category: Some("dad jokes".to_string()),
            r#type: JokeType::Single,
            safe: None,
            lang: None,
        })
    }

    fn get_supported_categories(&self) -> Vec<String> {
        vec!["dad jokes".to_string()]
    }
}