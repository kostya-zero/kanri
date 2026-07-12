# Configuration Manual

Kanri uses a TOML configuration file to decide where projects are stored, which profile is active, and how autocomplete/recent-project behavior works.

## Configuration file

On startup, Kanri creates a default configuration file if it does not exist. The file is stored in Kanri's platform configuration directory as `config.toml`.

Common locations are:

- **Windows**: the platform config directory for Kanri, usually under `%APPDATA%` or another directory selected by the OS/environment.
- **macOS**: `$HOME/Library/Application Support/kanri/config.toml` or the configured XDG location.
- **Linux**: `$XDG_CONFIG_HOME/kanri/config.toml` or `$HOME/.config/kanri/config.toml`.

To print the exact path on your machine, run:

```shell
kanri config path
```

Kanri generates defaults from your environment where possible, including `VISUAL`, `EDITOR`, `SHELL`, and on Windows `COMSPEC`.

> [!NOTE]
> On Windows, Kanri appends `.cmd` to Code-family editor names when needed. This applies to editors such as Visual Studio Code, VS Codium, Cursor, and Windsurf.

## Configuration structure

The generated values depend on your platform and environment. This example shows the schema; replace the editor and shell values with programs available on your machine.

```toml
version = "2"

[options]
projects_directory = "/home/user/Projects"
current_profile = "default"
display_hidden = false

[profiles.default]
editor = "code"
editor_args = ["."]
editor_fork_mode = true
shell = "bash"

[recent]
enabled = true
recent_project = ""

[autocomplete]
enabled = true
always_accept = true
```

Configuration uses strict keys. Unknown fields are rejected.

## Manage configuration from the CLI

```shell
# Open config.toml in the current profile's editor.
kanri config edit

# Print the config path.
kanri config path

# Print the most recent project.
kanri config recent

# Clear the recent project.
kanri config recent --clear

# Reset configuration to defaults.
kanri config reset
```

## Parameters

### `version`

Configuration schema version. Kanri uses this to run automatic migrations when possible.

### `[options]`

- `projects_directory` - Directory containing your project folders. By default, Kanri uses the first existing directory named `Projects`, `Code`, `Dev`, `Development`, `Workspace`, `Workspaces`, `Work`, `Repos`, `Repositories`, `Source`, `Sources`, `Git`, or `GitHub` next to your home directory. If none exists, it uses `$HOME/Projects`.
- `current_profile` - Name of the active profile in `[profiles]`.
- `display_hidden` - Whether directories whose names start with `.` are listed as projects. Defaults to `false`.

### `[profiles]`

Profiles configure editor and shell behavior. See [Profiles](PROFILES.md).

### `[recent]`

- `enabled` - Enables recent project tracking. Defaults to `true`.
- `recent_project` - Name of the most recently opened project. Use `kanri open -` to open it.

### `[autocomplete]`

- `enabled` - Enables project-name autocomplete for commands that support it. Defaults to `true`.
- `always_accept` - Automatically accepts the best autocomplete suggestion. Defaults to `true`.

## Related files

Kanri also stores blueprints in the configuration directory:

```text
<config directory>/blueprints/*.lua
```

See [Blueprints and Lua API](BLUEPRINTS.md).
