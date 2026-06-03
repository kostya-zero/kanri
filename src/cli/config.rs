use clap::{ArgAction, Args, Subcommand};

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
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub clear: bool,
}
