use std::process::exit;

use anyhow::{Result, anyhow};
use clap::Parser;
use kanri::{
    cli::{Cli, Commands, ConfigCommands, TemplatesCommands},
    commands::{config, root, templates},
    config::Config,
    platform,
    templates::Templates,
    terminal::print_error,
};

fn check_env() -> Result<()> {
    let config_path = platform::config_file();
    if !config_path.exists() {
        let default_config: Config = Config::default();
        default_config
            .save(config_path)
            .map_err(|e| anyhow!(e.to_string()))?;
    }

    let templates_path = platform::templates_file();
    if !templates_path.exists() {
        let templates = Templates::new();
        templates
            .save(templates_path)
            .map_err(|e| anyhow!(e.to_string()))?;
    }

    Ok(())
}

fn print_version() {
    let mode = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    println!("kanri {} {mode}", env!("CARGO_PKG_VERSION"));
}

fn main() {
    colored::control::set_override(std::env::var("NO_COLOR").is_err());
    let cli = Cli::parse();

    if cli.version {
        print_version();
        return;
    }

    if let Err(e) = check_env() {
        print_error(&e.to_string());
        exit(1);
    }

    if cli.cmd.is_none() {
        println!("Nothing to do. Use `kanri --help` to see available commands.");
        return;
    }

    let result = match cli.cmd.unwrap() {
        Commands::New(args) => root::handle_new(args),
        Commands::Clone(args) => root::handle_clone(args),
        Commands::Open(args) => root::handle_open(args),
        Commands::List(args) => root::handle_list(args),
        Commands::Rename(args) => root::handle_rename(args),
        Commands::Remove(args) => root::handle_remove(args),
        Commands::Templates { command } => match command {
            TemplatesCommands::New => templates::handle_new(),
            TemplatesCommands::List(args) => templates::handle_list(args),
            TemplatesCommands::Edit => templates::handle_edit(),
            TemplatesCommands::Path => templates::handle_path(),
            TemplatesCommands::Info(args) => templates::handle_info(args),
            TemplatesCommands::Clear => templates::handle_clear(),
            TemplatesCommands::Remove(args) => templates::handle_remove(args),
        },
        Commands::Config { command } => match command {
            ConfigCommands::Path => config::handle_path(),
            ConfigCommands::Edit => config::handle_edit(),
            ConfigCommands::Recent(args) => config::handle_recent(args),
            ConfigCommands::Reset => config::handle_reset(),
        },
        Commands::Zen => root::handle_zen(),
    };

    if let Err(e) = result {
        print_error(&e.to_string());
        exit(1);
    }
}
