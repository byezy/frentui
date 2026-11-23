# frentui

[All projects](../../../WORKSPACE.md)

Project record for this tree, reviewed 19 November 2026. Git facts are from this machine at review time, taken before this file was added.

| | |
|---|---|
| Path | `~/development/tools/fren/frentui` |
| Kind | Rename toolkit |
| Stack | Rust |
| Repo | byezy/frentui |
| State | clean, in sync, 2026-01-04 |

## What it is

Interactive TUI (Text User Interface) for batch file renaming using freneng.

Source of the description: `README.md`.

## Repository and code state

This folder is its own git repository.

- Branch: `main`
- Upstream: `origin/main`
- Remote: https://github.com/byezy/frentui.git
- Ahead / behind: 0 ahead, 0 behind
- Commits: 2
- Latest commit: 2026-01-04 `d38d6aa` Release v0.1.0: Initial release of frentui
- Working tree: clean

## Review

Second pass on 16 November 2026. How this was judged, and how to refresh it, is in [_workspace/review/README.md](../../../_workspace/review/README.md).

Depth for this pass: a file census, the test filenames, the README, and the git snapshot. Tests were not executed. There is no coverage percentage.

**Tests.** First-party source census: 45 source files, 19 test-like files. The count skips `node_modules`, build output, `.svelte-kit`, `third_party`, and rustdoc dumps. Shell, Markdown, and YAML are not in the source count. Runners seen: Cargo.toml. Sample test files: `action_tests.rs`, `app_tests.rs`, `args_tests.rs`, `color_tests.rs`.

**Code quality.** Test files are present (19 beside 45 source files). That ratio is not a coverage figure: UI and platform files inflate the source count, and a few focused tests can still cover the domain. TODO/FIXME/HACK/XXX markers in scanned source: 7 (including src/archive/steps/common.rs (1), src/archive/steps/review_files.rs (1), src/archive/steps/step_files.rs (5)). Nineteen Rust integration tests sit under `tests/`. A rustdoc HTML dump of ratatui 0.30.0 is on disk and is gitignored (`/rustdoc-ratatui-0.30.0`), so it is not part of the crate. TODO markers found are in `src/archive/`, the old step code.

**Risks.** Git snapshot shows nothing more urgent than the test notes above.
