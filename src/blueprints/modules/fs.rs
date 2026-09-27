use std::{fs, path::PathBuf};

use mlua::prelude::*;

use crate::terminal::{print_action_add, print_action_remove, print_action_run};

fn fs_error(action: &str, error: std::io::Error) -> mlua::Error {
    mlua::Error::runtime(format!("failed to {action}: {error}"))
}

pub fn create_fs_module(
    lua: &Lua,
    current_dir: impl Into<PathBuf>,
    quiet: bool,
) -> LuaResult<LuaTable> {
    let current_dir = current_dir.into();
    let working_dir = current_dir.clone();
    let read_dir = current_dir.clone();
    let remove_file_dir = current_dir.clone();
    let remove_dir_dir = current_dir.clone();
    let move_dir = current_dir.clone();
    let exists_dir = current_dir.clone();
    let is_file_dir = current_dir.clone();
    let is_dir_dir = current_dir.clone();
    let create_dir_dir = current_dir.clone();

    let fs_table = lua.create_table_from([
        (
            "write",
            lua.create_function(move |_, (path, content): (String, String)| {
                fs::write(working_dir.join(&path), content)
                    .map_err(|error| fs_error("write a file", error))?;
                if !quiet {
                    print_action_add(&format!("Created a file: {}", path));
                }
                Ok(())
            })?,
        ),
        (
            "read",
            lua.create_function(move |_, path: String| {
                fs::read_to_string(read_dir.join(path))
                    .map_err(|error| fs_error("read a file", error))
            })?,
        ),
        (
            "remove_file",
            lua.create_function(move |_, path: String| {
                fs::remove_file(remove_file_dir.join(&path))
                    .map_err(|error| fs_error("remove a file", error))?;
                if !quiet {
                    print_action_remove(&format!("Removed a file: {}", path));
                }
                Ok(())
            })?,
        ),
        (
            "remove_dir",
            lua.create_function(move |_, path: String| {
                fs::remove_dir_all(remove_dir_dir.join(&path))
                    .map_err(|error| fs_error("remove a directory", error))?;
                if !quiet {
                    print_action_remove(&format!("Removed a directory: {}", path));
                }
                Ok(())
            })?,
        ),
        (
            "move",
            lua.create_function(move |_, (from, to): (String, String)| {
                fs::rename(move_dir.join(&from), move_dir.join(&to))
                    .map_err(|error| fs_error("move a path", error))?;
                if !quiet {
                    print_action_run(&format!("Moved item from '{}' to '{}'", from, to));
                }
                Ok(())
            })?,
        ),
        (
            "exists",
            lua.create_function(move |_, path: String| Ok(exists_dir.join(path).exists()))?,
        ),
        (
            "is_file",
            lua.create_function(move |_, path: String| Ok(is_file_dir.join(path).is_file()))?,
        ),
        (
            "is_dir",
            lua.create_function(move |_, path: String| Ok(is_dir_dir.join(path).is_dir()))?,
        ),
        (
            "create_dir",
            lua.create_function(move |_, path: String| {
                fs::create_dir_all(create_dir_dir.join(&path))
                    .map_err(|error| fs_error("create a directory", error))?;
                if !quiet {
                    print_action_add(&format!("Created a directory: {}", path));
                }
                Ok(())
            })?,
        ),
    ])?;

    Ok(fs_table)
}
