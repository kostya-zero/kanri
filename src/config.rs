use crate::platform;
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

    #[error("File system error occurred: {0}.")]
    FileSystemError(#[from] std::io::Error),
}

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub options: GeneralOptions,
    pub editor: EditorOptions,
    pub shell: ShellOptions,
    pub recent: RecentOptions,
    pub autocomplete: AutocompleteOptions,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct GeneralOptions {
    pub projects_directory: PathBuf,
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
            display_hidden: false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct EditorOptions {
    pub program: String,
    pub fork_mode: bool,
    pub args: Vec<String>,
}

impl Default for EditorOptions {
    fn default() -> Self {
        let new_editor = platform::default_editor().to_string();
        let mut new_args: Vec<String> = Vec::new();
        let mut fork_mode = false;

        match new_editor.as_str() {
            "code" | "code-insiders" | "codium" | "code-oss" | "windsurf" | "zed" => {
                new_args.push(".".to_string());
                fork_mode = true;
            }
            _ => {}
        }

        Self {
            program: new_editor,
            fork_mode,
            args: new_args,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct ShellOptions {
    pub program: String,
    pub args: Vec<String>,
}

impl Default for ShellOptions {
    fn default() -> Self {
        let program = platform::default_shell().to_string();
        let args = match program.as_str() {
            "powershell.exe" | "powershell" | "pwsh.exe" | "pwsh" => {
                vec!["-NoLogo".into(), "-Command".into()]
            }
            "cmd" | "cmd.exe" => vec!["/C".into()],
            "zsh" | "bash" | "fish" | "sh" => vec!["-c".into()],
            _ => vec!["-c".into()],
        };
        Self { program, args }
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

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
