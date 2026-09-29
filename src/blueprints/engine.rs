use std::path::PathBuf;

use mlua::prelude::*;
use mlua::{AppDataRef, LuaOptions, StdLib};

use crate::blueprints::modules::fs::create_fs_module;
use crate::blueprints::modules::path::create_path_module;
use crate::blueprints::modules::process::create_process_module;
use crate::blueprints::modules::project::create_project_module;
use crate::blueprints::modules::system::create_system_module;

/// Project directory that blueprint modules resolve relative paths against.
pub struct ProjectDir(pub PathBuf);

impl ProjectDir {
    /// Returns the project directory stored in Lua app data.
    pub fn get(lua: &Lua) -> LuaResult<AppDataRef<'_, Self>> {
        lua.app_data_ref::<Self>()
            .ok_or_else(|| mlua::Error::runtime("project directory is not set"))
    }
}

pub struct BlueprintEngine {
    lua: Lua,
    file_name: String,
}

impl BlueprintEngine {
    pub fn init(
        current_dir: impl Into<PathBuf>,
        file_name: impl Into<String>,
        project_name: impl Into<String>,
        quiet: bool,
    ) -> LuaResult<Self> {
        let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default())?;

        lua.set_app_data(ProjectDir(current_dir.into()));

        lua.globals().set("fs", create_fs_module(&lua, quiet)?)?;
        lua.globals().set("system", create_system_module(&lua)?)?;
        lua.globals().set("path", create_path_module(&lua)?)?;
        lua.globals()
            .set("project", create_project_module(&lua, project_name.into())?)?;
        lua.globals()
            .set("process", create_process_module(&lua, quiet)?)?;

        Ok(Self {
            lua,
            file_name: file_name.into(),
        })
    }

    pub fn run(&self, source: &str) -> LuaResult<()> {
        self.lua.load(source).set_name(&self.file_name).exec()
    }

    pub fn check(&self, source: &str) -> LuaResult<()> {
        self.lua
            .load(source)
            .set_name(&self.file_name)
            .into_function()
            .map(|_| ())
    }
}
