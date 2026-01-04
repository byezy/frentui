# Changelog

All notable changes to `frentui` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-03

### Added
- Initial release of `frentui` as a standalone crate
- Interactive TUI (Text User Interface) for batch file renaming
- Single-screen interface with all sections visible simultaneously
- Working directory selection with path completion
- File matching with glob pattern support
- Exclusion patterns for filtering files
- Renaming rule editor with template support
- Real-time preview pane showing rename operations
- Validation section displaying issues and warnings
- Apply section for executing rename operations
- Undo functionality for reversing rename operations
- Comprehensive keyboard navigation
- Mouse support for interactive elements
- Scrollable sections for large file lists
- Template registry integration
- Error handling with clear messages
- State preservation throughout workflow
- Integration with `freneng` library for core renaming functionality

### Features
- **Single-Screen Layout**: All sections visible at once for better overview
- **Interactive Navigation**: Keyboard and mouse support for all operations
- **Real-Time Preview**: See rename results before applying
- **Template Support**: Use predefined patterns or create custom ones
- **Validation**: Comprehensive validation with issue reporting
- **Undo Support**: Safely reverse rename operations
- **Path Completion**: Tab completion for directory paths
- **Scrollable Lists**: Handle large file sets with scrollable sections

[0.1.0]: https://github.com/byezy/frentui/releases/tag/v0.1.0
