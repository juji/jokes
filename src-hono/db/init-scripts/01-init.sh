#!/bin/bash
set -e

# This script runs when the PostgreSQL container starts for the first time

echo "Initializing jokes database..."

# Create the uuid-ossp extension
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
    
    -- Create a test user (optional)
    -- CREATE USER jokes_user WITH PASSWORD 'jokes_password';
    -- GRANT ALL PRIVILEGES ON DATABASE jokes_db TO jokes_user;
EOSQL

echo "Database initialization completed!"
