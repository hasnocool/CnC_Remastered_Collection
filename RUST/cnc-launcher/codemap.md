# RUST/cnc-launcher/

## Responsibility

The workspace's client application frame and largest crate: winit/wgpu window shell, main menu state machine, campaign/local/online game select, sidebar+production UI, network lobby/relay (`cnc-net-lobby` bin), screenshot capture (`cnc-screenshots` bin) and a broad in-game command set (see BUILD_STATUS: 58 tests; menu + campaigns + lobby + migration + production + attacks/sell/deploy + rejoin + minimap/camera/save/replay/chat/control-groups wiring).

## Design

- Three binaries + a small library (`lib.rs`: `pub mod lobby; menu; ready; types;`):
  - `src/main.rs` (10 146 lines) — window, event loop (`RedrawRequested` ≈ 60 FPS), `create_renderer` for `RendererKind::Wgpu|Software`, `Args`/`GameKind` selection, per-engine gameplay (TD sidebar via `Buildable`, RA via `cnc_red_alert::game::GameCommand`), tileset-driven terrain drawing helpers (`TilesetPalette`, `PackTerrain`, `draw_pack_tile_family`, `ra_terrain_tile`), save/replay paths (`ra_save_path_for(slot)`, `default_network_replay_path`), briefing screens (`draw_td_briefing`).
  - `src/menu.rs` (688 L) — pure state machine, deliberately window/renderer-free so it is unit-testable: `MainMenu`, `Screen` enum, `MenuCommand` requests.
  - `src/ready.rs` (337 L) — CNCREADY 1 out-of-band ready-up protocol for the host/join lobby (`parse_ready_line`/`format_ready_line`, `ReadyTracker` roster over `lobby::LobbyState` with an `all_ready` start gate), deliberately beside the lockstep stream like chat; poll-loop receive/send hooks are not yet wired in `main.rs`.
  - `src/bin/net_lobby.rs` (694 L) — standalone UDP harness driving `cnc_tiberian_dawn::network::LiveSession`: per-frame lockstep heartbeats, duplicate-dropping relay, host-migration elections; window is a thin status view.
  - `src/bin/screenshots.rs` — headless menu screenshot capture.
- Configuration: `clap` `Args`, `config`, `dotenv`, `tracing/tracing-subscriber` logging; renderer abstraction stays `cnc_renderer::Renderer`-based so wgpu and software paths share gameplay code.

## Flow

winit events → win event loop → `MainMenu` screens → start campaign/skirmish/online → instantiate `cnc_tiberian_dawn::game::Game` or `cnc_red_alert::game::Game` → per-frame simulation + `Renderer` draw + `cnc_audio` playback; network games run through `LiveSession`/`CommandQueue` (UTC UDP peer traffic relayed by the net-lobby binary or in-process).

## Integration

- Depends on workspace crates: core, both engines, renderer, audio, asset. Notably it does **not** depend on `cnc-net-directory` (lobby browsing/matchmaking is a separate service not wired into launcher code yet — evidence: launcher Cargo.toml deps list).
- Consumer of assets (`assets/atlas.png`, tileset packs) and mission/scenario INIs under `assets/missions`/campaign listing; integration test `tests/cli_campaign.rs` pins the campaign-CLI contract.
