use std::path::{Path, PathBuf};

use mlua::Variadic;
use mlua::prelude::*;

pub fn create_path_module(lua: &Lua) -> LuaResult<LuaTable> {
    let path_table = lua.create_table_from([
        (
            "join",
            lua.create_function(|_, parts: Variadic<String>| {
                let mut path = PathBuf::new();
                for part in parts {
                    if !part.is_empty() {
                        path.push(part);
                    }
                }
                Ok(if path.as_os_str().is_empty() {
                    ".".to_string()
                } else {
                    path.to_string_lossy().to_string()
                })
            })?,
        ),
        (
            "parent",
            lua.create_function(|_, path: String| {
                Ok(Path::new(&path).parent().map(|parent| {
                    if parent.as_os_str().is_empty() {
                        ".".to_string()
                    } else {
                        parent.to_string_lossy().to_string()
                    }
                }))
            })?,
        ),
        (
            "basename",
            lua.create_function(|_, path: String| {
                Ok(Path::new(&path)
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string()))
            })?,
        ),
        (
            "extension",
            lua.create_function(|_, path: String| {
                Ok(Path::new(&path)
                    .extension()
                    .map(|extension| extension.to_string_lossy().to_string()))
            })?,
        ),
        (
            "stem",
            lua.create_function(|_, path: String| {
                Ok(Path::new(&path)
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().to_string()))
            })?,
        ),
        (
            "is_absolute",
            lua.create_function(|_, path: String| Ok(Path::new(&path).is_absolute()))?,
        ),
    ])?;

    Ok(path_table)
}
