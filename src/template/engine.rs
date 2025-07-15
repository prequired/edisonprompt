use handlebars::Handlebars;
use regex::Regex;
use std::collections::HashMap;
use crate::error::{PromptedsError, Result};

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    variable_regex: Regex,
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        
        let variable_regex = Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\}\}")
            .expect("Invalid variable regex");
        
        Self {
            handlebars,
            variable_regex,
        }
    }
    
    /// Extract variable names from a template
    pub fn extract_variables(&self, template: &str) -> Result<Vec<String>> {
        let mut variables = Vec::new();
        
        for cap in self.variable_regex.captures_iter(template) {
            if let Some(var_name) = cap.get(1) {
                let name = var_name.as_str().to_string();
                if !variables.contains(&name) {
                    variables.push(name);
                }
            }
        }
        
        Ok(variables)
    }
    
    /// Validate template syntax
    pub fn validate_template(&mut self, template: &str) -> Result<()> {
        // Try to register template to validate syntax
        let temp_name = uuid::Uuid::new_v4().to_string();
        match self.handlebars.register_template_string(&temp_name, template) {
            Ok(_) => Ok(()),
            Err(e) => Err(PromptedsError::TemplateValidation { 
                details: e.to_string() 
            })
        }
    }
    
    /// Render template with variables
    pub fn render(&mut self, template: &str, variables: &HashMap<String, String>) -> Result<String> {
        // First validate the template
        self.validate_template(template)?;
        
        // Register template
        let template_name = uuid::Uuid::new_v4().to_string();
        self.handlebars.register_template_string(&template_name, template)
            .map_err(|e| PromptedsError::TemplateCompilation(e))?;
        
        // Render with variables
        let result = self.handlebars.render(&template_name, variables)
            .map_err(|e| PromptedsError::Template(e))?;
        
        Ok(result)
    }
    
    /// Get missing variables from template given provided variables
    pub fn get_missing_variables(
        &self, 
        template: &str, 
        provided: &HashMap<String, String>
    ) -> Result<Vec<String>> {
        let required = self.extract_variables(template)?;
        let missing: Vec<String> = required
            .into_iter()
            .filter(|var| !provided.contains_key(var))
            .collect();
        
        Ok(missing)
    }
    
    /// Validate variable name
    pub fn validate_variable_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(PromptedsError::InvalidVariableName {
                name: name.to_string(),
                reason: "Variable name cannot be empty".to_string(),
            });
        }
        
        let var_regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$")?;
        if !var_regex.is_match(name) {
            return Err(PromptedsError::InvalidVariableName {
                name: name.to_string(),
                reason: "Variable name must start with letter or underscore, followed by letters, numbers, or underscores".to_string(),
            });
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_variables() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, your {{type}} is ready!";
        let vars = engine.extract_variables(template).unwrap();
        assert_eq!(vars, vec!["name", "type"]);
    }
    
    #[test]
    fn test_render_template() {
        let mut engine = TemplateEngine::new();
        let template = "Hello {{name}}!";
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "World".to_string());
        
        let result = engine.render(template, &vars).unwrap();
        assert_eq!(result, "Hello World!");
    }
    
    #[test]
    fn test_missing_variables() {
        let engine = TemplateEngine::new();
        let template = "Hello {{name}}, your {{type}} is ready!";
        let mut provided = HashMap::new();
        provided.insert("name".to_string(), "John".to_string());
        
        let missing = engine.get_missing_variables(template, &provided).unwrap();
        assert_eq!(missing, vec!["type"]);
    }
    
    #[test]
    fn test_validate_variable_name() {
        assert!(TemplateEngine::validate_variable_name("valid_name").is_ok());
        assert!(TemplateEngine::validate_variable_name("_underscore").is_ok());
        assert!(TemplateEngine::validate_variable_name("name123").is_ok());
        assert!(TemplateEngine::validate_variable_name("123invalid").is_err());
        assert!(TemplateEngine::validate_variable_name("").is_err());
        assert!(TemplateEngine::validate_variable_name("invalid-name").is_err());
    }
}