import { Hono } from 'hono';
import { serve } from '@hono/node-server';
import { JokeManager } from './providers/joke-manager';
import * as db from './db';

const app = new Hono();
const jokeManager = new JokeManager();

// Middleware to initialize database connection
app.use('*', async (c, next) => {
  try {
    // Initialize database if not already done
    if (!process.env.DB_INITIALIZED) {
      await db.init();
      process.env.DB_INITIALIZED = 'true';
    }
    await next();
  } catch (error) {
    console.error('Database initialization error:', error);
    await next();
  }
});

// fetcher endpoint
app.get('/fetch', async (c) => {
  try {
    const jokes = await jokeManager.getMultipleJokes(100);

    // insert into db
    const { inserted, duplicates, totalProcessed } = await db.insertJokes(jokes);
    return c.json({ ok: true, inserted, duplicates, totalProcessed });

  } catch (error) {
    return c.json({ error: 'Failed to fetch jokes' }, 500);
  }
});

// Health check endpoint
app.get('/health', (c) => {
  return c.json({ 
    status: 'ok', 
    timestamp: new Date().toISOString(),
    service: 'jokes-fetcher'
  });
});

// Database health check
app.get('/db/health', async (c) => {
  try {
    const isHealthy = await db.testConnection();
    return c.json({ 
      database: isHealthy ? 'connected' : 'disconnected',
      timestamp: new Date().toISOString()
    });
  } catch (error) {
    return c.json({ 
      database: 'error',
      error: error instanceof Error ? error.message : 'Unknown error',
      timestamp: new Date().toISOString()
    }, 500);
  }
});

// API documentation endpoint
app.get('/', (c) => {
  return c.json({
    name: 'Jokes API Fetcher',
    version: '1.0.0',
    description: 'A comprehensive joke API Fetcher with multiple providers',
    endpoints: {
      'GET /health': 'Service health check',
      'GET /fetch': 'Fetch random joke, and put it in the db',
      'GET /db/health': 'Database health check'
    },
    providers: jokeManager.getProviders().map(p => ({
      name: p.name,
      baseUrl: p.baseUrl,
      categoriesCount: p.categories.length
    }))
  });
});

// Start the server
const port = parseInt(process.env.PORT || '3000');

console.log(`🚀 Starting Jokes API server on port ${port}...`);

serve({
  fetch: app.fetch,
  port
}, (info) => {
  console.log(`✅ Server is running on http://localhost:${info.port}`);
  console.log(`📚 API documentation: http://localhost:${info.port}/`);
  console.log(`🎭 Random joke: http://localhost:${info.port}/jokes/random`);
});

export default app;
