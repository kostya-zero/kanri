# Profiles

Kanri profiles let you switch editor and shell settings without rewriting your configuration. The active profile is selected by `options.current_profile` in `config.toml`.

Profiles affect commands that launch external programs, especially:

- `kanri open <project>`
- `kanri open <project> --shell`
- `kanri config edit`
- `kanri blueprints new <name>` when Kanri opens the new blueprint in your editor

## Profile structure

Profiles live under the `[profiles]` table in the configuration file:

```toml
[profiles.default]
editor = "code"
editor_args = ["."]
editor_fork_mode = true
shell = "bash"
```

Each profile has these fields:

- `editor` - Program used for editor sessions.
- `editor_args` - Extra arguments passed to the editor. Code-family editors usually use `["."]` so the project directory opens as a workspace.
- `editor_fork_mode` - If `true`, Kanri starts the editor and returns immediately instead of waiting for it to exit.
- `shell` - Program used for shell sessions.

Unknown profile fields are rejected when the configuration is loaded.

> [!NOTE]
> Older Kanri versions used `shell_args`. Current Kanri no longer supports that field and can migrate it out of existing configuration files.

## Default profile

Kanri creates a `default` profile on first run. It detects a default editor from `VISUAL` or `EDITOR`, and a default shell from `SHELL` or `COMSPEC` on Windows.

For Code-family editors such as `code`, `code-insiders`, `codium`, `code-oss`, `cursor`, and `windsurf`, Kanri enables fork mode and adds `.` to `editor_args`. On Windows, Kanri also appends `.cmd` where needed.

## Managing profiles from the CLI

```shell
# Create a profile interactively.
kanri profiles new

# List profiles. The current profile is marked.
kanri profiles list

# Show profile details.
kanri profiles get default

# Set active profile.
kanri profiles set default

# Remove a profile with confirmation.
kanri profiles remove old-profile

# Remove without prompting.
kanri profiles remove old-profile --yes
```

Kanri prevents removing the current active profile. Switch to another profile first with `kanri profiles set <name>`.

## Opening projects with profiles

```shell
# Uses the current profile's editor, editor_args, and editor_fork_mode.
kanri open my-project

# Uses the current profile's shell.
kanri open my-project --shell
```

Shell sessions receive `KANRI_SESSION=1` in their environment.
