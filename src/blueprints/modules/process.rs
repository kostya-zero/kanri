use std::{io::ErrorKind, process::Command};

use mlua::prelude::*;
use which::which;

use crate::blueprints::engine::ProjectDir;
use crate::print_action_run;

#[derive(Clone, Default)]
struct RunOptions {
    pub capture: bool,
}

impl FromLua for RunOptions {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let LuaValue::Table(table) = value else {
            return Err(mlua::Error::runtime("options must be a table"));
        };
        Ok(Self {
            capture: table.get::<Option<bool>>("capture")?.unwrap_or(false),
        })
    }
}

#[derive(Default)]
struct RunResult {
    code: Option<i32>,
    success: bool,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl LuaUserData for RunResult {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("code", |_, this| Ok(this.code));
        fields.add_field_method_get("success", |_, this| Ok(this.success));
        fields.add_field_method_get("stdout", |_, this| Ok(this.stdout.clone()));
        fields.add_field_method_get("stderr", |_, this| Ok(this.stderr.clone()));
    }
}

fn command_error(error: std::io::Error) -> mlua::Error {
    let message = match error.kind() {
        ErrorKind::NotFound => "program not found",
        ErrorKind::Interrupted => "program was interrupted",
        ErrorKind::PermissionDenied => "not enough permissions to execute program",
        error => &format!("unexpected error occurred: {error}"),
    };
    mlua::Error::runtime(message)
}

fn print_command(cmd: &String, args: &[String]) {
    let mut command: String = String::from(cmd);
    for i in args {
        command.push_str(&format!(" {}", i));
    }

    print_action_run!("{command}");
}

pub fn create_process_module(lua: &Lua) -> LuaResult<LuaTable> {
    let process_table = lua.create_table_from([
        (
            "which",
            lua.create_function(|_, executable: String| match which(executable) {
                Ok(p) => Ok(Some(p.to_string_lossy().to_string())),
                Err(_) => Ok(None),
            })?,
        ),
        (
            "run",
            lua.create_function(
                move |lua,
                      (program, arguments, options): (
                    String,
                    Option<Vec<String>>,
                    Option<RunOptions>,
                )| {
                    if program.is_empty() {
                        return Err(mlua::Error::runtime("program cannot be empty"));
                    }
                    let mut command = Command::new(&program);
                    let capture = options.is_some_and(|o| o.capture);
                    let args: Vec<String> = arguments.unwrap_or_default();
                    command.args(&args).current_dir(&ProjectDir::get(lua)?.0);
                    print_command(&program, &args);

                    if capture {
                        let output = command.output().map_err(command_error)?;
                        let result = RunResult {
                            code: output.status.code(),
                            success: output.status.success(),
                            stdout: Some(String::from_utf8_lossy(&output.stdout).into_owned()),
                            stderr: Some(String::from_utf8_lossy(&output.stderr).into_owned()),
                        };
                        Ok(result)
                    } else {
                        let output = command.status().map_err(command_error)?;
                        let result = RunResult {
                            code: output.code(),
                            success: output.success(),
                            ..Default::default()
                        };
                        Ok(result)
                    }
                },
            )?,
        ),
    ])?;

    Ok(process_table)
}
