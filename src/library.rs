use std::{
    collections::HashSet,
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use crate::program::{LaunchOptions, launch_program};
use anyhow::Result;
use thiserror::Error;

/// Represents errors that can occur in the Library operations.
#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Project with the same name already exists.")]
    AlreadyExists,

    #[error("Project not found.")]
    ProjectNotFound,

    #[error("Invalid path to the projects directory.")]
    InvalidPath,

    #[error("File system error occurred.")]
    FileSystemError,

    #[error("Failed to clone repository.")]
    CloneFailed,

    #[error("Could not rename due to error: {0}")]
    FailedToRename(String),

    #[error("This name of the project is not allowed.")]
    InvalidProjectName,

    #[error("An unexpected I/O error occurred: {source}.")]
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
#[derive(Debug)]
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
    projects: Vec<Project>,
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
    ) -> Result<Vec<Project>, LibraryError> {
        let dir_entries = fs::read_dir(path)?;

        let mut projects: Vec<Project> = dir_entries
            .filter_map(|entry_result| {
                let entry = entry_result.ok()?;
                let name = entry.file_name();
                let name_string = name.to_string_lossy();

                if Self::is_valid_project(&entry, &name_string, display_hidden) {
                    Some(Project::new(&name_string, entry.path()))
                } else {
                    None
                }
            })
            .collect();

        // Check if .ignore is present in this directory
        if Path::new(path).join(".ignore").exists() {
            let paths = Self::get_ignored_paths(path)?;
            projects.retain(|project| !paths.contains(&project.name));
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

        let cwd = self.base_path.to_str().unwrap();

        let launch_options = LaunchOptions {
            program: "git".to_string(),
            args,
            cwd: Some(cwd.to_string()),
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

        self.projects.push(Project {
            name: name.to_string(),
            path,
        });
        Ok(())
    }

    /// Deletes a project directory from the library.
    pub fn delete(&mut self, name: &str) -> Result<(), LibraryError> {
        fs::remove_dir_all(self.base_path.join(name)).map_err(|_| LibraryError::FileSystemError)?;
        self.projects.retain(|p| p.name != name);
        Ok(())
    }

    /// Checks if a project with the given name exists in the library.
    pub fn contains(&self, name: &str) -> bool {
        self.projects.iter().any(|x| x.name == *name)
    }

    /// Returns a slice of all projects in the library.
    pub fn get_vec(&self) -> &[Project] {
        &self.projects
    }

    /// Returns a vector of all project names in the library.
    pub fn get_names(&self) -> Vec<&str> {
        self.projects.iter().map(|p| p.name.as_str()).collect()
    }

    /// Retrieves a project by name.
    pub fn get(&self, name: &str) -> Result<&Project, LibraryError> {
        self.projects
            .iter()
            .find(|x| x.name == name)
            .ok_or(LibraryError::ProjectNotFound)
    }

    /// Checks if the library has no projects.
    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }

    /// Renames a project in the library.
    pub fn rename(&mut self, old_name: &str, new_name: &str) -> Result<(), LibraryError> {
        if !self.contains(old_name) {
            return Err(LibraryError::ProjectNotFound);
        }

        if self.contains(new_name) {
            return Err(LibraryError::AlreadyExists);
        }

        if SYSTEM_DIRECTORIES.contains(&new_name) {
            return Err(LibraryError::InvalidProjectName);
        }

        let old_path = self.base_path.join(old_name);
        let new_path = self.base_path.join(new_name);

        fs::rename(old_path, &new_path)
            .map_err(|e| LibraryError::FailedToRename(e.kind().to_string()))?;

        if let Some(project) = self.projects.iter_mut().find(|p| p.name == old_name) {
            project.name = new_name.to_string();
            project.path = new_path;
        }

        Ok(())
    }
}
