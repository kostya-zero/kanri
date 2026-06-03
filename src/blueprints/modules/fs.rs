use std::{fs, path::PathBuf};

use mlua::prelude::*;

pub fn create_fs_module<I: Into<PathBuf>>(lua: &Lua, current_dir: I) -> LuaTable {
    let fs_table = lua.create_table().unwrap();
    let current_dir = current_dir.into();

    {
        let current_dir = current_dir.clone();

        let fs_write = lua
            .create_function(move |_, (path, content): (String, String)| {
                let final_path = current_dir.join(path);
                let result = fs::write(final_path, content);
                match result {
                    Ok(()) => Ok(()),
                    Err(_) => Err(mlua::Error::runtime("failed to write a file")),
                }
            })
            .unwrap();

        fs_table.set("write", fs_write).unwrap();
    }

    {
        let current_dir = current_dir.clone();

        let fs_read = lua
            .create_function(move |_, path: String| {
                let final_path = current_dir.join(path);
                let result = fs::read_to_string(final_path);
                match result {
                    Ok(s) => Ok(s),
                    Err(_) => Err(mlua::Error::runtime("failed to read a file")),
                }
            })
            .unwrap();

        fs_table.set("read", fs_read).unwrap();
    }

    fs_table
}
