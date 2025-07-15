use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub editor: EditorConfig,
    pub clipboard: ClipboardConfig,
    pub search: SearchConfig,
    pub output: OutputConfig,
    pub template: TemplateConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database file path (supports ~ expansion)
    pub path: PathBuf,
    /// Connection timeout in milliseconds
    #[serde(default = "default_db_timeout")]
    pub timeout_ms: u64,
    /// Enable WAL mode for better performance
    #[serde(default = "default_true")]
    pub wal_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Editor command (defaults to $EDITOR environment variable)
    pub command: Option<String>,
    /// Fallback editor if $EDITOR is not set
    #[serde(default = "default_fallback_editor")]
    pub fallback: String,
    /// Additional arguments to pass to editor
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardConfig {
    /// Clipboard operation timeout in milliseconds
    #[serde(default = "default_clipboard_timeout")]
    pub timeout_ms: u64,
    /// Enable clipboard fallback to file I/O in headless environments
    #[serde(default = "default_true")]
    pub enable_fallback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Default limit for search results
    #[serde(default = "default_search_limit")]
    pub limit: usize,
    /// Enable search result highlighting by default
    #[serde(default = "default_true")]
    pub highlight: bool,
    /// FTS5 ranking algorithm
    #[serde(default = "default_ranking")]
    pub ranking: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Enable colored output by default
    #[serde(default = "default_true")]
    pub color: bool,
    /// Default output format
    #[serde(default = "default_format")]
    pub format: String,
    /// Table column widths
    #[serde(default)]
    pub table_widths: TableWidths,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableWidths {
    #[serde(default = "default_name_width")]
    pub name: usize,
    #[serde(default = "default_variables_width")]
    pub variables: usize,
    #[serde(default = "default_tags_width")]
    pub tags: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Strict variable checking - error on undefined variables
    #[serde(default = "default_false")]
    pub strict_variables: bool,
    /// Default template syntax (handlebars)
    #[serde(default = "default_template_syntax")]
    pub syntax: String,
}

// Default value functions
fn default_db_timeout() -> u64 { 5000 }
fn default_clipboard_timeout() -> u64 { 5000 }
fn default_search_limit() -> usize { 50 }
fn default_fallback_editor() -> String { "nano".to_string() }
fn default_ranking() -> String { "bm25".to_string() }
fn default_format() -> String { "table".to_string() }
fn default_template_syntax() -> String { "handlebars".to_string() }
fn default_name_width() -> usize { 30 }
fn default_variables_width() -> usize { 20 }
fn default_tags_width() -> usize { 20 }
fn default_true() -> bool { true }
fn default_false() -> bool { false }

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            editor: EditorConfig::default(),
            clipboard: ClipboardConfig::default(),
            search: SearchConfig::default(),
            output: OutputConfig::default(),
            template: TemplateConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("~/.local/share/edisonprompt/prompts.db"),
            timeout_ms: default_db_timeout(),
            wal_mode: default_true(),
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            command: None,
            fallback: default_fallback_editor(),
            args: Vec::new(),
        }
    }
}

impl Default for ClipboardConfig {
    fn default() -> Self {
        Self {
            timeout_ms: default_clipboard_timeout(),
            enable_fallback: default_true(),
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            limit: default_search_limit(),
            highlight: default_true(),
            ranking: default_ranking(),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            color: default_true(),
            format: default_format(),
            table_widths: TableWidths::default(),
        }
    }
}

impl Default for TableWidths {
    fn default() -> Self {
        Self {
            name: default_name_width(),
            variables: default_variables_width(),
            tags: default_tags_width(),
        }
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            strict_variables: default_false(),
            syntax: default_template_syntax(),
        }
    }
}