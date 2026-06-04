use std::path::PathBuf;

use mlua::prelude::*;
use mlua::{LuaOptions, StdLib};

use crate::blueprints::modules::fs::create_fs_module;

pub struct BlueprintEngine {
    lua: Lua,
    current_dir: PathBuf,
}

impl BlueprintEngine {
    pub fn init<I: Into<PathBuf> + Clone>(current_dir: I) -> BlueprintEngine {
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
            .set("os", create_fs_module(&lua, current_dir.clone()))
            .unwrap();

        BlueprintEngine { lua, current_dir }
    }

    pub fn run(&self, source: &str) -> LuaResult<()> {
        self.lua.load(source).exec()
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
}
