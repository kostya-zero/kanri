use mlua::prelude::*;

use crate::blueprints::engine::ProjectDir;

pub fn create_project_module(lua: &Lua, project_name: impl Into<String>) -> LuaResult<LuaTable> {
    let project_name = project_name.into();

    let project_table = lua.create_table_from([
        (
            "path",
            lua.create_function(|lua, ()| Ok(ProjectDir::get(lua)?.0.clone()))?,
        ),
        (
            "name",
            lua.create_function(move |_, ()| Ok(project_name.clone()))?,
        ),
    ])?;

    Ok(project_table)
}
