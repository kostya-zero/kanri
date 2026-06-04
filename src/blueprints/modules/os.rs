use dir_spec::runtime;
use mlua::prelude::*;
use std::{env, io::ErrorKind, path::PathBuf, process::Command};

use crate::runtime_error;

pub fn create_fs_module<I: Into<PathBuf>>(lua: &Lua, current_dir: I) -> LuaTable {
    let os_table = lua.create_table().unwrap();
    let current_dir = current_dir.into();

    {
        let os_system = lua.create_function(|_, ()| Ok(env::consts::OS)).unwrap();
        os_table.set("system", os_system).unwrap();
    }

    {
        let os_arch = lua.create_function(|_, ()| Ok(env::consts::ARCH)).unwrap();
        os_table.set("arch", os_arch).unwrap();
    }

    {
        let current_dir = current_dir.clone();
        let os_exec = lua
            .create_function(move |_, (cmd, args): (String, Vec<String>)| {
                if cmd.is_empty() {
                    return Err(mlua::Error::runtime("program cannot be empty"));
                }
                let mut command = Command::new(cmd);
                if !args.is_empty() {
                    command.args(args);
                }
                command.current_dir(current_dir.clone());
                let result = command.status();
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => runtime_error!("program not found"),
                        ErrorKind::Interrupted => runtime_error!("program was interrupted"),
                        ErrorKind::PermissionDenied => {
                            runtime_error!("not enough permissions to execute program")
                        }
                        _ => runtime_error!("unknown error occurred"),
                    },
                }
            })
            .unwrap();
        os_table.set("exec", os_exec).unwrap();
    }

    os_table
}
