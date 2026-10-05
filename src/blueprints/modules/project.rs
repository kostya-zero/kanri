use mlua::prelude::*;

use crate::blueprints::engine::ProjectDir;

pub fn create_project_module(lua: &Lua, project_name: impl Into<String>) -> LuaResult<LuaTable> {
    let project_name = project_name.into();

    let project_table = lua.create_table_from([
        (
            "path",
            lua.create_string(ProjectDir::get(lua)?.0.clone().to_str().unwrap())?,
        ),
        ("name", lua.create_string(project_name.clone())?),
    ])?;

    Ok(project_table)
}
