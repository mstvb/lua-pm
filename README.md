# lua-pm

A lightweight Lua package and project manager written in Rust. It helps you manage Lua dependencies and run Lua scripts with those dependencies automatically linked.

## Features

- **Project Initialization**: Easily set up a new Lua project with a `lua_pkg.toml` file.
- **Dependency Management**: Add, remove, and list dependencies from URLs (Git raw files, `.zip`, or `.tar.gz`).
- **Isolation**: Downloads dependencies into a local `lua_modules` folder, keeping your system clean.
- **Integrated Runtime**: Run Lua scripts with the `lua_modules` directory automatically added to the Lua `package.path`.
- **Reproducibility**: Reinstall all dependencies listed in your configuration file with a single command.

## Installation

To install `lua-pm` globally, ensure you have Rust installed and run:

```bash
cargo install --path .
```

Alternatively, you can build it and use the binary directly:

```bash
cargo build --release
```

The binary will be located at `target/release/lua-pm`.

## Usage

### Initialize a Project

Create a new `lua_pkg.toml` file in the current directory.

```bash
lua-pm init
```

### Add a Dependency

Add a new dependency to your project. `lua-pm` supports:
- Direct links to `.lua` files, `.zip` archives, and `.tar.gz` archives.
- GitHub repositories using the `owner/repo` format or the full GitHub URL.

```bash
lua-pm add <URL_OR_REPO> <NAME>
```

Examples:
```bash
# From a direct URL
lua-pm add https://raw.githubusercontent.com/rxi/json.lua/master/json.lua json

# From a GitHub repository (owner/repo)
lua-pm add rxi/json.lua json-repo

# From a full GitHub URL
lua-pm add https://github.com/rxi/json.lua json-url-repo
```

### Install Dependencies

Download all dependencies listed in `lua_pkg.toml`. This is useful after cloning a project.

```bash
lua-pm install
```

### List Dependencies

Show all tracked dependencies in the current project.

```bash
lua-pm list
```

### Remove a Dependency

Remove a dependency from `lua_pkg.toml` and delete its files from `lua_modules`.

```bash
lua-pm remove <NAME>
```

### Run a Lua Script

Execute a Lua script. `lua-pm` automatically configures the Lua `package.path` so you can `require` your installed modules directly.

```bash
lua-pm run <SCRIPT_PATH>
```

Example:
```bash
lua-pm run main.lua
```

## Configuration File (`lua_pkg.toml`)

The `lua_pkg.toml` file stores project metadata and dependency URLs.

```toml
[project]
name = "my_lua_project"
version = "0.1.0"

[dependencies]
json = "https://raw.githubusercontent.com/rxi/json.lua/master/json.lua"
```

## Project Structure

- `lua_pkg.toml`: Project configuration and dependency list.
- `lua_modules/`: Directory where dependencies are installed.
- `src/`: Rust source code for the package manager.


## Acknowledgments

This project was inspired by the need for a simple and efficient Lua package manager. Special thanks to the Lua community for their contributions and support.
[RustDoc](target/doc/lua_pm/index.html)