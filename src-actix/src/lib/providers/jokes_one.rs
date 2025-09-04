use super::types::{Joke, JokeContent, JokeProvider, JokeType};
use async_trait::async_trait;
use reqwest::Client;

pub struct JokesOneProvider {
    client: Client,
    api_key: Option<String>,
}

impl JokesOneProvider {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl JokeProvider for JokesOneProvider {
    fn name(&self) -> &str {
        "Jokes One API"
    }

    fn base_url(&self) -> &str {
        "https://api.jokes.one"
    }

    async fn get_random_joke(&self) -> Result<Joke, Box<dyn std::error::Error + Send + Sync>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        if let Some(ref key) = self.api_key {
            headers.insert("X-JokesOne-Api-Secret", key.parse().unwrap());
        }

        let response = self.client
            .get(&format!("{}/jod", self.base_url()))
            .headers(headers)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data: serde_json::Value = resp.json().await?;
                    Ok(Joke {
                        id: data["joke"].as_array()
                            .and_then(|arr| arr.first())
                            .and_then(|j| j["id"].as_str())
                            .map(|s| s.to_string()),
                        joke: JokeContent {
                            content: data["joke"].as_array()
                                .and_then(|arr| arr.first())
                                .and_then(|j| j["text"].as_str())
                                .map(|s| s.to_string())
                                .or_else(|| data["contents"]["jokes"].as_array()
                                    .and_then(|arr| arr.first())
                                    .and_then(|j| j["joke"]["text"].as_str())
                                    .map(|s| s.to_string())),
                            setup: None,
                            punchline: None,
                        },
                        category: data["joke"].as_array()
                            .and_then(|arr| arr.first())
                            .and_then(|j| j["category"].as_str())
                            .map(|s| s.to_string()),
                        r#type: JokeType::Single,
                        safe: None,
                        lang: None,
                    })
                } else {
                    // Fallback joke if API fails
                    Ok(Joke {
                        joke: JokeContent {
                            content: Some("Why don't scientists trust atoms? Because they make up everything!".to_string()),
                            setup: None,
                            punchline: None,
                        },
                        category: Some("science".to_string()),
                        r#type: JokeType::Single,
                        id: None,
                        safe: None,
                        lang: None,
                    })
                }
            }
            Err(_) => {
                // Fallback joke if network fails
                Ok(Joke {
                    joke: JokeContent {
                        content: Some("Why don't scientists trust atoms? Because they make up everything!".to_string()),
                        setup: None,
                        punchline: None,
                    },
                    category: Some("science".to_string()),
                    r#type: JokeType::Single,
                    id: None,
                    safe: None,
                    lang: None,
                })
            }
        }
    }

    fn get_supported_categories(&self) -> Vec<String> {
        vec![
            "general".to_string(),
            "dad".to_string(),
            "programming".to_string(),
            "science".to_string(),
        ]
    }
}