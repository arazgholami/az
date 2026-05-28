# az

`az` is a small, sane terminal text editor written in PHP with NO external dependencies except PHP CLI.
It opens ready to type, supports project folders, tabs, a project tree, quick opening, command actions, line numbers, simple shortcuts, syntax highlighting, and autocomplete.

![az screenshot](az-screenshot.jpg)

## Features

- Keyboard-only editing
- Opens files or folders
- Project tree sidebar
- Tabs with Alt+1 to Alt+9
- Quick open with Ctrl+O
- Open files and jump to a line with `file.php:20`
- Jump to a line in the current file with `:20`
- Project search from the quick open interface
- Function/symbol opening from quick open
- Command palette with Ctrl+P
- Save, create files, and run editor actions from the command palette
- Visible line numbers
- Welcome screen for startup and Ctrl+/ help
- Find and replace
- Case-insensitive find/replace with `%term`
- Word wrapping
- UTF-8 input support
- Syntax highlighting for PHP, Blade, HTML, and CSS
- Autocomplete for HTML, CSS, and PHP

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/arazgholami/az/refs/heads/main/install.sh | sh
```

Then run:

```sh
az
```

Or open a file or folder:

```sh
az file.php
az ~/my-project
```

## Manual install

```sh
mkdir -p ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/arazgholami/az/refs/heads/main/az -o ~/.local/bin/az
chmod +x ~/.local/bin/az
```

Make sure `~/.local/bin` is in your `PATH`:

```sh
export PATH="$HOME/.local/bin:$PATH"
```

## Requirements

- Linux, macOS, or another Unix-like terminal
- PHP CLI

On Debian or Ubuntu:

```sh
sudo apt install php-cli
```

## Shortcuts

| Shortcut | Action |
| --- | --- |
| Ctrl+S | Save |
| Ctrl+O | Quick open files, symbols, search, or jump to line |
| Ctrl+P | Command palette |
| Ctrl+T | Switch between tree and editor |
| Ctrl+H | Hide or show tree |
| Ctrl+F | Find |
| Ctrl+R | Replace |
| Ctrl+W | Remove current line |
| Ctrl+Z | Undo |
| Ctrl+D | Close tab |
| Ctrl+/ | Show welcome/help screen |
| Ctrl+Q | Quit |
| Tab | Complete suggestion |
| Enter | Accept selected completion |
| Alt+1..9 | Switch tab |

## Quick open

Ctrl+O opens the quick open menu.

Examples:

```txt
main.blade.php
main.blade.php:20
:20
```

Use `file.php:20` to open a file and jump to line 20.
Use `:20` to jump to line 20 in the current file.

## Find and replace

Normal search is case-sensitive.
Prefix the search term with `%` to ignore uppercase and lowercase differences.

Examples:

```txt
hello
%hello
```

## Notes

az is intentionally simple. It is not trying to replace a full IDE. It is a fast terminal editor for small projects, quick fixes, and focused editing.
