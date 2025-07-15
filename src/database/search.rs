use crate::database::models::{Prompt, SearchResult};
use crate::error::Result;
use rusqlite::{params, Connection};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct SearchEngine<'a> {
    conn: &'a Connection,
}

impl<'a> SearchEngine<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn search(&self, query: &str, limit: usize, highlight: bool) -> Result<Vec<SearchResult>> {
        let sql = if highlight {
            "SELECT p.id, p.name, p.content, p.created_at, p.updated_at, 
                    fts.rank,
                    highlight(prompts_fts, 0, '<mark>', '</mark>') as highlighted_name,
                    highlight(prompts_fts, 1, '<mark>', '</mark>') as highlighted_content
             FROM prompts_fts fts
             INNER JOIN prompts p ON p.rowid = fts.rowid
             WHERE prompts_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2"
        } else {
            "SELECT p.id, p.name, p.content, p.created_at, p.updated_at, 
                    fts.rank, '' as highlighted_name, '' as highlighted_content
             FROM prompts_fts fts
             INNER JOIN prompts p ON p.rowid = fts.rowid
             WHERE prompts_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2"
        };

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params![query, limit as i64], |row| {
            let id = Uuid::parse_str(&row.get::<_, String>(0)?).unwrap();
            let created_at = DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                .unwrap().with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                .unwrap().with_timezone(&Utc);
            
            let highlighted_content = if highlight {
                let highlighted: String = row.get(7)?;
                if !highlighted.is_empty() {
                    Some(highlighted)
                } else {
                    None
                }
            } else {
                None
            };

            Ok((
                id,
                row.get::<_, String>(1)?, // name
                row.get::<_, String>(2)?, // content
                created_at,
                updated_at,
                row.get::<_, f64>(5)?, // rank
                highlighted_content,
            ))
        })?;

        let mut results = Vec::new();
        for row in rows {
            let data = row?;
            
            // Get variables and tags for the full prompt
            let variables = self.get_prompt_variables(&data.0)?;
            let tags = self.get_prompt_tags(&data.0)?;

            let prompt = Prompt {
                id: data.0,
                name: data.1,
                content: data.2,
                variables,
                tags,
                created_at: data.3,
                updated_at: data.4,
            };

            results.push(SearchResult {
                prompt,
                score: data.5,
                highlighted_content: data.6,
            });
        }

        Ok(results)
    }

    fn get_prompt_variables(&self, prompt_id: &Uuid) -> Result<Vec<crate::database::models::Variable>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, description, default_value FROM variables WHERE prompt_id = ?1"
        )?;

        let rows = stmt.query_map(params![prompt_id.to_string()], |row| {
            Ok(crate::database::models::Variable {
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
}