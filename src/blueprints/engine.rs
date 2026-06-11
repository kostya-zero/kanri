use std::path::{Path, PathBuf};

use mlua::prelude::*;
use mlua::{LuaOptions, StdLib};

use crate::blueprints::modules::fs::create_fs_module;
use crate::blueprints::modules::os::create_os_module;
use crate::blueprints::modules::project::create_project_module;

pub struct BlueprintEngine {
    lua: Lua,
    file_name: String,
    current_dir: PathBuf,
}

impl BlueprintEngine {
    pub fn init(
        current_dir: impl Into<PathBuf>,
        file_name: impl Into<String>,
        project_name: impl Into<String>,
    ) -> LuaResult<Self> {
        let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default())?;

        let current_dir = current_dir.into();

        lua.globals()
            .set("fs", create_fs_module(&lua, current_dir.clone())?)?;
        lua.globals()
            .set("os", create_os_module(&lua, current_dir.clone())?)?;
        lua.globals().set(
            "project",
            create_project_module(&lua, current_dir.clone(), project_name.into())?,
        )?;

        Ok(Self {
            lua,
            current_dir,
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

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
}
