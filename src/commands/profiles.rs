use anyhow::{Result, bail};
use colored::Colorize;

use crate::{
    cli::ProfilesRemoveArgs,
    config::{Config, Profile, fork_editor_program},
    platform, print_done, print_title,
    terminal::{ask_dialog, ask_string_dialog},
};

pub fn handle_new() -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;

    let profile_name = ask_string_dialog("Name for new profile?", true)?;
    if profile_name.is_empty() {
        bail!("Profile name can't be empty.");
    }

    if config.is_profile_exist(&profile_name) {
        bail!("Profile with the same name already exists.")
    }

    let editor = ask_string_dialog("Which editor you want to assign (program name)?", true)?;

    if editor.is_empty() {
        bail!("Editor name is empty.")
    }

    let (editor, editor_args, editor_fork_mode) = match fork_editor_program(&editor) {
        Some(program) => (program, vec![".".to_string()], true),
        None => {
            let fork_mode = ask_dialog("Do you want to run your editor forked?", false, true)?;
            (editor, Vec::new(), fork_mode)
        }
    };

    let shell = ask_string_dialog("Which shell you want to assign (program name)?", true)?;

    if shell.is_empty() {
        bail!("Shell name is empty.")
    }

    let profile = Profile {
        editor,
        editor_fork_mode,
        editor_args,
        shell,
    };

    config.profiles.insert(profile_name, profile);
    config.save(config_path)?;

    print_done!("Your profile has been saved. You can edit it in your configuration file.");
    Ok(())
}

pub fn handle_set(name: &str) -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;

    if !config.is_profile_exist(name) {
        bail!("Profile {name} not found.")
    }

    config.options.current_profile = name.to_string();
    config.save(config_path)?;

    print_done!("Switched current profile to '{name}'.");

    Ok(())
}

pub fn handle_list() -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let profiles = config.profiles.keys();

    print_title!("Your profiles");
    for i in profiles {
        println!(
            "  {i} {}",
            if config.options.current_profile == *i {
                "(current)".dimmed()
            } else {
                "".into()
            }
        );
    }

    Ok(())
}

pub fn handle_get(name: &str) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let profile = config.get_profile(name)?;

    print_title!("Profile");
    // There should be a better way to display it.
    println!("  {}: {}", "Editor".bold(), profile.editor);
    println!("  {}: {}", "Shell".bold(), profile.shell);
    Ok(())
}

pub fn handle_remove(args: ProfilesRemoveArgs) -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;

    if !config.is_profile_exist(&args.name) {
        bail!("Profile {} not found.", args.name)
    }

    if config.options.current_profile == args.name {
        bail!("Select another profile first before you can delete current one.")
    }

    if !args.yes && !ask_dialog("Do you want to delete this profile?", false, false)? {
        print_done!("Aborted");
        return Ok(());
    }

    config.profiles.shift_remove(&args.name);
    config.save(&config_path)?;
    print_done!("Profile has been removed.");

    Ok(())
}
