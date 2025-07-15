use crate::{
    database::Database,
    error::Result,
    cli::OutputFormat,
    cli::output::OutputFormatter,
};

pub struct SearchCommand;

impl SearchCommand {
    pub fn execute(
        database: &Database,
        query: String,
        highlight: bool,
        limit: usize,
        format: OutputFormat,
        color: bool,
    ) -> Result<()> {
        // Perform search
        let results = database.search_prompts(&query, limit, highlight)?;
        
        if results.is_empty() {
            println!("No results found for query: '{}'", query);
            return Ok(());
        }
        
        // Format and display results
        let formatter = OutputFormatter::new(color);
        let output = formatter.format_search_results(&results, &format);
        println!("{}", output);
        
        println!("\nFound {} result(s) for query: '{}'", results.len(), query);
        
        Ok(())
    }
}