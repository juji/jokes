# Jokes API

A comprehensive joke API with multiple providers and PostgreSQL storage.

## Quick Start

### Prerequisites
- Node.js 18+
- Docker and Docker Compose
- pnpm (recommended) or npm

### Setup

1. **Start PostgreSQL with Docker:**
   ```bash
   npm run docker:up
   ```

2. **Run database migrations:**
   ```bash
   npm run migrate:up
   ```

3. **Start the API server:**
   ```bash
   npm run dev
   ```

4. **Or do it all at once:**
   ```bash
   npm run setup  # Starts Docker + runs migrations
   npm run dev    # Start the server
   ```

### API Endpoints

- `GET /` - API documentation
- `GET /health` - Service health check
- `GET /fetch` - Fetch 100 random jokes and store in database
- `GET /db/health` - Database health check

### Database Management

**Docker Commands:**
```bash
npm run docker:up       # Start PostgreSQL
npm run docker:down     # Stop PostgreSQL
npm run docker:logs     # View PostgreSQL logs
npm run docker:reset    # Reset database (removes all data)
npm run docker:admin    # Start with pgAdmin (localhost:8080)
```

**Migration Commands:**
```bash
npm run migrate:up      # Run pending migrations
npm run migrate:down    # Rollback one migration
npm run migrate:create  # Create new migration
```

### Database Access

**Direct PostgreSQL connection:**
- Host: `localhost`
- Port: `5432`
- Database: `jokes_db`
- User: `postgres`
- Password: `password`

**pgAdmin (optional):**
- URL: http://localhost:8080
- Email: `admin@jokes.local`
- Password: `admin`

### Environment Variables

Copy `.env.example` to `.env` and adjust as needed:

```bash
cp .env.example .env
```

### Project Structure

```
src/
├── providers/           # Joke API providers
├── db/                 # Database utilities
│   ├── migrations/     # SQL migrations
│   └── init-scripts/   # Docker init scripts
├── docker-compose.yml  # PostgreSQL setup
├── index.ts           # Hono API server
└── demo.ts            # Fetch jokes demo
```

### Development

```bash
# Install dependencies
pnpm install

# Start development server
npm run dev

# Build for production
npm run build
npm start
```
