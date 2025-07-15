use crate::database::models::{PromptSummary, SearchResult};
use crate::cli::OutputFormat;
use colored::*;
use serde_json;

pub struct OutputFormatter {
    color: bool,
}

impl OutputFormatter {
    pub fn new(color: bool) -> Self {
        Self { color }
    }

    pub fn format_prompt_list(&self, prompts: &[PromptSummary], format: &OutputFormat, names_only: bool) -> String {
        if names_only {
            return prompts.iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join("\n");
        }

        match format {
            OutputFormat::Table => self.format_table(prompts),
            OutputFormat::Json => serde_json::to_string_pretty(prompts).unwrap_or_default(),
            OutputFormat::Plain => self.format_plain(prompts),
        }
    }

    pub fn format_search_results(&self, results: &[SearchResult], format: &OutputFormat) -> String {
        match format {
            OutputFormat::Table => self.format_search_table(results),
            OutputFormat::Json => serde_json::to_string_pretty(results).unwrap_or_default(),
            OutputFormat::Plain => self.format_search_plain(results),
        }
    }

    fn format_table(&self, prompts: &[PromptSummary]) -> String {
        if prompts.is_empty() {
            return "No prompts found.".to_string();
        }

        let mut output = String::new();
        
        // Header
        let header = format!(
            "{:<30} {:<10} {:<10} {:<20}",
            "Name", "Variables", "Tags", "Updated"
        );
        
        if self.color {
            output.push_str(&header.bold().to_string());
        } else {
            output.push_str(&header);
        }
        output.push('\n');
        
        // Separator
        output.push_str(&"-".repeat(72));
        output.push('\n');

        // Rows
        for prompt in prompts {
            let updated = prompt.updated_at.format("%Y-%m-%d %H:%M").to_string();
            let row = format!(
                "{:<30} {:<10} {:<10} {:<20}",
                crate::utils::truncate_string(&prompt.name, 28),
                prompt.variable_count,
                prompt.tag_count,
                updated
            );
            output.push_str(&row);
            output.push('\n');
        }

        output
    }

    fn format_search_table(&self, results: &[SearchResult]) -> String {
        if results.is_empty() {
            return "No results found.".to_string();
        }

        let mut output = String::new();
        
        // Header
        let header = format!(
            "{:<30} {:<10} {:<50}",
            "Name", "Score", "Content Preview"
        );
        
        if self.color {
            output.push_str(&header.bold().to_string());
        } else {
            output.push_str(&header);
        }
        output.push('\n');
        
        // Separator
        output.push_str(&"-".repeat(92));
        output.push('\n');

        // Rows
        for result in results {
            let content_preview = if let Some(ref highlighted) = result.highlighted_content {
                crate::utils::truncate_string(highlighted, 48)
            } else {
                crate::utils::truncate_string(&result.prompt.content.replace('\n', " "), 48)
            };
            
            let row = format!(
                "{:<30} {:<10.2} {:<50}",
                crate::utils::truncate_string(&result.prompt.name, 28),
                result.score,
                content_preview
            );
            output.push_str(&row);
            output.push('\n');
        }

        output
    }

    fn format_plain(&self, prompts: &[PromptSummary]) -> String {
        prompts.iter()
            .map(|p| format!("{} ({} vars, {} tags)", p.name, p.variable_count, p.tag_count))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_search_plain(&self, results: &[SearchResult]) -> String {
        results.iter()
            .map(|r| format!("{} (score: {:.2})", r.prompt.name, r.score))
            .collect::<Vec<_>>()
            .join("\n")
    }
}