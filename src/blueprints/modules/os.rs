use mlua::prelude::*;
use std::{env, io::ErrorKind, path::PathBuf, process::Command};

fn command_error(error: std::io::Error) -> mlua::Error {
    let message = match error.kind() {
        ErrorKind::NotFound => "program not found",
        ErrorKind::Interrupted => "program was interrupted",
        ErrorKind::PermissionDenied => "not enough permissions to execute program",
        _ => "unknown error occurred",
    };
    mlua::Error::runtime(message)
}

pub fn create_os_module(lua: &Lua, current_dir: impl Into<PathBuf>) -> LuaResult<LuaTable> {
    let os_table = lua.create_table()?;
    let current_dir = current_dir.into();

    os_table.set("system", lua.create_function(|_, ()| Ok(env::consts::OS))?)?;
    os_table.set("arch", lua.create_function(|_, ()| Ok(env::consts::ARCH))?)?;

    let os_exec = lua.create_function(move |_, (cmd, args): (String, Vec<String>)| {
        if cmd.is_empty() {
            return Err(mlua::Error::runtime("program cannot be empty"));
        }

        let mut command = Command::new(cmd);
        command.args(args).current_dir(&current_dir);
        command.status().map(|_| ()).map_err(command_error)
    })?;
    os_table.set("exec", os_exec)?;

    Ok(os_table)
}
