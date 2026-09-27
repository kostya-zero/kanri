use std::path::PathBuf;

use mlua::prelude::*;

pub fn create_project_module(
    lua: &Lua,
    current_dir: impl Into<PathBuf>,
    project_name: impl Into<String>,
) -> LuaResult<LuaTable> {
    let current_dir = current_dir.into();
    let project_name = project_name.into();

    let project_table = lua.create_table_from([
        (
            "path",
            lua.create_function(move |_, ()| Ok(current_dir.clone()))?,
        ),
        (
            "name",
            lua.create_function(move |_, ()| Ok(project_name.clone()))?,
        ),
    ])?;

    Ok(project_table)
}
