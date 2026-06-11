# 🗂️ Kanri

![Crates.io Version](https://img.shields.io/crates/v/kanri) ![GitHub branch check runs](https://img.shields.io/github/check-runs/kostya-zero/kanri/main)

Kanri is a cross-platform CLI for managing local project directories. It can create, clone, list, rename, remove, and open projects from a single configured workspace.

Kanri is available for Windows, Linux, and macOS. Compatibility with *BSD systems is not guaranteed.

> [!NOTE]
> This project is in beta. Some changes in newer versions may not be backward compatible with previous versions.

## Installation

```shell
# Compile and install Kanri.
cargo install kanri

# Install precompiled binaries, if available (requires cargo-binstall).
cargo binstall kanri
```

You can also install Kanri from [GitHub Releases](https://github.com/kostya-zero/kanri/releases). To build from source, see [Building Kanri](docs/BUILDING.md).

## Documentation

- [Configuration Manual](docs/CONFIGURATION.md)
- [Profiles](docs/PROFILES.md)
- [Blueprints and Lua API](docs/BLUEPRINTS.md)
- [Building Kanri](docs/BUILDING.md)

## Usage

Kanri stores projects in the directory configured by `options.projects_directory`. On first run, Kanri creates a default configuration file based on your environment.

### Create projects

```shell
# Create an empty project.
kanri new bookshelf

# Create a project from a Lua blueprint.
kanri new bookshelf --blueprint rust
kanri new bookshelf -b rust
```

Blueprints are Lua scripts for project initialization. See [Blueprints and Lua API](docs/BLUEPRINTS.md).

> [!NOTE]
> The old template system has been replaced by blueprints. Existing templates can be migrated with:
>
> ```shell
> kanri blueprints migrate-templates
> ```

### Clone repositories

```shell
# Clone into the projects directory.
kanri clone https://github.com/example/project.git

# Clone with a custom directory name or branch.
kanri clone https://github.com/example/project.git --name my-project --branch main
```

### List projects

```shell
kanri list
kanri list --pure
```

By default, Kanri hides projects whose names start with a dot. Configure `options.display_hidden` to change this behavior.

### Open projects

```shell
# Open in the configured editor.
kanri open bookshelf

# Alias.
kanri o bookshelf

# Open a shell in the project.
kanri open bookshelf --shell

# Print the project path instead of opening it.
kanri open bookshelf --path
```

Use `kanri open -` to open the most recent project when recent project tracking is enabled.

### Rename and remove projects

```shell
kanri rename old-name new-name

# Prompts for confirmation in interactive terminals.
kanri remove bookshelf

# Required for non-interactive removal.
kanri remove bookshelf --yes
```

### Profiles

Profiles control which editor and shell Kanri uses.

```shell
kanri profiles new
kanri profiles list
kanri profiles get default
kanri profiles set default
kanri profiles remove old-profile --yes
```

See [Profiles](docs/PROFILES.md).

### Backup and import

```shell
# Save config and blueprints to kanri_backup.json.
kanri backup

# Save to a custom file.
kanri backup ./backup.json

# Restore from a backup. This overwrites the current configuration.
kanri import ./backup.json
```

### Quick help

```shell
kanri --help
kanri config --help
kanri blueprints --help
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
