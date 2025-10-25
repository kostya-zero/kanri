use anyhow::Result;
use std::{fs, path::Path};
use thiserror::Error;

use serde::{Deserialize, Serialize};

use crate::{config::Config, templates::Templates};

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("Failed to write backup file.")]
    WriteFailed,

    #[error("Failed to format backup to JSON.")]
    FormatFailed,

    #[error("Cannot find backup file.")]
    FileNotFound,

    #[error("Error parsing backup file: {0}.")]
    BadConfiguration(String),

    #[error("File system error occurred: {0}.")]
    FileSystemError(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize)]
pub struct Backup {
    pub config: Config,
    pub templates: Templates,
}

pub fn load_backup(path: impl AsRef<Path>) -> Result<Backup, BackupError> {
    let content = fs::read_to_string(path).map_err(BackupError::FileSystemError)?;
    serde_json::from_str::<Backup>(&content)
        .map_err(|e| BackupError::BadConfiguration(e.to_string()))
}

pub fn save_backup(path: impl AsRef<Path>, backup: Backup) -> Result<(), BackupError> {
    let serialized = serde_json::to_string(&backup).map_err(|_| BackupError::FormatFailed)?;
    fs::write(path, serialized).map_err(BackupError::FileSystemError)
}
