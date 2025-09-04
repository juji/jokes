pub mod types;
pub mod jokes_api;
pub mod dad_jokes;
pub mod chuck_norris;
pub mod official_joke;
pub mod sv443_joke;
pub mod jokes_one;
pub mod manager;

pub use types::*;
pub use jokes_api::JokesApiProvider;
pub use dad_jokes::DadJokesProvider;
pub use chuck_norris::ChuckNorrisProvider;
pub use official_joke::OfficialJokeProvider;
pub use sv443_joke::Sv443JokeProvider;
pub use jokes_one::JokesOneProvider;
pub use manager::{JokeManager, JokeWithProvider, ProviderInfo};

use std::sync::Arc;

lazy_static::lazy_static! {
    pub static ref ALL_PROVIDERS: Vec<Arc<dyn JokeProvider>> = vec![
        Arc::new(JokesApiProvider::new()),
        Arc::new(DadJokesProvider::new()),
        Arc::new(ChuckNorrisProvider::new()),
        Arc::new(OfficialJokeProvider::new()),
        Arc::new(Sv443JokeProvider::new()),
        Arc::new(JokesOneProvider::new(None)),
    ];
}