use crate::config::ConfigError;
use anyhow::Result;

pub fn migrate_config(value: &mut toml::Value) -> Result<bool, ConfigError> {
    let version = value
        .get("version")
        .and_then(toml::Value::as_str)
        .unwrap_or("1");

    match version {
        "1" => {
            migrate_v1_to_v2(value)?;
            Ok(true)
        }
        "2" => Ok(false),
        _ => Err(ConfigError::BadConfiguration(format!(
            "unknown configuration version: {}",
            version
        ))),
    }
}

fn migrate_v1_to_v2(value: &mut toml::Value) -> Result<(), ConfigError> {
    let root = value.as_table_mut().ok_or_else(|| {
        ConfigError::BadConfiguration("configuration root must be a table".to_string())
    })?;

    if let Some(profiles) = root.get_mut("profiles").and_then(toml::Value::as_table_mut) {
        for (_, profile) in profiles {
            if let Some(profile_table) = profile.as_table_mut() {
                profile_table.remove("shell_args");
            }
        }
    }

    root.insert("version".to_string(), toml::Value::String("2".to_string()));

    Ok(())
}
