

-- ==================================================
-- Migration Down: Drop all created objects
-- ==================================================

-- Drop trigger and function
DROP TRIGGER IF EXISTS update_jokes_updated_at ON jokes;
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop table (this will also drop all indexes and constraints)
DROP TABLE IF EXISTS jokes;

-- Drop extension (optional, might be used by other tables)
-- DROP EXTENSION IF EXISTS "uuid-ossp";
