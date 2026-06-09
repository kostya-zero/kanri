use anyhow::{Result, anyhow, bail, ensure};
use std::{fs, path::Path};

use crate::{
    blueprints::{engine::BlueprintEngine, storage::Blueprints},
    cli::{BlueprintsCommands, BlueprintsNewArgs, BlueprintsRemoveArgs, CheckArgs},
    config::Config,
    platform,
    program::{LaunchOptions, launch_program},
    terminal::{print_done, print_title},
};

pub fn handle(command: BlueprintsCommands) -> Result<()> {
    match command {
        BlueprintsCommands::New(args) => handle_new(args),
        BlueprintsCommands::List => handle_list(),
        BlueprintsCommands::BlueprintsCheckArgs(args) => handle_check(args),
        BlueprintsCommands::Remove(args) => handle_remove(args),
    }
}

fn handle_new(args: BlueprintsNewArgs) -> Result<()> {
    let blueprint_path = blueprint_path(&args.name)?;
    ensure!(!blueprint_path.exists(), "Blueprint already exists.");

    if let Some(parent) = blueprint_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(
        &blueprint_path,
        "-- Write your blueprint here.\n-- Example: fs.write(\"README.md\", \"# New project\\n\")\n",
    )?;

    let config = Config::load(platform::config_file())?;
    let profile = config.get_profile(&config.options.current_profile)?;
    if profile.editor.is_empty() {
        bail!("Editor program name is not set in the configuration file.");
    }

    let file_path = blueprint_path.to_string_lossy().to_string();
    let launch_options = LaunchOptions {
        program: &profile.editor,
        args: vec![file_path],
        cwd: None,
        fork_mode: profile.editor_fork_mode,
        quiet: false,
        env: None,
    };

    println!("The editor will launch with opened file.");
    launch_program(launch_options).map_err(|e| anyhow!(e.to_string()))?;
    print_done("Blueprint has been created.");
    Ok(())
}

fn handle_list() -> Result<()> {
    let blueprints = Blueprints::load_from_path(&platform::blueprints_dir())?;
    let mut blueprints_vec = blueprints.get_blueprints().clone();
    blueprints_vec.sort();

    if blueprints_vec.is_empty() {
        println!("No blueprints found.");
        return Ok(());
    }

    print_title("Your blueprints");
    for blueprint in blueprints_vec {
        println!(" {blueprint}");
    }
    Ok(())
}

fn handle_check(args: CheckArgs) -> Result<()> {
    let blueprints_dir = platform::blueprints_dir();
    let blueprints = Blueprints::load_from_path(&blueprints_dir)?;
    let blueprint_code = blueprints.get_blueprint(args.name.clone())?;

    let engine = BlueprintEngine::init(Path::new("."), format!("{}.lua", args.name));
    if let mlua::Result::Err(e) = engine.check(&blueprint_code) {
        bail!("Check failed: {}", e);
    }

    print_done("Blueprint is valid.");

    Ok(())
}

fn handle_remove(args: BlueprintsRemoveArgs) -> Result<()> {
    let blueprint_path = blueprint_path(&args.name)?;
    ensure!(blueprint_path.exists(), "Blueprint not found.");

    fs::remove_file(blueprint_path)?;
    print_done("Blueprint has been removed.");
    Ok(())
}

fn blueprint_path(name: &str) -> Result<std::path::PathBuf> {
    let file_name = blueprint_file_name(name)?;
    Ok(platform::blueprints_dir().join(file_name))
}

fn blueprint_file_name(name: &str) -> Result<String> {
    let name = name.trim();
    ensure!(!name.is_empty(), "Blueprint name cannot be empty.");

    let path = Path::new(name);
    ensure!(
        path.file_name().and_then(|v| v.to_str()) == Some(name),
        "Blueprint name must not contain path separators."
    );

    if let Some(extension) = path.extension().and_then(|v| v.to_str()) {
        ensure!(extension == "lua", "Blueprint file extension must be .lua.");
    }

    let stem = path
        .file_stem()
        .and_then(|v| v.to_str())
        .ok_or_else(|| anyhow!("Blueprint name cannot be empty."))?;
    ensure!(!stem.is_empty(), "Blueprint name cannot be empty.");

    Ok(format!("{stem}.lua"))
}
