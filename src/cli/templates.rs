use clap::{ArgAction, Args, Subcommand};

#[derive(Subcommand)]
pub enum TemplatesCommands {
    /// Create new template.
    New(TemplatesNewArgs),

    /// List available templates.
    List(TemplatesListArgs),

    /// Edit templates.
    Edit,

    /// Clear all templates.
    Clear,

    /// Prints the path to the file with templates.
    Path,

    /// View information about template.
    Get(TemplatesGetArgs),

    /// Remove template.
    Remove(TemplatesRemoveArgs),
}

#[derive(Args)]
pub struct TemplatesNewArgs {
    pub name: Option<String>,
}

#[derive(Args)]
pub struct TemplatesListArgs {
    /// Display list without styling
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub pure: bool,
}

#[derive(Args)]
pub struct TemplatesGetArgs {
    /// Name of the template.
    pub name: String,

    /// Display list without styling
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub pure: bool,
}

#[derive(Args)]
pub struct TemplatesRemoveArgs {
    /// Name of the template to remove.
    pub name: String,
}
