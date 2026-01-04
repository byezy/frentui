# frentui

Interactive TUI (Text User Interface) for batch file renaming using `freneng`.

## Overview

`frentui` provides a guided, menu-driven workflow for batch file renaming. It's designed for users who prefer an interactive experience over command-line arguments.

This tool is separate from `frencli` to keep the CLI tool lean and focused, while providing a rich interactive experience for those who want it.

## Features

- **Single-Screen Interface**: All sections visible simultaneously for better overview
- **Interactive Navigation**: Full keyboard and mouse support
- **Real-Time Preview**: See rename results before applying changes
- **Template Support**: Use predefined patterns or create custom ones
- **File Matching**: Glob pattern support for selecting files
- **Exclusion Patterns**: Filter out files you don't want to rename
- **Validation**: Comprehensive validation with issue reporting
- **Undo Support**: Safely reverse rename operations
- **Path Completion**: Tab completion for directory paths
- **Scrollable Lists**: Handle large file sets efficiently
- **State Preservation**: All choices maintained throughout the workflow

## Installation

### Via Cargo

```bash
cargo install frentui
```

### From Source

```bash
git clone https://github.com/byezy/frentui.git
cd frentui
cargo build --release
cp ./target/release/frentui ~/.local/bin/
```

## Usage

```bash
frentui
```

The TUI displays all sections on a single screen:

- **Working Directory**: Select the directory to work in
- **Match Files**: Define glob patterns to match files
- **Exclusions**: Add patterns to exclude files
- **Renaming Rule**: Enter a rename pattern or select a template
- **Preview Pane**: See the rename preview in real-time
- **Validation**: View any validation issues or warnings
- **Apply**: Execute the rename operations
- **Undo**: Reverse the last batch of renames

### Keyboard Navigation

- Use `Tab` / `Shift+Tab` to navigate between sections
- `Enter` activates the focused section
- `Esc` cancels dialogs or returns to previous state
- Arrow keys navigate within lists and tables
- `Ctrl+C` or `q` quits the application

### Mouse Support

- Click on sections to focus them
- Click on list items to select them
- Use scroll wheel to navigate lists and tables

**📋 [Changelog](CHANGELOG.md)** - See what's new in each version

## Relationship to Other Tools

- **freneng**: Core renaming engine library (used by both `frencli` and `frentui`)
- **frencli**: CLI tool for batch file renaming (command-line interface)
- **frentui**: Interactive TUI for batch file renaming (this tool)

## License

This project is licensed under the MIT License.

