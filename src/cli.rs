use clap::{Args, Parser, Subcommand};

/// Yet another manager for your projects.
#[derive(Parser)]
#[command(
    name = "kanri",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    arg_required_else_help = true,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Commands>,

    /// Print the version of Kanri.
    #[arg(short, long)]
    pub version: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create new project.
    New(NewArgs),

    /// Clone Git repository (requires git to be installed).
    Clone(CloneArgs),

    /// Open project in editor or shell [alias: o]
    #[command(alias = "o")]
    Open(OpenArgs),

    /// List available projects [alias: ls]
    #[command(alias = "ls")]
    List(ListArgs),

    /// Rename project.
    Rename(RenameArgs),

    /// Remove project [alias: rm]
    #[command(alias = "rm")]
    Remove(RemoveArgs),

    /// Manage blueprints
    Blueprints {
        #[command(subcommand)]
        command: BlueprintsCommands,
    },

    /// Manage your configuration.
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Manage your profiles.
    Profiles {
        #[command(subcommand)]
        command: ProfilesCommands,
    },

    /// Backup configuration and blueprints into a file.
    Backup(BackupArgs),

    /// Import the configuration and blueprints from backup file. Will overwrite the current ones.
    Import(ImportArgs),

    /// Display the Zen of Kanri.
    Zen,
}

#[derive(Args)]
pub struct NewArgs {
    /// Name for a new project.
    pub name: String,

    // Blueprint to use for a new project.
    #[arg(short, long)]
    pub blueprint: Option<String>,

    /// Template to use for a new project.
    #[arg(short, long)]
    pub template: Option<String>,

    /// Hide the logs and the output of running commands.
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Args)]
pub struct CloneArgs {
    /// URL of repository to clone.
    pub remote: String,

    /// Directory name for the cloned repository.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Branch to clone.
    #[arg(short, long)]
    pub branch: Option<String>,
}

#[derive(Args)]
pub struct OpenArgs {
    /// Name of the project to open.
    pub name: String,

    /// Open shell in this project.
    #[arg(short, long)]
    pub shell: bool,

    /// Display the path to the project instead of opening it.
    #[arg(short, long)]
    pub path: bool,

    /// Disable autocomplete. Usable for integrations.
    #[arg(long)]
    pub skip_autocomplete: bool,
}

#[derive(Args)]
pub struct ListArgs {
    /// Display list without styling
    #[arg(short, long)]
    pub pure: bool,
}

#[derive(Args)]
pub struct RenameArgs {
    /// Old project name.
    pub old_name: String,

    /// New project name.
    pub new_name: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Name of the project to remove.
    pub name: String,

    /// Confirm the removal.
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args)]
pub struct BackupArgs {
    /// The path where to write backup file.
    pub output_file: Option<String>,
}

#[derive(Args)]
pub struct ImportArgs {
    /// The path to the backup file.
    pub file: String,

    /// Confirm the import.
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Subcommand)]
pub enum BlueprintsCommands {
    /// Create new blueprint.
    New {
        /// Name of new blueprint.
        name: String,
    },

    /// Edit an existing blueprint.
    Edit {
        /// Name of blueprint to edit.
        name: String,
    },

    /// List available blueprints.
    List,

    /// Perform a migration of templates to blueprints.
    MigrateTemplates,

    /// Check blueprint on syntax errors. It doesn't execute the code.
    Check {
        /// Name of blueprint to check.
        name: String,
    },

    /// Remove blueprint.
    Remove {
        /// Name of blueprint to remove.
        name: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Edit configuration file.
    Edit,

    /// Get path to the configuration file.
    Path,

    /// Get the recent project name.
    Recent(RecentArgs),

    /// Reset your configuration.
    Reset,
}

#[derive(Args)]
pub struct RecentArgs {
    /// Clear the recent project.
    #[arg(short, long)]
    pub clear: bool,
}

#[derive(Subcommand)]
pub enum ProfilesCommands {
    /// Create new profile.
    New,

    /// Set profile as default.
    Set {
        /// Name of profile to set as current.
        name: String,
    },

    /// Get information about profile.
    Get {
        /// Name of profile to get information about.
        name: String,
    },

    /// List all available profiles.
    List,

    /// Remove profile.
    Remove(ProfilesRemoveArgs),
}

#[derive(Args)]
pub struct ProfilesRemoveArgs {
    /// Name of profile to remove.
    pub name: String,

    /// Force removal of profile
    #[arg(short, long)]
    pub yes: bool,
}
