use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "edisonprompt",
    version = env!("CARGO_PKG_VERSION"),
    author = "EdisonPrompt Contributors",
    about = "Lightning-fast AI prompt management CLI",
    long_about = "A professional CLI tool for managing AI prompts with template variables, full-text search, and local-first architecture."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Specify custom config file path
    #[arg(short, long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,
    
    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new prompt from clipboard or input
    Add {
        /// Prompt name (alphanumeric, hyphens, underscores only)
        #[arg(value_parser = validate_prompt_name)]
        name: String,
        
        /// Use interactive mode for content input
        #[arg(short, long)]
        interactive: bool,
        
        /// Add tags to the prompt
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
        
        /// Force overwrite if prompt exists
        #[arg(short, long)]
        force: bool,
    },
    
    /// Retrieve and render a prompt with variables
    Get {
        /// Prompt name to retrieve
        name: String,
        
        /// Variable values in key=value format
        #[arg(long = "var", value_parser = parse_variable)]
        variables: Vec<(String, String)>,
        
        /// Copy result to clipboard
        #[arg(short, long)]
        copy: bool,
        
        /// Output raw content without rendering
        #[arg(short, long)]
        raw: bool,
    },
    
    /// List prompts with optional filtering
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
        
        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
        
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
        
        /// Sort by field
        #[arg(short, long, value_enum, default_value = "name")]
        sort: SortField,
        
        /// Show only names
        #[arg(long)]
        names_only: bool,
    },
    
    /// Search prompts by content using full-text search
    Search {
        /// Search query
        query: String,
        
        /// Highlight search terms in results
        #[arg(long)]
        highlight: bool,
        
        /// Limit number of results  
        #[arg(short, long, default_value = "50")]
        limit: usize,
        
        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
    
    /// Edit an existing prompt in your editor
    Edit {
        /// Prompt name to edit
        name: String,
        
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
    
    /// Delete a prompt permanently
    Delete {
        /// Prompt name to delete
        name: String,
        
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        
        /// Force delete without confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Export prompts to JSON format
    Export {
        /// Output file path (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
        
        /// Pretty print JSON
        #[arg(short, long)]
        pretty: bool,
    },
    
    /// Import prompts from JSON format
    Import {
        /// Input file path (stdin if not specified)
        #[arg(short, long)]
        input: Option<PathBuf>,
        
        /// Merge strategy for existing prompts
        #[arg(short, long, value_enum, default_value = "skip")]
        merge: MergeStrategy,
        
        /// Dry run - show what would be imported
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Generate shell completions
    Completions {
        /// Shell type
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Table,
    Json,
    Plain,
}

#[derive(ValueEnum, Clone)]
pub enum SortField {
    Name,
    Created,
    Updated,
}

#[derive(ValueEnum, Clone)]
pub enum MergeStrategy {
    Skip,
    Overwrite,
    Rename,
}

#[derive(ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

fn validate_prompt_name(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if name.len() > 100 {
        return Err("Name too long (max 100 characters)".to_string());
    }
    if !regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap().is_match(name) {
        return Err("Name can only contain letters, numbers, hyphens, and underscores".to_string());
    }
    Ok(name.to_string())
}

fn parse_variable(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err("Variable must be in key=value format".to_string());
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}