import { query } from './init';

// Type definitions for database operations
interface JokeInput {
  id?: string;
  joke: {
    content?: string;
    setup?: string;
    punchline?: string;
  };
  category?: string;
  type?: 'single' | 'twopart';
  safe?: boolean;
  lang?: string;
  provider: string;
}

interface JokeRecord {
  id: string;
  external_id: string | null;
  joke: object;
  category: string | null;
  type: string | null;
  safe: boolean;
  lang: string;
  provider: string;
  created_at: Date;
  updated_at: Date;
}

interface InsertedJoke {
  id: string;
  external_id: string | null;
  provider: string;
  created_at: Date;
  updated_at: Date;
}

interface DuplicateJoke {
  external_id: string | null;
  provider: string;
}

interface BatchInsertResult {
  inserted: InsertedJoke[];
  duplicates: DuplicateJoke[];
  totalProcessed: number;
}

/**
 * Function to insert a joke into the database
 */
export async function insertJoke(joke: JokeInput): Promise<JokeRecord> {
  const insertQuery = `
    INSERT INTO jokes (
      external_id, 
      joke, 
      category, 
      type, 
      safe, 
      lang, 
      provider
    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
    ON CONFLICT (external_id, provider) 
    DO UPDATE SET
      joke = EXCLUDED.joke,
      category = EXCLUDED.category,
      type = EXCLUDED.type,
      safe = EXCLUDED.safe,
      lang = EXCLUDED.lang,
      updated_at = CURRENT_TIMESTAMP
    RETURNING *;
  `;

  const values = [
    joke.id || null,
    JSON.stringify(joke.joke), // Store as JSON
    joke.category?.toLowerCase() || null,
    joke.type || null,
    joke.safe !== undefined ? joke.safe : true,
    joke.lang || 'en',
    joke.provider
  ];

  try {
    const result = await query(insertQuery, values);
    return result.rows[0];
  } catch (error) {
    console.error('❌ Failed to insert joke:', error);
    throw error;
  }
}

/**
 * Function to insert multiple jokes efficiently using batch insert
 * Returns object with inserted jokes and duplicates information
 */
export async function insertJokes(jokes: JokeInput[]): Promise<BatchInsertResult> {
  if (jokes.length === 0) {
    return { inserted: [], duplicates: [], totalProcessed: 0 };
  }

  // Prepare arrays for each column
  const externalIds: (string | null)[] = [];
  const jokeContents: string[] = [];
  const categories: (string | null)[] = [];
  const types: (string | null)[] = [];
  const safeFlags: boolean[] = [];
  const languages: string[] = [];
  const providers: string[] = [];

  // Process each joke
  jokes.forEach(joke => {
    externalIds.push(joke.id || null);
    jokeContents.push(JSON.stringify(joke.joke));
    categories.push(joke.category?.toLowerCase() || null);
    types.push(joke.type || null);
    safeFlags.push(joke.safe !== undefined ? joke.safe : true);
    languages.push(joke.lang || 'en');
    providers.push(joke.provider);
  });

  const batchInsertQuery = `
    INSERT INTO jokes (
      external_id, 
      joke, 
      category, 
      type, 
      safe, 
      lang, 
      provider
    )
    SELECT * FROM UNNEST(
      $1::varchar[],  -- external_ids
      $2::jsonb[],    -- jokes
      $3::varchar[],  -- categories
      $4::varchar[],  -- types
      $5::boolean[],  -- safe flags
      $6::varchar[],  -- languages
      $7::varchar[]   -- providers
    ) AS t(external_id, joke, category, type, safe, lang, provider)
    ON CONFLICT (external_id, provider) 
    DO NOTHING
    RETURNING id, external_id, provider, created_at, updated_at;
  `;

  const values = [
    externalIds,
    jokeContents,
    categories,
    types,
    safeFlags,
    languages,
    providers
  ];

  try {
    const result = await query(batchInsertQuery, values);
    const inserted: InsertedJoke[] = result.rows;
    
    // Find duplicates by comparing what was sent vs what was inserted
    const insertedKeys = new Set(
      inserted.map((row: InsertedJoke) => `${row.external_id || 'null'}:${row.provider}`)
    );
    
    const duplicates: DuplicateJoke[] = jokes
      .map(joke => ({ external_id: joke.id || null, provider: joke.provider }))
      .filter(joke => !insertedKeys.has(`${joke.external_id || 'null'}:${joke.provider}`));
    
    return {
      inserted,
      duplicates,
      totalProcessed: jokes.length
    };
    
  } catch (error) {
    console.error('❌ Failed to batch insert jokes:', error);
    console.error('First few jokes for debugging:', jokes.slice(0, 3));
    throw error;
  }
}

/**
 * Function to get jokes from the database
 */
export async function getJokes(filters: {
  limit?: number;
  offset?: number;
  category?: string;
  type?: 'single' | 'twopart';
  provider?: string;
  safe?: boolean;
} = {}): Promise<JokeRecord[]> {
  let queryText = `
    SELECT 
      id,
      external_id,
      joke,
      category,
      type,
      safe,
      lang,
      provider,
      created_at,
      updated_at
    FROM jokes
  `;

  const conditions: string[] = [];
  const values: any[] = [];
  let paramCount = 0;

  // Add filters
  if (filters.category) {
    paramCount++;
    conditions.push(`category = $${paramCount}`);
    values.push(filters.category.toLowerCase());
  }

  if (filters.type) {
    paramCount++;
    conditions.push(`type = $${paramCount}`);
    values.push(filters.type);
  }

  if (filters.provider) {
    paramCount++;
    conditions.push(`provider = $${paramCount}`);
    values.push(filters.provider);
  }

  if (filters.safe !== undefined) {
    paramCount++;
    conditions.push(`safe = $${paramCount}`);
    values.push(filters.safe);
  }

  if (conditions.length > 0) {
    queryText += ` WHERE ` + conditions.join(' AND ');
  }

  queryText += ` ORDER BY created_at DESC`;

  if (filters.limit) {
    paramCount++;
    queryText += ` LIMIT $${paramCount}`;
    values.push(filters.limit);
  }

  if (filters.offset) {
    paramCount++;
    queryText += ` OFFSET $${paramCount}`;
    values.push(filters.offset);
  }

  try {
    const result = await query(queryText, values);
    return result.rows as JokeRecord[];
  } catch (error) {
    console.error('❌ Failed to get jokes:', error);
    throw error;
  }
}

interface JokeStats {
  total_jokes: string;
  total_providers: string;
  total_categories: string;
  single_jokes: string;
  twopart_jokes: string;
  safe_jokes: string;
  unsafe_jokes: string;
}

interface ProviderStats {
  provider: string;
  joke_count: string;
  single_count: string;
  twopart_count: string;
  safe_count: string;
  unsafe_count: string;
  last_added: Date;
}

/**
 * Function to get joke statistics
 */
export async function getJokeStats(): Promise<JokeStats> {
  const statsQuery = `
    SELECT 
      COUNT(*) as total_jokes,
      COUNT(DISTINCT provider) as total_providers,
      COUNT(DISTINCT category) as total_categories,
      COUNT(CASE WHEN type = 'single' THEN 1 END) as single_jokes,
      COUNT(CASE WHEN type = 'twopart' THEN 1 END) as twopart_jokes,
      COUNT(CASE WHEN safe = true THEN 1 END) as safe_jokes,
      COUNT(CASE WHEN safe = false THEN 1 END) as unsafe_jokes
    FROM jokes;
  `;

  try {
    const result = await query(statsQuery);
    return result.rows[0] as JokeStats;
  } catch (error) {
    console.error('❌ Failed to get joke stats:', error);
    throw error;
  }
}

/**
 * Function to get joke count by provider
 */
export async function getJokeCountByProvider(provider?: string): Promise<ProviderStats[]> {
  let countQuery = `
    SELECT 
      provider,
      COUNT(*) as joke_count,
      COUNT(CASE WHEN type = 'single' THEN 1 END) as single_count,
      COUNT(CASE WHEN type = 'twopart' THEN 1 END) as twopart_count,
      COUNT(CASE WHEN safe = true THEN 1 END) as safe_count,
      COUNT(CASE WHEN safe = false THEN 1 END) as unsafe_count,
      MAX(created_at) as last_added
    FROM jokes
  `;

  const values: any[] = [];

  if (provider) {
    countQuery += ` WHERE provider = $1`;
    values.push(provider);
  }

  countQuery += ` GROUP BY provider ORDER BY joke_count DESC`;

  try {
    const result = await query(countQuery, values);
    return result.rows as ProviderStats[];
  } catch (error) {
    console.error('❌ Failed to get joke count by provider:', error);
    throw error;
  }
}
