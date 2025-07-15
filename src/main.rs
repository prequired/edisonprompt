use clap::Parser;
use edisonprompt::{
    cli::{Cli, Commands},
    cli::commands::*,
    ConfigManager, Database, ClipboardManager,
    error::Result,
    utils,
};
use std::process;

fn main() {
    if let Err(e) = run() {
        utils::print_error(&e.to_string());
        process::exit(e.exit_code());
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();
    
    // Initialize configuration
    let config_manager = if let Some(config_path) = args.config {
        ConfigManager::with_custom_config(config_path)?
    } else {
        ConfigManager::new()?
    };
    
    let config = config_manager.config();
    
    // Initialize database
    let db_path = config_manager.get_database_path();
    let mut database = Database::new(db_path)?;
    
    // Initialize clipboard manager
    let mut clipboard = ClipboardManager::new(
        config.clipboard.timeout_ms,
        config.clipboard.enable_fallback,
    );
    
    // Determine if colors should be used
    let use_color = !args.no_color && config.output.color;
    
    // Execute command
    match args.command {
        Commands::Add { name, interactive, tags, force } => {
            AddCommand::execute(&mut database, &mut clipboard, name, interactive, tags, force)?;
        }
        
        Commands::Get { name, variables, copy, raw } => {
            GetCommand::execute(&database, &mut clipboard, name, variables, copy, raw)?;
        }
        
        Commands::List { tag, format, limit, sort, names_only } => {
            ListCommand::execute(&database, tag, format, limit, sort, names_only, use_color)?;
        }
        
        Commands::Search { query, highlight, limit, format } => {
            SearchCommand::execute(&database, query, highlight, limit, format, use_color)?;
        }
        
        Commands::Edit { name, yes } => {
            EditCommand::execute(
                &mut database,
                config.editor.command.as_deref(),
                &config.editor.fallback,
                name,
                yes,
            )?;
        }
        
        Commands::Delete { name, yes, force } => {
            DeleteCommand::execute(&mut database, name, yes, force)?;
        }
        
        Commands::Export { output, tag, pretty } => {
            ExportCommand::execute(&database, output, tag, pretty)?;
        }
        
        Commands::Import { input, merge, dry_run } => {
            ImportCommand::execute(&mut database, input, merge, dry_run)?;
        }
        
        Commands::Completions { shell } => {
            CompletionsCommand::execute(shell)?;
        }
    }
    
    Ok(())
}