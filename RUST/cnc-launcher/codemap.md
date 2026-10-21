# RUST/cnc-launcher/

## Responsibility
Main launcher application for the C&C Remastered Collection Rust port. Provides game selection (TD/RA), skirmish/mission lobby, network matchmaking, options management, and rendering backend selection. Dual binary: library + multiple example binaries for testing.

## Design Patterns
- **Egui/eframe GUI**: Immediate-mode UI for menus, lobby, options, game setup
- **Renderer Abstraction**: `RendererKind` enum (Wgpu, Software, Headless) with dynamic backend selection
- **Options Persistence**: `options.rs` - TOML-based config with validation, migration, defaults
- **Network Lobby**: `lobby.rs`/`lobby_screen.rs` - matchmaking, peer handshake, host migration, mesh relay
- **Game Launch**: Spawns `cnc-tiberian-dawn` or `cnc-red-alert` game instances via `Game::new`
- **Event Integration**: `events_subscriber.rs` bridges `cnc-events` to launcher UI
- **Score Tracking**: `score.rs` - match results, stats, leaderboard sync
- **Ready State**: `ready.rs` - player ready/check logic for multiplayer
- **Binaries**: Multiple `[[bin]]` targets for integration testing (`net_lobby`, `screenshots`, `game_screenshots`, `game_view`)

## Data & Control Flow
1. **Startup**: `main.rs` → `LauncherApp::new()` → loads `Options` → initializes `RendererKind`
2. **Menu**: `menu.rs` - game select (TD/RA), mode (skirmish/mission/multiplayer), options
3. **Lobby**: `lobby_screen.rs` → `lobby.rs` - matchmaking, player list, ready state, map select
4. **Network**: `net_lobby` binary tests `LockstepSession`, `UdpTransport`, handshake, migration
5. **Game Launch**: Selected game crate `Game::new(scenario)` → runs simulation loop
6. **Options**: `options.rs` - persist/load TOML, validate `RendererKind`, apply at runtime
7. **Shutdown**: Graceful disconnect, save options, flush logs

## Public API
- **types**: `LauncherApp`, `RaLauncherApp`, `RendererKind`, `DEFAULT_GAME_*`, `GameMode`, `MapInfo`
- **menu**: `MenuScreen`, `MenuAction`
- **lobby**: `Lobby`, `LobbyState`, `PlayerInfo`, `LobbyEvent`
- **lobby_screen**: `LobbyScreen`
- **ready**: `ReadyState`, `ReadyEvent`
- **options**: `Options`, `OptionsError`, `load_options`, `save_options`
- **events_subscriber**: `EventsSubscriber`
- **score**: `ScoreTracker`, `MatchResult`
- **Binaries**: `net_lobby`, `screenshots`, `game_screenshots`, `game_view`

## Integration Points
- **Dependencies**: cnc-core, cnc-tiberian-dawn, cnc-red-alert, cnc-renderer, cnc-events, cnc-logs, cnc-console, eframe, egui, winit, tokio, serde, toml, anyhow, thiserror, clap, config, dotenv, ctrlc, parking_lot, once_cell, chrono, uuid, crossbeam-channel
- **Consumes**: Both game engines, renderer, events, logs
- **Network**: `cnc-net-directory` for matchmaking (planned)
- **Config**: `Launcher.toml` in config dir

## Key Files
- src/main.rs - Application entry, eframe bootstrap, LauncherApp
- src/lib.rs - Module declarations, re-exports
- src/menu.rs - Main menu, game/mode selection
- src/lobby.rs - Network lobby logic, matchmaking, handshake
- src/lobby_screen.rs - Lobby UI screen
- src/options.rs - Options persistence (TOML), validation, migration
- src/ready.rs - Player ready state management
- src/events_subscriber.rs - cnc-events bridge to UI
- src/score.rs - Match scoring, stats
- src/types.rs - Shared types (RendererKind, GameMode, MapInfo)
- src/bin/net_lobby.rs - Network lobby integration test
- src/bin/screenshots.rs - Screenshot test binary
- src/bin/game_screenshots.rs - In-game screenshot test
- src/bin/game_view.rs - Game view test binary

