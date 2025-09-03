import { Pool, PoolConfig } from 'pg';

interface DatabaseConfig {
  host?: string;
  port?: number;
  database?: string;
  user?: string;
  password?: string;
  ssl?: boolean;
  max?: number;
  idleTimeoutMillis?: number;
  connectionTimeoutMillis?: number;
}

let pool: Pool | null = null;

/**
 * Initialize PostgreSQL connection pool
 */
export async function init(config?: DatabaseConfig): Promise<Pool> {
  try {
    // Default configuration
    const defaultConfig: PoolConfig = {
      host: process.env.DB_HOST || 'localhost',
      port: parseInt(process.env.DB_PORT || '5432'),
      database: process.env.DB_NAME || 'jokes_db',
      user: process.env.DB_USER || 'postgres',
      password: process.env.DB_PASSWORD || 'password',
      ssl: process.env.NODE_ENV === 'production',
      max: 20, // Maximum number of clients in the pool
      idleTimeoutMillis: 30000, // Close idle clients after 30 seconds
      connectionTimeoutMillis: 2000, // Return an error after 2 seconds if connection could not be established
    };

    // Merge with provided config
    const finalConfig = { ...defaultConfig, ...config };

    // Create new pool
    pool = new Pool(finalConfig);

    // Test the connection
    const client = await pool.connect();
    console.log('✅ PostgreSQL database connected successfully');
    console.log(`📊 Database: ${finalConfig.database} on ${finalConfig.host}:${finalConfig.port}`);
    
    // Release the test client
    client.release();

    return pool;
  } catch (error) {
    console.error('❌ Failed to connect to PostgreSQL database:', error);
    throw error;
  }
}

/**
 * Get the current database pool
 */
export function getPool(): Pool {
  if (!pool) {
    throw new Error('Database not initialized. Call init() first.');
  }
  return pool;
}

/**
 * Close the database connection pool
 */
export async function close(): Promise<void> {
  if (pool) {
    await pool.end();
    pool = null;
    console.log('🔌 Database connection pool closed');
  }
}

/**
 * Execute a query with the pool
 */
export async function query(text: string, params?: any[]): Promise<any> {
  if (!pool) {
    throw new Error('Database not initialized. Call init() first.');
  }
  
  const start = Date.now();
  const result = await pool.query(text, params);
  const duration = Date.now() - start;
  
  console.log('🔍 Query executed:', { text, duration, rows: result.rowCount });
  return result;
}

/**
 * Test database connection
 */
export async function testConnection(): Promise<boolean> {
  try {
    const result = await query('SELECT NOW() as current_time');
    console.log('⏰ Database time:', result.rows[0].current_time);
    return true;
  } catch (error) {
    console.error('❌ Database connection test failed:', error);
    return false;
  }
}
