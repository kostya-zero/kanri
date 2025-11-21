# Configuration Manual

This manual provides guidance on configuring Kanri for your workspace.

### Configuration File

If a configuration file does not exist in your file system, Kanri will generate one in the following locations:

- **Windows**: `%USERPROFILE%\kanri\config.toml`
- **macOS**: `$HOME/Library/Application Support/kanri`
- **Linux** and **macOS**: `$HOME/.config/kanri/config.toml`

Kanri generates the configuration based on your environment settings, such as the `EDITOR` and `SHELL` environment variables.

>[!NOTE]
> On Windows, Kanri appends `.cmd` to the `program` field for specific editors. This is because Kanri requires the `.cmd` files to launch these editors. The affected editors are:
>
> - Visual Studio Code
> - Visual Studio Code - Insiders
> - Code - OSS
> - VS Codium
> - Windsurf

### Default configuration structure

```toml
[options]
projects_directory = '/home/user'
display_hidden = false

[profiles.default]
editor = "code"
editor_args = ["."]
editor_fork_mode = true
shell = "bash"
shell_args = ["-c"]

[recent]
enabled = true
recent_project = "example"

[autocomplete]
enabled = true
always_accept = true
```

For more information about the fields in the configuration, refer to the [Parameters section](#parameters).

### Manage configuration

Kanri allows you to manage the configuration through the `config` subcommand. Here is a list of available actions:

- `edit` - Opens the configuration file in the editor specified in the `[editor]` section of the configuration. This allows you to manually edit the configuration settings.
- `path` - Gets the path to the configuration file.
- `reset` - Resets the configuration to its default settings.

# Parameters

This section details the configuration parameters available in the `config.toml` file, organized by their respective sections.

### `options`

- `projects_directory` - Path to the directory containing your projects. By default, it uses the path to the user's home directory.
- `display_hidden` - Controls whether hidden directories are displayed. By default, set to `false`.

### `profiles`

Read more in [docs/PROFILES.md](PROFILE.md).

### `recent`

- `enabled` - Controls whether the recent projects feature is enabled. By default, set to `true`.
- `recent_project` - Name of the most recent project. This field is used to store the name of the most recently opened project.

### `autocomplete`

- `enabled` - Controls whether the autocomplete feature is enabled. By default, set to `true`.
- `always_accept` - Determines whether the autocomplete feature should automatically accept the suggestion. If set to `true`, it will automatically select the suggestion. By default, this is set to `true`.
