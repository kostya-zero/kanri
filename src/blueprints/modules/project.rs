use std::path::PathBuf;

use mlua::prelude::*;

pub fn create_project_module<I: Into<PathBuf>>(
    lua: &Lua,
    current_dir: I,
    project_name: String,
) -> LuaTable {
    let project_table = lua.create_table().unwrap();
    let current_dir = current_dir.into();

    {
        let current_dir = current_dir.clone();
        let project_path = lua
            .create_function(move |_, ()| Ok(current_dir.clone()))
            .unwrap();
        project_table.set("path", project_path).unwrap();
    }

    {
        let name = project_name.clone();
        let project_name = lua.create_function(move |_, ()| Ok(name.clone())).unwrap();
        project_table.set("name", project_name).unwrap();
    }

    project_table
}
