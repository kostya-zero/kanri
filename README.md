# 🗂️ Kanri

![Crates.io Version](https://img.shields.io/crates/v/kanri) ![GitHub branch check runs](https://img.shields.io/github/check-runs/kostya-zero/kanri/main)

Yet another manager for your projects.

Kanri is a command-line tool designed for managing your projects.
It offers a simple and user-friendly interface for managing your projects using CLI.
Kanri is available for Windows, Linux, and macOS (compatibility with *BSD systems is not guaranteed).

> [!NOTE]
> This project is in beta. Some changes in newer version may not be backward compatible with previous versions and may require actions from user for an update.

## Installation

You can install Kanri with [Cargo](https://doc.rust-lang.org/cargo/) using the following commands:

```shell
# Compile and install Kanri.
cargo install kanri

# Install precompiled binaries (requires cargo-binstall).
cargo binstall kanri
```

You can also install Kanri from [GitHub Releases](https://github.com/kostya-zero/kanri/releases). If you prefer to build Kanri from source, please refer to the [Building Kanri](docs/BUILDING.md) guide.

## Usage

Before using Kanri, configure it according to your workspace setup. Detailed configuration options are available in the [Configuration Manual](docs/CONFIGURATION.md).

### List Projects

Kanri simplifies project management. To view a list of your projects, use the `list` subcommand:

```shell
kanri list
```

> [!NOTE]
> By default, Kanri hides projects whose names start with a dot (e.g., `.hidden_project`). You can change this behavior by configuring the `display_hidden` parameter as described in the [Configuration Manual](docs/CONFIGURATION.md).

### Managing Projects

Creating and removing projects with Kanri is straightforward:

```shell
# Create a new project.
kanri new bookshelf

# Create a project from a blueprint.
kanri new bookshelf --blueprint rust

# Remove an existing project.
kanri remove bookshelf
```

Blueprints are Lua scripts for project initialization. See the [Blueprints and Lua API](docs/BLUEPRINTS.md) guide for usage and API details.

### Working with Projects

Open project directly in your configured editor or shell using the `open` subcommand:

```shell
# Open the project in your editor.
kanri open bookshelf

# Open the project in your shell.
kanri open bookshelf --shell
```

### Quick Help

For assistance with commands, use the `--help` flag:

```shell
# General help.
kanri --help

# Help for a specific subcommand.
kanri config --help
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
