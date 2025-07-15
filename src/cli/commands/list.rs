use crate::{
    database::Database,
    error::Result,
    cli::{OutputFormat, SortField},
    cli::output::OutputFormatter,
};

pub struct ListCommand;

impl ListCommand {
    pub fn execute(
        database: &Database,
        tag: Option<String>,
        format: OutputFormat,
        limit: Option<usize>,
        sort: SortField,
        names_only: bool,
        color: bool,
    ) -> Result<()> {
        // Get prompts from database
        let mut prompts = database.list_prompts(tag.as_deref(), limit)?;
        
        // Sort the results
        match sort {
            SortField::Name => prompts.sort_by(|a, b| a.name.cmp(&b.name)),
            SortField::Created => prompts.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            SortField::Updated => prompts.sort_by(|a, b| b.updated_at.cmp(&a.updated_at)),
        }
        
        // Format and display results
        let formatter = OutputFormatter::new(color);
        let output = formatter.format_prompt_list(&prompts, &format, names_only);
        println!("{}", output);
        
        if !names_only && prompts.len() > 0 {
            println!("\nTotal: {} prompt(s)", prompts.len());
        }
        
        Ok(())
    }
}