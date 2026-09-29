use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlueprintsError {
    #[error("Not enough permission to access directory with blueprints")]
    PermissionDenied,

    #[error("Failed to list files in directory.")]
    CannotReadDirectory,

    #[error("Blueprint not found.")]
    NotFound,

    #[error("Blueprint with this name already exists.")]
    AlreadyExists,

    #[error("An unexpected I/O error occurred: {source}.")]
    IoError { source: std::io::Error },
}

pub struct Blueprints {
    blueprints: Vec<String>,
    path: PathBuf,
}

impl Blueprints {
    /// Loads all blueprints from given path.
    pub fn load_from_path(path: &Path) -> Result<Self, BlueprintsError> {
        Ok(Self {
            blueprints: Self::collect_lua_files(path)?,
            path: path.to_path_buf(),
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

    /// Get content of a blueprint.
    pub fn get_blueprint(&self, name: impl AsRef<str>) -> Result<String, BlueprintsError> {
        let name = name.as_ref();
        if !self.contains(name) {
            return Err(BlueprintsError::NotFound);
        }

        let path = self.path.join(name).with_added_extension("lua");
        Ok(fs::read_to_string(path)?)
    }

    fn contains(&self, name: &str) -> bool {
        self.blueprints.iter().any(|blueprint| blueprint == name)
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
