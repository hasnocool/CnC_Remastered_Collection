# RUST/cnc-classic/

## Responsibility
Strict classic mode binary for C&C Remastered Collection. Enforces deterministic lockstep simulation with original Westwood rules preserved, ABI-safe networking. Minimal binary - prints version and exits (GUI/launcher integration pending).

## Design Patterns
- Version Embedding: classic_version() from Cargo.toml via env! macro
- ABI Stability: Public API surface frozen for network compatibility
- Rules Preservation: Simulation logic delegates to cnc-tiberian-dawn/cnc-red-alert with classic rule flags
- Single Binary: [[bin]] entry point, no library surface (currently)

## Data & Control Flow
1. Launch: main() prints version string with crate version
2. Future: Will initialize classic-mode scenario, connect to lobby, run lockstep loop
3. Rule Flags: Classic mode toggles (e.g., no quality-of-life changes, original balance, original pathfinding)

## Public API
- Binary: cnc-classic - prints version, exits
- Library (planned): classic_version() -> &str, ClassicConfig, ClassicGame::new()

## Integration Points
- Dependencies: egui, egui-wgpu, egui-winit, winit, pixels, anyhow, thiserror, serde
- Consumes: cnc-tiberian-dawn, cnc-red-alert (simulation), cnc-core (fixed math), cnc-renderer (rendering)
- Network: Intended for cnc-net-directory matchmaking

## Key Files
- src/main.rs - Binary entry point, version print
- Cargo.toml - Bin configuration, GUI dependencies (egui/wgpu/winit)

