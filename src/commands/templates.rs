use anyhow::{Result, anyhow, bail, ensure};
use std::{fs, io::Write};
use tempfile::NamedTempFile;

use crate::{
    cli::{TemplatesGetArgs, TemplatesListArgs, TemplatesNewArgs, TemplatesRemoveArgs},
    config::Config,
    platform,
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{ask_dialog, print_title},
};

pub fn handle_new(args: TemplatesNewArgs) -> Result<()> {
    ensure!(args.name.is_some(), "provide a name for template.");
    let name = args.name.unwrap();
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "# Write your command here. They will be added to template."
    )?;
    let file_path = file.path().to_str().unwrap();

    let config = Config::load(platform::config_file())?;
    let profile = config.get_profile(&config.options.current_profile)?;
    let launch_options = LaunchOptions {
        program: &profile.editor,
        args: vec![file_path.to_string()],
        cwd: None,
        fork_mode: false,
        quiet: false,
        env: None,
    };

    println!("The editor will launch with opened file.");
    launch_program(launch_options)?;

    let content = fs::read_to_string(file_path)?;
    let mut commands: Vec<String> = Vec::new();

    for line in content.lines() {
        if line.starts_with("#") {
            continue;
        }

        commands.push(line.to_string());
    }

    ensure!(!commands.is_empty(), "no commands entered.");

    println!("Creating template...");
    let templates_path = platform::templates_file();
    let mut templates = Templates::load(&templates_path)?;
    templates.add_template(&name, commands)?;
    if templates.save(templates_path).is_ok() {
        println!("Template has been created.");
    } else {
        bail!("failed to save templates.");
    }
    Ok(())
}

pub fn handle_list(args: TemplatesListArgs) -> Result<()> {
    let templates = Templates::load(platform::templates_file())?;
    if templates.is_empty() {
        println!("No templates found.");
        return Ok(());
    }

    if !args.pure {
        print_title("Templates");
    }

    for template in templates.list_templates().iter() {
        println!("{}{template}", if args.pure { "" } else { " " })
    }
    Ok(())
}

pub fn handle_edit() -> Result<()> {
    let config_path = platform::config_file();
    let config = Config::load(&config_path)?;
    let profile = config.get_profile(&config.options.current_profile)?;
    let editor = profile.editor.clone();
    if editor.is_empty() {
        bail!("editor program name is not set in the configuration file.");
    }

    let templates_path = platform::templates_file();
    let mut editor_args = profile.editor_args.clone();
    editor_args.push(templates_path.to_string_lossy().to_string());

    let launch_options = LaunchOptions {
        program: &editor,
        args: editor_args,
        fork_mode: false,
        quiet: false,
        cwd: None,
        env: None,
    };

    launch_program(launch_options).map_err(|e| anyhow!(e.to_string()))
}

pub fn handle_path() -> Result<()> {
    println!("{}", platform::templates_file().display());
    Ok(())
}

pub fn handle_get(args: TemplatesGetArgs) -> Result<()> {
    let templates = Templates::load(platform::templates_file())?;
    match templates.get_template(&args.name) {
        Some(template) => {
            if !args.pure {
                print_title("Commands of this template");
            }
            for command in template.iter() {
                println!("{}{command}", if args.pure { "" } else { " " });
            }
        }
        None => {
            bail!("Template not found.");
        }
    }
    Ok(())
}

pub fn handle_clear() -> Result<()> {
    let templates_path = platform::templates_file();
    let mut templates = Templates::load(&templates_path)?;
    if ask_dialog("Clear all templates?", false, true) {
        templates.clear();
        templates.save(templates_path)?;
        println!("Templates storage has been cleared.");
    } else {
        println!("Aborted.");
    }
    Ok(())
}

pub fn handle_remove(args: TemplatesRemoveArgs) -> Result<()> {
    let templates_path = platform::templates_file();
    let mut templates = Templates::load(&templates_path)?;
    templates.remove_template(&args.name)?;
    templates.save(templates_path)?;
    println!("Template has been removed.");
    Ok(())
}
