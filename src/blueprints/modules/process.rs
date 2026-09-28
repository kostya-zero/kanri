use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use mlua::prelude::*;
use which::which;

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

                match command.status() {
                    Ok(status) => Ok(status.code()),
                    Err(e) => Err(mlua::Error::runtime(format!("failed to run program: {e}"))),
                }
            })?,
        ),
    ])?;

    Ok(process_table)
}
