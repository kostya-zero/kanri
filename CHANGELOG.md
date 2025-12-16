# Kanri changelog

## Next

- Kanri will launch editor to create a new template instead of using dialog.
- Improved performance for Library.
- Command `templates info` has been renamed to `templates get`.
- Command `profiles info` has been renamed to `profiles get`.
- Current profile will be marked in the `profiles list` command.

## 0.9.0

- **Backups**. Kanri will be able to create backups of your configuration and templates files, and restore them if needed.
- **Profiles**. You can create multiple profiles with different configurations, and switch between them. Each profile will have its own editor, shell and options for them.
- Some dialogues now shows user's selection.
- Added initial support for configuration migration (users from 0.8.2 and below should migrate manually).

## 0.8.2

- Fixed an issue where the `kanri templates edit` command used the templates file path instead of the configuration file path to load user's settings.

## 0.8.1

- Optimized the algorithm for `.ignore` file.
- Added `--path` option to the `open` command to print path instead of launching editor/shell. Useful for integrations with other software.
- Added support for `NO_COLOR` environment variable.
- Removed underlines for titles.

## 0.8.0

- When Kanri runs template commands, it will now add `KANRI_PROJECT` environment variable with the name of the project. So you can use it in your template commands, e.g. for initializing Go modules with `go mod init $KANRI_PROJECT`.
- When you are running shell session with Kanri, it will add `KANRI_SESSION`, so you can use it in your shell scripts to check if you are running in Kanri session.
- Added `always_accept` option to the configuration of autocomplete. If this option is set to `true`, Kanri will not ask for confirmation when you are trying to open or remove a project, and it will always accept the suggestion.
- Added `templates path` command that returns the path to the templates file.
- Added `config recent` command to get the recent project opened.
- The output messages has been rewritten to be more concise.
- When removing a project, the spinner will appear.
- Added autocomplete support for `remove` subcommand.
- Added icons for some CLI messages.
- Added `--pure` flag for `list` and `templates list` to display lists without styles.
- Added `rm` as alias to `remove` command.
- Configuration and templates files are now being lazily loaded, which improves performance and startup time.
- Various internal refactoring and code improvements for better maintainability and error handling.

## 0.7.1

- Added `ls` as alias to `list` command.
- Added `o` as alias to `open` command.
- Command `delete` renamed to `remove`.
- Various internal refactoring and code improvements for better maintainability and error handling.

## 0.7.0

- Templates are now stored in a separate `templates.json` file, created in the same directory as the configuration file. You can add templates manually or manage them using the `templates` command.
- Added `edit` subcommand for `templates` to launch editor with opened `templates.json` file.
- Added support for the `code-insiders` editor.
- The configuration file has a new layout:

  ```toml
  [options]
  projects_directory = '/home/user'
  display_hidden = false

  [editor]
  program = "nvim"
  fork_mode = false
  args = []

  [shell]
  program = "bash"
  args = ["-c"]

  [recent]
  enabled = true
  recent_project = "example"

  [autocomplete]
  enabled = true
  ```

  **Note**: You need to reset your current configuration file; the new version is not compatible with the old format.

- The `shell` section now includes an `args` field. The values in this field will be used when running commands to initialize a project with a template. For example, in PowerShell:

  ```toml
  [shell]
  program = "pwsh.exe"
  args = ["-NoLogo", "-c"]
  # Enjo will execute: powershell.exe -NoLogo -c "{command}"
  ```

- Removed the spinner animation when removing a project.
- Various internal refactorings and code improvements for better maintainability and error handling.

## 0.6.1

- Mark the most recently opened project in the `list` command output.
- More concise output for all commands.
- Improve help messages for the `clone` and `rename` commands.
- Various internal refactorings and code improvements for better maintainability and error handling.
- Fixes for internal tests.

## 0.6.0

- The default value of `autocomplete` option are set to `true`.
- Removed the icons from the output.
- Added a spinner when deleting a project.
- Reduces storage writes.
- You can use `--force` to force project deletion.
- Migrate project to Rust 2024 edition.
- Performance improvements.
- Some changes in the wording of the messages.

## 0.5.2

- Fixed bug when trying to create a new project without specified template Enjo was showing an `Template not found` message.
- Some changes in output icons.

## 0.5.1

- Remove spinner animation when removing project.
- Project directory will be removed if template is not found.
- Enjo will return an error if template command exited with non-zero code.
- Performance improvements.

## 0.5.0

- **Autocomplete**. Enjo will now complete project names in the `open` and `delete` commands. This option can be configured in the configuration file.
- The templates file have been moved to a storage file. You need to add all of your templates again.
- You can open your recent project using dash symbol (`-`) in the `open` command. Example: `enjo open -` will open your recent project.
- Added support for Windsurf editor when generating default configuration.

## 0.4.0

- **Templates**. Now you can create templates to generate projects from. Use `enjo templates --help` for help.
- New global argument `--hidden` allows to display hidden even if `display_option` is set to `false`.
- Fixed `unknown t switch` message from Git.
- Increased stability of `clone` command.
- Enjo will warn you if you are cloning repository which name starts with dot.

## 0.3.0

- **The configuration file has been restructured.** If you used Enjo before version `0.2.1`, you need to reset your configuration file. See [configuration manual](docs/CONFIGURATION.md) for more details.
- Added `fork_mode` option to the `editor` section. This option determines whether the editor should be started as a separate process or whether Enjo needs to wait until it exits.
- Added new subcommand `rename` that allows you to rename project.

## 0.2.1

- System directories will be hidden regardless of the value of the `display_hidden` parameter.
- Confirmation to reset the configuration is now implemented through a dialog instead of an argument.
- When deleting a project, Enjo will ask for confirmation if the project is not empty.
- If `zed`, `code` or `codium` is used as the editor, Enjo will add a dot to the editor arguments.
- Slight changes to the wording of the messages.

## 0.2.0

- You can clone your projects from remote Git repository with `clone` command.
- You can hide hidden and system directories from list of projects with `display_hidden` option in your configuration file.
- When Enjo generates a new configuration on Windows, it will use `code.cmd` instead of `code`.
- Some changes in messages.
- Some internal improvements.

## 0.1.1

- If you don't have a configuration file, Enjo will generate one and warn you about it.
- Default values for arguments are now hidden.
- Enjo will use a description from the package manifest rather than the one written in code.

## 0.1.0

First release of Enjo!
