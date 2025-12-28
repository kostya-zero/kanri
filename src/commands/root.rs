use anyhow::{Result, anyhow, bail, ensure};
use colored::Colorize;
use std::time::{Duration, Instant};

use crate::{
    autocomplete,
    backup::{Backup, load_backup, save_backup},
    cli::{BackupArgs, CloneArgs, ImportArgs, ListArgs, NewArgs, OpenArgs, RemoveArgs, RenameArgs},
    config::Config,
    library::{CloneOptions, Library},
    platform,
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{ask_dialog, generate_progress, print_error, print_progress, print_title},
};

fn resolve_project_name(project_name: &str, config: &Config, projects: &Library) -> Option<String> {
    if project_name == "-" && config.recent.enabled {
        Some(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled {
        autocomplete::autocomplete(
            project_name,
            projects.get_names().iter().map(|k| k.as_str()).collect(),
            config,
        )
    } else {
        Some(project_name.to_string())
    }
}

fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("project name cannot be empty."));
    }

    if name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
        return Err(anyhow!("project name contains invalid characters."));
    }

    if name == "." || name == ".." {
        return Err(anyhow!("project name cannot be '.' or '..'."));
    }

    // One more check for Windows
    #[cfg(windows)]
    {
        let reserved = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];
        if reserved.contains(&name.to_uppercase().as_str()) {
            return Err(anyhow!(
                "project name cannot be a reserved name on Windows."
            ));
        }
    }

    Ok(())
}

pub fn handle_new(args: NewArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let projects_dir = &config.options.projects_directory;
    let mut projects = Library::new(projects_dir, config.options.display_hidden)?;

    validate_project_name(&args.name)?;
    projects.create(&args.name)?;

    if let Some(template_name) = args.template {
        let templates = Templates::load(platform::templates_file())?;
        let template = templates
            .get_template(&template_name)
            .ok_or_else(|| anyhow!("template '{template_name}' not found."))?;

        let profile = config.get_profile(&config.options.current_profile)?;
        let program = &profile.shell;
        ensure!(
            !program.is_empty(),
            "shell is not configured in the configuration file."
        );

        println!(
            "Generating project '{}' from '{}' template...",
            &args.name, &template_name
        );

        let project_path = projects_dir.join(&args.name);
        let total_commands = template.len();
        let env_map = vec![(String::from("KANRI_PROJECT"), args.name.clone())];
        let started_time = Instant::now();
        for (idx, command) in template.iter().enumerate() {
            let current = idx + 1;
            print_progress(command, current, total_commands);

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
                    .delete(&args.name)
                    .map_err(|err| anyhow!("additionally, cleanup failed: {}", err))?;
                return Err(anyhow!("template command '{}' failed: {}", command, e));
            }
        }

        let elapsed_time = started_time.elapsed().as_millis();
        println!("Generated '{}' in {elapsed_time} ms.", args.name);
    } else {
        println!("Created.");
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

    println!("Repository has been cloned.");
    Ok(())
}

pub fn handle_open(args: OpenArgs) -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let project_name = args
        .name
        .ok_or_else(|| anyhow!("project name is required."))?;

    let name = resolve_project_name(&project_name, &config, &projects)
        .ok_or_else(|| anyhow!("project not found."))?;

    let project = projects
        .get(&name)
        .map_err(|_| anyhow!("project not found."))?;

    if args.path {
        println!("{}", project.path.to_string_lossy());
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
        "required program is not specified in configuration file."
    );

    let mut launch_options = LaunchOptions {
        program,
        args: launch_args,
        cwd: Some(&project.path),
        fork_mode,
        quiet: false,
        env: None,
    };

    if args.shell {
        let env_map = Vec::from([(String::from("KANRI_SESSION"), "1".to_string())]);
        launch_options.env = Some(env_map);
    }

    if args.shell {
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
        println!("Editor launched.");
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
    for project in projects.get_vec().iter() {
        if args.pure {
            println!("{}", project.name);
        } else {
            let is_recent = if project.name == recent.as_str() {
                "(recent)".dimmed()
            } else {
                "".dimmed()
            };
            println!(" {} {is_recent}", project.name);
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

    let old_name = args
        .old_name
        .ok_or_else(|| anyhow!("provide a project name to rename."))?;
    let new_name = args
        .new_name
        .ok_or_else(|| anyhow!("provide a new name for a project."))?;

    validate_project_name(&new_name)?;

    projects.rename(&old_name, &new_name)?;
    println!("Project '{old_name}' has been renamed to '{new_name}'.");
    Ok(())
}

pub fn handle_remove(args: RemoveArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let mut projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let name = args
        .name
        .ok_or_else(|| anyhow!("provide a name of project to remove."))?;

    let project_name = resolve_project_name(&name, &config, &projects)
        .ok_or_else(|| anyhow!("project not found."))?;

    let project = projects
        .get(&project_name)
        .map_err(|_| anyhow!("project not found."))?;

    if !project.is_empty()
        && !args.force
        && !ask_dialog("The project is not empty. Continue?", false, false)
    {
        println!("Canceled.");
        return Ok(());
    }

    let spinner = generate_progress().with_message("Removing project...");

    spinner.enable_steady_tick(Duration::from_millis(100));
    projects.delete(&project_name)?;
    spinner.finish_and_clear();

    println!("Project '{project_name}' has been removed.");
    Ok(())
}

pub fn handle_backup(args: BackupArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let templates = Templates::load(platform::templates_file())?;

    let backup_file_path = args.output_file.unwrap_or("kanri_backup.json".to_string());

    let backup = Backup { config, templates };
    save_backup(&backup_file_path, backup)?;
    println!("Backup saved to `{backup_file_path}`.");
    Ok(())
}

pub fn handle_import(args: ImportArgs) -> Result<()> {
    if args.file.is_none() {
        bail!("ppecify a path where the backup file is located.");
    }

    let backup = load_backup(args.file.unwrap())?;

    backup.config.save(platform::config_file())?;
    backup.templates.save(platform::templates_file())?;
    println!("Backup has been imported.");
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
