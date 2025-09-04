# Jokes API Projects

This repository contains two implementations of a jokes API:

1. **api-shuttle** - Built with [Shuttle.dev](https://shuttle.dev) (Rust framework)
2. **actix** - Built with [Actix Web](https://actix.rs/) (standard Rust web framework)

## 🏗️ Architecture

### api-shuttle (Shuttle.dev)
- **Framework:** Shuttle.dev
- **Endpoints:**
  - `GET /` - Root endpoint (Hello World)
  - `GET /jokes` - Retrieve jokes with filtering
  - `GET /jokes/random` - Get random joke
- **Database:** PostgreSQL
- **Documentation:** Swagger UI at `/swagger-ui/`

### actix (Actix Web)
- **Framework:** Actix Web
- **Endpoints:**
  - `GET /` - Root endpoint (Hello World)
  - `GET /hello/{name}` - Personalized greetings
  - `GET/POST/PUT/PATCH/DELETE /method/*` - HTTP method demonstrations
  - `GET /query/*` - Query parameter handling
  - `POST /upload/*` - File upload endpoints
  - `POST /validate/*` - Data validation endpoints
- **Database:** None (demonstration endpoints only)
- **Documentation:** Swagger UI at `/swagger-ui/`
- **Containerization:** Docker support

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ ([install](https://rustup.rs/))
- Docker (for actix deployment)
- PostgreSQL (for api-shuttle development)

### Running api-shuttle (Shuttle.dev)

1. **Install Shuttle CLI:**
   ```bash
   cargo install cargo-shuttle
   ```

2. **Login to Shuttle:**
   ```bash
   cargo shuttle login
   ```

3. **Run locally:**
   ```bash
   cd api-shuttle
   cargo shuttle run
   ```

4. **Deploy to Shuttle:**
   ```bash
   cargo shuttle deploy
   ```

### Running actix (Actix Web)

#### Option 1: Native Rust
```bash
cd actix
cargo run
```
Server starts at: http://localhost:3333

#### Option 2: Docker
```bash
cd actix
# Build and run
docker-compose up --build

# Or use the build script
chmod +x build.sh
./build.sh
```

#### Option 3: Custom Port
```bash
# Environment variable
PORT=3000 cargo run

# Or with Docker
PORT=3000 docker-compose up
```

## 📚 API Documentation

### api-shuttle
- **Swagger UI:** `http://localhost:8000/swagger-ui/` (local) or your Shuttle URL
- **OpenAPI Spec:** `/api-docs/openapi.json`

### actix
- **Swagger UI:** `http://localhost:3333/swagger-ui/`
- **OpenAPI Spec:** `/api-docs/openapi.json`

## 🐳 Docker Support (actix)

### Build Image
```bash
cd actix
docker build -t actix-api .
```

### Run Container
```bash
# Default port 3333
docker run -p 3333:3333 actix-api

# Custom port
docker run -p 3000:3333 -e PORT=3333 actix-api
```

### Docker Compose
```bash
cd actix
docker-compose up -d
```

## 🔧 Development

### Project Structure
```
├── api-shuttle/          # Shuttle.dev project
│   ├── src/
│   │   ├── routes/
│   │   │   ├── jokes/    # Joke endpoints
│   │   │   └── root.rs   # Root endpoint
│   │   ├── lib.rs        # Providers & database
│   │   └── main.rs       # Server entry point
│   └── Cargo.toml
├── actix/                # Actix Web project
│   ├── src/
│   │   └── routes/       # All migrated endpoints
│   ├── Dockerfile        # Multi-stage build
│   ├── docker-compose.yml
│   ├── build.sh          # Build script
│   └── .gitignore
└── README.md
```

### Available Endpoints

#### api-shuttle Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Hello World |
| GET | `/jokes` | List jokes with filters |
| GET | `/jokes/random` | Get random joke |

#### actix Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Hello World |
| GET | `/hello/{name}` | Personalized greeting |
| GET/POST/PUT/PATCH/DELETE | `/method/*` | HTTP method demos |
| GET | `/query/*` | Query parameter examples |
| POST | `/upload/*` | File upload examples |
| POST | `/validate/*` | Data validation examples |

## 🧪 Testing

### api-shuttle
```bash
cd api-shuttle
cargo test
```

### actix
```bash
cd actix
cargo test
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Commit your changes: `git commit -am 'Add some feature'`
6. Push to the branch: `git push origin feature/your-feature`
7. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Shuttle.dev Documentation](https://docs.shuttle.dev/)
- [Actix Web Documentation](https://actix.rs/docs/)
- [Rust Programming Language](https://www.rust-lang.org/)

## 📞 Support

If you have any questions or issues:

1. Check the [Issues](https://github.com/your-repo/issues) page
2. Create a new issue with detailed information
3. Contact the maintainers

---

**Note:** The api-shuttle project is optimized for Shuttle.dev deployment, while the actix project provides a standard Rust web server implementation with Docker support.