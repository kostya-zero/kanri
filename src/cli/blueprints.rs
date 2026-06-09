use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum BlueprintsCommands {
    /// Create new blueprint.
    New(BlueprintsNewArgs),

    /// List available blueprints.
    List,

    /// Check blueprint on syntax errors. It doesn't execute the code.
    BlueprintsCheckArgs(CheckArgs),

    /// Remove blueprint.
    Remove(BlueprintsRemoveArgs),
}

#[derive(Args)]
pub struct BlueprintsNewArgs {
    /// Name of new blueprint.
    pub name: String,
}

#[derive(Args)]
pub struct CheckArgs {
    /// Name of blueprint to check.
    pub name: String,
}

#[derive(Args)]
pub struct BlueprintsRemoveArgs {
    /// Name of blueprint to remove.
    pub name: String,
}
