# RUST/cnc-launcher/src/

## Responsibility

Crate sources: the giant window/game-shell `main.rs`, the testable menu state machine `menu.rs`, the out-of-band ready-up protocol `ready.rs`, the persisted settings module `options.rs` plus `lobby.rs`/`types.rs`, and two side binaries `bin/net_lobby.rs` and `bin/screenshots.rs`.

## Design

- `main.rs` (10 146 L) — flat module organically grown rather than sub-moduled: definitions `struct Args`, `enum GameKind` (TD/RA), `enum RendererKind`, `enum BuildAction`, `struct Sidebar`/`SidebarEntry`/`SidebarHit`, draw helpers `build_label(Buildable)`, `draw_td_briefing(&mut dyn Renderer, &Mission, ...)`, tileset family picker `water_shore_family(neighbours)` and per-engine handlers; paths helpers (`default_campaign_progress_path`, `default_td_progress_path`, `ra_save_path_for(slot: usize)`, `default_network_replay_path`). Drives both engines' public command APIs and persisting save/replay/progress/health-bar, minimap, chat, control-group and game-speed features listed in BUILD_STATUS.
- `menu.rs` (688 L) — `MainMenu` state machine with `Screen` navigation and `MenuCommand` outputs; no window/renderer ownership by design (pure simulation of UI transitions), unit-tested.
- `ready.rs` (337 L) — CNCREADY 1 ready-up protocol, deliberately out-of-band like chat: `parse_ready_line`/`format_ready_line` handle `CNCREADY 1 <player> <0|1>` datagrams (trailing newline tolerated, malformed input returns `None` so callers fall through to the lockstep handler instead of panicking). `ReadyTracker` keeps the roster in a `lobby::LobbyState` (`mark_ready`/`unregister`/`clear`/`roster`, name-sorted for deterministic lobby rows) and exposes the host start gate `set_expected`/`expected_count`/`all_ready`, which stays false while `expected == 0` so an empty lobby can never start. 17 unit tests. The module docs name the integration points (parse before `LiveSession::handle_datagram`, send per peer address via `format_ready_line`, draw from `ReadyTracker::roster`); the receive/send hooks inside the UDP poll loops are not wired yet.
- `bin/net_lobby.rs` (694 L) — wraps `cnc_tiberian_dawn::network::{LiveSession, CommandQueue, HostMigration, MeshRelay}` with a real UDP loop: heartbeat frames, relay w/ dedup, election logic; window shows status.
- `bin/screenshots.rs` (113 L) — captures launcher menu screenshots to PNG (used to validate UI changes).
- `lib.rs` — exposes `pub mod lobby; menu; ready; types;` so tests and other binaries share the state machine, the lobby peer/ready model and the ready-up protocol.

## Flow

`main.rs::main()` parses `Args` → builds window/renderer → menu loop (`MainMenu` screens) → chosen game kind spins up its engine loop; every redraw renders engine state through `cnc_renderer::Renderer` and samples input to issue engine commands. Network games: `bin/net_lobby.rs` relays `NetPacket`s between peers; host migration handled via `PeerHandshake`/`HostMigration` state machines.

## Integration

- Consumes all workspace graphics/audio/logic crates + `clap/config/dotenv/tracing` infra.
- Provides the external CLI surfaces tested by `tests/cli_campaign.rs` (RA campaign listing without a window).
