# Actix Web Hello World

A simple hello world application using Actix Web framework in Rust.

## Configuration

The application uses environment variables for configuration:

- `HOST`: Server host address (default: `127.0.0.1`)
- `PORT`: Server port (default: `8080`)

You can set these variables in a `.env` file or export them directly.

## Running the application

1. Make sure you have Rust installed
2. Navigate to the src-actix directory
3. (Optional) Copy `.env.example` to `.env` and modify as needed:
   ```bash
   cp .env.example .env
   ```
4. Run the application:
   ```bash
   cargo run
   ```

Alternatively, you can run with custom environment variables:
```bash
HOST=0.0.0.0 PORT=3000 cargo run
```

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
