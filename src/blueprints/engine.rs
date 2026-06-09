use std::path::PathBuf;

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
    pub fn init<I: Into<PathBuf> + Clone>(
        current_dir: I,
        file_name: String,
        project_name: String,
    ) -> BlueprintEngine {
        let lua = Lua::new_with(
            StdLib::ALL_SAFE | StdLib::MATH | StdLib::TABLE | StdLib::STRING | StdLib::UTF8,
            LuaOptions::default(),
        )
        .unwrap();

        let current_dir = current_dir.into();

        lua.globals()
            .set("fs", create_fs_module(&lua, current_dir.clone()))
            .unwrap();

        lua.globals()
            .set("os", create_os_module(&lua, current_dir.clone()))
            .unwrap();

        lua.globals()
            .set(
                "project",
                create_project_module(&lua, &current_dir, project_name),
            )
            .unwrap();

        BlueprintEngine {
            lua,
            current_dir,
            file_name,
        }
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

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
}
