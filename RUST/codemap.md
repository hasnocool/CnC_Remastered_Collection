# RUST/

## Responsibility

Rust workspace reimplementing the C&C Remastered Collection engines as a modern, cross-platform client: deterministic game simulation (Tiberian Dawn + Red Alert), asset pipeline, renderer, audio, map editor, launcher and a decentralised network directory. Workspace root (`Cargo.toml`, resolver "2") defines fifteen member crates and the shared dependency versions every member pins to via `[workspace.dependencies]`.

## Design

- Cargo workspace, edition 2021, version 0.1.1, GPL-3.0-or-later. Members (dependency order): `cnc-core` → `cnc-renderer`, `cnc-audio`, `cnc-asset` → `cnc-tiberian-dawn`, `cnc-red-alert`, `cnc-ai` → `cnc-map-editor` → `cnc-launcher`; plus the standalone `cnc-net-directory`. Total: 15 member crates.
- All external deps are declared once in `[workspace.dependencies]` and inherited with `{ workspace = true }`: `wgpu 22`, `winit 0.30`, `rodio 0.19`, `serde/serde_json`, `thiserror/anyhow`, `bincode`, `flate2/lzw`, `nom`, `clap`, `tokio`, `image`, `symphonia/vorbis_rs/hound`, etc. Libraries error via typed `thiserror` types; binaries use `anyhow`.
- Per-crate standard layout: `src/` modules, `[[bin]]` targets where applicable (`cnc-asset-cli`, `cnc-launcher`, `cnc-net-lobby`, `cnc-screenshots`, `cnc-directory`), integration tests in `tests/`.
- `BUILD_STATUS.md` is the verification ledger (last verified: cargo 1.98.1; `cargo check --workspace --all-targets` clean, 952 tests passed). Non-member leftovers at root: `src/` (hello-world stub, no `[[package]]`), `take_screenshots.rs` (standalone reference source), plus data dirs `assets/`, `maps/`, `art/`, `docs/`, `scripts/`.

## Flow

Build targets: `cargo check --workspace --all-targets`, `cargo test --workspace` (launchers/directory include headless and doc tests). Runtime entry point is `cnc-launcher`: it creates the winit window, drives wgpu rendering, loads assets through `cnc-asset`, simulates via `cnc-tiberian-dawn` or `cnc-red-alert`, plays audio through `cnc-audio`. Online play leaves the workspace mesh to `cnc-tiberian-dawn`'s UDP lockstep + relay and, separately, to `cnc-net-directory` nodes for lobby browsing/matchmaking.

## Integration

- Internal dependency edges (from each crate's `Cargo.toml [dependencies]`): `cnc-core` has none (leaf). `cnc-renderer`, `cnc-audio`, `cnc-asset` depend only on `cnc-core`. `cnc-tiberian-dawn` and `cnc-red-alert` depend on core+renderer+audio+asset. `cnc-map-editor` depends on core, renderer, asset and `cnc-ai`. `cnc-launcher` depends on core, renderer, audio, asset, both engines. `cnc-net-directory` and `cnc-ai` are independent of the game crates (net-directory is workspace-level self-contained; `cnc-ai` is consumed by `cnc-map-editor`).
- Upstream context lives one level up: the C++ sources under `REDALERT/` and `TIBERIANDAWN/` are the behaviour reference; `assets/` and `maps/` hold generated runtime data consumed by the launcher/editor.
