use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum BlueprintsCommands {
    /// Create new blueprint.
    New(BlueprintsNewArgs),

    /// List available blueprints.
    List,

    /// Remove blueprint.
    Remove(BlueprintsRemoveArgs),
}

#[derive(Args)]
pub struct BlueprintsNewArgs {
    /// Name of new blueprint.
    pub name: String,
}

#[derive(Args)]
pub struct BlueprintsRemoveArgs {
    /// Name of blueprint to remove.
    pub name: String,
}
