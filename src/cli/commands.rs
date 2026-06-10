use clap::Subcommand;

use super::{
    BackupArgs, BlueprintsCommands, CloneArgs, ConfigCommands, ImportArgs, ListArgs, NewArgs,
    OpenArgs, ProfilesCommands, RemoveArgs, RenameArgs,
};

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

    /// Backup configuration and templates into a file.
    Backup(BackupArgs),

    /// Import the configuration and templates from backup file. Will overwrite the current ones.
    Import(ImportArgs),

    /// Display the Zen of Kanri.
    Zen,
}
