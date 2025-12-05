use anyhow::{Result, bail};
use colored::Colorize;

use crate::{
    cli::{ProfilesInfoArgs, ProfilesRemoveArgs, ProfilesSetArgs},
    config::{Config, Profile},
    platform,
    terminal::{ask_dialog, ask_string_dialog, print_done, print_title},
};

pub fn handle_new() -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;

    let profile_name = ask_string_dialog("Name for new profile?", true);

    if config.is_profile_exist(&profile_name) {
        bail!("Profile with the same name already exists.")
    }

    let mut editor = ask_string_dialog("Which editor you want to assign (program name)?", true);

    if editor.is_empty() {
        bail!("Editor name is empty.")
    }

    let editor_fork_mode: bool;
    let mut editor_args: Vec<String> = Vec::with_capacity(1);

    match editor.as_str() {
        "code" | "code-insiders" | "codium" | "code-oss" | "cursor" | "windsurf" | "zed"
        | "code.cmd" | "code-insiders.cmd" | "codium.cmd" | "code-oss.cmd" | "windsurf.cmd"
        | "cursor.cmd" => {
            if editor != "zed" && !editor.ends_with(".cmd") {
                editor.push_str(".cmd");
            }

            editor_fork_mode = true;
            editor_args.push(".".to_string());
        }
        _ => {
            editor_fork_mode = ask_dialog("Do you want to run your editor forked?", false, true);
        }
    }

    let shell = ask_string_dialog("Which shell you want to assign (program name)?", true);

    if shell.is_empty() {
        bail!("Shell name is empty.")
    }

    let mut shell_args: Vec<String> = Vec::with_capacity(3);

    match shell.as_str() {
        "pwsh" | "pwsh.exe" | "powershell" | "powershell.exe" => {
            shell_args.push("-NoLogo".to_string());
            shell_args.push("-Command".to_string());
        }
        _ => {
            shell_args.push("-c".to_string());
        }
    }

    let profile = Profile {
        editor,
        editor_fork_mode,
        editor_args,
        shell,
        shell_args,
    };

    config.profiles.insert(profile_name, profile);
    config.save(config_path)?;

    print_done("Your profile has been saved. You can edit it in your configuration file.");
    Ok(())
}

pub fn handle_set(args: ProfilesSetArgs) -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;

    if !config.is_profile_exist(&args.name) {
        bail!("Profile {} not found.", args.name)
    }

    config.options.current_profile = args.name.clone();
    config.save(config_path)?;

    print_done(&format!("Switched current profile to {}", args.name));

    Ok(())
}

pub fn handle_list() -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let profiles = config.profiles.keys();

    print_title("Your profiles");
    for i in profiles {
        println!(
            " {i} {}",
            if config.options.current_profile == *i {
                "(current)".dimmed()
            } else {
                "".into()
            }
        );
    }

    Ok(())
}

pub fn handle_info(args: ProfilesInfoArgs) -> Result<()> {
    let config = Config::load(platform::config_file())?;
    let profile = config.get_profile(&args.name)?;

    print_title("Profile");
    // There should be a better way to display it.
    println!(" {}: {}", "Editor".bold(), profile.editor);
    println!(" {}: {}", "Shell".bold(), profile.shell);
    Ok(())
}

pub fn handle_remove(args: ProfilesRemoveArgs) -> Result<()> {
    let config_path = platform::config_file();
    let mut config = Config::load(&config_path)?;

    if !config.is_profile_exist(&args.name) {
        bail!("Profile {} not found.", args.name)
    }

    let confirmation = ask_dialog("Do you want to delete this profile?", false, false);

    if !confirmation {
        println!("Aborted");
        return Ok(());
    }

    config.profiles.shift_remove(&args.name).unwrap();
    print_done("Removed.");
    Ok(())
}
