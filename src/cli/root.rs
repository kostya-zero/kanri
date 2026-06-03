use clap::{ArgAction, Parser};
use std::env;

use super::Commands;

/// Yet another manager for your projects.
#[derive(Parser)]
#[command(
    name = "kanri",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    subcommand_required = true,
    arg_required_else_help = true,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Commands>,

    /// Print the version of Kanri.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub version: bool,
}
