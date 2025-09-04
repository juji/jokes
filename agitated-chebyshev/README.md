# Agitated Chebyshev - Jokes API (Shuttle)

This is a Rust web application built with Actix Web and deployed on Shuttle, providing a jokes API with multiple provider integrations.

## Features

- Multiple joke providers (Chuck Norris, Dad Jokes, Official Joke API, etc.)
- Database persistence with PostgreSQL
- RESTful API with OpenAPI documentation
- Deployed on Shuttle platform with managed database

## Local Development

1. Install dependencies:
```bash
cargo install cargo-shuttle
```

2. Start the application locally:
```bash
cargo shuttle run
```

3. The API will be available at:
   - Main API: http://localhost:8000
   - Swagger UI: http://localhost:8000/swagger-ui/

## Deployment

Deploy to Shuttle:
```bash
cargo shuttle deploy
```

## API Endpoints

- `GET /` - Root endpoint
- `GET /hello` - Hello endpoint
- `GET /query/structured` - Get structured jokes
- `GET /query/unstructured` - Get unstructured jokes
- `POST /validation/*` - Various validation endpoints
- `GET /swagger-ui/` - API documentation

## Environment Variables

For local development, create a `.env` file:
```
DATABASE_URL=postgresql://postgres:password@localhost:5432/actix_jokes
```

Note: In production on Shuttle, the database is automatically provisioned and managed.

## Original Project

This project was migrated from the `src-actix` directory to work with Shuttle's platform.
