-- Migration Up: Create jokes table with all constraints and indexes

-- Enable uuid-ossp extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create jokes table
CREATE TABLE jokes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    external_id VARCHAR(255),
    joke JSONB NOT NULL,
    category VARCHAR(100),
    type VARCHAR(20) CHECK (type IN ('single', 'twopart')),
    safe BOOLEAN NOT NULL DEFAULT true,
    lang VARCHAR(10) NOT NULL DEFAULT 'en',
    provider VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Add constraint to ensure proper joke content based on type
ALTER TABLE jokes ADD CONSTRAINT check_joke_type_content
    CHECK (
        (type = 'single' AND joke->>'content' IS NOT NULL AND joke->>'setup' IS NULL AND joke->>'punchline' IS NULL)
        OR
        (type = 'twopart' AND joke->>'content' IS NULL AND joke->>'setup' IS NOT NULL AND joke->>'punchline' IS NOT NULL)
    );

-- Add unique constraint to prevent duplicate jokes
ALTER TABLE jokes ADD CONSTRAINT unique_external_id_provider UNIQUE (external_id, provider);

-- Create indexes for better query performance
CREATE INDEX idx_jokes_category ON jokes(category);
CREATE INDEX idx_jokes_type ON jokes(type);
CREATE INDEX idx_jokes_provider ON jokes(provider);
CREATE INDEX idx_jokes_safe ON jokes(safe);
CREATE INDEX idx_jokes_created_at ON jokes(created_at);

-- Create GIN index on JSONB joke field for better JSON queries
CREATE INDEX idx_jokes_joke_gin ON jokes USING gin (joke);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_jokes_updated_at
    BEFORE UPDATE ON jokes
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
