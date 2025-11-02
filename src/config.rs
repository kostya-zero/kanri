use crate::platform;
use indexmap::{IndexMap, indexmap};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to write configuration file.")]
    WriteFailed,

    #[error("Failed to format configuration to TOML.")]
    FormatFailed,

    #[error("Cannot find configuration file.")]
    FileNotFound,

    #[error("Error parsing configuration file: {0}.")]
    BadConfiguration(String),

    #[error("Profile '{0}' was not found.")]
    ProfileNotFound(String),

    #[error("File system error occurred: {0}.")]
    FileSystemError(#[from] std::io::Error),
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub version: String,
    pub options: GeneralOptions,
    pub profiles: IndexMap<String, Profile>,
    pub recent: RecentOptions,
    pub autocomplete: AutocompleteOptions,
}

impl Default for Config {
    fn default() -> Self {
        // Getting default editor
        let editor = platform::default_editor().to_string();
        let mut editor_args: Vec<String> = Vec::new();
        let mut editor_fork_mode = false;

        match editor.as_str() {
            "code" | "code-insiders" | "codium" | "code-oss" | "windsurf" | "zed" => {
                editor_args.push(".".to_string());
                editor_fork_mode = true;
            }
            _ => {}
        }

        // Getting default shell
        let shell = platform::default_shell().to_string();
        let shell_args = match shell.as_str() {
            "powershell.exe" | "powershell" | "pwsh.exe" | "pwsh" => {
                vec!["-NoLogo".into(), "-Command".into()]
            }
            "cmd" | "cmd.exe" => vec!["/C".into()],
            "zsh" | "bash" | "fish" | "sh" => vec!["-c".into()],
            _ => vec!["-c".into()],
        };

        let profiles = indexmap! {
            String::from("default") => Profile {
                editor,
                editor_args,
                editor_fork_mode,
                shell,
                shell_args
            }
        };

        Self {
            version: "1".to_string(),
            options: GeneralOptions::default(),
            profiles,
            recent: RecentOptions::default(),
            autocomplete: AutocompleteOptions::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct Profile {
    pub editor: String,
    pub editor_args: Vec<String>,
    pub editor_fork_mode: bool,
    pub shell: String,
    pub shell_args: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct GeneralOptions {
    pub projects_directory: PathBuf,
    pub current_profile: String,
    pub display_hidden: bool,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct AutocompleteOptions {
    pub enabled: bool,
    pub always_accept: bool,
}

impl Default for AutocompleteOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            always_accept: true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct RecentOptions {
    pub enabled: bool,
    pub recent_project: String,
}

impl Default for RecentOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            recent_project: String::new(),
        }
    }
}

impl Default for GeneralOptions {
    fn default() -> Self {
        Self {
            projects_directory: platform::default_projects_dir(),
            current_profile: "default".to_string(),
            display_hidden: false,
        }
    }
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(&path).map_err(|_| ConfigError::FileNotFound)?;
        toml::from_str::<Config>(&content)
            .map_err(|e| ConfigError::BadConfiguration(format!("{e}")))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|_| ConfigError::WriteFailed)?;
        }
        let content = toml::to_string(self).map_err(|_| ConfigError::FormatFailed)?;
        fs::write(path, content).map_err(|_| ConfigError::WriteFailed)
    }

    pub fn get_profile(&self, name: &str) -> Result<&Profile, ConfigError> {
        if let Some(p) = self.profiles.get(name) {
            Ok(p)
        } else {
            Err(ConfigError::ProfileNotFound(name.to_string()))
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
