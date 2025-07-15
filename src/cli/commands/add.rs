use crate::{
    clipboard::ClipboardManager,
    database::Database,
    error::{PromptedsError, Result},
    template::TemplateEngine,
    database::models::{Prompt, Variable},
    utils,
};
use std::io::{self, Read, Write};

pub struct AddCommand;

impl AddCommand {
    pub fn execute(
        database: &mut Database,
        clipboard: &mut ClipboardManager,
        name: String,
        interactive: bool,
        tags: Vec<String>,
        force: bool,
    ) -> Result<()> {
        // Check if prompt already exists
        if !force && database.prompt_exists(&name)? {
            return Err(PromptedsError::PromptAlreadyExists { name });
        }
        
        // Get content from clipboard or interactive input
        let content = if interactive {
            Self::get_interactive_content()?
        } else {
            Self::get_clipboard_content(clipboard)?
        };
        
        // Extract variables from template
        let template_engine = TemplateEngine::new();
        let variable_names = template_engine.extract_variables(&content)?;
        let variables = if interactive && !variable_names.is_empty() {
            Self::get_interactive_variables(variable_names)?
        } else {
            variable_names.into_iter()
                .map(Variable::new)
                .collect()
        };
        
        // Create prompt
        let prompt = Prompt::new(name.clone(), content)
            .with_variables(variables)
            .with_tags(tags);
        
        // Save to database
        database.create_prompt(&prompt)?;
        
        utils::print_success(&format!("Added prompt '{}'", name));
        if !prompt.variables.is_empty() {
            utils::print_info(&format!(
                "Extracted {} variable(s): {}", 
                prompt.variables.len(),
                prompt.variables.iter()
                    .map(|v| v.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        
        Ok(())
    }
    
    fn get_clipboard_content(clipboard: &mut ClipboardManager) -> Result<String> {
        match clipboard.get_text() {
            Ok(content) => {
                if content.trim().is_empty() {
                    return Err(PromptedsError::TemplateValidation {
                        details: "Clipboard content is empty".to_string(),
                    });
                }
                Ok(content)
            }
            Err(_) => {
                utils::print_warning("Clipboard not available. Please enter content manually:");
                Self::get_interactive_content()
            }
        }
    }
    
    fn get_interactive_content() -> Result<String> {
        println!("Enter prompt content (end with Ctrl+D on Unix or Ctrl+Z on Windows):");
        print!("> ");
        io::stdout().flush()?;
        
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        
        if content.trim().is_empty() {
            return Err(PromptedsError::TemplateValidation {
                details: "Content cannot be empty".to_string(),
            });
        }
        
        Ok(content.trim().to_string())
    }
    
    fn get_interactive_variables(variable_names: Vec<String>) -> Result<Vec<Variable>> {
        println!("\nFound {} variable(s) in template. Please provide descriptions:", variable_names.len());
        
        let mut variables = Vec::new();
        for name in variable_names {
            print!("Description for '{}' (optional): ", name);
            io::stdout().flush()?;
            
            let mut description = String::new();
            io::stdin().read_line(&mut description)?;
            let description = description.trim();
            
            print!("Default value for '{}' (optional): ", name);
            io::stdout().flush()?;
            
            let mut default_value = String::new();
            io::stdin().read_line(&mut default_value)?;
            let default_value = default_value.trim();
            
            let mut variable = Variable::new(name);
            if !description.is_empty() {
                variable = variable.with_description(description.to_string());
            }
            if !default_value.is_empty() {
                variable = variable.with_default(default_value.to_string());
            }
            
            variables.push(variable);
        }
        
        Ok(variables)
    }
}