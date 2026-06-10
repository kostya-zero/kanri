use std::{fs, path::PathBuf};

use mlua::prelude::*;

pub fn create_fs_module(lua: &Lua, current_dir: impl Into<PathBuf>) -> LuaResult<LuaTable> {
    let fs_table = lua.create_table()?;
    let current_dir = current_dir.into();

    let write_dir = current_dir.clone();
    let fs_write = lua.create_function(move |_, (path, content): (String, String)| {
        fs::write(write_dir.join(path), content)
            .map_err(|error| mlua::Error::runtime(format!("failed to write a file: {error}")))
    })?;
    fs_table.set("write", fs_write)?;

    let fs_read = lua.create_function(move |_, path: String| {
        fs::read_to_string(current_dir.join(path))
            .map_err(|error| mlua::Error::runtime(format!("failed to read a file: {error}")))
    })?;
    fs_table.set("read", fs_read)?;

    Ok(fs_table)
}
