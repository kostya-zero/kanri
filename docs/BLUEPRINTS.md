# Blueprints and Lua API

Blueprints are Lua scripts that initialize a newly created Kanri project. They support file generation, conditional logic, OS-specific setup, and command output handling.

## Table of contents

- [Storage location](#storage-location)
- [Managing blueprints](#managing-blueprints)
- [Using a blueprint](#using-a-blueprint)
- [Lua runtime](#lua-runtime)
- [Example blueprint](#example-blueprint)
- [Modules](#modules)
  - [`fs`](#fs-module)
  - [`path`](#path-module)
  - [`process`](#process-module)
  - [`project`](#project-module)
  - [`system`](#system-module)
- [Error handling](#error-handling)

## Storage location

Blueprints are stored as `.lua` files in Kanri's configuration directory under `blueprints`:

```text
<config directory>/blueprints/*.lua
```

Use `kanri config path` to find the configuration directory on your machine.

The blueprint name is the file stem. For example, `rust.lua` is used as `rust`.

## Managing blueprints

```shell
# Create a blueprint and open it in your configured editor.
kanri blueprints new rust

# List available blueprints.
kanri blueprints list

# Edit an existing blueprint.
kanri blueprints edit rust

# Check Lua syntax without running the blueprint.
kanri blueprints check rust

# Remove a blueprint.
kanri blueprints remove rust
```

Blueprint names must not contain path separators. Use the blueprint stem, such as `rust`, with `kanri new --blueprint`, `kanri blueprints edit`, and `kanri blueprints check`.

## Using a blueprint

Pass a blueprint to `kanri new` with `--blueprint` or `-b`:

```shell
kanri new my-app --blueprint rust
kanri new my-app -b rust
```

Kanri creates the project directory, then runs the blueprint inside that directory. If the blueprint cannot be found or the Lua script fails, Kanri reports the error and removes the newly created project directory.

## Lua runtime

Blueprints run in an embedded Lua 5.4 runtime with all standard libraries except `debug`. The standard `os` library is available as usual, for example `os.getenv` for environment variables and `os.date` or `os.time` for dates.

Standard `io` and `os` functions that take paths, such as `io.open`, `os.remove`, and `os.rename`, resolve relative paths from the directory Kanri was started in, not from the project directory. Use the `fs` module to work with project files.

Kanri also injects these global modules:

- `fs` for filesystem operations in the project directory.
- `path` for working with paths.
- `process` for finding and running programs.
- `project` for information about the project being created.
- `system` for platform information.

## Example blueprint

```lua
local name = project.name()

fs.write("README.md", "# " .. name .. "\n")
fs.create_dir("src")
fs.write("src/main.rs", [[
fn main() {
    println!("Hello from Kanri!");
}
]])

fs.write("Cargo.toml", string.format([[
[package]
name = "%s"
version = "0.1.0"
edition = "2024"
]], name))

if pcall(process.which, "git") then
    process.run("git", { "init" })
end
```

## Modules

### `fs` module

All relative paths are resolved from the project directory. Paths are not sandboxed: an absolute path or `..` can access files outside it. Run only blueprints you trust.

| Function | Returns | Description |
| --- | --- | --- |
| `fs.write(path, content)` | `nil` | Writes `content` to a file, replacing it if it exists. |
| `fs.read(path)` | `string` | Reads a file as UTF-8 text. |
| `fs.remove_file(path)` | `nil` | Removes a file. |
| `fs.remove_dir(path)` | `nil` | Removes a directory and all of its contents. |
| `fs.move(from, to)` | `nil` | Renames or moves a file or directory on the same filesystem. |
| `fs.exists(path)` | `boolean` | Returns whether a path exists. |
| `fs.is_file(path)` | `boolean` | Returns whether a path is a regular file. |
| `fs.is_dir(path)` | `boolean` | Returns whether a path is a directory. |
| `fs.create_dir(path)` | `nil` | Creates a directory and missing parent directories. |
| `fs.append(path, content)` | `nil` | Appends `content` and a trailing newline to a file, creating the file if it does not exist. |

Example:

```lua
if not fs.exists("src") then
    fs.create_dir("src")
end

fs.write("src/index.js", "console.log('hello')\n")
```

### `path` module

Path operations use the host operating system's path syntax. They work on strings without accessing the filesystem. Results use the host's directory separator.

| Function | Returns | Description |
| --- | --- | --- |
| `path.join(...)` | `string` | Joins path parts. Empty parts are ignored; an absolute part replaces preceding parts. Returns `.` when no parts remain. Does not normalize `..`. |
| `path.parent(path)` | `string` or `nil` | Parent directory, or `nil` if there is none. Returns `.` for a file in the current directory. |
| `path.basename(path)` | `string` or `nil` | Final path component, or `nil` if there is no file name. |
| `path.extension(path)` | `string` or `nil` | Final extension without the dot, or `nil` if there is none. |
| `path.stem(path)` | `string` or `nil` | Final file name without its last extension, or `nil` if there is no file name. |
| `path.is_absolute(path)` | `boolean` | Whether the path is absolute on the host OS. |

```lua
local source = path.join("src", "main.rs")
print(path.basename(source))                -- main.rs
print(path.stem(source))                    -- main
print(path.extension(source))               -- rs
```

### `process` module

Programs run with the project directory as their working directory. `process.run` inherits the terminal streams unless Kanri is running in quiet mode.

| Function | Returns | Description |
| --- | --- | --- |
| `process.which(executable)` | `string` | Returns the resolved executable path from `PATH`. Raises a Lua runtime error if the executable is not found. |
| `process.run(program, args)` | `number` or `nil` | Runs a program with a list of arguments and returns its exit code. Returns `nil` if the process ended without an exit code. |

An empty program name or a failure to start the process raises a Lua runtime error. A non-zero exit code is returned and does not itself raise an error.

Use `pcall` to check whether a program is installed without stopping the blueprint:

```lua
if pcall(process.which, "git") then
    local status = process.run("git", { "init" })
    if status ~= 0 then
        error("git init failed with status " .. tostring(status))
    end
end
```

### `project` module

| Function | Returns | Description |
| --- | --- | --- |
| `project.name()` | `string` | Name passed to `kanri new`. |
| `project.path()` | `string` | Path to the project directory. |

Example:

```lua
fs.write("README.md", "# " .. project.name() .. "\n")
print("Generating project at " .. tostring(project.path()))
```

### `system` module

| Function | Returns | Description |
| --- | --- | --- |
| `system.system()` | `string` | Operating system name, such as `windows`, `linux`, or `macos`. |
| `system.arch()` | `string` | CPU architecture, such as `x86_64` or `aarch64`. |
| `system.exe_suffix()` | `string` | Executable suffix for the platform, such as `.exe` on Windows or an empty string elsewhere. |
| `system.temp_dir()` | `string` | Path to the system temporary directory. |

Example:

```lua
if system.system() == "windows" then
    fs.write("run.bat", "@echo off
echo hello
")
else
    fs.write("run.sh", "#!/usr/bin/env sh
echo hello
")
end
```

## Error handling

Any Lua error stops blueprint execution:

```lua
if not fs.exists("package.json") then
    error("package.json was not generated")
end
```

Filesystem errors and process launch errors are converted into Lua runtime errors. Check the value returned by `process.run` when a non-zero exit code should stop blueprint execution.
