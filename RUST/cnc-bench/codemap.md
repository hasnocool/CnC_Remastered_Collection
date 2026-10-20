# RUST/cnc-bench/

## Responsibility

Criterion benchmark crate for the C&C Rust port. It measures real engine paths
from `cnc-tiberian-dawn` and `cnc-red-alert` (and shared types from
`cnc-core`) — simulation throughput, cold-start boots and pathfinding — with
fully deterministic setup. It exposes no metric functions of its own: all
measurement lives in the criterion bench targets, and the library only holds
the shared fixtures they use.

## Structure

- `Cargo.toml` (`cnc-bench`, 0.1.0) — member of the 14-crate workspace
  (`RUST/Cargo.toml`). Uses `criterion = { workspace = true }` (workspace pin
  `0.5.1`, already in `Cargo.lock`) as a `[dev-dependencies]`, and declares two
  `[[bench]]` targets with `harness = false` (criterion provides its own main
  via `criterion_main!`).
- `src/lib.rs` — shared deterministic bench fixtures only:
  `bench_scenario_ini()` (editor-dialect RA scenario text), `bench_scenario()`
  (parsed via `Scenario::from_ini`), `frame_delta()` (exact 1/60 s in
  `Fixed16_16` via `from_ratio` — `from_f32` is documented as
  non-deterministic in `cnc-core`), plus the frame/command-interval constants.
  Two tests validate the fixtures themselves.
- `benches/sim_throughput.rs` — group `sim_throughput/td` (`fixed_dt_frames`):
  `TdGame::new` + N frames of `Game::update` at 60 FPS dt; group
  `sim_throughput/ra` (`scenario_frames_with_commands`):
  `RaGame::from_scenario` + frame loop that also applies
  `GameCommand::Move` at a fixed interval. Setup runs inside
  `iter_batched` so the measured routine is only the frame loop;
  throughput is reported in frames per iteration.
- `benches/cold_start.rs` — group `cold_start` with four benches:
  `ra_scenario_from_ini` (INI parse), `ra_game_from_scenario` (RA boot),
  `td_game_new` (TD boot), `td_find_path` (A* on a generated 64x64 `CellGrid`
  with a deterministic wall).

## Integration

- Depends on: `cnc-core` (`Fixed16_16`, `Vec2`, `Vec2i`),
  `cnc-tiberian-dawn` (`game::Game`, `map::CellGrid`, `pathfind::find_path`),
  `cnc-red-alert` (`game::Game` + `GameCommand`, `scenario::Scenario`), and
  `criterion` as a dev-dependency.
- Consumed by: nothing yet; run with `cargo bench -p cnc-bench`
  (`-- --test` for the one-pass smoke run used by CI-style checks).

## Notes / drift checklist

- Engine APIs exercised (re-verified at implementation time): TD `Game::new`
  & `Game::update` (`game.rs`), TD `find_path` (`pathfind.rs`), RA
  `Game::from_scenario` / `update` / `apply_command` (`game.rs`), RA
  `Scenario::from_ini` (`scenario.rs`). These crates are edited intermittently
  by other sessions — if a signature moves, adapt the benches; the fixtures in
  `src/lib.rs` are the single place to update.
- The 0.1.0 crate version is pinned locally (no `workspace = true`
  inheritance). The next release that includes the removed stub API should be
  cut as `0.2.0` (see CHANGELOG "Removed"); until then the crate is
  pre-consumer.
