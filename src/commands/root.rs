use anyhow::{Context, Result, anyhow, bail, ensure};
use colored::Colorize;
use indexmap::IndexMap;
use std::{fs, time::Duration};

use crate::{
    autocomplete,
    backup::{Backup, load_backup, save_backup},
    blueprints::{engine::BlueprintEngine, storage::Blueprints},
    cli::{BackupArgs, CloneArgs, ImportArgs, ListArgs, NewArgs, OpenArgs, RemoveArgs, RenameArgs},
    config::Config,
    library::{CloneOptions, Library},
    platform::{self},
    print_done, print_error, print_title,
    program::{LaunchOptions, launch_program},
    terminal::{ask_dialog, generate_progress, is_terminal},
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

    let blueprint_obj: Option<(String, String)> = if let Some(blueprint_name) = args.blueprint {
        let blueprint_code = Blueprints::read(&blueprint_name)?;
        Some((blueprint_name, blueprint_code))
    } else {
        None
    };

    projects.create(&args.name)?;

    if let Some(blueprint) = blueprint_obj {
        let project_dir = projects_dir.join(&args.name);
        let file_name = Blueprints::path(&blueprint.0)?
            .file_name()
            .expect("blueprint path always ends with a file name")
            .to_string_lossy()
            .to_string();
        let engine = BlueprintEngine::init(project_dir, file_name, args.name.clone(), args.quiet)
            .map_err(|e| anyhow!(e.to_string()))?;
        if !args.quiet {
            println!(
                "Running blueprint engine for '{}' blueprint...",
                blueprint.0
            );
        }
        if let mlua::Result::Err(e) = engine.run(&blueprint.1) {
            print_error!("An error occurred in Lua engine: {}", e);
            projects.delete(&args.name)?;
            bail!("Failed to generate project from blueprint. See Lua error above.")
        }

        if !args.quiet {
            print_done!(
                "Generated '{}' from blueprint '{}'.",
                args.name,
                blueprint.0
            );
        }
        return Ok(());
    }

    if !args.quiet {
        print_done!("Created an empty project with name '{}'.", args.name);
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

    print_done!("Repository has been cloned.");
    Ok(())
}

pub fn handle_open(args: OpenArgs) -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let name = resolve_project_name(&args.name, &config, &projects, args.skip_autocomplete)
        .ok_or_else(|| anyhow!("Project not found."))?;

    let Some(path) = projects.get(&name) else {
        return Err(anyhow!("Project not found."));
    };

    if args.path {
        println!("{}", path.to_string_lossy());
        return Ok(());
    }

    if !is_terminal() {
        return Err(anyhow!(
            "Opening projects in non-interactive mode is not supported."
        ));
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
        print_done!("Editor launched.");
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
        print_title!("Your projects");
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

    projects.rename(&args.old_name, &args.new_name)?;
    print_done!(
        "Project '{}' has been renamed to '{}'.",
        args.old_name,
        args.new_name
    );
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

    if !is_terminal() && !args.yes {
        return Err(anyhow!(
            "Confirmation with `--yes` is required for non-interactive sessions."
        ));
    }

    if !args.yes
        && !ask_dialog(
            &format!("Do you want to delete '{}'?", project_name),
            false,
            false,
        )?
    {
        print_done!("Canceled.");
        return Ok(());
    }

    let spinner = generate_progress().with_message("Removing project...");

    spinner.enable_steady_tick(Duration::from_millis(100));
    let result = projects.delete(&project_name);
    spinner.finish_and_clear();
    result?;

    print_done!("Project '{project_name}' has been removed.");
    Ok(())
}

pub fn handle_backup(args: BackupArgs) -> Result<()> {
    // Load config
    let config = Config::load(platform::config_file())?;

    // Load blueprints
    let blueprints = Blueprints::load_from_path(&platform::blueprints_dir())?;
    let mut blueprints_map: IndexMap<String, String> = IndexMap::new();

    for name in blueprints.get_blueprints() {
        blueprints_map.insert(format!("{name}.lua"), Blueprints::read(name)?);
    }

    let backup_file_path = args.output_file.unwrap_or("kanri_backup.json".to_string());

    let backup = Backup {
        config: toml::Value::try_from(config)?,
        blueprints: blueprints_map,
    };
    save_backup(&backup_file_path, backup)?;
    print_done!("Backup saved to `{backup_file_path}`.");
    Ok(())
}

pub fn handle_import(args: ImportArgs) -> Result<()> {
    if !args.yes
        && !ask_dialog(
            "This action will overwrite your current configuration and will overwrite existing blueprints. Continue?",
            false,
            true,
        )?
    {
        print_done!("Aborting.");
        return Ok(());
    }

    let backup = load_backup(args.file)?;

    // Validate everything before writing, so a broken backup leaves the current setup untouched.
    let (config, _) = Config::from_value(backup.config)?;
    let mut blueprints = Vec::with_capacity(backup.blueprints.len());
    for (name, content) in &backup.blueprints {
        let path = Blueprints::path(name)
            .with_context(|| format!("Invalid blueprint name '{name}' in backup"))?;
        blueprints.push((name, path, content));
    }

    let mut failed_blueprints: Vec<&str> = Vec::new();
    for (name, path, content) in blueprints {
        if let Err(e) = fs::write(&path, content) {
            print_error!("Failed to write blueprint '{}': {}", name, e);
            failed_blueprints.push(name);
        }
    }

    // The config goes last so it's only replaced when all blueprints were imported.
    ensure!(
        failed_blueprints.is_empty(),
        "Failed to import {} blueprint(s): {}. Configuration was not changed.",
        failed_blueprints.len(),
        failed_blueprints.join(", ")
    );
    config.save(platform::config_file())?;

    print_done!("Backup has been imported.");
    Ok(())
}

const KANRI_ZEN: [&str; 10] = [
    "Projects should be simple.",
    "Each command does one thing well.",
    "Configuration is explicit.",
    "Sensible defaults guide the way.",
    "The shell is a friend.",
    "Blueprints accelerate your workflow.",
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
