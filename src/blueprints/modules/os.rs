use mlua::prelude::*;
use std::env;

use crate::blueprints::engine::ProjectDir;

pub fn create_os_module(lua: &Lua) -> LuaResult<LuaTable> {
    let os_table = lua.create_table_from([
        ("system", lua.create_function(|_, ()| Ok(env::consts::OS))?),
        ("arch", lua.create_function(|_, ()| Ok(env::consts::ARCH))?),
        (
            "family",
            lua.create_function(|_, ()| Ok(env::consts::FAMILY))?,
        ),
        (
            "exe_suffix",
            lua.create_function(|_, ()| Ok(env::consts::EXE_SUFFIX))?,
        ),
        (
            "dir_separator",
            lua.create_function(|_, ()| Ok(std::path::MAIN_SEPARATOR.to_string()))?,
        ),
        (
            "path_separator",
            lua.create_function(|_, ()| Ok(if cfg!(windows) { ";" } else { ":" }))?,
        ),
        (
            "temp_dir",
            lua.create_function(|_, ()| Ok(env::temp_dir().to_string_lossy().to_string()))?,
        ),
        (
            "env",
            lua.create_function(|_, name: String| Ok(env::var(name).ok()))?,
        ),
        (
            "current_dir",
            lua.create_function(|lua, ()| {
                Ok(ProjectDir::get(lua)?.0.to_string_lossy().to_string())
            })?,
        ),
    ])?;

    Ok(os_table)
}
