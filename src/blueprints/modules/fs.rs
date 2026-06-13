use std::{fs, path::PathBuf};

use mlua::prelude::*;

use crate::terminal::{print_action_add, print_action_remove, print_action_run};

fn fs_error(action: &str, error: std::io::Error) -> mlua::Error {
    mlua::Error::runtime(format!("failed to {action}: {error}"))
}

pub fn create_fs_module(lua: &Lua, current_dir: impl Into<PathBuf>) -> LuaResult<LuaTable> {
    let fs_table = lua.create_table()?;
    let current_dir = current_dir.into();

    let write_dir = current_dir.clone();
    let fs_write = lua.create_function(move |_, (path, content): (String, String)| {
        fs::write(write_dir.join(&path), content)
            .map_err(|error| fs_error("write a file", error))?;
        print_action_add(&format!("Created a file: {}", path));
        Ok(())
    })?;
    fs_table.set("write", fs_write)?;

    let read_dir = current_dir.clone();
    let fs_read = lua.create_function(move |_, path: String| {
        fs::read_to_string(read_dir.join(path)).map_err(|error| fs_error("read a file", error))
    })?;
    fs_table.set("read", fs_read)?;

    let remove_file_dir = current_dir.clone();
    let fs_remove_file = lua.create_function(move |_, path: String| {
        fs::remove_file(remove_file_dir.join(&path))
            .map_err(|error| fs_error("remove a file", error))?;
        print_action_remove(&format!("Removed a file: {}", path));
        Ok(())
    })?;
    fs_table.set("remove_file", fs_remove_file)?;

    let remove_dir_dir = current_dir.clone();
    let fs_remove_dir = lua.create_function(move |_, path: String| {
        fs::remove_dir_all(remove_dir_dir.join(&path))
            .map_err(|error| fs_error("remove a directory", error))?;
        print_action_remove(&format!("Removed a directory: {}", path));
        Ok(())
    })?;
    fs_table.set("remove_dir", fs_remove_dir)?;

    let move_dir = current_dir.clone();
    let fs_move = lua.create_function(move |_, (from, to): (String, String)| {
        fs::rename(move_dir.join(&from), move_dir.join(&to))
            .map_err(|error| fs_error("move a path", error))?;
        print_action_run(&format!("Moved item from '{}' to '{}'", from, to));
        Ok(())
    })?;
    fs_table.set("move", fs_move)?;

    let exists_dir = current_dir.clone();
    let fs_exists =
        lua.create_function(move |_, path: String| Ok(exists_dir.join(path).exists()))?;
    fs_table.set("exists", fs_exists)?;

    let is_file_dir = current_dir.clone();
    let fs_is_file =
        lua.create_function(move |_, path: String| Ok(is_file_dir.join(path).is_file()))?;
    fs_table.set("is_file", fs_is_file)?;

    let is_dir_dir = current_dir.clone();
    let fs_is_dir =
        lua.create_function(move |_, path: String| Ok(is_dir_dir.join(path).is_dir()))?;
    fs_table.set("is_dir", fs_is_dir)?;

    let create_dir_dir = current_dir.clone();
    let fs_create_dir = lua.create_function(move |_, path: String| {
        fs::create_dir_all(create_dir_dir.join(&path))
            .map_err(|error| fs_error("create a directory", error))?;
        print_action_add(&format!("Created a directory: {}", path));
        Ok(())
    })?;
    fs_table.set("create_dir", fs_create_dir)?;

    Ok(fs_table)
}
