use anyhow::{Result, anyhow, bail, ensure};
use std::{fs, path::Path};

use crate::{
    blueprints::{engine::BlueprintEngine, storage::Blueprints},
    cli::{BlueprintsCheckArgs, BlueprintsCommands, BlueprintsNewArgs, BlueprintsRemoveArgs},
    config::Config,
    platform,
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{print_done, print_title},
};

pub fn handle(command: BlueprintsCommands) -> Result<()> {
    match command {
        BlueprintsCommands::New(args) => handle_new(args),
        BlueprintsCommands::List => handle_list(),
        BlueprintsCommands::Check(args) => handle_check(args),
        BlueprintsCommands::MigrateTemplates => handle_migrate(),
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
    let mut blueprints_vec = blueprints.get_blueprints().to_vec();
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

fn handle_check(args: BlueprintsCheckArgs) -> Result<()> {
    let blueprints_dir = platform::blueprints_dir();
    let blueprints = Blueprints::load_from_path(&blueprints_dir)?;
    let blueprint_code = blueprints.get_blueprint(args.name.clone())?;

    let engine = BlueprintEngine::init(Path::new("."), format!("{}.lua", args.name), "check")
        .map_err(|e| anyhow!(e.to_string()))?;
    if let mlua::Result::Err(e) = engine.check(&blueprint_code) {
        bail!("Check failed: {}", e);
    }

    print_done("Blueprint is valid.");

    Ok(())
}

fn handle_migrate() -> Result<()> {
    let blueprints_path = platform::blueprints_dir();
    let templates_path = platform::templates_file();

    if !templates_path.exists() {
        bail!("Templates file not found. There is no need to migrate.");
    }

    let templates = Templates::load(&templates_path)?;
    let templates_list = templates.list_templates();

    for template in templates_list {
        let template_commands = templates.get_template(&template).unwrap();
        let mut blueprint_code = String::new();
        for command in template_commands {
            if command.is_empty() {
                continue;
            }
            let command_splitted: Vec<&str> = command.split_whitespace().collect();

            blueprint_code.push_str(&format!(
                "os.exec(\"{}\"",
                command_splitted.first().unwrap()
            ));

            if command_splitted.len() == 1 {
                blueprint_code.push_str(", {})\r\n");
                continue;
            }

            blueprint_code.push_str(", {");
            for (i, c) in command_splitted.iter().enumerate() {
                if i == 0 {
                    continue;
                }

                if i != 1 {
                    blueprint_code.push(',');
                }

                blueprint_code.push_str(&format!("\"{}\"", c));
            }

            blueprint_code.push_str("})\r\n");
        }

        let blueprint_name = format!("{}.lua", template);
        let blueprint_path = blueprints_path.join(&blueprint_name);
        fs::write(blueprint_path, blueprint_code)
            .map_err(|e| anyhow!("Failed to write blueprint '{}': {}", blueprint_name, e))?;
    }

    fs::remove_file(templates_path)
        .map_err(|e| anyhow!("Failed to remove templates file: {}", e))?;

    print_done("Migration completed! Templates file has been deleted.");

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
