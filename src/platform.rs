use std::{borrow::Cow, env, path::PathBuf};

pub fn config_dir() -> PathBuf {
    dir_spec::config_home()
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("kanri")
}

pub fn config_file() -> PathBuf {
    config_dir().join("config.toml")
}
pub fn templates_file() -> PathBuf {
    config_dir().join("templates.json")
}

pub fn blueprints_dir() -> PathBuf {
    config_dir().join("blueprints")
}

pub fn default_editor() -> Cow<'static, str> {
    if let Ok(v) = env::var("VISUAL").or_else(|_| env::var("EDITOR")) {
        return Cow::Owned(v);
    }
    #[cfg(target_os = "windows")]
    {
        Cow::Borrowed("code.cmd")
    }
    #[cfg(target_os = "macos")]
    {
        Cow::Borrowed("code")
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        Cow::Borrowed("nvim")
    }
}

pub fn default_shell() -> Cow<'static, str> {
    if let Ok(v) = env::var("SHELL") {
        return Cow::Owned(v);
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(v) = env::var("COMSPEC") {
            return Cow::Owned(v);
        }

        Cow::Borrowed("powershell.exe")
    }

    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(target_os = "macos")]
        {
            Cow::Borrowed("zsh")
        }

        #[cfg(not(target_os = "macos"))]
        {
            Cow::Borrowed("bash")
        }
    }
}

pub fn default_projects_dir() -> PathBuf {
    dir_spec::home()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Projects")
}
