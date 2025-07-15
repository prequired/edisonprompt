use crate::{
    database::Database,
    error::Result,
    database::models::ExportData,
    utils,
};
use std::path::PathBuf;
use std::fs;
use chrono::Utc;

pub struct ExportCommand;

impl ExportCommand {
    pub fn execute(
        database: &Database,
        output: Option<PathBuf>,
        tag: Option<String>,
        pretty: bool,
    ) -> Result<()> {
        // Get all prompts or filtered by tag
        let prompts = if let Some(tag_filter) = tag.as_deref() {
            // Get prompts with specific tag - we'll need to implement this
            database.list_prompts(Some(tag_filter), None)?
                .into_iter()
                .map(|summary| database.get_prompt(&summary.name))
                .collect::<Result<Vec<_>>>()?
        } else {
            database.get_all_prompts()?
        };
        
        // Create export data
        let export_data = ExportData {
            version: env!("CARGO_PKG_VERSION").to_string(),
            exported_at: Utc::now(),
            prompts,
        };
        
        // Serialize to JSON
        let json = if pretty {
            serde_json::to_string_pretty(&export_data)?
        } else {
            serde_json::to_string(&export_data)?
        };
        
        // Output to file or stdout
        match output {
            Some(path) => {
                fs::write(&path, json)?;
                utils::print_success(&format!(
                    "Exported {} prompt(s) to {}", 
                    export_data.prompts.len(),
                    path.display()
                ));
            }
            None => {
                println!("{}", json);
            }
        }
        
        Ok(())
    }
}