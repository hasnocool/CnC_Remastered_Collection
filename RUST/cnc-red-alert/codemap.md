# RUST/cnc-red-alert/

## Responsibility

Red Alert game-logic crate: the RA simulation data model and end-to-end skirmish loop — allied/soviet houses (5 countries), production queues, naval/air/ground units, Tesla defenses, superweapons (Chronosphere/Iron Curtain/GPS), MCV deployment, spy/thief infiltration, gap generators, A* pathfinding, scenario-INI loading and an `AiCommander` — shared fixed-point economy with TD but separate data model (lib.rs docs).

## Design

- Independent entity structs, not an inheritance tree (`Aircraft`, `Ship`, `Submarine`, `MCV`, `TeslaCoil`, `GapGenerator`, `Spy`, `Thief` etc.), orchestrated by `game.rs` (3617 L): `Game`, `GameCommand`, `GameUnit`/`GameStructure`, `UnitRole`, `WeaponSpec`, `TargetRef`, `Shot`, `BuildError`, `GameStatus`, `ScenarioOutcome`, `ObjectiveState`, `RunningTrigger`, `AiCommander`.
- Determinism-first effects: `superweapon.rs` docs — "Effects are deliberately data-oriented (positions and timers) so they are deterministic in lockstep play"; each weapon charges over ten minutes, consumes its charge, restarts.
- Scenario layer (`scenario.rs`): loads the Mobius-editor INI dialect ([Map], [PlayerStarts], [Terrain], [Overlay] with overlay id 3 = ore, [Objects], [Triggers]) into `Scenario`/`ScenarioObject`/`ScenarioTrigger`.
- Render bridge (`render.rs`, 25 L): `entity_asset`/`render_entity` adapters mapping entity structs to asset names rather than adding methods everywhere.

## Flow

Scenario INI → `Scenario::load` → `Game::new` → loop: `GameCommand`s (build/attack/sell/deploy/stop/repair/power/scatter/cancel per BUILD_STATUS) applied per tick → house production queues (`ProductionItem`), `pathfind::find_path(MoveClass)` for movement, Tesla/`nava`l/air/superweapon updates, win/loss evaluation → `ScenarioOutcome`.

## Integration

- Depends on `cnc-core` (fixpoint/`ClassId`/rules), `cnc-renderer` (Sprite/render trait), `cnc-audio`, `cnc-asset`.
- Consumed by `cnc-launcher/src/main.rs` (RA game mode drives `cnc_red_alert::GameCommand`); `cnc-map-editor`'s `RedAlertPlugin` supplies catalogs/theaters for RA scenario authoring.
- Authored RA campaign scenarios live in `../maps/` and are pinned loadable by `tests/campaign_scenarios.rs`; 63 unit + 1 campaign integration tests per BUILD_STATUS.
