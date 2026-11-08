use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::platform;

#[derive(Debug, Error)]
pub enum BlueprintsError {
    #[error("Not enough permission to access directory with blueprints.")]
    PermissionDenied,

    #[error("Failed to list files in directory.")]
    CannotReadDirectory,

    #[error("Blueprint not found.")]
    NotFound,

    #[error("Blueprint name cannot be empty.")]
    EmptyName,

    #[error("Blueprint name must not contain path separators.")]
    PathSeparatorInName,

    #[error("Blueprint file extension must be .lua.")]
    InvalidExtension,

    #[error("An unexpected I/O error occurred: {source}")]
    IoError { source: std::io::Error },
}

pub struct Blueprints {
    blueprints: Vec<String>,
}

impl Blueprints {
    /// Loads all blueprints from given path.
    pub fn load_from_path(path: &Path) -> Result<Self, BlueprintsError> {
        Ok(Self {
            blueprints: Self::collect_lua_files(path)?,
        })
    }

    /// Collects all lua files from directory and returns a vector with their filenames without
    /// extension.
    fn collect_lua_files(path: &Path) -> Result<Vec<String>, BlueprintsError> {
        let mut files = Vec::new();

        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|extension| extension.to_str()) != Some("lua")
                || path.is_dir()
            {
                continue;
            }

            if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str())
                && !stem.is_empty()
            {
                files.push(stem.to_string());
            }
        }

        Ok(files)
    }

    /// Get all blueprints.
    pub fn get_blueprints(&self) -> &[String] {
        &self.blueprints
    }

    /// Returns the content of a blueprint
    pub fn read(name: &str) -> Result<String, BlueprintsError> {
        fs::read_to_string(Self::path(name)?).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                BlueprintsError::NotFound
            } else {
                e.into()
            }
        })
    }

    /// Validates the blueprint name and returns the path to its file.
    /// Accepts names with or without the `.lua` extension.
    pub fn path(name: &str) -> Result<PathBuf, BlueprintsError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(BlueprintsError::EmptyName);
        }

        let path = Path::new(name);
        if path.file_name().and_then(|v| v.to_str()) != Some(name) {
            return Err(BlueprintsError::PathSeparatorInName);
        }

        if let Some(extension) = path.extension().and_then(|v| v.to_str())
            && extension != "lua"
        {
            return Err(BlueprintsError::InvalidExtension);
        }

        let stem = path
            .file_stem()
            .and_then(|v| v.to_str())
            .filter(|stem| !stem.is_empty())
            .ok_or(BlueprintsError::EmptyName)?;

        Ok(platform::blueprints_dir().join(format!("{stem}.lua")))
    }
}

fn read_dir(path: &Path) -> Result<fs::ReadDir, BlueprintsError> {
    fs::read_dir(path).map_err(|error| match error.kind() {
        ErrorKind::PermissionDenied => BlueprintsError::PermissionDenied,
        ErrorKind::NotFound => BlueprintsError::CannotReadDirectory,
        _ => BlueprintsError::IoError { source: error },
    })
}

impl From<std::io::Error> for BlueprintsError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            _ => Self::IoError { source: error },
        }
    }
}
