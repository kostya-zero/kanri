use std::{
    collections::HashSet,
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use crate::program::{LaunchOptions, ProgramError, launch_program};
use anyhow::{Result, anyhow};
use indexmap::IndexMap;
use thiserror::Error;

/// Represents errors that can occur in the Library operations.
#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Name is already taken.")]
    AlreadyExists,

    #[error("Directory with projects is not found.")]
    DirectoryNotFound,

    #[error("Not enough permission to access directory with projects.")]
    PermissionDenied,

    #[error("Provided path is not a directory.")]
    NotADirectory,

    #[error("Project not found.")]
    ProjectNotFound,

    #[error("Invalid path to the projects directory.")]
    InvalidPath,

    #[error("Failed to clone repository: {source}.")]
    CloneFailed { source: ProgramError },

    #[error("This name is not allowed.")]
    InvalidProjectName,

    #[error("{0}")]
    CustomError(String),

    #[error("An unexpected I/O error occurred: {source}.")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}

const IGNORED_NAMES: [&str; 7] = [
    ".",
    "..",
    "$RECYCLE.BIN",
    "System Volume Information",
    "msdownld.tmp",
    ".Trash-1000",
    "-",
];

pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Project name cannot be empty."));
    }

    if name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
        return Err(anyhow!("Project name contains invalid characters."));
    }

    if IGNORED_NAMES.iter().any(|r| name.eq_ignore_ascii_case(r)) {
        return Err(anyhow!("This name is not allowed."));
    }

    // One more check for Windows
    #[cfg(windows)]
    {
        let reserved = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];
        if reserved.iter().any(|r| name.eq_ignore_ascii_case(r)) {
            return Err(anyhow!(
                "Project name cannot be a reserved name on Windows."
            ));
        }
    }

    Ok(())
}

/// An options struct for cloning a repository using Git.
#[derive(Clone, Default)]
pub struct CloneOptions {
    pub remote: String,
    pub branch: Option<String>,
    pub name: Option<String>,
}

/// The Library struct manages a collection of projects in a specified directory.
pub struct Library {
    projects: IndexMap<String, PathBuf>,
    base_path: PathBuf,
}

impl Library {
    /// Makes a new Library instance.
    pub fn new(path: &Path, display_hidden: bool) -> Result<Self, LibraryError> {
        if !path.is_dir() {
            return Err(LibraryError::InvalidPath);
        }
        let projects = Self::collect_projects(path, display_hidden)?;

        Ok(Self {
            projects,
            base_path: path.to_path_buf(),
        })
    }

    /// Collects projects from the specified directory path.
    pub fn collect_projects(
        path: &Path,
        display_hidden: bool,
    ) -> Result<IndexMap<String, PathBuf>, LibraryError> {
        let dir_entries = match fs::read_dir(path) {
            Ok(d) => d,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => return Err(LibraryError::DirectoryNotFound),
                ErrorKind::PermissionDenied => return Err(LibraryError::PermissionDenied),
                ErrorKind::NotADirectory => return Err(LibraryError::NotADirectory),
                _ => return Err(LibraryError::IoError { source: e }),
            },
        };

        let mut projects: IndexMap<String, PathBuf> = IndexMap::new();

        for entry_result in dir_entries {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(e) => match e.kind() {
                    ErrorKind::PermissionDenied => return Err(LibraryError::PermissionDenied),
                    _ => return Err(LibraryError::IoError { source: e }),
                },
            };
            let name = entry.file_name();
            let name_string = name.to_string_lossy();

            if Self::is_valid_project(&entry, &name_string, display_hidden) {
                projects.insert(name_string.to_string(), entry.path());
            }
        }

        // Check if .ignore is present in this directory
        if path.join(".ignore").exists() {
            let paths = Self::get_ignored_paths(path)?;
            projects.retain(|k, _| !paths.contains(k));
        }

        Ok(projects)
    }

    /// Checks if a directory entry is a valid project.
    fn is_valid_project(entry: &fs::DirEntry, name: &str, display_hidden: bool) -> bool {
        if !display_hidden && name.starts_with('.') {
            return false;
        }

        entry.file_type().is_ok_and(|ft| ft.is_dir()) && !IGNORED_NAMES.contains(&name)
    }

    /// Gets ignored paths from a .ignore file in the base path.
    fn get_ignored_paths(base_path: &Path) -> Result<HashSet<String>, LibraryError> {
        let ignore_path = base_path.join(".ignore");
        let ignore_content = fs::read_to_string(ignore_path)?;

        let paths: HashSet<String> = ignore_content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    Some(trimmed.to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(paths)
    }

    /// Clones a Git repository into the library.
    #[must_use = "result may indicate clone failure"]
    pub fn clone(&self, options: &CloneOptions) -> Result<(), LibraryError> {
        let mut args = vec!["clone".to_string(), options.remote.clone()];

        if let Some(name) = &options.name {
            args.push(name.to_owned());
        }

        if let Some(branch) = &options.branch {
            args.push("-b".to_string());
            args.push(branch.to_owned());
        }

        let launch_options = LaunchOptions {
            program: "git",
            args,
            cwd: Some(&self.base_path),
            fork_mode: false,
            quiet: false,
            env: None,
        };

        launch_program(launch_options).map_err(|e| LibraryError::CloneFailed { source: e })
    }

    /// Creates a new project directory in the library.
    pub fn create(&mut self, name: &str) -> Result<(), LibraryError> {
        let path = self.base_path.join(name);
        if path.exists() {
            return Err(LibraryError::AlreadyExists);
        }

        validate_project_name(name).map_err(|e| LibraryError::CustomError(e.to_string()))?;

        match fs::create_dir(&path) {
            Ok(()) => {}
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => return Err(LibraryError::AlreadyExists),
                ErrorKind::PermissionDenied => return Err(LibraryError::PermissionDenied),
                _ => return Err(LibraryError::IoError { source: e }),
            },
        };

        self.projects.insert(name.to_string(), path);
        Ok(())
    }

    /// Deletes a project directory from the library.
    pub fn delete(&mut self, name: &str) -> Result<(), LibraryError> {
        fs::remove_dir_all(self.base_path.join(name))?;
        self.projects.retain(|k, _| k != name);
        Ok(())
    }

    /// Checks if a project with the given name exists in the library.
    pub fn contains(&self, name: &str) -> bool {
        self.projects.contains_key(name)
    }

    /// Returns a vector of all project names in the library.
    pub fn get_names(&self) -> Vec<&String> {
        self.projects.keys().collect()
    }

    /// Retrieves a project by name.
    pub fn get(&self, name: &str) -> Option<&PathBuf> {
        self.projects.get(name)
    }

    /// Returns a reference to a map of all projects in the library.
    pub fn get_all(&self) -> &IndexMap<String, PathBuf> {
        &self.projects
    }

    /// Checks if the library has no projects.
    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }

    /// Renames a project in the library.
    pub fn rename(&mut self, old_name: &str, new_name: &str) -> Result<(), LibraryError> {
        if !self.projects.contains_key(old_name) {
            return Err(LibraryError::ProjectNotFound);
        }

        if self.projects.contains_key(new_name) {
            return Err(LibraryError::AlreadyExists);
        }

        validate_project_name(new_name).map_err(|e| LibraryError::CustomError(e.to_string()))?;

        let old_path = self.base_path.join(old_name);
        let new_path = self.base_path.join(new_name);

        match fs::rename(old_path, &new_path) {
            Ok(()) => {}
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => return Err(LibraryError::PermissionDenied),
                _ => return Err(LibraryError::IoError { source: e }),
            },
        };

        // Using swap_remove because I dont think it will matter not only in tests.
        self.projects.swap_remove(old_name);
        self.projects.insert(new_name.to_string(), new_path);

        Ok(())
    }
}
