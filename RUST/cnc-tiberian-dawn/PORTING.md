# Porting notes: Tiberian Dawn C++ to Rust

The Rust simulation deliberately follows the original engine in this
repository instead of inventing new values. This file maps what was ported and
where to find it.

## Constants

| C++ definition | Source | Rust |
|---|---|---|
| `ICON_LEPTON_W = 256` | DISPLAY.H | `td_rules::CELL_LEPTON_W` |
| `ICON_PIXEL_W = 24` | DISPLAY.H | `td_rules::ICON_PIXEL_W` |
| `PIXEL_LEPTON_W = 256 / 24 = 10` | DISPLAY.H | `td_rules::PIXEL_LEPTON_W` |
| `TICKS_PER_SECOND = 15` | DEFINES.H | `td_rules::TICKS_PER_SECOND` |
| MPH values (8, 18, 30, ...) | DEFINES.H | numbers in `td_rules::UnitStats` |

## Game logic

| C++ | Source | Rust |
|---|---|---|
| `Weapons[WEAPON_COUNT]` | CONST.CPP | `td_rules::WeaponType::stats` |
| `Warheads[WARHEAD_COUNT]` | CONST.CPP | `td_rules::WarheadType::{modifiers, spread_factor}` |
| `BulletTypeClass::Pointers` | BBDATA.CPP | `td_rules::BulletType` (speed, warhead, invisible) |
| `UnitTypeClass` / `InfantryTypeClass` stats | UDATA.CPP / IDATA.CPP | `td_rules::UnitType::stats` |
| `Fixed_To_Cardinal` | COORDA.ASM | `td_rules::fixed_to_cardinal` |
| `Modify_Damage` | COMBAT.CPP | `td_rules::modify_damage` |
| `Explosion_Damage` radius | COMBAT.CPP | `td_rules::EXPLOSION_RANGE_LEPTONS` + `Game::update_projectiles` splash loop |
| `TechnoClass::Rearm_Delay` | TECHNO.CPP | `WeaponStats::cooldown_seconds` and `first_shot_seconds` |
| `TurretClass::AI` / `PrimaryFacing` rotation | TURRET.CPP / TECHNO.CPP | `Game::update_facings`, `Unit::rotate_toward` / `aiming_at` |
| `TechnoClass::Rearm_Delay` two-shot alternation | TECHNO.CPP | `Unit::next_rearm_delay` / `note_shot_fired` |
| `TechnoClass::Fire_At` (projectile spawn, rearm) | TECHNO.CPP | `Game::update_attacks` |
| `FootClass::Approach_Target` | FOOT.CPP | `Game::update_attacks` out-of-range branch (stops 0x00B7 leptons short) |
| `FootClass::Mission_Guard` / `Mission_Hunt` target scan | FOOT.CPP | `Game::update_auto_acquire` (THREAT_RANGE for human, weapon range for AI) |
| `TechnoClass::Threat_Range` (twice weapon range, clamped 0x0A00) | TECHNO.CPP | `Game::update_auto_acquire` |
| `MapClass::Nearby_Location` | MAP.CPP | `Game::nearby_cell` (expanding box edge scan, 1 cell spacing) |
| `BulletClass::AI` movement / impact | BULLET.CPP | `weapon::Projectile::update` (light-speed invisible bullets snap to target) |
| `BulletClass` homing / arcing / fuel flags | BULLET.CPP / BBDATA.CPP | `BulletType::{is_homing, is_arcing, is_fueled}`, `Projectile` altitude/ticks |
| Homing re-aim every other frame | BULLET.CPP | `Game::update_projectiles` retarget on even ages |
| `DriveClass / FlyClass` speed accumulation | DRIVE.CPP / FLY.CPP | `td_rules::mph_to_px_per_sec_vehicle` (mph * 15 / 10) |
| `InfantryClass` movement | INFANTRY.CPP | `td_rules::mph_to_px_per_sec_infantry` (mph leptons/tick) |
| `FootClass::Find_Path` | FINDPATH.CPP | `pathfind::find_path` (8-direction A*) |
| `FootClass::Can_Enter_Cell` / `CellClass::Is_Clear_To_Move` | FOOT.CPP / CELL.CPP | `CellGrid::is_passable` + `Game::occupied_cells` |
| `DriveClass::While_Moving` cell stepping | DRIVE.CPP | `Unit::advance_order` waypoint following |
| `BuildingTypeClass` stats | BDATA.CPP | `td_rules::StructureType::stats` |
| `HouseClass::Power_Fraction` | HOUSE.CPP | `td_rules::power_fraction`, `Game::refresh_house_power` |
| `TechnoTypeClass::Time_To_Build` | TECHNO.CPP | `td_rules::production_seconds` + `Game::queue_unit` |
| House production queue | HOUSE.CPP | `House::update_production` / `Game::update_production` |
| `MapClass::Logic` tiberium growth/spread | MAP.CPP | `CellGrid::tiberium_cycle` |
| `UnitClass::Harvesting` / `Mission_Harvest` | UNIT.CPP | `Game::update_harvesters` |
| `CellClass::Reduce_Tiberium` | CELL.CPP | `CellGrid::reduce_tiberium` |
| `DriveClass::Offload_Tiberium_Bail` | DRIVE.CPP | 25 credits per step in `Game::update_harvesters` |
| `BuildingClass::Mission_Harvest` dock/unload | BUILDING.CPP | refinery proximity unload in `Game::update_harvesters` |
| Harvester carry capacity (28 steps x 25) | TYPE.H | `td_rules::HARVESTER_STEP_COUNT` / `HARVESTER_STEP_CREDITS` |
| `BuildingTypeClass::Legal_Placement` | BDATA.CPP | `Game::can_place_building` |
| Construction Yard structure production | BUILDING.CPP / HOUSE.CPP | `Game::queue_building`, `place_ready_building` |
| `HouseClass::Available_Money` / `Spend_Money` | HOUSE.CPP | `House::available_money` / `spend_credits` (tiberium first) |
| `HouseClass::Harvested` / `Adjust_Capacity` | HOUSE.CPP | `House::harvested` / `adjust_capacity` |
| Secondary weapon selection | UDATA.CPP / TARCOM.CPP | `UnitType::secondary_weapon`, `Game::update_attacks` fallback |
| Ion Cannon / Temple structures | BDATA.CPP | `StructureType::{IonCannon, TempleOfNod}` |
| `IonCannon` / `NukeStrike` timers | HOUSE.CPP / DEFINES.H | `House::charge_superweapons` (600 s / 840 s) |
| `Special_Weapon_AI` / `Place_Special_Blast` | HOUSE.CPP | `Game::fire_ion_cannon` / `fire_nuke` |
| Ion / atom blast damage | ANIM.CPP | `Game::apply_blast` (600 PB / 1000 Fire) |
| Dynamic cell reservation | DRIVE.CPP / FOOT.CPP | `Unit::reserved_cell`, `Game::occupied_cells` |
| `AircraftTypeClass` stats (Orca/Apache) | AADATA.CPP | `td_rules::UnitType::{Orca, Apache}` |
| Helicopter flight, ammo and helipad rearm | AIRCRAFT.CPP | `Unit::is_aircraft`, `Game::update_aircraft` |
| Helipad factory (produces aircraft) | BDATA.CPP | `StructureType::Helipad`, `ProductionKind::Aircraft` |
| Refinery/Silo storage capacity | BDATA.CPP | `StructureStats::capacity`, `Game::refresh_house_power` |
| Commando / engineer `InfantryTypeClass` rows | IDATA.CPP (563-597 / 486-528) | `td_rules::UnitType::{Commando, Engineer}`, `Unit::{is_commando, is_engineer}` |
| Commando action rewrite (`ACTION_SABOTAGE`) | INFANTRY.CPP:2891-2901 | `Game::order_sabotage` |
| `Mission_Sabotage` planting (IsGoingToBlow / CountDown(20) / WhomToRepay / Scatter) | INFANTRY.CPP:717-735 | `Building::plant_c4`, `Game::update_special_missions` |
| Demolition timeout (`IsGoingToBlow && CountDown.Expired()` → 5000 WARHEAD_FIRE) | BUILDING.CPP:1229-1246 | `Building::detonate` / `Building::sabotage_damage` in `Game::update_special_missions` |
| `Mission_Capture` (`tech->Captured(House); Delete_This();`) | INFANTRY.CPP:669-698 | `Game::order_capture` / `Building::capture` / `Game::consume_unit` |
| `BuildingClass::Can_Capture` (`Class->IsCaptureable`) | BUILDING.CPP:3873-3885 | `StructureStats::is_captureable` + `Building::is_captureable` |
| `Building` demolition / capture flags | BUILDING.H:95-144 | `Building::{is_going_to_blow, count_down, whom_to_repay, is_captured}` |
| Lockstep `Command::Sabotage` apply arm | network.rs `Command::Sabotage` | `Game::apply_command` |

## Deliberate differences

- The Rust world is in pixels (24 px per cell) while the C++ coordinate space
  is in leptons (256 per cell); all conversions live in `td_rules`.
- Pathfinding is an octile A* over the same 8-direction cell grid; the C++
  Find_Path uses edge-following with overlap masks and track animation. Rust
  rejects corner-cutting, avoids occupied cells and re-reserves each unit's
  next waypoint cell every tick so group moves do not collapse onto one cell.
- `Nearby_Location` picks from up to ten clear cells using `Frame % count`;
  Rust deterministically takes the first free cell so tests are reproducible.
- Bullets now model homing re-aim, arcing height and fuelled flight; the arc
  is a deterministic sine curve rather than full lepton ballistics.
- All remaining logic items are ported: economy, production/placement,
  aircraft, secondary weapons, projectile flight, superweapons and dynamic
  cell reservation. Tiberian Dawn has no VesselClass (see the TIBERIANDAWN
  codemap), so naval is not applicable to this crate; RA naval would need
  REDALERT/VESSEL.CPP. Missile launch trails and the Ion Cannon beam are
  implemented, as are rotor/track animation, staggered per-cell blasts and
  richer blast visuals. Remaining work is art-asset integration (SHP/VXL
  sprites) and the Red Alert naval port.
- The build sidebar is a Rust-side UI in cnc-launcher; the original GlyphX
  client sidebar is not part of this source drop.
- Superweapon visuals are simple expanding quads; Nuke damage follows
  ANIM.CPP's 9x9 per-cell Explosion_Damage, while Ion Cannon applies its
  single centre blast.
- Harvester steps go through House tiberium storage (25 credits per step,
  capped by refinery/silo capacity) and spending consumes tiberium before
  cash, as in HouseClass::Spend_Money.
- Commando sabotage and engineer capture are ported as mission-level flows:
  the unit approaches with the ordinary move order and the charge is armed when
  it enters the building cell (`Game::update_special_missions`), which is the
  Per_Cell_Process check without the lepton-accurate centre crossing.
  `Special.IsScatter` is modelled as "the mission ends, the unit goes idle",
  because the Rust port has no Scatter flee movement; after detonation the
  building does not run the fire/burning animation chain, and `WhomToRepay` is
  stored for kill credit but the TD port has no kill-credit ledger to pay it
  out. Capturing keeps the crew/`IsCaptured` flag and moves the structure in
  the house registries, but does not replay the capture animation or the
  production reset.
- The launcher UI does not offer the two new special rows on the sidebar yet
  and has no click-to-order path for sabotage/capture; `Game::order_sabotage`,
  `Game::order_capture` and `Game::apply_command` are the headless entry points
  (network/replay/tests) until the UI is wired up.

## Presentation assets

The source drop does not include SHP/VXL art, so sprites are authored locally
and round-tripped as PNG + JSON. Two pipelines write the same atlas format
(`atlas.png` + `atlas.json` with frame rects keyed by ClassId Debug names):

- **AI art (preferred).** `RUST/scripts/generate_ai_assets.sh` calls
  `scripts/ai_sprites.py`, which drives the local image generator
  (`/mnt/aux/projects/image-generator`, SDXL) to render one flat-magenta
  concept image per class, then chroma-keys, despills, crops to the largest
  connected silhouette, quantizes and packs 64x64 cells into a 512x128 sheet.
  Raw 768x768 renders stay in `assets/ai_raw/` (re-running `generate` only
  fills in missing classes), each processed cell is also written to
  `assets/sprites/<ClassId>.png` for hand editing, and
  `assets/ai_preview.png` is a 2x contact sheet. Use `CNC_IMAGE_GENERATOR`
  to point at another checkout, and `generate` / `assemble` to run one half
  at a time.
- **Procedural fallback.** `RUST/scripts/generate_assets.sh [style] [out]` runs
  `cnc-asset-cli generate-atlas` with `neon` (default) or `classic` shapes.
- A checked-in copy of the AI output (atlas, manifest, 2x preview and the 16
  processed 64x64 frames) lives in `RUST/art/`, so the art survives a clean
  checkout without the 11 MB of raw SDXL renders in `assets/ai_raw/`.
  Point the game at it with `CNC_ASSETS=art`.
- cnc-launcher loads `$CNC_ASSETS` or `assets/` at startup and uploads the
  sheet through `Renderer::set_atlas`; units and buildings draw sprites via
  `Game::render_with_sprites` when the backend supports sprites (wgpu), and
  fall back to placeholder quads otherwise. When no atlas can be loaded, one
  is generated procedurally in memory.

## Verification

- `td_rules` unit tests compare weapon rows against CONST.CPP values, the
  damage formula against COMBAT.CPP, and the speed/lepton conversions against
  DISPLAY.H.
- `game` tests cover selection, formation placement, guard acquisition,
  explosion splash damage, wall detours, building-aware paths and turret
  firing gates.
- `unit` tests cover two-shot rearm alternation and turret rotation.
- `weapon` tests cover arcing altitude and fuelled fuse expiry.
- `pathfind` tests cover diagonal movement, wall detours, blocked cells and
  unreachable goals.
- `cnc-asset` tests cover atlas generation, frame coverage and PNG/JSON
  save/load round-trips.
- `game` tests cover sprite coverage and that rendering with an atlas emits
  unit/building sprites.
- `game` tests also cover house power totals, factory-gated production,
  tiberium-first spending, storage capacity and overflow, spawned units,
  tiberium growth/spread, harvesting and refinery unloading, construction yard
  placement validation, aircraft ammo/rearm/overflight, homing re-aim,
  primary/secondary weapon selection, Ion Cannon/Nuke charge and blasts, and
  dynamic cell reservation.
- `td_rules` tests pin the commando and engineer rows against IDATA.CPP, the
  20 s / 5000-point C4 constants against INFANTRY.CPP and BUILDING.CPP, and the
  BDATA.CPP `IsCaptureable` column for every structure row.
- `unit` tests pin the commando/engineer role flags and class ids.
- `building` tests pin the `IsGoingToBlow`/`CountDown`/`WhomToRepay` trio, the
  WARHEAD_FIRE detonation damage and the ownership transfer on capture.
- `game` tests cover the full commando sabotage flow (approach → plant C4 →
  countdown → 5000-point detonation), the engineer capture flow (owner transfer
  plus consumption), the BDATA captureable gate, `Command::Sabotage` round-trip
  through the network codec plus `Game::apply_command`, and that two identical
  games tick the same command stream to identical sabotage state.
