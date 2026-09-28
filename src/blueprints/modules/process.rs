use std::{
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Stdio},
};

use mlua::prelude::*;
use which::which;

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

pub fn create_process_module(
    lua: &Lua,
    current_dir: impl Into<PathBuf>,
    quiet: bool,
) -> LuaResult<LuaTable> {
    let current_dir = current_dir.into();
    let process_table = lua.create_table_from([
        (
            "which",
            lua.create_function(|_, executable: String| match which(executable) {
                Ok(p) => Ok(p.to_str().unwrap().to_string()),
                Err(_) => todo!(),
            })?,
        ),
        (
            "run",
            lua.create_function(move |_, (program, arguments): (String, Vec<String>)| {
                if program.is_empty() {
                    return Err(mlua::Error::runtime("program cannot be empty"));
                }
                let mut command = Command::new(&program);
                if quiet {
                    command
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null());
                }

                command.args(&arguments).current_dir(&current_dir);
                if !quiet {
                    print_command(&program, &arguments);
                }

                match command.status() {
                    Ok(status) => Ok(status.code()),
                    Err(e) => Err(command_error(e)),
                }
            })?,
        ),
    ])?;

    Ok(process_table)
}
