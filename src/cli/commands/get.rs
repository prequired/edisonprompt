use crate::{
    clipboard::ClipboardManager,
    database::Database,
    error::Result,
    template::TemplateEngine,
    utils,
};
use std::collections::HashMap;
use std::io::{self, Write};

pub struct GetCommand;

impl GetCommand {
    pub fn execute(
        database: &Database,
        clipboard: &mut ClipboardManager,
        name: String,
        variables: Vec<(String, String)>,
        copy: bool,
        raw: bool,
    ) -> Result<()> {
        // Get the prompt
        let prompt = database.get_prompt(&name)?;
        
        if raw {
            // Output raw content without rendering
            println!("{}", prompt.content);
            if copy && clipboard.is_available() {
                clipboard.set_text(&prompt.content)?;
                utils::print_info("Copied raw content to clipboard");
            }
            return Ok(());
        }
        
        // Convert variables to HashMap
        let mut variable_map: HashMap<String, String> = variables.into_iter().collect();
        
        // Add default values for missing variables
        for var in &prompt.variables {
            if !variable_map.contains_key(&var.name) {
                if let Some(ref default) = var.default_value {
                    variable_map.insert(var.name.clone(), default.clone());
                }
            }
        }
        
        // Check for missing variables and prompt for them
        let mut template_engine = TemplateEngine::new();
        let missing = template_engine.get_missing_variables(&prompt.content, &variable_map)?;
        
        if !missing.is_empty() {
            for var_name in missing {
                // Check if we have a description for this variable
                let description = prompt.variables
                    .iter()
                    .find(|v| v.name == var_name)
                    .and_then(|v| v.description.as_ref());
                
                if let Some(desc) = description {
                    print!("Enter value for '{}' ({}): ", var_name, desc);
                } else {
                    print!("Enter value for '{}': ", var_name);
                }
                io::stdout().flush()?;
                
                let mut value = String::new();
                io::stdin().read_line(&mut value)?;
                variable_map.insert(var_name, value.trim().to_string());
            }
        }
        
        // Render the template
        let rendered = template_engine.render(&prompt.content, &variable_map)?;
        
        // Output the result
        println!("{}", rendered);
        
        // Copy to clipboard if requested
        if copy && clipboard.is_available() {
            clipboard.set_text(&rendered)?;
            utils::print_info("Copied rendered prompt to clipboard");
        }
        
        Ok(())
    }
}