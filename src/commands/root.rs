use anyhow::{Result, anyhow, ensure};
use colored::Colorize;
use std::{
    path::Path,
    time::{Duration, Instant},
};

use crate::{
    autocomplete,
    cli::{CloneArgs, ListArgs, NewArgs, OpenArgs, RemoveArgs, RenameArgs},
    config::Config,
    library::{CloneOptions, Library},
    platform,
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{
        ask_dialog, generate_progress, print_done, print_error, print_progress, print_title,
    },
};

fn resolve_project_name(project_name: &str, config: &Config, projects: &Library) -> Option<String> {
    if project_name == "-" && config.recent.enabled {
        Some(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled {
        autocomplete::autocomplete(project_name, projects.get_names(), config)
    } else {
        Some(project_name.to_string())
    }
}

fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Project name cannot be empty."));
    }

    if name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
        return Err(anyhow!("Project name contains invalid characters."));
    }

    if name == "." || name == ".." {
        return Err(anyhow!("Project name cannot be '.' or '..'."));
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
                "Project name cannot be a reserved name on Windows."
            ));
        }
    }

    Ok(())
}

pub fn handle_new(args: NewArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let mut projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name for a new project."))?;

    validate_project_name(&name)?;

    projects.create(&name)?;

    if let Some(template_name) = args.template {
        let templates = Templates::load(platform::templates_file())?;
        let template = templates
            .get_template(&template_name)
            .ok_or_else(|| anyhow!("Template '{}' not found.", template_name))?;

        println!("Generating project from '{}' template...", &name);

        let program = &config.shell.program;
        ensure!(
            !program.is_empty(),
            "Shell is not configured in the configuration file."
        );
        let project_path = Path::new(&config.options.projects_directory)
            .join(&name)
            .to_string_lossy()
            .to_string();
        let total_commands = template.len() as i8;
        let env_map = vec![(String::from("KANRI_PROJECT"), name.clone())];
        let started_time = Instant::now();
        for (idx, command) in template.iter().enumerate() {
            let current = idx as i8 + 1;
            print_progress(command, current, total_commands);

            let mut args_vec = config.shell.args.clone();
            args_vec.push(command.clone());

            let launch_options = LaunchOptions {
                program: program.to_string(),
                args: args_vec,
                cwd: Some(project_path.clone()),
                fork_mode: false,
                quiet: args.quiet,
                env: Some(env_map.clone()),
            };

            if let Err(e) = launch_program(launch_options) {
                print_error("Failed to apply template. Cleaning up...");
                projects
                    .delete(&name)
                    .map_err(|err| anyhow!("Additionally, cleanup failed: {}", err))?;
                return Err(anyhow!("Template command '{}' failed: {}", command, e));
            }
        }

        let elapsed_time = started_time.elapsed().as_millis();
        print_done(&format!("Generated '{name}' in {elapsed_time} ms."));
    } else {
        print_done("Created.");
    }

    Ok(())
}

pub fn handle_clone(args: CloneArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;

    let remote = args
        .remote
        .ok_or_else(|| anyhow!("You need to provide a remote URL."))?;

    let clone_options = CloneOptions {
        remote,
        name: args.name,
        branch: args.branch,
    };

    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    projects.clone(&clone_options)?;

    print_done("Cloned.");
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
        .ok_or_else(|| anyhow!("Project name is required."))?;

    let name = resolve_project_name(&project_name, &config, &projects)
        .ok_or_else(|| anyhow!("Project not found."))?;

    let project = projects
        .get(&name)
        .map_err(|_| anyhow!("Project not found."))?;

    if args.path {
        println!("{}", project.path.display());
        return Ok(());
    }

    let (program, launch_args, fork_mode) = if args.shell {
        (&config.shell.program, Vec::<String>::new(), false)
    } else {
        (
            &config.editor.program,
            config.editor.args.clone(),
            config.editor.fork_mode,
        )
    };

    ensure!(
        !program.is_empty(),
        "Required program is not specified in configuration file."
    );

    if config.recent.enabled && name != config.recent.recent_project {
        config.recent.recent_project = name.clone();
        config.save(config_path)?;
    }

    let mut launch_options = LaunchOptions {
        program: program.to_string(),
        args: launch_args,
        cwd: Some(project.path.to_string_lossy().into_owned()),
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
    for project in projects.get_vec().iter() {
        if args.pure {
            println!("{}", project.name);
        } else {
            let is_recent = if project.name == recent.as_str() {
                "(recent)".dimmed().normal()
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
        .ok_or_else(|| anyhow!("Provide a project name to rename."))?;
    let new_name = args
        .new_name
        .ok_or_else(|| anyhow!("Provide a new name for a project."))?;

    validate_project_name(&new_name)?;

    projects.rename(&old_name, &new_name)?;
    print_done("Renamed.");
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
        .ok_or_else(|| anyhow!("Provide a name of project to remove."))?;

    let project_name = resolve_project_name(&name, &config, &projects)
        .ok_or_else(|| anyhow!("Project not found."))?;

    let project = projects
        .get(&project_name)
        .map_err(|_| anyhow!("Project not found."))?;

    if !project.is_empty()
        && !args.force
        && !ask_dialog("The project is not empty. Continue?", false)
    {
        print_done("Canceled.");
        return Ok(());
    }

    let spinner = generate_progress().with_message("Removing project...");

    spinner.enable_steady_tick(Duration::from_millis(100));
    projects.delete(&project_name)?;
    spinner.finish_and_clear();

    print_done("Removed.");
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
