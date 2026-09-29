use mlua::prelude::*;
use std::env;

pub fn create_system_module(lua: &Lua) -> LuaResult<LuaTable> {
    let system_table = lua.create_table_from([
        ("system", lua.create_function(|_, ()| Ok(env::consts::OS))?),
        ("arch", lua.create_function(|_, ()| Ok(env::consts::ARCH))?),
        (
            "exe_suffix",
            lua.create_function(|_, ()| Ok(env::consts::EXE_SUFFIX))?,
        ),
        (
            "temp_dir",
            lua.create_function(|_, ()| Ok(env::temp_dir().to_string_lossy().to_string()))?,
        ),
    ])?;

    Ok(system_table)
}
