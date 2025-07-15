pub mod settings;
pub mod paths;

use crate::error::Result;
use settings::Config;
use paths::ConfigPaths;
use std::path::Path;

pub use settings::*;
pub use paths::*;

pub struct ConfigManager {
    paths: ConfigPaths,
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let paths = ConfigPaths::new()?;
        paths.ensure_dirs()?;
        
        let config = Self::load_config(&paths)?;
        
        Ok(Self { paths, config })
    }

    pub fn with_custom_config<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let paths = ConfigPaths::new()?;
        paths.ensure_dirs()?;
        
        let config = Self::load_config_from_file(config_path)?;
        
        Ok(Self { paths, config })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn paths(&self) -> &ConfigPaths {
        &self.paths
    }

    fn load_config(paths: &ConfigPaths) -> Result<Config> {
        let config_file = paths.config_file();
        
        if config_file.exists() {
            Self::load_config_from_file(config_file)
        } else {
            // Create default config file
            let default_config = Config::default();
            Self::save_config(&default_config, &config_file)?;
            Ok(default_config)
        }
    }

    fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn save_config(config: &Config, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(config)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn get_database_path(&self) -> std::path::PathBuf {
        if self.config.database.path.to_string_lossy().starts_with('~') {
            ConfigPaths::expand_home(&self.config.database.path)
        } else if self.config.database.path.is_relative() {
            self.paths.data_dir().join(&self.config.database.path)
        } else {
            self.config.database.path.clone()
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize config manager")
    }
}