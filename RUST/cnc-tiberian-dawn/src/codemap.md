# RUST/cnc-tiberian-dawn/src/

## Responsibility

Twelve flat modules: game core, rules, entities, mission system, netcode, savegames, AI; `lib.rs` re-exports the primary types.

## Design

- `game.rs` (4154 L) — `Game`, `GameStatus`, `SidebarOption`, `Buildable`, plus effect primitives (`Blast`, `Strike`, `Beam`); owns map+houses+units+buildings, simulation stepping, and rendering coordination via `cnc_renderer`.
- `network.rs` (2872 L) — full lockstep stack: `CommandQueue` (frame command buffering), `LockstepSession` + `LockstepError`, `UdpTransport`/`Datagram`/`NetPacket` (+ `PacketError`), handshake set (`HandshakeMessage`/`PeerHandshake`/`PeerState`/`PeerEvent`/`HandshakeStep` + `HandshakeError`/`JoinReject`), host migration (`HostMigration`/`MigrateMessage`/`MigrationState`/`MigrationError`), `MeshRelay` relay, `LiveSession`/`LiveEvent` net UI harness, `NetworkConfig` validation, and `Replay` (+ `ReplayError`) record/playback.
- `td_rules.rs` (1294 L) — ported tables: `WeaponType`, `WarheadType`, `ArmorType`, `UnitType`, `StructureType` (cited to CONST.CPP/BBDATA/IDATA/UDATA).
- `mission.rs` (1111 L) — declarative triggers: `Trigger`, `Condition`, `Action`, `Comparison`, `Objective`, `Script`, `Mission`, `MissionOutcome`; loads TD scenario INIs (editor-dialect compatible).
- `map.rs` (552 L) — `Map`, `CellGrid` terrain/overlay/occupancy cell grid.
- `unit.rs` (509 L) — `Unit`, `Infantry`, `Vehicle`, `Aircraft` entity types.
- `weapon.rs` (365 L) — `Weapon`, `Projectile`, `Warhead` conversions from rules tables into fixed-point sim values.
- `building.rs` (181 L) — `Building`, `Factory`, `PowerPlant`, `Refinery` structures.
- `house.rs` (185 L) — `House` credits/power/production state.
- `ai.rs` (372 L) — `AI` production planner: `AIDifficulty`, `ProductionWeight`; Rules.ini-driven costs/TechLevel/prereqs with hardcoded fallback; deterministic via caller-supplied roll.
- `pathfind.rs` (241 L) — A* over `CellGrid` (per docs: contract reproduction of `FootClass::Find_Path` from FINDPATH.CPP, 8-direction edge following with movement costs/diagonal handling).
- `savegame.rs` (306 L) — `GameSnapshot`, `SAVE_MAGIC`, `SAVE_VERSION` — versioned binary save format (entities referenced as indices, so no `Code_Pointers` dance needed).

## Flow

Per frame: lockstep `CommandQueue` feeds `Game::tick` → AI planner emits production commands → entity updates (units/buildings/weapons pathfind on `Map`) → triggers evaluate → render via engine-visible entity list. Cold loads: mission INI → `Game`; hot loads: `GameSnapshot` savegames or `Replay` streams.

## Integration

- Consumes core/renderer/audio/asset crates; consumed by `cnc-launcher` (window gameplay) and the editor's plugin catalog for TD authors (agent scripts target its entities through `cnc-ai` runtime commands).
- `tests/live_session.rs` drives the relay/migration network state machines; `tests/campaign_missions.rs` pins mission-file loadability.
