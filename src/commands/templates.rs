use anyhow::{Result, anyhow, bail, ensure};

use crate::{
    cli::{TemplatesInfoArgs, TemplatesListArgs, TemplatesRemoveArgs},
    config::Config,
    platform::{self, config_file},
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{ask_dialog, ask_string_dialog, print_done, print_title},
};

pub fn handle_new() -> Result<()> {
    let name = ask_string_dialog("Name of new template?");
    if name.is_empty() {
        bail!("Incorrect name for a template.");
    }

    let mut commands: Vec<String> = Vec::new();
    loop {
        let command = ask_string_dialog("Enter a command (press enter to finish):");
        if command.is_empty() {
            break;
        }
        commands.push(command);
    }

    ensure!(!commands.is_empty(), "No commands entered.");

    println!("Creating template...");
    let templates_path = platform::templates_file();
    let mut templates = Templates::load(&templates_path)?;
    templates.add_template(&name, commands)?;
    if templates.save(templates_path).is_ok() {
        print_done("Created.");
    } else {
        bail!("Failed to save templates.");
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
    let templates_path = platform::templates_file();
    let config_path = platform::config_file();
    let config = Config::load(&config_path)?;
    let editor = &config.editor.program;
    if editor.is_empty() {
        bail!("Editor program name is not set in the configuration file.");
    }

    let mut editor_args = config.editor.args.clone();
    editor_args.push(templates_path.to_string_lossy().to_string());

    let launch_options = LaunchOptions {
        program: editor.to_string(),
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

pub fn handle_info(args: TemplatesInfoArgs) -> Result<()> {
    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name of the template."))?;

    let templates = Templates::load(platform::templates_file())?;
    match templates.get_template(&name) {
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
    if ask_dialog("Clear all templates?", false) {
        templates.clear();
        templates.save(templates_path)?;
        print_done("Cleared.");
    } else {
        print_done("Aborted.");
    }
    Ok(())
}

pub fn handle_remove(args: TemplatesRemoveArgs) -> Result<()> {
    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name of template to delete."))?;
    let templates_path = platform::templates_file();
    let mut templates = Templates::load(&templates_path)?;
    templates.remove_template(&name)?;
    templates.save(templates_path)?;
    print_done("Removed.");
    Ok(())
}
