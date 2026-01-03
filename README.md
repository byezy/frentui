# frentui

Interactive TUI (Text User Interface) for batch file renaming using `freneng`.

## Overview

`frentui` provides a guided, menu-driven workflow for batch file renaming. It's designed for users who prefer an interactive experience over command-line arguments.

This tool is separate from `frencli` to keep the CLI tool lean and focused, while providing a rich interactive experience for those who want it.

## Features

- **Guided Workflow**: Step-by-step menu-driven interface
- **Flexible Navigation**: Go back or jump to any step
- **State Preservation**: All choices maintained throughout the workflow
- **Preview Before Apply**: See changes before committing
- **Template Integration**: Select from predefined templates
- **Error Recovery**: Clear error messages with retry options

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

The tool will guide you through:
1. Selecting a directory
2. Choosing files to process
3. Defining a rename pattern (or selecting a template)
4. Previewing changes
5. Configuring rename options
6. Applying the rename
7. Viewing results

## Status

🚧 **In Development** - This project is currently in the design phase.

See the [design document](../design/frentui/INTERACTIVE_WORKFLOW_DESIGN.md) for the planned implementation.

## Relationship to Other Tools

- **freneng**: Core renaming engine library (used by both `frencli` and `frentui`)
- **frencli**: CLI tool for batch file renaming (command-line interface)
- **frentui**: Interactive TUI for batch file renaming (this tool)

## License

MIT OR Apache-2.0

