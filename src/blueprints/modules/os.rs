use mlua::prelude::*;
use std::{
    env,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::terminal::print_action_run;

fn command_error(error: std::io::Error) -> mlua::Error {
    let message = match error.kind() {
        ErrorKind::NotFound => "program not found",
        ErrorKind::Interrupted => "program was interrupted",
        ErrorKind::PermissionDenied => "not enough permissions to execute program",
        _ => "unknown error occurred",
    };
    mlua::Error::runtime(message)
}

fn print_command(cmd: &String, args: &[String]) {
    let mut command: String = String::from(cmd);
    for i in args {
        command.push_str(&format!(" {}", i));
    }

    print_action_run(&command);
}

pub fn create_os_module(
    lua: &Lua,
    current_dir: impl Into<PathBuf>,
    quiet: bool,
) -> LuaResult<LuaTable> {
    let current_dir = current_dir.into();
    let cwd = current_dir.clone();
    let exec_dir = current_dir.clone();
    let status_dir = current_dir.clone();
    let output_dir = current_dir.clone();

    let os_table = lua.create_table_from([
        ("system", lua.create_function(|_, ()| Ok(env::consts::OS))?),
        ("arch", lua.create_function(|_, ()| Ok(env::consts::ARCH))?),
        (
            "family",
            lua.create_function(|_, ()| Ok(env::consts::FAMILY))?,
        ),
        (
            "exe_suffix",
            lua.create_function(|_, ()| Ok(env::consts::EXE_SUFFIX))?,
        ),
        (
            "dir_separator",
            lua.create_function(|_, ()| Ok(std::path::MAIN_SEPARATOR.to_string()))?,
        ),
        (
            "path_separator",
            lua.create_function(|_, ()| Ok(if cfg!(windows) { ";" } else { ":" }))?,
        ),
        (
            "temp_dir",
            lua.create_function(|_, ()| Ok(env::temp_dir().to_string_lossy().to_string()))?,
        ),
        (
            "env",
            lua.create_function(|_, name: String| Ok(env::var(name).ok()))?,
        ),
        (
            "current_dir",
            lua.create_function(move |_, ()| Ok(cwd.to_string_lossy().to_string()))?,
        ),
        (
            "exec",
            lua.create_function(move |_, (cmd, args): (String, Vec<String>)| {
                if cmd.is_empty() {
                    return Err(mlua::Error::runtime("program cannot be empty"));
                }

                let mut command = Command::new(&cmd);

                if quiet {
                    command
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null());
                }
                command.args(&args).current_dir(&exec_dir);
                if !quiet {
                    print_command(&cmd, &args);
                }
                command.status().map(|_| ()).map_err(command_error)
            })?,
        ),
        (
            "exec_status",
            lua.create_function(move |_, (cmd, args): (String, Vec<String>)| {
                if cmd.is_empty() {
                    return Err(mlua::Error::runtime("program cannot be empty"));
                }

                let mut command = Command::new(&cmd);
                if quiet {
                    command
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null());
                }
                command.args(&args).current_dir(&status_dir);
                if !quiet {
                    print_command(&cmd, &args);
                }
                command
                    .status()
                    .map(|status| status.code().unwrap_or(-1))
                    .map_err(command_error)
            })?,
        ),
        (
            "exec_output",
            lua.create_function(move |_, (cmd, args): (String, Vec<String>)| {
                if cmd.is_empty() {
                    return Err(mlua::Error::runtime("program cannot be empty"));
                }

                let mut command = Command::new(&cmd);
                if quiet {
                    command
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null());
                }
                command.args(&args).current_dir(&output_dir);
                if !quiet {
                    print_command(&cmd, &args);
                }
                command
                    .output()
                    .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
                    .map_err(command_error)
            })?,
        ),
    ])?;

    Ok(os_table)
}
