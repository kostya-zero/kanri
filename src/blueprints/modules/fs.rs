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
    Ok(ProjectDir::get(lua)?.0.join(path))
}

pub fn create_fs_module(lua: &Lua, quiet: bool) -> LuaResult<LuaTable> {
    let fs_table = lua.create_table_from([
        (
            "write",
            lua.create_function(move |lua, (path, content): (String, String)| {
                let path = resolve(lua, &path)?;
                let overwrite = path.exists();
                fs::write(&path, content).map_err(|error| fs_error("write a file", error))?;
                if !quiet && !overwrite {
                    print_action_add(&format!("Created a file: {}", path.to_string_lossy()));
                } else if !quiet && overwrite {
                    print_action_add(&format!("Overwritten a file: {}", path.to_string_lossy()));
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
            lua.create_function(move |lua, (path, content): (String, String)| {
                let mut file = File::options()
                    .append(true)
                    .create(true)
                    .open(resolve(lua, &path)?)
                    .map_err(|error| fs_error("open a file", error))?;
                write!(file, "{content}").map_err(|error| fs_error("append to a file", error))?;
                if !quiet {
                    print_action_add(&format!("Appended to a file: {}", path));
                }
                Ok(())
            })?,
        ),
    ])?;

    Ok(fs_table)
}
