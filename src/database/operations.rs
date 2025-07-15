use crate::database::models::{Prompt, Variable, PromptSummary};
use crate::error::{PromptedsError, Result};
use rusqlite::{params, Connection};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct DatabaseOperations<'a> {
    conn: &'a Connection,
}

impl<'a> DatabaseOperations<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        // Insert prompt
        tx.execute(
            "INSERT INTO prompts (id, name, content, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
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
                params![
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
                params![tag],
            )?;

            // Get tag ID
            let tag_id: i64 = tx.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                params![tag],
                |row| row.get(0),
            )?;

            // Link prompt to tag
            tx.execute(
                "INSERT OR IGNORE INTO prompt_tags (prompt_id, tag_id) VALUES (?1, ?2)",
                params![prompt.id.to_string(), tag_id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_prompt(&self, name: &str) -> Result<Prompt> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content, created_at, updated_at FROM prompts WHERE name = ?1"
        )?;

        let prompt_data = stmt.query_row(params![name], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        }).map_err(|_| PromptedsError::PromptNotFound { name: name.to_string() })?;

        let id = Uuid::parse_str(&prompt_data.0)?;
        let created_at = DateTime::parse_from_rfc3339(&prompt_data.3)?.with_timezone(&Utc);
        let updated_at = DateTime::parse_from_rfc3339(&prompt_data.4)?.with_timezone(&Utc);

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

    pub fn update_prompt(&self, prompt: &Prompt) -> Result<()> {
        let tx = self.conn.transaction()?;

        // Update prompt
        tx.execute(
            "UPDATE prompts SET content = ?1, updated_at = ?2 WHERE id = ?3",
            params![
                prompt.content,
                prompt.updated_at.to_rfc3339(),
                prompt.id.to_string()
            ],
        )?;

        // Delete existing variables
        tx.execute(
            "DELETE FROM variables WHERE prompt_id = ?1",
            params![prompt.id.to_string()],
        )?;

        // Insert new variables
        for variable in &prompt.variables {
            tx.execute(
                "INSERT INTO variables (prompt_id, name, description, default_value) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![
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
            params![prompt.id.to_string()],
        )?;

        // Insert new tags
        for tag in &prompt.tags {
            tx.execute(
                "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                params![tag],
            )?;

            let tag_id: i64 = tx.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                params![tag],
                |row| row.get(0),
            )?;

            tx.execute(
                "INSERT INTO prompt_tags (prompt_id, tag_id) VALUES (?1, ?2)",
                params![prompt.id.to_string(), tag_id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn delete_prompt(&self, name: &str) -> Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM prompts WHERE name = ?1",
            params![name],
        )?;

        if rows_affected == 0 {
            return Err(PromptedsError::PromptNotFound { name: name.to_string() });
        }

        Ok(())
    }

    pub fn prompt_exists(&self, name: &str) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM prompts WHERE name = ?1",
            params![name],
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
            let created_at = DateTime::parse_from_rfc3339(&row.get::<_, String>(1)?)?.with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)?.with_timezone(&Utc);

            Ok(PromptSummary {
                name: row.get(0)?,
                variable_count: row.get::<_, i64>(3)? as usize,
                tag_count: row.get::<_, i64>(4)? as usize,
                created_at,
                updated_at,
            })
        })?;

        let mut summaries = Vec::new();
        for row in rows {
            summaries.push(row?);
        }

        Ok(summaries)
    }

    fn get_prompt_variables(&self, prompt_id: &Uuid) -> Result<Vec<Variable>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, description, default_value FROM variables WHERE prompt_id = ?1"
        )?;

        let rows = stmt.query_map(params![prompt_id.to_string()], |row| {
            Ok(Variable {
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

    fn get_prompt_tags(&self, prompt_id: &Uuid) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.name FROM tags t 
             INNER JOIN prompt_tags pt ON t.id = pt.tag_id 
             WHERE pt.prompt_id = ?1"
        )?;

        let rows = stmt.query_map(params![prompt_id.to_string()], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }

        Ok(tags)
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
            let id = Uuid::parse_str(&data.0)?;
            let created_at = DateTime::parse_from_rfc3339(&data.3)?.with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&data.4)?.with_timezone(&Utc);

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
}