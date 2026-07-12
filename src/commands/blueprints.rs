use anyhow::{Result, anyhow, bail, ensure};
use std::{fs, path::Path};

use crate::{
    blueprints::{engine::BlueprintEngine, storage::Blueprints},
    cli::{
        BlueprintsCheckArgs, BlueprintsCommands, BlueprintsEditArgs, BlueprintsNewArgs,
        BlueprintsRemoveArgs,
    },
    config::Config,
    platform,
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{print_done, print_title},
};

pub fn handle(command: BlueprintsCommands) -> Result<()> {
    match command {
        BlueprintsCommands::New(args) => handle_new(args),
        BlueprintsCommands::Edit(args) => handle_edit(args),
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

    open_blueprint_in_editor(&blueprint_path)?;
    print_done("Blueprint has been created.");
    Ok(())
}

fn handle_edit(args: BlueprintsEditArgs) -> Result<()> {
    let blueprint_path = blueprint_path(&args.name)?;
    ensure!(blueprint_path.exists(), "Blueprint not found.");

    open_blueprint_in_editor(&blueprint_path)?;
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

    let engine = BlueprintEngine::init(Path::new("."), format!("{}.lua", args.name), "check", true)
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
        let template_commands = templates
            .get_template(&template)
            .ok_or_else(|| anyhow!("Template '{}' not found during migration.", template))?;
        let mut blueprint_code = String::new();
        for command in template_commands {
            if let Some(line) = template_command_to_blueprint_line(command)? {
                blueprint_code.push_str(&line);
            }
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

fn template_command_to_blueprint_line(command: &str) -> Result<Option<String>> {
    if command.trim().is_empty() {
        return Ok(None);
    }

    let parts = split_template_command(command)?;
    ensure!(!parts.is_empty(), "Template command cannot be empty.");

    let program = lua_string(&parts[0]);
    let args = parts
        .iter()
        .skip(1)
        .map(|arg| lua_string(arg))
        .collect::<Vec<_>>()
        .join(", ");

    Ok(Some(format!("os.exec({program}, {{{args}}})\n")))
}

fn split_template_command(command: &str) -> Result<Vec<String>> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut in_token = false;

    for ch in command.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            in_token = true;
            continue;
        }

        match (quote, ch) {
            (Some('\''), '\'') => {
                quote = None;
                in_token = true;
            }
            (Some('\''), _) => {
                current.push(ch);
                in_token = true;
            }
            (Some('"'), '"') => {
                quote = None;
                in_token = true;
            }
            (Some('"'), '\\') => {
                escaped = true;
                in_token = true;
            }
            (Some('"'), _) => {
                current.push(ch);
                in_token = true;
            }
            (None, '\'' | '"') => {
                quote = Some(ch);
                in_token = true;
            }
            (None, '\\') => {
                escaped = true;
                in_token = true;
            }
            (None, ch) if ch.is_whitespace() => {
                if in_token {
                    parts.push(std::mem::take(&mut current));
                    in_token = false;
                }
            }
            (None, _) => {
                current.push(ch);
                in_token = true;
            }
            (Some(_), _) => unreachable!("only single and double quotes are supported"),
        }
    }

    if escaped {
        bail!("Template command ends with an unfinished escape sequence.");
    }

    if let Some(quote) = quote {
        bail!("Template command contains an unclosed {quote} quote.");
    }

    if in_token {
        parts.push(current);
    }

    Ok(parts)
}

fn lua_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');

    for ch in value.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            ch if ch.is_control() => output.push_str(&format!("\\{}", ch as u32)),
            ch => output.push(ch),
        }
    }

    output.push('"');
    output
}

fn handle_remove(args: BlueprintsRemoveArgs) -> Result<()> {
    let blueprint_path = blueprint_path(&args.name)?;
    ensure!(blueprint_path.exists(), "Blueprint not found.");

    fs::remove_file(blueprint_path)?;
    print_done("Blueprint has been removed.");
    Ok(())
}

fn open_blueprint_in_editor(blueprint_path: &Path) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let profile = config.get_profile(&config.options.current_profile)?;
    if profile.editor.is_empty() {
        bail!("Editor program name is not set in the configuration file.");
    }

    let mut editor_args = profile.editor_args.clone();
    editor_args.push(blueprint_path.to_string_lossy().to_string());

    let launch_options = LaunchOptions {
        program: &profile.editor,
        args: editor_args,
        cwd: None,
        fork_mode: profile.editor_fork_mode,
        quiet: false,
        env: None,
    };

    println!("The editor will launch with opened file.");
    launch_program(launch_options).map_err(|e| anyhow!(e.to_string()))
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
