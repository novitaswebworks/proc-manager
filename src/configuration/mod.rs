use crate::errors::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub containers: Option<Vec<String>>,
    pub processes: Option<Vec<String>>,
    pub services: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub database_path: Option<PathBuf>,
    pub log_path: Option<PathBuf>,
    pub theme: Option<ThemeConfig>,
    pub workspaces: Option<HashMap<String, WorkspaceConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub default_view: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Config::default();
        if let Some(dirs) = ProjectDirs::from("com", "novitas", "nman") {
            let config_dir = dirs.config_dir();
            fs::create_dir_all(config_dir).ok();
            let config_file = config_dir.join("config.toml");
            
            if config_file.exists() {
                if let Ok(contents) = fs::read_to_string(&config_file) {
                    if let Ok(parsed) = toml::from_str::<Config>(&contents) {
                        config = parsed;
                    }
                }
            } else {
                let default_toml = r#"# nman Configuration File
[theme]
default_view = "ProcessList" # Options: ProcessList, DockerList, ServiceList

# [workspaces.backend]
# containers = ["postgres", "redis"]
# processes = ["node", "python3"]
# services = ["nginx"]
"#;
                fs::write(&config_file, default_toml).ok();
            }
        }
        Ok(config)
    }

    pub fn get_database_path(&self) -> PathBuf {
        self.database_path.clone().unwrap_or_else(|| {
            if let Some(dirs) = ProjectDirs::from("com", "novitas", "nman") {
                let data_dir = dirs.data_dir();
                fs::create_dir_all(data_dir).ok();
                data_dir.join("nman.db")
            } else {
                std::env::current_dir().unwrap_or_default().join("nman.db")
            }
        })
    }
    
    pub fn get_log_path(&self) -> PathBuf {
        self.log_path.clone().unwrap_or_else(|| {
            if let Some(dirs) = ProjectDirs::from("com", "novitas", "nman") {
                let state_dir = dirs.state_dir().unwrap_or(dirs.data_dir());
                fs::create_dir_all(state_dir).ok();
                state_dir.join("nman.log")
            } else {
                std::env::current_dir().unwrap_or_default().join("nman.log")
            }
        })
    }
}
