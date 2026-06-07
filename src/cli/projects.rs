use clap::{ArgAction, Args};

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

    /// Hide the output of running commands.
    #[arg(short, long, action = ArgAction::SetTrue)]
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
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub shell: bool,

    /// Display the path to the project instead of opening it.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub path: bool,

    /// Disable autocomplete. Usable for integrations.
    #[arg(long, action = ArgAction::SetTrue)]
    pub skip_autocomplete: bool,
}

#[derive(Args)]
pub struct ListArgs {
    /// Display list without styling
    #[arg(short, long, action = ArgAction::SetTrue)]
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
    #[arg(short, long, action = ArgAction::SetTrue)]
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
}
