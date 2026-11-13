## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13

#### Added
- Real Criterion 0.5 benchmark suite (`criterion = { workspace = true }`, workspace pin 0.5.1, already in `Cargo.lock`) with two `[[bench]]` targets (`harness = false`), replacing the `fn main() {}` stubs (2026-10-19):
  - `benches/sim_throughput.rs`: `sim_throughput/td/fixed_dt_frames` (TD `Game::new` + N frames of `Game::update` at 60 FPS) and `sim_throughput/ra/scenario_frames_with_commands` (RA `Game::from_scenario` + frame loop with periodic `GameCommand::Move` via `Game::apply_command`); frames-per-iteration throughput reported, setup isolated via `iter_batched`.
  - `benches/cold_start.rs`: `cold_start/ra_scenario_from_ini` (editor-dialect INI parse), `cold_start/ra_game_from_scenario` (RA boot), `cold_start/td_game_new` (TD boot), `cold_start/td_find_path` (A* on a generated 64x64 `CellGrid`).
- `src/lib.rs` now provides the shared deterministic bench fixtures instead of fake metrics: `bench_scenario_ini()`, `bench_scenario()`, `frame_delta()` (exact 1/60 s via `Fixed16_16::from_ratio`, avoiding the `from_f32` path documented as non-deterministic in `cnc-core`), plus frame/command-interval constants.
- Fixture validity tests (`bench_scenario_fixture_parses`, `frame_delta_is_sixtieth_of_a_second`).

#### Changed
- `codemap.md` rewritten to describe the real benchmark crate (was stale scaffold text claiming a `_stub()` library and empty bench targets).

#### Removed
- Fake benchmark stubs and their tautological threshold tests: `sim_throughput` (240-iteration wrapping-add loop, no game call), `cold_start_seconds` (10k-iteration add loop, no game boot), `baseline_memory_bytes` (constant 4 MiB allocation). The criterion benches measure the real engine paths and fully supersede them; nothing in the workspace referenced `cnc_bench`.

- Enforced semantic versioning and documentation standards (2026-10-19)
- Phase 9.4 performance profiling benchmarks added (sim_throughput, cold_start_seconds, baseline_memory_bytes) — placeholder implementations only, superseded by the criterion suite above
