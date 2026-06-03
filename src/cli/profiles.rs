use clap::{ArgAction, Args, Subcommand};

#[derive(Subcommand)]
pub enum ProfilesCommands {
    /// Create new profile.
    New,

    /// Set profile as default.
    Set(ProfilesSetArgs),

    /// Get information about profile.
    Get(ProfilesGetArgs),

    /// List all available profiles.
    List,

    /// Remove profile.
    Remove(ProfilesRemoveArgs),
}

#[derive(Args)]
pub struct ProfilesSetArgs {
    /// Name of profile to set as current.
    pub name: String,
}

#[derive(Args)]
pub struct ProfilesGetArgs {
    /// Name of profile to get information about.
    pub name: String,
}

#[derive(Args)]
pub struct ProfilesRemoveArgs {
    /// Name of profile to remove.
    pub name: String,

    /// Force removal of profile
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub yes: bool,
}
