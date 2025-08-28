use anyhow::{Result, anyhow, bail};

use crate::{
    cli::RecentArgs,
    config::Config,
    platform,
    program::{LaunchOptions, launch_program},
    terminal::{ask_dialog, print_done},
};

pub fn handle_path() -> Result<()> {
    println!("{}", platform::config_file().display());
    Ok(())
}

pub fn handle_edit() -> Result<()> {
    let path = platform::config_file();
    let config = Config::load(&path)?;
    let editor = &config.editor.program;
    if editor.is_empty() {
        bail!("Editor program name is not set in the configuration file.");
    }

    let mut editor_args = config.editor.args.clone();
    editor_args.push(path.to_str().unwrap().to_string());

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

pub fn handle_recent(args: RecentArgs) -> Result<()> {
    let path = platform::config_file();
    let mut config = Config::load(&path)?;

    if !config.recent.enabled {
        bail!("Recent feature is disabled in the configuration file.");
    }

    if args.clear {
        if config.recent.recent_project.is_empty() {
            bail!("Nothing to clear.");
        }
        config.recent.recent_project.clear();
        config.save(path)?;
        print_done("Cleared.");
        return Ok(());
    }

    if config.recent.recent_project.is_empty() {
        bail!("No recent project found.");
    }

    println!("{}", config.recent.recent_project);

    Ok(())
}

pub fn handle_reset() -> Result<()> {
    let path = platform::config_file();
    let mut config = Config::load(&path)?;
    if ask_dialog("Reset your current configuration?", false) {
        config.reset();
        config.save(path)?;
        print_done("Reset.");
    } else {
        print_done("Aborted.");
    }
    Ok(())
}
