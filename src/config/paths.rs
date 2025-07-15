use crate::error::{PromptedsError, Result};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

pub struct ConfigPaths {
    project_dirs: ProjectDirs,
}

impl ConfigPaths {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("dev", "edisonprompt", "edisonprompt")
            .ok_or(PromptedsError::ConfigDirError)?;
        
        Ok(Self { project_dirs })
    }

    /// Get the configuration directory path
    pub fn config_dir(&self) -> &Path {
        self.project_dirs.config_dir()
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> &Path {
        self.project_dirs.data_dir()
    }

    /// Get the cache directory path
    pub fn cache_dir(&self) -> &Path {
        self.project_dirs.cache_dir()
    }

    /// Get the default config file path
    pub fn config_file(&self) -> PathBuf {
        self.config_dir().join("config.toml")
    }

    /// Get the default database file path
    pub fn database_file(&self) -> PathBuf {
        self.data_dir().join("prompts.db")
    }

    /// Ensure all required directories exist
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(self.config_dir())?;
        std::fs::create_dir_all(self.data_dir())?;
        std::fs::create_dir_all(self.cache_dir())?;
        Ok(())
    }

    /// Expand home directory in path
    pub fn expand_home<P: AsRef<Path>>(path: P) -> PathBuf {
        let path = path.as_ref();
        if let Ok(stripped) = path.strip_prefix("~") {
            if let Some(home) = dirs::home_dir() {
                return home.join(stripped);
            }
        }
        path.to_path_buf()
    }
}

impl Default for ConfigPaths {
    fn default() -> Self {
        Self::new().expect("Failed to initialize config paths")
    }
}