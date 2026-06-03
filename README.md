# az text editor 2.0

`az` is a small, sane terminal text editor for code and text.

Version 2.0 is a complete rewrite in Rust. It keeps the original idea: open fast, stay simple, feel familiar, and make terminal editing less annoying.

It opens ready to type, works with files or project folders, and includes the practical things you expect from a modern editor: tabs, project tree, quick open, command palette, line numbers, find and replace, word wrap, syntax highlighting, autocomplete, and recovery files.

![az](az-editor.jpg)

## Why az?

`az` is for quick edits, small projects, server work, focused writing, and code changes directly inside the terminal.
It is a simple editor that behaves like a normal text editor, stays keyboard-friendly, and gets out of your way.

## What is new in 2.0?

- Completely rewritten in Rust
- Faster startup and rendering
- Better handling for huge files
- Modular language support through Rust plugins
- Mixed syntax highlighting for files that contain PHP, HTML, CSS, Blade, and JavaScript together
- File colors in the project tree based on extension

## Features

- Written in Rust
- Keyboard-first editing
- Opens files or folders
- Project tree sidebar
- Different tree colors for different file extensions
- Tabs with `Alt+1` to `Alt+9`
- Quick open with `Ctrl+O`
- Open files and jump to a line with `file.php:20`
- Jump to a line in the current file with `:20`
- Project search from quick open
- Function and symbol opening from quick open
- Command palette with `Ctrl+P`
- Save, create files, switch language mode, and run editor actions from the command palette
- Visible line numbers
- Welcome screen on startup
- `Ctrl+/` shows the same welcome/help screen
- Find and replace
- Case-insensitive search by default
- Case-sensitive search with `%term`
- Word wrapping
- UTF-8 input support
- Tokyo Night inspired interface colors
- Syntax highlighting through plugins
- Autocomplete through plugins
- Huge file editing support
- Recovery files for unsaved work
- Terminal cleanup on quit

## Language plugins

Language support lives in `src/plugins`.

Included plugins:

- PHP
- Blade
- HTML
- CSS
- JavaScript
- Example plugin skeleton

Each plugin is a separate Rust file. A plugin can provide syntax highlighting, autocomplete, symbol extraction, and tree colors.

Read `PLUGIN_GUIDE.md` to add a new language.

## Build and install

Build from source:

```sh
./build.sh
```

`build.sh` builds the editor, creates `./az`, installs it to:

```text
~/.local/bin/az
```

and adds `~/.local/bin` to your `PATH` in `~/.profile` when needed.

After the first install, restart your terminal or run:

```sh
. ~/.profile
```

Then run:

```sh
az
```

To install into a different folder:

```sh
AZ_BIN_DIR="$HOME/bin" ./build.sh
```

## Remote install

```sh
curl -fsSL https://raw.githubusercontent.com/arazgholami/az/refs/heads/main/install.sh | sh
```

The remote installer clones the repository, builds the Rust version, and installs the `az` binary to `~/.local/bin`.

## Requirements

Use one of these:

- Cargo, recommended
- Rust compiler, `rustc`

No external Rust crates are required.

## Basic usage

```sh
az file.php
az project/
```

Useful shortcuts:

```text
Ctrl+S   Save
Ctrl+O   Quick open
Ctrl+P   Command palette
Ctrl+F   Find
Ctrl+R   Replace
Ctrl+T   Switch tree/editor
Ctrl+H   Hide/show tree
Ctrl+D   Close tab
Ctrl+Q   Quit
Ctrl+/   Help
```

## License

WTFPL
