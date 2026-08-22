use crate::errors::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_path: PathBuf,
    pub log_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let (db_path, log_path) = match ProjectDirs::from("com", "novatask", "novatask") {
            Some(dirs) => {
                let data_dir = dirs.data_dir();
                let state_dir = dirs.state_dir().unwrap_or(data_dir);
                let data_local_dir = dirs.data_local_dir();

                std::fs::create_dir_all(data_dir).ok();
                std::fs::create_dir_all(state_dir).ok();
                std::fs::create_dir_all(data_local_dir).ok();

                (data_dir.join("novatask.db"), state_dir.join("novatask.log"))
            }
            None => {
                let current_dir = std::env::current_dir().unwrap_or_default();
                (
                    current_dir.join("novatask.db"),
                    current_dir.join("novatask.log"),
                )
            }
        };

        Self {
            database_path: db_path,
            log_path,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Here we could load from a TOML file. For Phase 0, we just return default.
        Ok(Config::default())
    }
}
