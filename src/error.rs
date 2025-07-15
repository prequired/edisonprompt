use thiserror::Error;

#[derive(Error, Debug)]
pub enum PromptedsError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Template error: {0}")]
    Template(#[from] handlebars::RenderError),
    
    #[error("Template compilation error: {0}")]
    TemplateCompilation(#[from] handlebars::TemplateError),
    
    #[error("Clipboard error: {0}")]
    Clipboard(#[from] arboard::Error),
    
    #[error("Configuration deserialization error: {0}")]
    ConfigDe(#[from] toml::de::Error),
    
    #[error("Configuration serialization error: {0}")]
    ConfigSer(#[from] toml::ser::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("UUID parsing error: {0}")]
    Uuid(#[from] uuid::Error),
    
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    
    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),
    
    #[error("Prompt '{name}' not found")]
    PromptNotFound { name: String },
    
    #[error("Prompt '{name}' already exists")]
    PromptAlreadyExists { name: String },
    
    #[error("Invalid prompt name: {reason}")]
    InvalidPromptName { reason: String },
    
    #[error("Template validation failed: {details}")]
    TemplateValidation { details: String },
    
    #[error("Variable '{name}' is required but not provided")]
    MissingVariable { name: String },
    
    #[error("Invalid variable name '{name}': {reason}")]
    InvalidVariableName { name: String, reason: String },
    
    #[error("Editor error: {details}")]
    EditorError { details: String },
    
    #[error("Import error: {details}")]
    ImportError { details: String },
    
    #[error("Export error: {details}")]
    ExportError { details: String },
    
    #[error("Configuration directory not found or inaccessible")]
    ConfigDirError,
    
    #[error("Data directory not found or inaccessible")]
    DataDirError,
}

pub type Result<T> = std::result::Result<T, PromptedsError>;

impl PromptedsError {
    pub fn exit_code(&self) -> i32 {
        match self {
            PromptedsError::PromptNotFound { .. } => 1,
            PromptedsError::PromptAlreadyExists { .. } => 2,
            PromptedsError::InvalidPromptName { .. } => 3,
            PromptedsError::TemplateValidation { .. } => 4,
            PromptedsError::MissingVariable { .. } => 5,
            PromptedsError::InvalidVariableName { .. } => 6,
            PromptedsError::Database(_) => 10,
            PromptedsError::Io(_) => 11,
            PromptedsError::ConfigDe(_) => 12,
            PromptedsError::ConfigSer(_) => 12,
            PromptedsError::ConfigDirError => 13,
            PromptedsError::DataDirError => 14,
            _ => 99,
        }
    }
}