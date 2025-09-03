/**
 * @type {import('node-pg-migrate').ColumnDefinitions | undefined}
 */
exports.shorthands = undefined;

/**
 * @param pgm {import('node-pg-migrate').MigrationBuilder}
 * @param run {() => void | undefined}
 * @returns {Promise<void> | void}
 */
exports.up = (pgm) => {
  // Enable uuid-ossp extension for UUID generation
  pgm.createExtension('uuid-ossp', { ifNotExists: true });

  // Create jokes table
  pgm.createTable('jokes', {
    id: {
      type: 'uuid',
      primaryKey: true,
      default: pgm.func('uuid_generate_v4()'),
    },
    external_id: {
      type: 'varchar(255)',
      notNull: false,
    },
    joke: {
      type: 'jsonb',
      notNull: true,
    },
    category: {
      type: 'varchar(100)',
      notNull: false,
    },
    type: {
      type: 'varchar(20)',
      notNull: false,
      check: "type IN ('single', 'twopart')",
    },
    safe: {
      type: 'boolean',
      notNull: true,
      default: true,
    },
    lang: {
      type: 'varchar(10)',
      notNull: true,
      default: 'en',
    },
    provider: {
      type: 'varchar(255)',
      notNull: true,
    },
    created_at: {
      type: 'timestamp',
      notNull: true,
      default: pgm.func('CURRENT_TIMESTAMP'),
    },
    updated_at: {
      type: 'timestamp',
      notNull: true,
      default: pgm.func('CURRENT_TIMESTAMP'),
    },
  });

  // Add constraint to ensure proper joke content based on type
  pgm.addConstraint('jokes', 'check_joke_type_content', {
    check: `(type = 'single' AND joke->>'content' IS NOT NULL AND joke->>'setup' IS NULL AND joke->>'punchline' IS NULL) OR (type = 'twopart' AND joke->>'content' IS NULL AND joke->>'setup' IS NOT NULL AND joke->>'punchline' IS NOT NULL)`,
  });

  // Add unique constraint to prevent duplicate jokes
  pgm.addConstraint('jokes', 'unique_external_id_provider', {
    unique: ['external_id', 'provider'],
  });

  // Create indexes for better query performance
  pgm.createIndex('jokes', 'category', { name: 'idx_jokes_category' });
  pgm.createIndex('jokes', 'type', { name: 'idx_jokes_type' });
  pgm.createIndex('jokes', 'provider', { name: 'idx_jokes_provider' });
  pgm.createIndex('jokes', 'safe', { name: 'idx_jokes_safe' });
  pgm.createIndex('jokes', 'created_at', { name: 'idx_jokes_created_at' });
  
  // Create GIN index on JSONB joke field for better JSON queries
  pgm.createIndex('jokes', 'joke', { 
    name: 'idx_jokes_joke_gin',
    method: 'gin'
  });

  // Create function to update updated_at timestamp
  pgm.sql(`
    CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS $$
    BEGIN
        NEW.updated_at = CURRENT_TIMESTAMP;
        RETURN NEW;
    END;
    $$ language 'plpgsql';
  `);

  // Create trigger to automatically update updated_at
  pgm.sql(`
    CREATE TRIGGER update_jokes_updated_at
        BEFORE UPDATE ON jokes
        FOR EACH ROW
        EXECUTE FUNCTION update_updated_at_column();
  `);
};

/**
 * @param pgm {import('node-pg-migrate').MigrationBuilder}
 * @param run {() => void | undefined}
 * @returns {Promise<void> | void}
 */
exports.down = (pgm) => {
  // Drop trigger and function
  pgm.sql('DROP TRIGGER IF EXISTS update_jokes_updated_at ON jokes;');
  pgm.sql('DROP FUNCTION IF EXISTS update_updated_at_column();');
  
  // Drop table (this will also drop all indexes and constraints)
  pgm.dropTable('jokes');
  
  // Drop extension (optional, might be used by other tables)
  // pgm.dropExtension('uuid-ossp');
};
