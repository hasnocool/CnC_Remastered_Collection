# RUST/cnc-map-editor/tests/

## Responsibility

Two integration tests exercising the real editor binary end-to-end (BUILD_STATUS: 80 unit + 2 integration).

## Design

- `headless.rs` (61 L) — module doc: "End-to-end check that the editor binary's headless mode generates a map, writes scenario INI and produces a PNG screenshot. Runs the real binary." Test `headless_binary_generates_map_ini_and_screenshot` invokes `cnc-map-editor --headless --seed 42 --screenshot ...` with help of default paths; asserts INI + PNG exist and parse.
- `validate.rs` (70 L) — module doc: "End-to-end checks for the --validate scenario CLI." Helpers `run(args) -> std::process::Output`; test `validate_accepts_a_valid_map_and_reports_issues` checks exit codes and reported issue list.

## Flow

Spawn the built binary with args in a temp dir → parse stdout/files → assert content (map INI loadable, screenshot PNG, validate diagnostics) → clean up.

## Integration

Pins the CLI contract of `cnc-map-editor/src/main.rs` (headless + validate branches relying on `generator.rs`, `MapModel::from/to_ini`, `validate()`), i.e. the interface future team members depend on when editing generator or model code.
