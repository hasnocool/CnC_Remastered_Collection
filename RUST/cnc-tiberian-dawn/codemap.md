# RUST/cnc-tiberian-dawn/

## Responsibility

Tiberian Dawn game logic crate: full game loop, map/house/unit/building entities, ported weapon/warhead tables from CONST.CPP/BBDATA.CPP/IDATA/UDATA, mission+trigger system, cell A* pathfinding, deterministic AI production planning driven by Rules.ini, network lockstep command queue with handshake/migration/relay/replay, and versioned binary savegames (BUILD_STATUS: **299 unit + 4 integration tests**, measured with `cargo test -p cnc-tiberian-dawn --no-fail-fast` — 299 passed / 1 failed lib, 1 `campaign_missions` + 3 `live_session` integration, 0 doc-tests; game logic + rules + lockstep + netcode + missions + savegames + sidebar/AI). *Test-count claim corrected 2026-10-20 (was “154 unit + 4 integration”); re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §C.* The one failing unit test (`scenario_loader::tests::records_become_executable_triggers`) is pre-existing baseline debt, not a documentation figure.

## Design

- Same shape as `cnc-red-alert` (independent entity structs orchestrated by `game.rs`, 4154 L) but TD-flavoured: `Game` owns map, houses, units, buildings, advances the demo simulation and renders through `cnc_renderer` (`game.rs` docs); `SidebarOption`/`Buildable` power the launcher sidebar.
- Rules port (`td_rules.rs`, 1294 L): `ArmorType`, `StructureType`, `UnitType`, `WarheadType`, `WeaponType` with values cited per source file (`CONST.CPP` Weapons/Warheads, `BBDATA.CPP` bullet types, `IDATA.CPP`/`UDATA.CPP` infantry/unit stats).
- Mission system (`mission.rs`, 1111 L): `Mission`, `Objective`, `Condition`, `Action`, `Comparison`, `Script`, `Trigger`, `MissionOutcome` — a compact declarative trigger dialect documented in the module header, plus INI scenario loading.
- Network stack (`network.rs`, 2872 L, largest) is layered: `Command`/`NetCommand`/`CommandQueue` → `LockstepSession`/`LockstepError` → UDP `UdpTransport`/`Datagram`/`NetPacket` → `HandshakeMessage`/`PeerHandshake`/`PeerState` → `HostMigration`/`MigrateMessage`/`MigrationState` → `MeshRelay` → `LiveSession`/`LiveEvent`/`NetworkConfig` and `Replay` recording/playback.
- Rules-registry integration: AI selection is deterministic via caller-supplied roll (`ai.rs` docs), reading costs/TechLevel/prereqs from `cnc_core::rules::RulesDatabase` with hardcoded TD tables as fallback.

## Flow

Scenario/mission INI or launcher session → `Game::new` → per tick: `CommandQueue` commands (local or lockstep-relayed), AI production planning, unit/weapon updates, `pathfind` routes on `CellGrid`, trigger evaluation → win/loss (`MissionOutcome`); savegames serialize a `GameSnapshot` (`SAVE_MAGIC`/`SAVE_VERSION` header) and `Replay` records command streams for reload.

## Integration

- Depends on core/renderer/audio/asset; consumed by `cnc-launcher/src/main.rs` (TD game mode, `Buildable` sidebar, `draw_td_briefing`) and the editor's `TiberianDawnPlugin`; `assets/missions/gdi_01.ini` and `../maps/` campaigns are pinned loadable by `tests/campaign_missions.rs`.
- 12 source modules (see src codemap) integrated via the `Game` struct; no engine crate depends on TD (launcher-only consumer).
