use crate::{
    database::Database,
    error::{PromptedsError, Result},
    template::TemplateEngine,
    utils,
};
use std::process::Command;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

pub struct EditCommand;

impl EditCommand {
    pub fn execute(
        database: &mut Database,
        editor_command: Option<&str>,
        fallback_editor: &str,
        name: String,
        yes: bool,
    ) -> Result<()> {
        // Get the prompt
        let mut prompt = database.get_prompt(&name)?;
        
        // Determine editor command
        let env_editor = std::env::var("EDITOR").ok();
        let editor = editor_command
            .or_else(|| env_editor.as_deref())
            .unwrap_or(fallback_editor);
        
        // Create temporary file with content
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(prompt.content.as_bytes())?;
        let temp_path = temp_file.path().to_path_buf();
        
        // Open editor
        let status = Command::new(editor)
            .arg(&temp_path)
            .status()
            .map_err(|e| PromptedsError::EditorError {
                details: format!("Failed to launch editor '{}': {}", editor, e),
            })?;
        
        if !status.success() {
            return Err(PromptedsError::EditorError {
                details: format!("Editor exited with non-zero status: {}", status),
            });
        }
        
        // Read the modified content
        let new_content = fs::read_to_string(&temp_path)?;
        
        // Check if content changed
        if new_content == prompt.content {
            utils::print_info("No changes made to prompt");
            return Ok(());
        }
        
        // Confirm changes if not using --yes
        if !yes {
            println!("Content changed. Preview:");
            println!("--- Original ---");
            println!("{}", prompt.content);
            println!("--- Modified ---");
            println!("{}", new_content);
            
            if !utils::confirm("Save changes?")? {
                utils::print_info("Changes discarded");
                return Ok(());
            }
        }
        
        // Update prompt content
        prompt.content = new_content.trim().to_string();
        prompt.updated_at = chrono::Utc::now();
        
        // Re-extract variables
        let template_engine = TemplateEngine::new();
        let variable_names = template_engine.extract_variables(&prompt.content)?;
        
        // Preserve existing variable descriptions and defaults
        let existing_vars: std::collections::HashMap<String, _> = prompt.variables
            .into_iter()
            .map(|v| (v.name.clone(), v))
            .collect();
        
        prompt.variables = variable_names.into_iter()
            .map(|name| {
                existing_vars.get(&name)
                    .cloned()
                    .unwrap_or_else(|| crate::database::models::Variable::new(name))
            })
            .collect();
        
        // Save changes
        database.update_prompt(&prompt)?;
        
        utils::print_success(&format!("Updated prompt '{}'", name));
        if !prompt.variables.is_empty() {
            utils::print_info(&format!(
                "Variables: {}", 
                prompt.variables.iter()
                    .map(|v| v.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        
        Ok(())
    }
}