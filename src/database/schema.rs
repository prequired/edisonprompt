use crate::error::Result;
use rusqlite::Connection;

pub const SCHEMA_VERSION: i32 = 1;

pub const INIT_SQL: &str = r#"
-- Enable required pragmas
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = 1000;
PRAGMA temp_store = MEMORY;

-- Main prompts table
CREATE TABLE IF NOT EXISTS prompts (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT prompts_name_length CHECK (length(name) > 0 AND length(name) <= 100),
    CONSTRAINT prompts_content_length CHECK (length(content) > 0 AND length(content) <= 100000)
);

-- Variables for prompts
CREATE TABLE IF NOT EXISTS variables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    prompt_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    default_value TEXT,
    FOREIGN KEY (prompt_id) REFERENCES prompts (id) ON DELETE CASCADE,
    UNIQUE (prompt_id, name)
);

-- Tags for categorization
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

-- Many-to-many relationship for prompt tags
CREATE TABLE IF NOT EXISTS prompt_tags (
    prompt_id TEXT NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (prompt_id, tag_id),
    FOREIGN KEY (prompt_id) REFERENCES prompts (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- Full-text search virtual table
CREATE VIRTUAL TABLE IF NOT EXISTS prompts_fts USING fts5(
    name, content, content='prompts', content_rowid='rowid'
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_prompts_name ON prompts(name);
CREATE INDEX IF NOT EXISTS idx_prompts_updated ON prompts(updated_at);
CREATE INDEX IF NOT EXISTS idx_variables_prompt ON variables(prompt_id);
CREATE INDEX IF NOT EXISTS idx_prompt_tags_prompt ON prompt_tags(prompt_id);
CREATE INDEX IF NOT EXISTS idx_prompt_tags_tag ON prompt_tags(tag_id);

-- Triggers for FTS synchronization
CREATE TRIGGER IF NOT EXISTS prompts_fts_insert AFTER INSERT ON prompts BEGIN
    INSERT INTO prompts_fts(rowid, name, content) VALUES (new.rowid, new.name, new.content);
END;

CREATE TRIGGER IF NOT EXISTS prompts_fts_delete AFTER DELETE ON prompts BEGIN
    INSERT INTO prompts_fts(prompts_fts, rowid, name, content) VALUES ('delete', old.rowid, old.name, old.content);
END;

CREATE TRIGGER IF NOT EXISTS prompts_fts_update AFTER UPDATE ON prompts BEGIN
    INSERT INTO prompts_fts(prompts_fts, rowid, name, content) VALUES ('delete', old.rowid, old.name, old.content);
    INSERT INTO prompts_fts(rowid, name, content) VALUES (new.rowid, new.name, new.content);
END;

-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO schema_version (version) VALUES (1);
"#;

pub fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute_batch(INIT_SQL)?;
    Ok(())
}

pub fn get_schema_version(conn: &Connection) -> Result<i32> {
    let version: i32 = conn.query_row(
        "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    Ok(version)
}

pub fn needs_migration(conn: &Connection) -> Result<bool> {
    let current_version = get_schema_version(conn)?;
    Ok(current_version < SCHEMA_VERSION)
}