use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Templates sturct represents a store of users templates.
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Templates(HashMap<String, Vec<String>>);

/// An error that can happen in Templates struct methods.
#[derive(Debug, Error, Deserialize)]
pub enum TemplatesError {
    #[error("This name is already taken.")]
    AlreadyExists,

    #[error("Template not found.")]
    TemplateNotFound,

    #[error("File system error occurred.")]
    FileSystemError,

    #[error("Failed to serialize templates data.")]
    SerializationError,

    #[error("Failed to deserialize templates data.")]
    DeserializationError,

    #[error("Commands in the template are empty.")]
    CommandsAreEmpty,
}

impl Templates {
    /// Creates an empty instance.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_template(
        &mut self,
        name: &str,
        commands: Vec<String>,
    ) -> Result<(), TemplatesError> {
        if self.0.contains_key(name) {
            return Err(TemplatesError::AlreadyExists);
        }

        if commands.iter().any(|cmd| cmd.trim().is_empty()) {
            return Err(TemplatesError::CommandsAreEmpty);
        }

        self.0.insert(name.to_string(), commands);
        Ok(())
    }

    pub fn get_template(&self, name: &str) -> Option<&Vec<String>> {
        self.0.get(name)
    }

    pub fn remove_template(&mut self, name: &str) -> Result<(), TemplatesError> {
        if self.0.remove(name).is_none() {
            return Err(TemplatesError::TemplateNotFound);
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn list_templates(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, TemplatesError> {
        let content = fs::read_to_string(path).map_err(|_| TemplatesError::FileSystemError)?;
        let templates: Self =
            serde_json::from_str(&content).map_err(|_| TemplatesError::DeserializationError)?;
        Ok(templates)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), TemplatesError> {
        let content =
            serde_json::to_string_pretty(self).map_err(|_| TemplatesError::SerializationError)?;
        fs::write(path, content).map_err(|_| TemplatesError::FileSystemError)
    }
}
