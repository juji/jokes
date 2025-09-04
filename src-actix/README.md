# Actix Web Hello World

A simple hello world application using Actix Web framework in Rust.

## Configuration

The application uses environment variables for configuration:

- `HOST`: Server host address (default: `127.0.0.1`)
- `PORT`: Server port (default: `8080`)
- `DATABASE_URL`: PostgreSQL connection string (required)

You can set these variables in a `.env` file or export them directly.

## Running the application

### Prerequisites
- Rust (latest stable version)
- PostgreSQL database

### Option 1: Using Docker Compose (Recommended for local development)

1. Navigate to the src-actix directory
2. Start the PostgreSQL database:
   ```bash
   docker-compose up -d postgres
   ```
3. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```
4. Run the application:
   ```bash
   cargo run
   ```

### Option 2: Using existing PostgreSQL

1. Make sure PostgreSQL is running and accessible
2. Navigate to the src-actix directory
3. Copy `.env.example` to `.env` and update `DATABASE_URL` with your database connection:
   ```bash
   cp .env.example .env
   ```
4. Run the application:
   ```bash
   cargo run
   ```

### Option 3: Run with custom environment variables
```bash
DATABASE_URL=postgresql://user:password@localhost:5432/db_name HOST=0.0.0.0 PORT=3000 cargo run
```

### Database Management (Optional)
To also run pgAdmin for database management:
```bash
docker-compose --profile admin up -d
```
Then access pgAdmin at http://localhost:8081 with admin@actix-jokes.local / admin

## Database Migrations

The application uses sqlx migrations to manage database schema changes. Migrations run automatically when the application starts via `db::migrate()`.

### Manual Migration Commands

```bash
# Run all pending migrations
sqlx migrate run

# Check migration status
sqlx migrate info

# Revert the latest migration
sqlx migrate revert

# Create a new migration
sqlx migrate add migration_description
```

### Migration Files

Migrations are stored in the `migrations/` directory with the format:
- `YYYYMMDDHHMMSS_description.sql` for up migrations
- `-- +migrate Down` section for down migrations

### Programmatic Migration

The application automatically runs migrations on startup by calling `db::migrate(database_url)` in `main.rs`.

## Endpoints

- `GET /` - Returns "Hello, World!"
- `GET /hello/{name}` - Returns "Hello, {name}!" where {name} is the path parameter

## Example requests

```bash
# Default configuration (localhost:8080)
curl http://localhost:8080/
curl http://localhost:8080/hello/Rust

# Custom port example
curl http://localhost:3000/
curl http://localhost:3000/hello/Rust
```
