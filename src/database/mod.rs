pub mod models;
pub mod schema;
pub mod search;

use crate::error::Result;
use search::SearchEngine;
use models::{Prompt, PromptSummary, SearchResult};
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_CREATE | 
            OpenFlags::SQLITE_OPEN_READ_WRITE
        )?;

        // Initialize schema
        schema::initialize_database(&conn)?;

        Ok(Self { conn })
    }

    pub fn create_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        // Insert prompt
        tx.execute(
            "INSERT INTO prompts (id, name, content, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                prompt.id.to_string(),
                prompt.name,
                prompt.content,
                prompt.created_at.to_rfc3339(),
                prompt.updated_at.to_rfc3339()
            ],
        )?;

        // Insert variables
        for variable in &prompt.variables {
            tx.execute(
                "INSERT INTO variables (prompt_id, name, description, default_value) 
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![
                    prompt.id.to_string(),
                    variable.name,
                    variable.description,
                    variable.default_value
                ],
            )?;
        }

        // Insert tags
        for tag in &prompt.tags {
            // Insert tag if it doesn't exist
            tx.execute(
                "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                rusqlite::params![tag],
            )?;

            // Get tag ID
            let tag_id: i64 = tx.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                rusqlite::params![tag],
                |row| row.get(0),
            )?;

            // Link prompt to tag
            tx.execute(
                "INSERT OR IGNORE INTO prompt_tags (prompt_id, tag_id) VALUES (?1, ?2)",
                rusqlite::params![prompt.id.to_string(), tag_id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_prompt(&self, name: &str) -> Result<Prompt> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content, created_at, updated_at FROM prompts WHERE name = ?1"
        )?;

        let prompt_data = stmt.query_row(rusqlite::params![name], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        }).map_err(|_| crate::error::PromptedsError::PromptNotFound { name: name.to_string() })?;

        let id = uuid::Uuid::parse_str(&prompt_data.0)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&prompt_data.3)?.with_timezone(&chrono::Utc);
        let updated_at = chrono::DateTime::parse_from_rfc3339(&prompt_data.4)?.with_timezone(&chrono::Utc);

        // Get variables
        let variables = self.get_prompt_variables(&id)?;

        // Get tags
        let tags = self.get_prompt_tags(&id)?;

        Ok(Prompt {
            id,
            name: prompt_data.1,
            content: prompt_data.2,
            variables,
            tags,
            created_at,
            updated_at,
        })
    }

    pub fn update_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        let tx = self.conn.transaction()?;

        // Update prompt
        tx.execute(
            "UPDATE prompts SET content = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![
                prompt.content,
                prompt.updated_at.to_rfc3339(),
                prompt.id.to_string()
            ],
        )?;

        // Delete existing variables
        tx.execute(
            "DELETE FROM variables WHERE prompt_id = ?1",
            rusqlite::params![prompt.id.to_string()],
        )?;

        // Insert new variables
        for variable in &prompt.variables {
            tx.execute(
                "INSERT INTO variables (prompt_id, name, description, default_value) 
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![
                    prompt.id.to_string(),
                    variable.name,
                    variable.description,
                    variable.default_value
                ],
            )?;
        }

        // Delete existing tag associations
        tx.execute(
            "DELETE FROM prompt_tags WHERE prompt_id = ?1",
            rusqlite::params![prompt.id.to_string()],
        )?;

        // Insert new tags
        for tag in &prompt.tags {
            tx.execute(
                "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                rusqlite::params![tag],
            )?;

            let tag_id: i64 = tx.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                rusqlite::params![tag],
                |row| row.get(0),
            )?;

            tx.execute(
                "INSERT INTO prompt_tags (prompt_id, tag_id) VALUES (?1, ?2)",
                rusqlite::params![prompt.id.to_string(), tag_id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn delete_prompt(&mut self, name: &str) -> Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM prompts WHERE name = ?1",
            rusqlite::params![name],
        )?;

        if rows_affected == 0 {
            return Err(crate::error::PromptedsError::PromptNotFound { name: name.to_string() });
        }

        Ok(())
    }

    pub fn prompt_exists(&self, name: &str) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM prompts WHERE name = ?1",
            rusqlite::params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn list_prompts(&self, tag_filter: Option<&str>, limit: Option<usize>) -> Result<Vec<PromptSummary>> {
        let mut sql = String::from(
            "SELECT p.name, p.created_at, p.updated_at,
                    COUNT(DISTINCT v.id) as variable_count,
                    COUNT(DISTINCT pt.tag_id) as tag_count
             FROM prompts p
             LEFT JOIN variables v ON p.id = v.prompt_id
             LEFT JOIN prompt_tags pt ON p.id = pt.prompt_id"
        );

        let mut params = Vec::new();

        if let Some(tag) = tag_filter {
            sql.push_str(" INNER JOIN tags t ON pt.tag_id = t.id WHERE t.name = ?1");
            params.push(tag);
        }

        sql.push_str(" GROUP BY p.id, p.name, p.created_at, p.updated_at ORDER BY p.updated_at DESC");

        if let Some(limit) = limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })?;

        let mut summaries = Vec::new();
        for row in rows {
            let data = row?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&data.1)?.with_timezone(&chrono::Utc);
            let updated_at = chrono::DateTime::parse_from_rfc3339(&data.2)?.with_timezone(&chrono::Utc);
            
            summaries.push(PromptSummary {
                name: data.0,
                variable_count: data.3 as usize,
                tag_count: data.4 as usize,
                created_at,
                updated_at,
            });
        }

        Ok(summaries)
    }

    pub fn search_prompts(&self, query: &str, limit: usize, highlight: bool) -> Result<Vec<SearchResult>> {
        let search = SearchEngine::new(&self.conn);
        search.search(query, limit, highlight)
    }

    pub fn get_all_prompts(&self) -> Result<Vec<Prompt>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content, created_at, updated_at FROM prompts ORDER BY updated_at DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut prompts = Vec::new();
        for row in rows {
            let data = row?;
            let id = uuid::Uuid::parse_str(&data.0)?;
            let created_at = chrono::DateTime::parse_from_rfc3339(&data.3)?.with_timezone(&chrono::Utc);
            let updated_at = chrono::DateTime::parse_from_rfc3339(&data.4)?.with_timezone(&chrono::Utc);

            let variables = self.get_prompt_variables(&id)?;
            let tags = self.get_prompt_tags(&id)?;

            prompts.push(Prompt {
                id,
                name: data.1,
                content: data.2,
                variables,
                tags,
                created_at,
                updated_at,
            });
        }

        Ok(prompts)
    }

    pub fn get_schema_version(&self) -> Result<i32> {
        schema::get_schema_version(&self.conn)
    }

    fn get_prompt_variables(&self, prompt_id: &uuid::Uuid) -> Result<Vec<models::Variable>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, description, default_value FROM variables WHERE prompt_id = ?1"
        )?;

        let rows = stmt.query_map(rusqlite::params![prompt_id.to_string()], |row| {
            Ok(models::Variable {
                name: row.get(0)?,
                description: row.get(1)?,
                default_value: row.get(2)?,
            })
        })?;

        let mut variables = Vec::new();
        for row in rows {
            variables.push(row?);
        }

        Ok(variables)
    }

    fn get_prompt_tags(&self, prompt_id: &uuid::Uuid) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.name FROM tags t 
             INNER JOIN prompt_tags pt ON t.id = pt.tag_id 
             WHERE pt.prompt_id = ?1"
        )?;

        let rows = stmt.query_map(rusqlite::params![prompt_id.to_string()], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }

        Ok(tags)
    }
}