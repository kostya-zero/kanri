use std::path::PathBuf;

use mlua::prelude::*;

pub fn create_project_module(
    lua: &Lua,
    current_dir: impl Into<PathBuf>,
    project_name: impl Into<String>,
) -> LuaResult<LuaTable> {
    let project_table = lua.create_table()?;
    let current_dir = current_dir.into();
    let project_name = project_name.into();

    project_table.set(
        "path",
        lua.create_function(move |_, ()| Ok(current_dir.clone()))?,
    )?;
    project_table.set(
        "name",
        lua.create_function(move |_, ()| Ok(project_name.clone()))?,
    )?;

    Ok(project_table)
}
