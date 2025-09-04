use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  info(
    title = "Jokes API",
    version = "1.0.0",
    description = "A jokes API built with Shuttle"
  ),
  paths(
    crate::routes::root::hello,
    crate::routes::jokes::retrieve::retrieve_jokes,
    crate::routes::jokes::random::random_joke,
  ),
  components(
    schemas(
      crate::routes::jokes::retrieve::RetrieveJokesParams,
      crate::routes::jokes::retrieve::JokeResponse,
      crate::routes::jokes::retrieve::JokeSummary,
      crate::routes::jokes::random::RandomJokeResponse,
      crate::routes::jokes::random::JokeDetail,
      crate::routes::jokes::random::JokeContent,
    )
  ),
  tags(
    (name = "root", description = "Root endpoint"),
    (name = "jokes", description = "Joke retrieval and management endpoints")
  )
)]
pub struct ApiDoc;
