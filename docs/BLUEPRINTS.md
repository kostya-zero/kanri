# Blueprints and Lua API

Blueprints are Lua scripts that initialize a newly created Kanri project. They replace the older templates system and are useful when a project needs more than a fixed list of shell commands: file generation, conditional logic, OS-specific setup, or command output handling.

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

# Migrate old templates to Lua blueprints.
kanri blueprints migrate-templates
```

Migration overwrites blueprints with matching names. After a successful migration, Kanri deletes the legacy `templates.json` file.

Blueprint names must not contain path separators. Use the blueprint stem, such as `rust`, with `kanri new --blueprint`, `kanri blueprints edit`, and `kanri blueprints check`.

## Using a blueprint

Pass a blueprint to `kanri new` with `--blueprint` or `-b`:

```shell
kanri new my-app --blueprint rust
kanri new my-app -b rust
```

Kanri creates the project directory, then runs the blueprint inside that directory. If the blueprint cannot be found or the Lua script fails, Kanri reports the error and removes the newly created project directory.

## Lua runtime

Blueprints run in an embedded Lua 5.4 runtime. Kanri enables Lua's safe standard libraries plus `math`, `table`, `string`, and `utf8`.

Kanri also injects three global modules:

- `fs` for filesystem operations.
- `os` for platform information and process execution.
- `project` for information about the project being created.

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

if os.exec_status("git", { "--version" }) == 0 then
    os.exec("git", { "init" })
end
```

## `fs` module

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

Example:

```lua
if not fs.exists("src") then
    fs.create_dir("src")
end

fs.write("src/index.js", "console.log('hello')\n")
```

## `os` module

Process commands run with the project directory as their working directory.

| Function | Returns | Description |
| --- | --- | --- |
| `os.system()` | `string` | Operating system name, such as `windows`, `linux`, or `macos`. |
| `os.arch()` | `string` | CPU architecture, such as `x86_64` or `aarch64`. |
| `os.family()` | `string` | OS family, usually `windows` or `unix`. |
| `os.exe_suffix()` | `string` | Executable suffix for the platform, such as `.exe` on Windows or an empty string elsewhere. |
| `os.dir_separator()` | `string` | Directory separator, such as `\` on Windows or `/` on Unix-like systems. |
| `os.path_separator()` | `string` | PATH separator, `;` on Windows or `:` elsewhere. |
| `os.temp_dir()` | `string` | Path to the system temporary directory. |
| `os.env(name)` | `string` or `nil` | Environment variable value, or `nil` if it is not set. |
| `os.current_dir()` | `string` | Project directory used by the blueprint engine. |
| `os.exec(command, args)` | `nil` | Runs a program with a list of arguments. Does not fail on a non-zero exit code. |
| `os.exec_status(command, args)` | `number` | Runs a program and returns its exit code. Returns `-1` if the process ended without an exit code. |
| `os.exec_output(command, args)` | `string` | Runs a program and returns stdout as text. |

Examples:

```lua
if os.system() == "windows" then
    fs.write("run.bat", "@echo off\necho hello\n")
else
    fs.write("run.sh", "#!/usr/bin/env sh\necho hello\n")
end

local status = os.exec_status("git", { "init" })
if status ~= 0 then
    error("git init failed with status " .. status)
end

local rustc_version = os.exec_output("rustc", { "--version" })
fs.write("RUST_VERSION.txt", rustc_version)
```

## `project` module

| Function | Returns | Description |
| --- | --- | --- |
| `project.name()` | `string` | Name passed to `kanri new`. |
| `project.path()` | `string` | Path to the project directory. |

Example:

```lua
fs.write("README.md", "# " .. project.name() .. "\n")
print("Generating project at " .. tostring(project.path()))
```

## Error handling

Any Lua error stops blueprint execution:

```lua
if not fs.exists("package.json") then
    error("package.json was not generated")
end
```

Filesystem errors and process launch errors are converted into Lua runtime errors. For external commands, use `os.exec_status` when you need to fail on non-zero exit codes.
