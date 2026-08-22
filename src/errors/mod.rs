#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Configuration Error: {0}")]
    ConfigError(String),

    #[error("Platform Error: {0}")]
    PlatformError(String),

    #[error("Event Error: {0}")]
    EventError(String),

    #[error("Unknown Error: {0}")]
    Unknown(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
