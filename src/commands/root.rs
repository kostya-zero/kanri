use anyhow::{Result, anyhow, ensure};
use colored::Colorize;
use std::time::{Duration, Instant};

use crate::{
    autocomplete,
    backup::{Backup, load_backup, save_backup},
    cli::{BackupArgs, CloneArgs, ImportArgs, ListArgs, NewArgs, OpenArgs, RemoveArgs, RenameArgs},
    config::Config,
    library::{CloneOptions, Library, validate_project_name},
    platform,
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{
        ask_dialog, ask_select, ask_string_dialog, generate_progress, is_terminal, print_done,
        print_error, print_title,
    },
};

fn resolve_project_name(
    project_name: &str,
    config: &Config,
    projects: &Library,
    skip: bool,
) -> Option<String> {
    if project_name == "-" && config.recent.enabled {
        Some(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled && !skip {
        let projects_list: Vec<&str> = projects.get_names().iter().map(|i| i.as_str()).collect();
        autocomplete::autocomplete(project_name, &projects_list, config)
    } else {
        Some(project_name.to_string())
    }
}

pub fn handle_new(args: NewArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let projects_dir = &config.options.projects_directory;
    let mut projects = Library::new(projects_dir, config.options.display_hidden)?;
    let is_terminal = is_terminal();

    let project_name: String;
    let mut template_name: Option<String> = None;

    if let Some(name) = args.name.clone() {
        project_name = name;
    } else {
        if is_terminal {
            let name = ask_string_dialog("Name for new project?", true);
            if name.is_empty() {
                return Err(anyhow!("Project name cannot be empty"));
            }
            project_name = name;
        } else {
            return Err(anyhow!("Project name is required"));
        }
    }

    validate_project_name(&project_name)?;

    if let Some(name) = args.template {
        template_name = Some(name);
    }

    let templates = Templates::load(platform::templates_file())?;
    if template_name.is_none() && !templates.is_empty() && args.name.is_none() && is_terminal {
        let mut items = vec!["none".to_string()];
        let mut keys = templates.get_names();
        items.append(&mut keys);
        let selected = ask_select(&items, true);
        if selected != 0 {
            template_name = Some(items[selected].clone());
        }
    }

    projects.create(&project_name)?;

    if let Some(template_name) = template_name {
        let template = templates
            .get_template(&template_name)
            .ok_or_else(|| anyhow!("Template '{template_name}' not found."))?;

        let profile = config.get_profile(&config.options.current_profile)?;
        let program = &profile.shell;
        ensure!(
            !program.is_empty(),
            "shell is not configured in the configuration file."
        );

        println!(
            "Generating project '{}' from '{}' template...",
            &project_name, &template_name
        );

        let project_path = projects_dir.join(&project_name);
        let env_map = vec![(String::from("KANRI_PROJECT"), project_name.clone())];
        let started_time = Instant::now();
        for command in template {
            println!("{} {}", "=>>".bright_blue().bold(), command.bold().white());

            let mut args_vec = profile.shell_args.clone();
            args_vec.push(command.clone());

            let launch_options = LaunchOptions {
                program,
                args: args_vec,
                cwd: Some(&project_path),
                fork_mode: false,
                quiet: args.quiet,
                env: Some(env_map.clone()),
            };

            if let Err(e) = launch_program(launch_options) {
                print_error("failed to apply template. Cleaning up...");
                projects
                    .delete(&project_name)
                    .map_err(|err| anyhow!("additionally, cleanup failed: {}", err))?;
                return Err(anyhow!("template command '{}' failed: {}", command, e));
            }
        }

        let elapsed_time = started_time.elapsed().as_millis();
        print_done(&format!(
            "Generated '{}' in {elapsed_time} ms.",
            project_name
        ));
    } else {
        print_done(&format!(
            "Created an empty project with name '{}'.",
            project_name
        ));
    }

    Ok(())
}

pub fn handle_clone(args: CloneArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;

    let clone_options = CloneOptions {
        remote: args.remote,
        name: args.name,
        branch: args.branch,
    };

    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    projects.clone(&clone_options)?;

    print_done("Repository has been cloned.");
    Ok(())
}

pub fn handle_open(args: OpenArgs) -> Result<()> {
    if !is_terminal() {
        return Err(anyhow!(
            "Opening projects in non-interactive mode is not supported"
        ));
    }

    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let name = resolve_project_name(&args.name, &config, &projects, args.skip_autocomplete)
        .ok_or_else(|| anyhow!("Project not found."))?;

    let project = projects.get(&name);
    if project.is_none() {
        return Err(anyhow!("Project not found."));
    }
    let path = project.unwrap();

    if args.path {
        println!("{}", path.to_string_lossy());
        return Ok(());
    }

    let profile = config.get_profile(&config.options.current_profile)?;

    let (program, launch_args, fork_mode) = if args.shell {
        (&profile.shell, Vec::<String>::new(), false)
    } else {
        (
            &profile.editor,
            profile.editor_args.clone(),
            profile.editor_fork_mode,
        )
    };

    ensure!(
        !program.is_empty(),
        "Required program is not specified in configuration file."
    );

    let mut launch_options = LaunchOptions {
        program,
        args: launch_args,
        cwd: Some(path),
        fork_mode,
        quiet: false,
        env: None,
    };

    if args.shell {
        let env_map = Vec::from([(String::from("KANRI_SESSION"), "1".to_string())]);
        launch_options.env = Some(env_map);
        println!(
            "{}",
            "======== STARTING SHELL SESSION ========".bold().white()
        );
    }

    launch_program(launch_options)?;

    if args.shell {
        println!(
            "{}",
            "========  SHELL SESSION ENDED  ========".bold().white()
        );
    }

    if config.recent.enabled && name != config.recent.recent_project {
        config.recent.recent_project = name;
        config.save(config_path)?;
    }

    if fork_mode {
        // Because only editor could be launched in fork mode.
        print_done("Editor launched.");
        return Ok(());
    }

    Ok(())
}

pub fn handle_list(args: ListArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    let recent = &config.recent.recent_project;

    if !args.pure {
        print_title("Your projects");
    }
    for name in projects.get_names() {
        if args.pure {
            println!("{}", name);
        } else {
            let is_recent = if name == recent.as_str() {
                "(recent)".dimmed()
            } else {
                "".dimmed()
            };
            println!("  {} {is_recent}", name);
        }
    }

    Ok(())
}

pub fn handle_rename(args: RenameArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let mut projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    validate_project_name(&args.new_name)?;

    projects.rename(&args.old_name, &args.new_name)?;
    print_done(&format!(
        "Project '{}' has been renamed to '{}'.",
        args.old_name, args.new_name
    ));
    Ok(())
}

pub fn handle_remove(args: RemoveArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let mut projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let project_name = resolve_project_name(&args.name, &config, &projects, false)
        .ok_or_else(|| anyhow!("Project not found."))?;

    if is_terminal() {
        if !args.yes
            && !ask_dialog(
                &format!("Do you want to delete '{}'?", project_name),
                false,
                false,
            )
        {
            print_done("Canceled.");
            return Ok(());
        }
    } else {
        if !args.yes {
            return Err(anyhow!(
                "Confirmation with `--yes` is required for non-interactive sessions"
            ));
        }
    }

    let spinner = generate_progress().with_message("Removing project...");

    spinner.enable_steady_tick(Duration::from_millis(100));
    let result = projects.delete(&project_name);
    if let Err(e) = result {
        spinner.finish_and_clear();
        return Err(anyhow!(e));
    }
    spinner.finish_and_clear();

    print_done(&format!("Project '{project_name}' has been removed."));
    Ok(())
}

pub fn handle_backup(args: BackupArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let templates = Templates::load(platform::templates_file())?;

    let backup_file_path = args.output_file.unwrap_or("kanri_backup.json".to_string());

    let backup = Backup { config, templates };
    save_backup(&backup_file_path, backup)?;
    print_done(&format!("Backup saved to `{backup_file_path}`."));
    Ok(())
}

pub fn handle_import(args: ImportArgs) -> Result<()> {
    let backup = load_backup(args.file)?;

    backup.config.save(platform::config_file())?;
    backup.templates.save(platform::templates_file())?;
    print_done("Backup has been imported.");
    Ok(())
}

const KANRI_ZEN: [&str; 10] = [
    "Projects should be simple.",
    "Each command does one thing well.",
    "Configuration is explicit.",
    "Sensible defaults guide the way.",
    "The shell is a friend.",
    "Templates accelerate your workflow.",
    "Cross-platform by design.",
    "Clear messages beat surprises.",
    "Your editor is respected.",
    "Enjoy your work.",
];

pub fn handle_zen() -> Result<()> {
    println!("{}", "========  THE ZEN OF KANRI  ========".bold().white());
    for line in KANRI_ZEN.iter() {
        println!(" {} {line}", "*".dimmed());
    }
    Ok(())
}
