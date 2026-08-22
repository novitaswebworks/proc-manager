#![allow(dead_code)]

use crate::configuration::Config;
use crate::errors::Result;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn init(config: &Config) -> Result<Self> {
        let db_url = format!("sqlite:{}?mode=rwc", config.database_path.to_string_lossy());

        tracing::info!("Initializing database at {}", db_url);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        // Initialize schema
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS workspaces (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT
            );

            CREATE TABLE IF NOT EXISTS workspace_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                workspace_id INTEGER NOT NULL,
                item_type TEXT NOT NULL, -- 'process', 'service', 'container'
                item_name TEXT NOT NULL, -- name of the process/service/container
                FOREIGN KEY(workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}
