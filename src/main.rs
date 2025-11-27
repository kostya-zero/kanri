use std::process::exit;

use anyhow::{Result, anyhow};
use clap::Parser;
use kanri::{
    cli::{Cli, Commands, ConfigCommands, ProfilesCommands, TemplatesCommands},
    commands::{config, profiles, root, templates},
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

const KANRI_BUILD_MODE: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

const KANRI_VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_version() {
    println!("kanri {KANRI_VERSION} {KANRI_BUILD_MODE}");
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
            TemplatesCommands::New(args) => templates::handle_new(args),
            TemplatesCommands::List(args) => templates::handle_list(args),
            TemplatesCommands::Edit => templates::handle_edit(),
            TemplatesCommands::Path => templates::handle_path(),
            TemplatesCommands::Get(args) => templates::handle_get(args),
            TemplatesCommands::Clear => templates::handle_clear(),
            TemplatesCommands::Remove(args) => templates::handle_remove(args),
        },
        Commands::Config { command } => match command {
            ConfigCommands::Path => config::handle_path(),
            ConfigCommands::Edit => config::handle_edit(),
            ConfigCommands::Recent(args) => config::handle_recent(args),
            ConfigCommands::Reset => config::handle_reset(),
        },
        Commands::Profiles { command } => match command {
            ProfilesCommands::New => profiles::handle_new(),
            ProfilesCommands::Set(args) => profiles::handle_set(args),
            ProfilesCommands::Info(args) => profiles::handle_info(args),
            ProfilesCommands::List => profiles::handle_list(),
            ProfilesCommands::Remove(args) => profiles::handle_remove(args),
        },
        Commands::Backup(args) => root::handle_backup(args),
        Commands::Import(args) => root::handle_import(args),
        Commands::Zen => root::handle_zen(),
    };

    if let Err(e) = result {
        print_error(&e.to_string());
        exit(1);
    }
}
