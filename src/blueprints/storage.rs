use anyhow::Result;
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlueprintsError {
    #[error("Not enough permission to access directory with blueprints")]
    PersmissionDenied,

    #[error("Failed to list files in directory.")]
    CannotReadDirectory,

    #[error("Blueprint not found.")]
    NotFound,

    #[error("Failed to read blueprint.")]
    ReadFailed,

    #[error("Blueprint with this name already exists.")]
    AlreadyExists,

    #[error("An unexpected I/O error occurred: {source}.")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Default)]
pub struct Blueprints {
    blueprints: Vec<String>,
    path: PathBuf,
}

impl Blueprints {
    /// Loads all blueprints from given path.
    pub fn load_from_path(path: &Path) -> Result<Blueprints, BlueprintsError> {
        let collected_files = Self::collect_lua_files(path)?;
        let storage = Blueprints {
            blueprints: collected_files,
            path: path.to_path_buf(),
        };
        Ok(storage)
    }

    /// Collects all lua files from directory and returns a vector with their filenames without
    /// extension.
    fn collect_lua_files(path: &Path) -> Result<Vec<String>, BlueprintsError> {
        let dir_entries = match fs::read_dir(path) {
            Ok(d) => d,
            Err(_) => return Err(BlueprintsError::CannotReadDirectory),
        };

        let mut files: Vec<String> = Vec::new();

        for entry_result in dir_entries {
            let entry = match entry_result {
                Ok(e) => e,
                Err(error) => match error.kind() {
                    ErrorKind::PermissionDenied => return Err(BlueprintsError::PersmissionDenied),
                    _ => return Err(BlueprintsError::IoError { source: error }),
                },
            };

            let name = entry.file_name();
            let name_string = name.to_string_lossy().to_string();
            if !name_string.ends_with(".lua") {
                continue;
            }

            // Name without extension
            let name_final = Path::new(&name_string)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();

            if !name_final.is_empty() {
                files.push(name_final.to_string());
            }
        }

        Ok(files)
    }

    /// Creates a new file in blueprints directory.
    pub fn create(&self, name: String) -> Result<PathBuf, BlueprintsError> {
        if self.blueprints.contains(&name) {
            return Err(BlueprintsError::AlreadyExists);
        }

        let new_blueprint_path = self.path.join(format!("{}.lua", name));
        fs::write(&new_blueprint_path, "").map_err(|e| BlueprintsError::IoError { source: e })?;
        Ok(new_blueprint_path)
    }

    /// Removes blueprint from blueprints directory.
    pub fn remove(&self, name: String) -> Result<(), BlueprintsError> {
        if !self.blueprints.contains(&name) {
            return Err(BlueprintsError::NotFound);
        }

        let blueprint_path = self.path.join(format!("{}.lua", name));
        fs::remove_file(blueprint_path).map_err(|e| BlueprintsError::IoError { source: e })?;
        Ok(())
    }

    /// Get all blueprints.
    pub fn get_blueprints(&self) -> &Vec<String> {
        &self.blueprints
    }

    /// Get content of a blueprint.
    pub fn get_blueprint(&self, name: String) -> Result<String, BlueprintsError> {
        if !self.blueprints.iter().any(|i| i == &name) {
            return Err(BlueprintsError::NotFound);
        }
        let blueprint_path = Path::new(&name).with_extension(".lua");
        let content =
            fs::read_to_string(blueprint_path).map_err(|_| BlueprintsError::ReadFailed)?;
        Ok(content)
    }
}
