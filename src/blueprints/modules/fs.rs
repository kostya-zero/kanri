use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use mlua::prelude::*;

use crate::blueprints::engine::ProjectDir;
use crate::terminal::{print_action_add, print_action_remove, print_action_run};

fn fs_error(action: &str, error: std::io::Error) -> mlua::Error {
    mlua::Error::runtime(format!("failed to {action}: {error}"))
}

/// Resolves `path` relative to the project directory stored in Lua app data.
fn resolve(lua: &Lua, path: &str) -> LuaResult<PathBuf> {
    let project_dir = lua
        .app_data_ref::<ProjectDir>()
        .ok_or_else(|| mlua::Error::runtime("project directory is not set"))?;
    Ok(project_dir.0.join(path))
}

pub fn create_fs_module(lua: &Lua, quiet: bool) -> LuaResult<LuaTable> {
    let fs_table = lua.create_table_from([
        (
            "write",
            lua.create_function(move |lua, (path, content): (String, String)| {
                fs::write(resolve(lua, &path)?, content)
                    .map_err(|error| fs_error("write a file", error))?;
                if !quiet {
                    print_action_add(&format!("Created a file: {}", path));
                }
                Ok(())
            })?,
        ),
        (
            "read",
            lua.create_function(|lua, path: String| {
                fs::read_to_string(resolve(lua, &path)?)
                    .map_err(|error| fs_error("read a file", error))
            })?,
        ),
        (
            "remove_file",
            lua.create_function(move |lua, path: String| {
                fs::remove_file(resolve(lua, &path)?)
                    .map_err(|error| fs_error("remove a file", error))?;
                if !quiet {
                    print_action_remove(&format!("Removed a file: {}", path));
                }
                Ok(())
            })?,
        ),
        (
            "remove_dir",
            lua.create_function(move |lua, path: String| {
                fs::remove_dir_all(resolve(lua, &path)?)
                    .map_err(|error| fs_error("remove a directory", error))?;
                if !quiet {
                    print_action_remove(&format!("Removed a directory: {}", path));
                }
                Ok(())
            })?,
        ),
        (
            "move",
            lua.create_function(move |lua, (from, to): (String, String)| {
                fs::rename(resolve(lua, &from)?, resolve(lua, &to)?)
                    .map_err(|error| fs_error("move a path", error))?;
                if !quiet {
                    print_action_run(&format!("Moved item from '{}' to '{}'", from, to));
                }
                Ok(())
            })?,
        ),
        (
            "exists",
            lua.create_function(|lua, path: String| Ok(resolve(lua, &path)?.exists()))?,
        ),
        (
            "is_file",
            lua.create_function(|lua, path: String| Ok(resolve(lua, &path)?.is_file()))?,
        ),
        (
            "is_dir",
            lua.create_function(|lua, path: String| Ok(resolve(lua, &path)?.is_dir()))?,
        ),
        (
            "create_dir",
            lua.create_function(move |lua, path: String| {
                fs::create_dir_all(resolve(lua, &path)?)
                    .map_err(|error| fs_error("create a directory", error))?;
                if !quiet {
                    print_action_add(&format!("Created a directory: {}", path));
                }
                Ok(())
            })?,
        ),
        (
            "append",
            lua.create_function(|_, (path, content): (String, String)| {
                let mut file = File::options()
                    .append(true)
                    .create(true)
                    .open(path)
                    .map_err(|e| mlua::Error::runtime(format!("failed to open file: {e}")))?;

                writeln!(&mut file, "{content}").map_err(|e| {
                    mlua::Error::runtime(format!("failed to write content to file: {e}"))
                })?;

                Ok(())
            })?,
        ),
    ])?;

    Ok(fs_table)
}
