use std::{
    collections::HashSet,
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use crate::program::{launch_program, LaunchOptions};
use anyhow::Result;
use indexmap::IndexMap;
use thiserror::Error;

/// Represents errors that can occur in the Library operations.
#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("project with the same name already exists.")]
    AlreadyExists,

    #[error("project not found.")]
    ProjectNotFound,

    #[error("invalid path to the projects directory.")]
    InvalidPath,

    #[error("file system error occurred.")]
    FileSystemError,

    #[error("failed to clone repository.")]
    CloneFailed,

    #[error("could not rename due to error: {0}")]
    FailedToRename(String),

    #[error("this name of the project is not allowed.")]
    InvalidProjectName,

    #[error("an unexpected I/O error occurred: {source}.")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}

const SYSTEM_DIRECTORIES: [&str; 6] = [
    ".",
    "..",
    "$RECYCLE.BIN",
    "System Volume Information",
    "msdownld.tmp",
    ".Trash-1000",
];

/// An options struct for cloning a repository using Git.
#[derive(Debug, Clone, Default)]
pub struct CloneOptions {
    pub remote: String,
    pub branch: Option<String>,
    pub name: Option<String>,
}

/// An abstraction representing a project in the library.
#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

impl Project {
    /// Makes a new Project instance.
    pub fn new(new_name: &str, new_path: PathBuf) -> Self {
        Self {
            name: new_name.to_string(),
            path: new_path,
        }
    }

    /// Checks if the project directory is empty.
    pub fn is_empty(&self) -> bool {
        fs::read_dir(&self.path)
            .map(|mut dir| dir.next().is_none())
            .unwrap_or(false)
    }
}

/// The Library struct manages a collection of projects in a specified directory.
#[derive(Debug)]
pub struct Library {
    projects: IndexMap<String, Project>,
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
    ) -> Result<IndexMap<String, Project>, LibraryError> {
        let dir_entries = fs::read_dir(path)?;
        let mut projects: IndexMap<String, Project> = IndexMap::new();

        for entry_result in dir_entries {
            let entry = entry_result.unwrap();
            let name = entry.file_name();
            let name_string = name.to_string_lossy();

            if Self::is_valid_project(&entry, &name_string, display_hidden) {
                projects.insert(
                    name_string.to_string(),
                    Project::new(&name_string, entry.path()),
                );
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

        entry.file_type().is_ok_and(|ft| ft.is_dir()) && !SYSTEM_DIRECTORIES.contains(&name)
    }

    /// Gets ignored paths from a .ignore file in the base path.
    fn get_ignored_paths(base_path: &Path) -> Result<HashSet<String>, LibraryError> {
        let ignore_path = base_path.join(".ignore");
        if !ignore_path.exists() {
            return Err(LibraryError::IoError {
                source: Error::new(ErrorKind::NotFound, "No .ignore file found"),
            });
        }

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

        launch_program(launch_options).map_err(|_| LibraryError::CloneFailed)
    }

    /// Creates a new project directory in the library.
    pub fn create(&mut self, name: &str) -> Result<(), LibraryError> {
        let path = self.base_path.join(name);
        if path.exists() {
            return Err(LibraryError::AlreadyExists);
        }
        fs::create_dir(&path).map_err(|e| LibraryError::IoError { source: e })?;

        self.projects.insert(
            name.to_string(),
            Project {
                name: name.to_string(),
                path,
            },
        );
        Ok(())
    }

    /// Deletes a project directory from the library.
    pub fn delete(&mut self, name: &str) -> Result<(), LibraryError> {
        fs::remove_dir_all(self.base_path.join(name)).map_err(|_| LibraryError::FileSystemError)?;
        self.projects.retain(|k, _| k != name);
        Ok(())
    }

    /// Checks if a project with the given name exists in the library.
    pub fn contains(&self, name: &str) -> bool {
        self.projects.contains_key(name)
    }

    /// Returns a slice of all projects in the library.
    pub fn get_vec(&self) -> Vec<&Project> {
        self.projects.iter().map(|p| p.1).collect()
    }

    /// Returns a vector of all project names in the library.
    pub fn get_names(&self) -> Vec<&String> {
        self.projects.keys().collect()
    }

    /// Retrieves a project by name.
    pub fn get(&self, name: &str) -> Result<&Project, LibraryError> {
        self.projects.get(name).ok_or(LibraryError::ProjectNotFound)
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

        if SYSTEM_DIRECTORIES.contains(&new_name) {
            return Err(LibraryError::InvalidProjectName);
        }

        let old_path = self.base_path.join(old_name);
        let new_path = self.base_path.join(new_name);

        fs::rename(old_path, &new_path)
            .map_err(|e| LibraryError::FailedToRename(e.kind().to_string()))?;

        // If first check have passed, then we can unwrap safely.
        let mut old_project = self.projects.get(old_name).cloned().unwrap();

        // Using swap_remove because I dont think it will matter not only in tests.
        self.projects.swap_remove(old_name);
        old_project.name = new_name.to_string();
        old_project.path = new_path;
        self.projects.insert(new_name.to_string(), old_project);

        Ok(())
    }
}
