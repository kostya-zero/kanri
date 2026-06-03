use anyhow::Result;
use std::{fs, io::ErrorKind, path::Path};

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

    #[error("An unexpected I/O error occurred: {source}.")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Default)]
pub struct Blueprints(Vec<String>);

impl Blueprints {
    pub fn load_from_path(path: &Path) -> Result<Blueprints, BlueprintsError> {
        let collected_files = Self::collect_lua_files(path)?;
        Ok(Blueprints(collected_files))
    }

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

    pub fn get_blueprints(&self) -> &Vec<String> {
        &self.0
    }

    pub fn get_blueprint(&self, name: String) -> Result<String, BlueprintsError> {
        if !self.0.iter().any(|i| i == &name) {
            return Err(BlueprintsError::NotFound);
        }
        let blueprint_path = Path::new(&name).with_extension(".lua");
        let content =
            fs::read_to_string(blueprint_path).map_err(|_| BlueprintsError::ReadFailed)?;
        Ok(content)
    }
}
