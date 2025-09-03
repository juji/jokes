use crate::providers::types::{Joke, JokeProvider};
use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
pub struct JokeManager {
    providers: Vec<Arc<dyn JokeProvider>>,
}

impl JokeManager {
    pub fn new(providers: Vec<Arc<dyn JokeProvider>>) -> Self {
        Self { providers }
    }

    pub fn with_all_providers() -> Self {
        Self::new(crate::providers::ALL_PROVIDERS.clone())
    }

    /// Get a random joke from a random provider
    pub async fn get_random_joke(&self) -> Result<JokeWithProvider, Box<dyn std::error::Error + Send + Sync>> {
        if self.providers.is_empty() {
            return Err("No providers available".into());
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.providers.len());
        let provider = &self.providers[index];

        let joke = provider.get_random_joke().await?;
        Ok(JokeWithProvider {
            joke,
            provider: provider.base_url().to_string(),
        })
    }

    /// Get a joke from a specific provider
    pub async fn get_joke_from_provider(&self, provider_name: &str) -> Result<JokeWithProvider, Box<dyn std::error::Error + Send + Sync>> {
        let provider = self.providers.iter()
            .find(|p| p.name().to_lowercase().contains(&provider_name.to_lowercase()))
            .ok_or_else(|| format!("Provider '{}' not found", provider_name))?;

        let joke = provider.get_random_joke().await?;
        Ok(JokeWithProvider {
            joke,
            provider: provider.base_url().to_string(),
        })
    }

    /// Get a joke by category from any provider that supports it
    pub async fn get_joke_by_category(&self, category: &str) -> Result<JokeWithProvider, Box<dyn std::error::Error + Send + Sync>> {
        let providers_with_category: Vec<_> = self.providers.iter()
            .filter(|p| p.get_supported_categories().iter()
                .any(|cat| cat.to_lowercase().contains(&category.to_lowercase())))
            .collect();

        if providers_with_category.is_empty() {
            // Fallback to random joke if no provider supports the category
            return self.get_random_joke().await;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..providers_with_category.len());
        let provider = providers_with_category[index];

        let joke = provider.get_joke_by_category(category).await?;
        Ok(JokeWithProvider {
            joke,
            provider: provider.base_url().to_string(),
        })
    }

    /// Get multiple jokes from random providers
    pub async fn get_multiple_jokes(&self, count: usize) -> Result<Vec<JokeWithProvider>, Box<dyn std::error::Error + Send + Sync>> {
        let mut jokes = Vec::new();

        for _ in 0..count {
            match self.get_random_joke().await {
                Ok(joke) => jokes.push(joke),
                Err(e) => eprintln!("Failed to get joke: {}", e),
            }
        }

        Ok(jokes)
    }

    /// List all available providers
    pub fn get_providers(&self) -> Vec<ProviderInfo> {
        self.providers.iter().map(|provider| ProviderInfo {
            name: provider.name().to_string(),
            base_url: provider.base_url().to_string(),
            categories: provider.get_supported_categories(),
        }).collect()
    }

    /// Get all available categories across all providers
    pub fn get_all_categories(&self) -> Vec<String> {
        let mut categories = std::collections::HashSet::new();

        for provider in &self.providers {
            for category in provider.get_supported_categories() {
                categories.insert(category);
            }
        }

        let mut result: Vec<_> = categories.into_iter().collect();
        result.sort();
        result
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JokeWithProvider {
    pub joke: Joke,
    pub provider: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderInfo {
    pub name: String,
    pub base_url: String,
    pub categories: Vec<String>,
}