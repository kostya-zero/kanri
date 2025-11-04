# Profiles

Kanri lets you define multiple *profiles* so you can tailor how projects open in different editors and shells without rewriting your configuration every time. Each profile stores launch details, and you can switch between them whenever you need another workflow.

## Profile structure

Profiles live in the configuration file under the `[profiles]` table. Every entry stores:

- `editor`: the program Kanri launches for editor sessions.
- `editor_args`: extra arguments passed to the editor.
- `editor_fork_mode`: whether Kanri should fork before returning control (defaults to `true` for Visual Studio Code–style editors).
- `shell`: the command used when opening a shell session.
- `shell_args`: the default arguments for that shell.

These keys map directly to the `Profile` definition in the source code, so any additional keys will be rejected by the configuration loader.

## Default profile

Kanri ships with a `default` profile. On first run Kanri detects a sensible editor and shell for your platform, adding smart defaults such as `-NoLogo` for PowerShell or `-c` for POSIX shells, and enabling fork mode for Code-family editors. You can edit the generated configuration file directly if you want to adjust these defaults.

The `current_profile` option under `[options]` selects which profile Kanri uses when running commands like `kanri open` or applying templates. Switching the current profile changes the editor and shell Kanri uses immediately.

## Managing profiles from the CLI

Use the `kanri profiles` subcommands to create and maintain profiles without editing the configuration file manually:

- `kanri profiles new`: interactive prompts ask for the profile name, editor, shell, and whether to fork the editor. Kanri pre-populates sensible defaults for Code-family editors and common shells.【F:src/commands/profiles.rs†L10-L61】
- `kanri profiles list`: prints the names of all configured profiles so you can confirm what is available.
- `kanri profiles info <name>`: shows the editor and shell assigned to a specific profile.
- `kanri profiles set <name>`: marks a profile as the current one by updating the configuration’s `current_profile` field.
- `kanri profiles remove <name>`: confirms before deleting a profile from the configuration to prevent accidental loss.

Whenever you invoke `kanri open`, the currently selected profile determines whether Kanri launches your editor or shell, which arguments it passes, and whether it forks the process. Use profiles to create dedicated setups for different languages, shells, or workflows, then switch between them with a single command.
