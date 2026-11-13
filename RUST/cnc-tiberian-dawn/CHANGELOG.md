## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- `Game::enable_enemy_ai(difficulty)`: one-call expert-AI enablement for every non-player house (skirmish/quickmatch); campaign scenarios stay trigger-driven unless invoked
- Savegames from earlier builds now fail with an explicit "incompatible — start a new mission" message (format version 2)
- RA `Expert_AI` port: difficulty-cadenced urgency ladder in `ai.rs` (`run_expert_ai`) — fire sale, power-plant rebuild, refinery/harvester income, weighted offense production, and attack waves targeting the nearest enemy unit (siege-move fallback onto buildings); `Game::update` drives per-house AI state (`Game::ai`, gated by `House::ai_enabled`), persisted in the snapshot
- Crate powerups (`CRATE.CPP` port): new `powerup` module with deterministic SplitMix64-seeded spawns on free land cells, drive-over collection (`Credits`/`Unit`/`Armor` rolls), 90 s lifetimes, and a persisted `PowerupState` so saves/replays reproduce the same sequence; `Unit::shield` halves incoming damage while active
- Campaign carry-over core (`CARRY.CPP` port): `carryover::Carryover` captures surviving units + credits at mission end (`Carryover::from_game`), sets the next mission's treasury (capped) and redeploys the survivors, with checksummed `CNCCARRY` file persistence
- `GameSnapshot` carries `powerups`; `SAVE_VERSION` 1 → 2 (older saves are rejected with a clear message)
- 11 new tests covering spawn determinism, pickup effects, crate expiry, carry-over round-trips and corruption rejection
- `UnitType::Commando` and `UnitType::Engineer` infantry with `is_commando`/`is_engineer` role flags (mirrors TIBERIANDAWN/IDATA.CPP)
- Building sabotage state (`is_going_to_blow`/`count_down`/`whom_to_repay`/`is_captured`); `plant_c4` starts a 20-second countdown to a 5000-point `WARHEAD_FIRE` detonation (mirrors BUILDING.CPP)
- Engineer building capture that consumes the infantry and transfers ownership; per-structure `StructureStats::is_captureable` BDATA flag
- Game-side `apply_sabotage` arm for the existing `Command::Sabotage` wire variant (no protocol change)
- 17 new tests covering sabotage/capture rules, IDATA/BDATA rows and network determinism
- Optional `briefing_image` mission key (`[Basic] BriefingImage`, with `BriefPicture` as an accepted alias) parsed as `Option<String>` with a serde default; missions written before the key existed parse to `None` (2 tests)
