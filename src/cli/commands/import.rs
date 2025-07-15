use crate::{
    database::Database,
    error::{PromptedsError, Result},
    database::models::ExportData,
    cli::MergeStrategy,
    utils,
};
use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};

pub struct ImportCommand;

impl ImportCommand {
    pub fn execute(
        database: &mut Database,
        input: Option<PathBuf>,
        merge: MergeStrategy,
        dry_run: bool,
    ) -> Result<()> {
        // Read JSON from file or stdin
        let json = match input {
            Some(path) => fs::read_to_string(path)?,
            None => {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            }
        };
        
        // Parse import data
        let import_data: ExportData = serde_json::from_str(&json)
            .map_err(|e| PromptedsError::ImportError {
                details: format!("Invalid JSON format: {}", e),
            })?;
        
        if dry_run {
            println!("Dry run - would import {} prompt(s):", import_data.prompts.len());
            for prompt in &import_data.prompts {
                let exists = database.prompt_exists(&prompt.name).unwrap_or(false);
                let action = if exists {
                    match merge {
                        MergeStrategy::Skip => "SKIP (exists)",
                        MergeStrategy::Overwrite => "OVERWRITE",
                        MergeStrategy::Rename => "RENAME",
                    }
                } else {
                    "CREATE"
                };
                println!("  {} - {}", prompt.name, action);
            }
            return Ok(());
        }
        
        let mut created = 0;
        let mut updated = 0;
        let mut skipped = 0;
        
        for mut prompt in import_data.prompts {
            let exists = database.prompt_exists(&prompt.name)?;
            
            if exists {
                match merge {
                    MergeStrategy::Skip => {
                        skipped += 1;
                        continue;
                    }
                    MergeStrategy::Overwrite => {
                        database.update_prompt(&prompt)?;
                        updated += 1;
                    }
                    MergeStrategy::Rename => {
                        // Find a unique name
                        let mut counter = 1;
                        let original_name = prompt.name.clone();
                        loop {
                            let new_name = format!("{}-{}", original_name, counter);
                            if !database.prompt_exists(&new_name)? {
                                prompt.name = new_name;
                                break;
                            }
                            counter += 1;
                        }
                        database.create_prompt(&prompt)?;
                        created += 1;
                    }
                }
            } else {
                database.create_prompt(&prompt)?;
                created += 1;
            }
        }
        
        utils::print_success(&format!(
            "Import complete: {} created, {} updated, {} skipped",
            created, updated, skipped
        ));
        
        Ok(())
    }
}