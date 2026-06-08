use anyhow::{Result, anyhow, bail, ensure};
use colored::Colorize;
use std::time::Duration;

use crate::{
    autocomplete,
    backup::{Backup, load_backup, save_backup},
    blueprints::{engine::BlueprintEngine, storage::Blueprints},
    cli::{BackupArgs, CloneArgs, ImportArgs, ListArgs, NewArgs, OpenArgs, RemoveArgs, RenameArgs},
    config::Config,
    library::{CloneOptions, Library, validate_project_name},
    platform,
    program::{LaunchOptions, launch_program},
    templates::Templates,
    terminal::{ask_dialog, generate_progress, is_terminal, print_done, print_title},
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
    if args.template.is_some() {
        bail!(
            "Templates are no longer supported and has been replaced with blueprints. If you want to migrate your templates, use `kanri templates migrate`."
        );
    }

    let config = Config::load(platform::config_file())?;
    let projects_dir = &config.options.projects_directory;
    let mut projects = Library::new(projects_dir, config.options.display_hidden)?;

    validate_project_name(&args.name)?;
    projects.create(&args.name)?;

    if let Some(blueprint) = args.blueprint {
        let blueprints_dir = platform::blueprints_dir();
        let blueprints = Blueprints::load_from_path(&blueprints_dir)?;
        let blueprint_code = blueprints.get_blueprint(blueprint.clone())?;

        let project_dir = projects_dir.join(&args.name);
        let engine = BlueprintEngine::init(project_dir, format!("{}.lua", blueprint));

        println!("Running blueprint engine for '{}' blueprint...", &blueprint);
        if let mlua::Result::Err(e) = engine.run(&blueprint_code) {
            bail!("an error occurred in Lua engine: {}", e);
        }

        print_done(
            format!(
                "Generated '{}' from blueprint '{}'.",
                &args.name, &blueprint,
            )
            .as_str(),
        );
        return Ok(());
    }

    print_done(&format!(
        "Created an empty project with name '{}'.",
        args.name
    ));

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
            )?
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
