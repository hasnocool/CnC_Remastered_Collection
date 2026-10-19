# Build Status

_Last verified: cargo 1.98.1 / rustc 1.98.1 on Linux._

## Verification

~~~
cargo check --workspace --all-targets   # clean (warnings only)
cargo test --workspace                  # 552 passed, 0 failed
~~~

Per-target tests:

| Target | Tests |
|---|---|
| cnc-core (unit + INI + Rules.ini) | 57 |
| cnc-asset (parsers + converters + Ogg) | 26 |
| cnc-asset-cli (end-to-end integration) | 8 |
| cnc-tiberian-dawn (game logic + rules + lockstep + netcode + missions + savegames + sidebar/AI) | 154 unit + 4 integration |
| cnc-red-alert (houses, tesla, gap, MCV, naval, air, spies, superweapons, game loop, scenarios, network builds/production/attack/sell/deploy/stop/repair/power/scatter/cancel) | 63 unit + 1 campaign integration |
| cnc-map-editor (model, generator, undo/redo, tools, plugins, app, headless + validate CLIs) | 80 unit + 2 integration |
| cnc-launcher (main menu + campaign/local/online game select + RA setup/campaign/scenario + network lobby/skirmish + LAN discovery + host-loss migration + production + attacks/sell/deploy + rejoin + TD mission select/finish/pause + pause save/load + RA/TD minimap click-to-center + setup preview + camera bookmarks + game speed + world zoom + campaign listing + health bars + edge panning + net chat + control groups + TD sell/repair + RA repair/power/scatter + save slots + TD completion + queue progress + replay save/playback + campaign advance + slot ages + net-lobby) | 58 |
| cnc-net-directory (registry, gossip mesh, discovery, torrent fallback, HTTP API/UI, CLI) | 47 unit + 5 integration + 1 doc |
| cnc-renderer (camera + sprite batch) | 3 |
| cnc-renderer (GPU smoke, ignored unless adapter present) | 2 |

## Phase 2: Asset Pipeline - COMPLETE

### Implemented Parsers
- **archive.rs**: MIX v1/v2 (.mix) - Zlib decompression, name tables, extract/extract_all
  - Payload offsets start after the name table; compressed-entry lengths are honoured.
- **shape.rs**: SHP (.shp) - Frame headers, RLE decoding, bounds, palette-to-RGBA
- **audio.rs**: VOC (.voc) - Creative Voice blocks 0x00-0x0A, AUD (.aud) - Westwood Audio
- **palette.rs**: PAL (.pal) - 768-byte RGB, gamma correction, grayscale, round-trip test

## Phase 3: Rendering - COMPLETE

- **sprite_batch.rs**: Instanced sprite batching (10K capacity), z-order + texture sorting
- **shaders/sprite_batch.wgsl**: Palette remapping (256 colors), alpha blending
- **camera.rs**: Orthographic camera, world<->screen transforms, zoom/pan
- **renderer.rs**: Renderer trait, WgpuRenderer (async), SoftwareRenderer fallback

## Phase 4: Asset Converter CLI - COMPLETE

convert.rs is the shared library behind the cnc-asset-cli binary; tests/cli.rs
exercises the binary itself.

| Command | Behaviour |
|---|---|
| mix -i X.MIX -o DIR [--list] | List or extract every MIX entry |
| shp -i X.SHP -o OUT [-p PAL] [--sprite-sheet] [--opaque] | Per-frame PNGs, or one sheet + JSON manifest |
| voc -i X.voc|X.aud -o OUT [--format wav|ogg] [--loop-start N --loop-end N] [-q Q] | WAV or Ogg Vorbis with loop points |
| pal -i X.PAL -o DIR [--format json|png|css] | Palette export |
| batch -i SRC -o OUT -g td|ra [-p PAL] [--sprite-sheet] [--audio-format wav|ogg] | Recursive conversion mirroring the tree |
| generate-atlas -o DIR [--style classic|neon] | Placeholder sprite atlas |

Details:
- SHP frames honour palette index 0 as transparent (disable with --opaque).
- Sprite sheets are packed horizontally and described by a JSON manifest.
- VOC loop blocks (0x09/0x0A) are preserved by default; CLI flags can override them.
- Ogg Vorbis (pure Rust binding to libvorbis) writes LOOPSTART/LOOPEND/LOOPLENGTH
  Vorbis comments; WAV output gets a RIFF smpl chunk with a forward loop.
- Batch conversion records per-file failures and exits non-zero if any asset fails.

## Phase 5: Game Logic - Shared Foundation (IN PROGRESS)

### 5.1 Command queue & lockstep - DONE for local/LAN transport
- Deterministic command ordering by (frame, player, hash), independent of UDP arrival.
- LockstepSession: validated scheduling, frame gating, ordered replay log, per-frame
  checksums, remote desync detection, replay reconstruction.
- Binary packet codec (magic CNCP, version 1) with defensive limits, trailing-byte
  checks and checksum validation.
- UdpTransport: explicit peer list, encoded multi-peer broadcast, validated receive,
  plus datagram dispatch that routes command and handshake frames on one socket.
- Replay persistence: versioned CNCR file (magic/version/player count/command records)
  with an FNV-1a trailer; `Replay::save/load` and `LockstepSession::save_replay/
  load_replay` reject corruption, truncation and unsupported versions.
- Peer join/leave handshake: CNCH `JoinRequest/JoinAccepted/JoinRejected/Leave`
  frames and a `PeerHandshake` state machine that validates slot numbers, room size,
  duplicate addresses and emits `PeerJoined/PeerLeft` events.
- Mesh relay: `MeshRelay` dedup cache plus `UdpTransport::relay_from` forwards a
  datagram to every peer except its source and drops repeated copies.
- Host migration: CNXC `HostLost`/`HostClaim` frames and a `HostMigration` state
  machine elect the lowest living player deterministically, reject stale/illegitimate
  claims and resynchronise peers that missed the loss frame.

### 5.2 Rules/INI registry - DONE (first pass)
- cnc-core/src/rules.rs parses Rules.ini into typed data: GeneralRules (RULES.CPP
  defaults), DifficultyRule, WeaponRule, WarheadRule, TypeRule, HouseRule and the
  Infantry/Units/Aircraft/Buildings categories.
- Unlisted sections are classified by their keys (Damage/ROF/Range/Warhead ->
  weapon, Verses/Spread -> warhead, Cost/Strength/Prerequisite -> type).
- INI fixes: decimal + 0x-hex integers, numeric/boolean disambiguation,
  case-insensitive lookup, multi-level inheritance, cycle detection.
- Costs, strength and production times use rules overrides.
- Weapons: Damage, ROF and Range overrides replace CONST.CPP values; Warhead=
  selects a warhead and Verses/Spread replace the armor-modifier table for both
  projectiles and explosion damage.
- Tech tree: Prerequisite= tokens are enforced for unit and building production,
  including GDI/NOD faction tokens and PYLE/HAND, HPAD/AFLD aliases; missing
  prerequisites are exposed through missing_prerequisites() for UI use.
- TechLevel gating: per-type build levels from Rules.ini `TechLevel=` (fallback:
  TDATA/UDATA/BDATA values) are checked against the house build level before
  production; houses default to level 98 (skirmish/multiplayer) and scenarios may
  lower it.
- Sidebar visibility: `Game::sidebar_options` / `visible_sidebar_options` expose
  rules cost, TechLevel, affordability, producer and unmet prerequisites; the
  launcher renders only visible rows and uses rules-derived costs.
- AI production weighting: `AI::production_weights` scores units from the rules
  registry (faction, TechLevel, prerequisites, cost, health-per-credit, difficulty)
  and `choose_weighted` makes a deterministic, lockstep-safe selection; the game
  exposes `ai_production_choice` and `ai_queue_production`.
- Launcher accepts --rules Rules.ini and applies the overrides to the demo game.

### 5.3 Campaign missions and triggers - DONE
- `cnc-tiberian-dawn/src/mission.rs`: objectives, triggers, scripts and a compact INI
  dialect (`Condition=`, `Actions=`, `Once=`, `[ScriptN]`).
- Level-triggered condition tree: elapsed time, unit/structure counts, HasStructure,
  credits, power, trigger-zone occupancy, flags, `&`/`|`/negation.
- Actions: win/lose, credits, spawn units, place structures, messages, flags,
  enable/disable triggers, complete objectives and run scripts.
- `Game::update_mission` evaluates triggers each tick and applies actions through the
  normal entity/house paths; `Game::start_mission`/`load_mission` apply player,
  TechLevel and starting-credit settings.
- Launcher `--mission FILE` starts a scenario; the HUD shows objectives, the latest
  mission message and the victory/defeat state. `assets/missions/gdi_01.ini` is a
  complete sample scenario.

### 5.4 Save/load of live game state - DONE
- `cnc-tiberian-dawn/src/savegame.rs` snapshots map, houses, units, buildings,
  projectiles, production counters, ready buildings, mission state (triggers, flags,
  objectives, log) and the loaded Rules.ini overrides.
- Versioned file header: `CNCSAVE\0` magic, u32 version, u64 payload length and FNV-1a
  64 checksum; corrupt, truncated and future-version saves are rejected.
- `Game::to_save_bytes`/`from_save_bytes` and `save_to_file`/`load_from_file`; the
  launcher binds F5 (quicksave to `$CNC_SAVE_DIR/quicksave.sav`) and F9 (quickload).
- Transient visuals (blasts, beams, strikes, delayed blasts) are intentionally not
  persisted; gameplay state and in-flight projectiles are.

### 5.5 Red Alert end-to-end - DONE
- Houses/factions: Allied/Soviet, five countries, country-unique structures, credits,
  power-gated production, queue/refund/cancel.
- Tesla Coil: charge/reload window, power gate, range check, fixed-point bolt damage.
- Gap Generator: powered shroud radius and drain.
- MCV: timed unpack/cancel/deploy into a Construction Yard, 2x2 clearance helper.
- Naval: surface movement, submarines (submerge/detect/torpedo/rearm).
- Aircraft/helipad/airfield: ammo, docking, rearm timer, capacities.
- Superweapons: 10-minute charge, Chronosphere teleport, Iron Curtain temporary
  invulnerability, GPS map reveal.
- Spy/Thief: refinery credit theft, radar reveal, power/production sabotage, capped
  thief carrying capacity.
- `map.rs`: deterministic RA skirmish grid with a north/south river, beaches and
  ore fields; mining, water count and summary helpers.
- `pathfind.rs`: 8-direction A* with ground/naval/air movement classes (ground
  needs land, naval needs water, aircraft ignore terrain and blockers).
- `game.rs`: `Game` with Allied/Soviet houses and credits, base setup, unit and
  structure spawning, role-based stats (infantry/vehicle/ship/submarine/aircraft/
  harvester), production queues that spawn at the matching factory, move/attack
  orders, target acquisition and reload-gated combat, Tesla Coil and AA fire,
  harvester ore income, Chronosphere teleport and Iron Curtain hardening, plus a
  renderer-agnostic `render` and `GameStatus`.
- Launcher mode: `cargo run -p cnc-launcher -- --game ra --renderer wgpu` opens an
  interactive RA skirmish (select/move, pan/zoom, queue units, Chronosphere) and
  `--frames N` exits after N presented frames. Verified for 120 frames on wgpu.
- Scenario files: editor-compatible `[Map]`/`[PlayerStarts]`/`[Terrain]`/
  `[Overlay]`/`[Objects]`/`[Triggers]` parser (terrain mapping, ore, class
  names or numeric ids) and `Game::from_scenario`; load with
  `--game ra --scenario FILE.ini`. Verified with a 24x20 scenario.
- Scenario triggers: `elapsed:N` conditions fire `win`/`lose`/`message:` actions
  into `Game::outcome` and `Game::messages`; the RA HUD shows the result.
- Skirmish AI: `AiCommander` queues affordable units and periodically orders idle
  troops toward the player's base; deterministic so it is testable.
- Skirmish setup screen: `--game ra` opens a pre-match screen for map size
  (48/64/96), Allied country and AI difficulty (Easy/Normal/Hard), then Enter
  starts; `--scenario` bypasses it. Verified by screenshot + OCR.
- Building placement: `Game::place_structure` validates bounds/occupancy, a live
  Construction Yard, and credits, then spends and spawns the structure. The RA
  client previews the cell (green/red) and places with keys P/B/W/T + left click.
- Campaign mode: `--campaign DIR` lists every scenario INI as a mission
  (mission-select screen), Enter deploys, and a player-relative victory advances
  to the next; finishing the list returns to select. Ten authored missions
  live in `maps/ra_campaign/` (Holding the Line, Counterstrike, Deep Strike,
  Final Strike, Red Tide, Endgame, Breakwater, Iron Coast, Frost Line, Red
  Horizon) with bases, attackers, ore, briefings/objectives and
  timed/battlefield triggers. Verified with a captured, OCR-checked
  mission-select screen.
- Generated-map missions extend the campaign: 07 Breakwater (Allies), 08 Iron
  Coast (Soviet), 09 Frost Line (Allies) and 10 Red Horizon (Soviet) were built
  from deterministic editor seeds (71/72/73/74, 28x28 temperate) and
  hand-authored with two bases, radar/command-post objectives and timed
  reinforcements. `authored_ra_campaign_scenarios_load` parses all ten files,
  checks terrain/start/objective/trigger invariants and constructs a `Game` for
  each; the CLI test asserts the ten-mission listing.
- Edge scrolling: when the cursor rests within 8 px of a window edge, the
  camera pans at 520 px/s divided by zoom (clamped to map bounds) in both game
  views; it pauses with the pause menu/briefing. `edge_pan_direction` maps the
  cursor to a direction, covered by `edge_pan_detects_edges_and_moves_camera`.
- Damage feedback: both game views draw health bars above damaged units and
  structures (green >50%, yellow >25%, red otherwise) using `health_fraction`
  and `health_bar_color`; full-health entities draw nothing.
  `health_bars_report_fraction_and_color` covers the math and colour steps.
- Control groups: **Alt+Shift+1..4** stores the current selection and
  **Alt+1..4** recalls it (RA stores unit ids, TD stores unit indices), with
  "Group N stored/recalled" notices. Empty slots are ignored.
  `control_groups_store_and_recall` covers store/recall and empty groups in
  both games.
- Game speed: **F6**/**F7** adjust the simulation rate between 0.25x and 3x in
  both single-player views (delta is scaled before `Game::update`), shown in the
  HUD and confirmed with a notice. Network games keep speed locked at 1x so
  lockstep is not disturbed; starting a skirmish resets it.
  `game_speed_adjusts_and_clamps` covers clamping, the network lock and TD.
- World zoom: **+**/**-** and the mouse wheel (or trackpad scroll) zoom both
  game views between 0.5x and 4x around the cursor, clamped to map bounds; a
  match starts at 2x so 24 px cells read closer to the original field of view
  instead of showing nearly the whole map. Status text, the build menu and the
  minimap are drawn with an unzoomed HUD camera so they keep a constant pixel
  size, and clicks/hovers map back through `hud_world`. The HUD and the pause
  overlay are submitted as their own layers via `Renderer::begin_layer`, which
  flushes the queued world geometry before the camera changes (the wgpu sprite
  batch shares one camera uniform, so a mid-frame camera swap alone would have
  re-transformed everything already queued).
  `zoom_defaults_clamp_and_pin_the_world_under_the_cursor` and
  `hud_world_maps_zoomed_clicks_to_the_same_screen_point` cover the limits,
  cursor anchoring and HUD mapping.
- Camera bookmarks (both game views): Ctrl+1..4 jumps to a stored view and
  Ctrl+Shift+1..4 stores the current camera; storing shows "View N saved" via
  the HUD notice line. `camera_bookmarks_save_and_jump` covers slot mapping,
  store/jump and empty slots in RA and TD.
- RA setup preview: the skirmish setup screen renders a 280 px terrain/ore
  preview of the selected map size (48/64/96) beside the options, rebuilt from
  the same deterministic `Map::new` used to start the match. Verified by a
  wgpu screenshot/OCR showing "MAP PREVIEW" and "Map size: 64 x 64".
  `ra_setup_preview_maps_are_playable` checks each size has a river and ore.
- RA minimap: the game view draws a bottom-right minimap (terrain, ore, own
  units/structures in blue, enemies in red and the camera viewport box) scaled
  to a 150 px panel with a step cap for large maps; the viewport box follows
  camera pan/zoom; clicking the minimap recenters the camera on that cell.
  `minimap_layout_scales_to_the_panel` and `minimap_clicks_center_the_camera`
  cover scaling and click-to-center for both games.
- Stop orders: **S** stops the selected units locally and broadcasts a `Stop`
  command; `stop_commands_clear_paths_and_targets` verifies the path/target are
  cleared and wrong owners are ignored.
- RA box selection: dragging with the left button over the world (after a
  minimap/placing check) draws a cyan marquee and selects every living unit of
  the player's house inside it; Shift adds to the selection. The camera shows
  the drag rectangle and the HUD hint reads "LMB select / drag box".
  `box_selection_only_takes_owned_units_inside` covers full-rect, tight-rect
  and enemy exclusion.
- Playable sides: `Game::player_owner` drives input, selection, placement and
  win/lose semantics, and flips the AI to the other house. The setup screen adds
  an Allies/Soviet side toggle with side-appropriate countries.
- Briefing/objective HUD: scenarios parse `Briefing` and `[Objectives]`; the
  mission-select screen shows the selected mission's side and briefing, and the
  in-game HUD lists objectives with completion marks (briefing only for the
  opening seconds). Triggers can complete an objective with `objective:N`.
- Campaign persistence: progress is read/written to
  `$CNC_SAVE_DIR/ra_campaign.ini` (default `saves/`), so a relaunch resumes at
  the saved mission; completing the last mission clears the file.
- Battlefield triggers: conditions now include `has:`/`!has:`,
  `structures:<owner>:<class>:<op>:<count>`, `units:...` and `capture:x,y:owner`,
  plus a `spawn:<class>,x,y[,owner]` action. Missions use waves, base-destruction
  objectives and command-post victory.
- Mission briefing overlay: scenarios with a briefing open a pre-match screen
  (mission name, side, briefing, objectives, Enter to begin) that pauses the
  simulation. Verified by captured/OCR-checked screenshots.
- Mid-mission save/load: `Game::to_save_string`/`from_save_string` persist map,
  terrain/ore, houses, units/structures (health/position), triggers, objectives,
  messages, superweapons, AI and elapsed time; `--game ra` binds F5/F9 to
  `$CNC_SAVE_DIR/ra_mission.sav`.
- Mission-select map preview: the campaign screen renders the selected mission's
  terrain and ore as a thumbnail alongside its side, briefing, map size,
  objective count and trigger count.
- Campaign completion screen: clearing the last mission shows a completion screen
  (missions cleared + final mission time) with Enter returning to mission select.
- Per-mission tracking + campaign difficulty: the progress file stores
  `Mission`, `Difficulty`, `Completed` indices and `BestTimes`; mission select
  marks cleared missions `[x]`, shows each best time and cycles Easy/Normal/Hard
  with D, which sets the AI tempo for every campaign mission.
- Replay confirmation: selecting an already-cleared mission asks for confirmation
  (Enter replays, Esc cancels); completion marks/best times persist after the
  completion screen so replays keep their records.
- Briefing art: a side-coloured diamond emblem with the mission number is drawn
  on the mission-select screen and the briefing overlay (blue for Allied, red
  for Soviet, gold trim).
- Mission-select filter + stats: F cycles All/Incomplete/Completed filtering
  (navigation follows the filtered list) and S opens a stats screen listing every
  mission's completion mark and best time with a total.
- Pause menu: Esc during play opens Resume / Restart mission / Abort to menu /
  Save game / Load game (Up/Down, Enter) with elapsed time, unit/structure
  counts and credits; the simulation pauses, restart reloads the active
  scenario (or starts a fresh skirmish), abort returns to the mission select or
  setup screen, and save/load write/read the same quicksave path as F5/F9.
  Save/load feedback ("Game saved", "Load failed: ...") appears in the HUD for
  four seconds; `pause_menu_saves_and_loads_the_game` covers a round trip.
- Result flow: after MISSION COMPLETE/FAILED the HUD says "Enter: continue".
  Enter advances the campaign on a victory, otherwise aborts to the mission
  select (campaign) or skirmish setup, so a finished skirmish no longer strands
  the player in a running match.
- Command routing: `GameCommand::Move` + `Game::apply_command` /
  `order_units_move` move exactly the listed units, and
  `GameCommand::Build` places a structure through `place_structure`, and
  `GameCommand::Produce` queues units via `queue_unit`, and
  `GameCommand::Attack` resolves a shared object id (`resolve_target`) and
  orders exactly the listed units through `order_units_attack`, and
  `GameCommand::CancelProduction` refunds and removes the front queue item
  (broadcast as the new `Command::CancelProduction` tag 14), and
  `GameCommand::Power` toggles a structure's power contribution through
  `set_structure_power`/`GameStructure::powered` (broadcast as the existing
  `Command::Power`), and `GameCommand::Repair` heals one owned structure for
  a cost proportional to
  the missing health via `repair_structure` (mirrored over the wire as the new
  `Command::RepairStructure` tag 13), and `GameCommand::Stop` clears the
  path/target of exactly the listed units via
  `order_units_stop`, and `GameCommand::Sell` removes a structure and refunds
  half its cost via
  `sell_structure`, and `GameCommand::Deploy` turns an MCV into a
  Construction Yard via `deploy_mcv`. The RA client queues right-click
  move/attack orders, production keys, structure sales and MCV deploys in
  `pending_commands` and drains them before each simulation tick, giving the
  network layer a single place to feed commands.
- Network command flow: `cnc-net-lobby --ra` runs a Red Alert game on each peer,
  queues real unit Move commands into LiveSession packets and applies every
  received packet through `GameCommand`. A 2-peer run applied 20/11 commands to
  the two local games over real UDP (all processes exited 0).
- NAT/port panel: the lobby window shows bind address/port, advertised address,
  local player/host, peer addresses and a “forward UDP <port>” hint;
  `--bind` selects the interface and `NetworkConfig` rejects non-IP bind
  addresses. Verified with a 2-peer `--bind 127.0.0.1` run (applied 29/16).

### 5.6 Map editor and procedural generator - DONE (runnable app)
- `MapModel`: terrain/overlay grids, actors with ids, triggers, player starts,
  bounds/validation, and scenario INI import/export (terrain rows, objects, triggers).
- Command pattern undo/redo: paint terrain/overlay, place/erase/move objects and
  add/remove triggers; tools only build commands and never mutate the model.
- Tools: terrain/overlay brush with Bresenham line drag, object placement, eraser,
  trigger placement (plus `rect_cells` helper).
- Game plugins: TD/RA theater lists, object catalogs, credits, naval flag, trigger
  templates and bounds/owner validation.
- `EditorUI`: tool routing, selection, undo/redo/delete, and renderer-agnostic
  terrain/object/grid/cursor drawing.
- Procedural generator (`generator.rs`): hash-based value-noise fBm elevation,
  island falloff, moisture-driven biomes (water/beach/grass/dirt/rough/rock/snow),
  independent tiberium/tree overlays, a starting tiberium patch per player and
  farthest-point player-start sampling with MCV placement. Deterministic per seed.
- Standalone binary (`src/main.rs` + `src/app.rs`): winit 0.30 + `WgpuRenderer`
  (a presenting CPU fallback through softbuffer when wgpu is unavailable or
  `--renderer software` is requested), screen-space immediate-mode UI with top toolbar
  (new/generate/open/save/undo/redo/grid/zoom/plugin), right inspector
  (seed/size/players/theater, terrain palette, filtered object palette,
  Objects/Triggers/History/Validate inspector tabs, file path), map pan/zoom,
  a clickable minimap with a viewport box, brush-stroke grouping as one undo
  step, and keyboard shortcuts (Ctrl+S/O/Z/Y/N/G/C/V, F5, Delete, Tab, Esc).
- Native file dialogs through `native-dialog` (zenity/kdialog on Linux, Win32
  common dialogs, Cocoa panels); the path field remains a typed fallback.
- Trigger inspector: add/select triggers, edit name/condition/action in buffered
  fields and commit with Enter, Apply or by clicking elsewhere — one undo step
  per edit (`SetTrigger`). Placing with the trigger tool selects the new trigger.
  The **Tpl** picker cycles templates from the active plugin registry (TD and RA
  each expose their own), and **Add** instantiates the selected template.
- Object inspector: an Objects tab lists every placed object for click-selection,
  with **Copy**/**Paste** (Ctrl+C / Ctrl+V). Group copy/paste preserves relative
  offsets, class and owner, skips occupied/out-of-bounds targets and records the
  whole paste as one undo step. A search field above the catalog filters by class
  name and the header reports visible/total counts.
- Object properties: the selected object(s) can cycle owner (neutral / GDI-Nod /
  Allies-Soviet) and be nudged one cell with X-/Y-/Y+/X+ buttons, both backed by
  grouped undo commands (`SetObjectOwner`, `MoveObject`).
- Trigger copy/paste: Ctrl+C/Ctrl+V with the Triggers tab active duplicates the
  selected trigger with a " copy" suffix and its condition/action intact.
- Trigger groups: an optional `Group` field is serialized as the fourth trigger
  column and shown as `Group/Name` in the inspector list.
- Scenario validation CLI: `--validate MAP.ini|DIR` prints map summaries and every
  validation issue (batch over a directory), exiting non-zero when invalid
  (covered by an integration test).
- Template gallery: `--export-templates DIR` writes an empty 32x32, generated
  64x64 duel and generated 48x48 arena scenario; all re-validate cleanly.
- Custom map dimensions: editable W/H fields (8-512) sit under the size presets;
  Generate and New use them for non-square maps.
- Trigger filtering: a filter field above the trigger list matches group, name or
  condition; the list shows at most three matches at once.
- Palette side hints: `GamePlugin::object_side` tags side-locked objects; TD
  labels VehicleE2/Orca as [GDI] and RA labels Chronosphere/GapGenerator as
  [Allies] and TeslaCoil/IronCurtain as [Soviet]. The tag renders in the palette
  button and is searchable ("gdi"/"soviet"). Ambiguous shared classes (TD
  Jeep/Buggy) stay untagged. Covered by `plugins_expose_side_locked_objects`
  and the palette search test.
- Object palette categories: a Cat button cycles All / Infantry / Vehicles /
  Aircraft / Buildings and combines with the text filter.
- Scenario browser: a Files inspector tab lists `maps/*.ini`, opens a scenario on
  click and offers a Refresh rescan.
- Object palette glyphs: buttons draw a small infantry/vehicle/aircraft/building
  glyph instead of a plain colour swatch.
- Side/owner validation: the Validate tab now combines map checks with
  plugin-level object owner/position validation and adds an owner-distribution
  line to the statistics.
- Placed-object owner/class filter: the Objects-tab List button cycles All /
  None / Player / Enemy and then every class present in the map; the list (and
  its selection/nudge targets) follows the active filter. The search field
  matches class names and owner labels, `#123` selects one object id, `@x,y`
  selects one cell, and rows show `#id class owner @x,y` (covered by
  `placed_search_filters_the_object_list`).
- Bulk class replacement: Shift-clicking a palette object while objects are
  selected swaps the whole selection to that class (one `SetObjectClass` undo
  step, ids/owners/cells preserved). Shift state is tracked from winit
  modifiers. `replace_selected_class_changes_all_and_undoes` covers it.
- Duplicate selection: **Ctrl+D** clones the selected object(s) one cell
  down-right as one grouped undo step, skipping occupied/out-of-bounds targets
  and selecting the copies. `duplicate_selection_copies_offset_and_undoes`
  covers the offset, owner/class preservation and undo.
- Selection shortcuts: **Ctrl+A** selects every placed object and
  **Ctrl+Shift+A** clears the selection; both update the primary object and
  status line. `select_all_and_clear_cover_the_map` covers both.
- Bulk delete: Delete with a marquee/shift selection removes every selected
  object as one grouped undo step (cursor delete still works with no
  selection); `delete_selected_objects_is_one_undo_step` covers removal and
  full restoration.
- Agent Studio "All" check: compiles every authored agent in one click,
  reporting one OK/FAIL line per agent into the report pane and tinting failing
  agents red in the list; failed ids are tracked until the next single or full
  check. `check_all_agents_reports_each_script` covers a good and a bad script.
- Toolbar grouping: the top toolbar draws separators between New/Generate,
  Open/Save, Undo/Redo, Grid and AI, with zoom at the end, so the action clusters
  are visually distinct without changing layout. The F1 overlay now also lists
  Ctrl+I.
- Map Info modal: **Ctrl+I** opens a centered editor for the scenario name,
  side, briefing and pipe-separated objectives. Enter advances fields and saves
  on the last one; Esc cancels; the whole edit is one undoable
  `SetScenarioInfo` command. `map_info_modal_commits_and_undoes` covers save,
  undo, Enter-save, Esc-discard and rendering.
- Shortcut help overlay: **F1** shows a centered overlay with the save/open/
  undo/redo/copy/paste keys, Tab grid, F5 regenerate, Delete and the inspector
  tabs; Esc or F1 closes it before any other Escape behaviour. The overlay is
  drawn over the whole editor and covered by
  `help_overlay_toggles_closes_and_renders`.
- Scenario naming: `MapModel` now carries the `[Map] Name` field, serializes it
  and restores it on open, so editing an existing scenario no longer drops its
  name when saving. Headless generation accepts `--name TEXT`, `--player SIDE`,
  `--briefing TEXT` and repeated `--objective TEXT`
  (`cnc-map-editor --headless --name "My Map" --player Soviet ... --out map.ini`);
  new maps default to "Untitled Map" with no side/briefing/objectives. Covered by the map round-trip test and the headless CLI
  test asserting `Name=Headless Scenario` in the written INI. Scenario side
  (`[Map] Player`), briefing and ordered `[Objectives]` are preserved through
  open/save as well, so editing an RA campaign mission no longer strips them;
  `ra_campaign_metadata_survives_editor_roundtrip` reopens `01.ini` and checks
  all four fields.
- Scenario thumbnails: hovering a Files-tab entry draws a cached terrain preview
  below the list (invalid/empty scenarios are skipped).
- Marquee selection: dragging on empty ground with the Select tool selects every
  object inside the rectangle (multi-selection is tracked alongside the primary
  object and highlighted on the map).
- History tab: lists the undo stack newest-first and the redo stack, with
  Undo/Redo buttons (backed by `History::undo_labels`/`redo_labels`).
- Validate tab: runs `MapModel::validate` (player starts, bounds, duplicate
  trigger ids/names) and lists issues, or reports the map as valid, then shows
  scenario statistics (map summary, building/unit counts, triggers, starts and
  the top terrain mix).
- Minimap: bottom-left overview of terrain, overlay, objects, starts and the
  visible viewport; clicking it recentres the camera on that cell.
- Drag-to-move: the Select tool picks the object under the cursor and drags it
  cell by cell; the whole gesture is a grouped undo step and occupied cells are
  rejected. A `MoveObject` command backs every step.
- Headless mode for CI/visual verification: generates a map, validates it,
  exercises an undoable edit round-trip, writes scenario INI and renders the
  full UI through `SoftwareRenderer` to PNG.
- CPU presentation fallback: `SoftPresenter` (softbuffer) blits the software
  renderer's ARGB frame into the winit window, so machines without a usable
  wgpu adapter stay interactive. `--renderer software` forces this path and
  `auto` falls back to it when wgpu initialization fails. Verified on Hyprland:
  the software backend rendered 300 frames and exited 0, and the wgpu path
  rendered 90 frames and exited 0; both runs were captured as screenshots.
- Verification run: `cargo run -p cnc-map-editor -- --headless --seed 42
  --size 96 --players 4 --out maps/demo_seed42.ini --screenshot /tmp/editor.png`
  passed; the interactive window ran 60 frames on wgpu and exited 0. See
  `cnc-map-editor/README.md`.

## Launcher Main Menu - DONE

- Boot: running `cnc-launcher` with no `--game` and no launch-mode flags
  (and `--frames 0`) opens the main menu instead of a demo scene. `--menu`
  forces it; an explicit `--game td|ra` or any of
  `--mission`, `--mission-dir`, `--campaign`, `--scenario`, `--replay`,
  `--net-role`, `--net-lobby` or a non-zero `--frames` keeps the direct game
  path unchanged.
- Screens: Main (Campaign / Local Game / Online Game / Quit), Campaign
  (Tiberian Dawn / Red Alert / Back), Local Game (TD skirmish / RA skirmish /
  Back) and Online Game (RA network lobby / TD unavailable / Back). The menu
  shows live scenario counts discovered in `maps/td_campaign` (9) and
  `maps/ra_campaign` (10), with `CNC_TD_CAMPAIGN` / `CNC_RA_CAMPAIGN`
  overrides; a campaign with no scenarios refuses to launch and explains the
  override.
- Input: Up/Down (or W/S) moves, Enter confirms, Esc backs out of a submenu and
  quits from the main page; the pointer can also hover/click rows. The menu
  repaints on navigation instead of waiting for a game tick.
- Shell: `LauncherShell` keeps the menu window alive (hidden) while either
  `LauncherApp` (TD) or `RaLauncherApp` (RA) owns a second window, so
  launching and returning never recreates an event loop (winit only allows one
  per process). Launching hides the menu and focuses the game window; Esc on a
  menu-owned game screen (TD mission select/briefing, RA setup/campaign
  select/briefing/lobby/campaign complete) shows the menu again and resets it
  to the main page.
- TD pause "Abort mission" now returns to the campaign selector when a mission
  list exists and to the main menu for a local skirmish, so no game path is a
  dead end.
- Tests: navigation/launch/back/quit, campaign scenario counts, disabled TD
  online, mouse rows, scenario counting and a headless SoftwareRenderer draw of
  every page (`main_menu_*`, `campaign_without_scenarios_reports_how_to_fix_it`,
  `scenario_count_ignores_non_ini_files`,
  `return_to_menu_flag_is_consumed_once`). End-to-end verified on Xvfb by
  driving the real binary with xdotool: TD local -> pause abort -> menu, TD
  campaign -> Esc -> menu, RA local -> Esc -> menu, RA online lobby -> Esc x2 ->
  menu, and Quit exiting the process.

## Core Types (Phase 1) - COMPLETE

- Fixed16_16, FloatBias, ClassId, CodePointers/DecodePointers, CCPtr/CCWeak/NoInit,
  ObjectPool, INI parser with inheritance and the typed Rules registry

## Workspace Structure
- 8 crates: cnc-core, cnc-renderer, cnc-audio, cnc-asset, cnc-tiberian-dawn,
  cnc-red-alert, cnc-map-editor, cnc-launcher
- All dependencies in workspace Cargo.toml

## TD Campaign Content - DONE (missions)

- Nine authored TD missions live in `maps/td_campaign/`: “GDI 01 - Holding the
  Line”, “Nod 01 - Breakout”, “GDI 02 - Counterstrike”, “GDI 03 - Scorched Earth”,
  “Nod 02 - Silent Extraction”, “GDI 04 - Hammer Fall”, “Nod 03 - Blackout”,
  “GDI 05 - Steel Rain” and “Nod 04 - Broken Arrow”.
  Each has a briefing, objectives, setup
  scripts that build/spawn the scenario forces, and win/lose triggers (timer,
  structure destroyed, Construction Yard lost).
- The newest missions add scripted beats: GDI 04 ("Hammer Fall") destroys a
  Nod War Factory with a mid-mission supply drop, Nod 03 ("Blackout") knocks out
  two GDI power plants before the uplink, GDI 05 ("Steel Rain") silences a Nod
  artillery park under a counter-battery timer, and Nod 04 ("Broken Arrow")
  raids a supply depot and radar station before the escort arrives.
- `cargo run -p cnc-launcher -- --list-campaign maps/ra_campaign` lists every RA
  scenario with its name, side, size, starts, objectives and triggers without
  opening a window; `cnc-launcher/tests/cli_campaign.rs` runs the binary and
  asserts all eight missions are reported.
- `cargo run -p cnc-launcher -- --list-missions maps/td_campaign` lists every
  mission with player/objective/trigger counts without opening a window
  (verified: 9 missions, 2 objectives each, 3-5 triggers).
- Integration test (`cnc-tiberian-dawn/tests/campaign_missions.rs`) parses all
  nine files; launching `--mission maps/td_campaign/01.ini` loads and runs the
  mission (verified in a timed smoke run).
- TD mission select: `--mission-dir DIR` opens a pre-game Select Mission screen
  (Up/Down, Enter to deploy, Esc to quit); selecting loads the mission into the
  running demo state. Verified with a captured/OCR-checked screen showing every
  mission in `maps/td_campaign`.
- TD completion tracking: victories mark the mission `[x]` and record a best
  time; the selector shows both, persisted to
  `$CNC_SAVE_DIR/td_campaign.ini` (faster replays replace the record).
- TD briefing overlay: loading a mission (from the selector or `--mission`) shows
  its name, player side, briefing and objectives; Enter begins, Esc quits.
- TD selector parity: F cycles All/Incomplete/Completed (navigation follows the
  filtered list) and S opens a TD stats screen with completion marks, best times
  and a total.
- TD side awareness: `Game::player_owner` is set from the mission's `Player`
  field. Unit click/box selection, move/attack orders, enemy picking, sidebar
  production, number-key production, Ion Cannon/Nuke strikes and building
  placement all use the player's house instead of hard-coded GDI, so the Nod
  missions (02, 05, 07) are actually playable. Savegames derive the side from
  the saved mission, keeping existing saves readable. The HUD title is
  player-relative ("Nod vs GDI") and the sidebar shows the player's superweapon
  charge (Ion for GDI, Nuke for Nod). Newly placed/demo buildings are tinted
  powered for both sides instead of always greying out Nod structures. Covered
  by `nod_mission_controls_the_nod_house` and the launcher's mission-select
  test.
- Save slots: both game views keep a current slot (1/3..3/3) shown as a pause
  menu entry; activating it cycles the slot and F5/F9 (and pause save/load) use
  that slot (`ra_mission.sav`, `ra_mission2.sav`, `ra_mission3.sav` /
  `quicksave*.sav`). Slot 0 keeps the historical filenames.
  `save_slots_cycle_and_use_distinct_paths` covers cycling, notices, wrap and
  both pause menus. The pause entry also shows the slot's age ("empty",
  "42s ago", "3m ago", "2h ago") via `save_slot_label`, tested by
  `save_slot_label_reports_age_and_empty`.
- Cancel production: **Backspace** cancels the front queue item in both game
  views, refunding its cost and reporting it in the HUD; RA broadcasts
  `CancelProduction` (network tag 14) and TD applies it locally.
  `cancelling_production_refunds_the_front_item` and
  `cancelling_td_production_refunds_the_front_item` cover both.
- RA scatter: **G** moves each selected unit to a deterministic nearby cell
  (offset derived from its id, clamped to the map), queuing one Move per unit
  locally and over the wire. `scatter_targets_are_deterministic_nearby_cells`
  and `scatter_orders_selected_units` cover it.
- RA structure power: **O** toggles the selected structure's power contribution
  (plant or drain), recomputing house power immediately and broadcasting
  `Power`; `toggling_structure_power_changes_house_power` and
  `network_power_command_toggles_remote_structure` cover it.
- RA structure repair: with an owned damaged structure selected (gold border),
  **R** queues a repair locally and broadcasts `RepairStructure`; the HUD shows
  "No repair needed" when it is full/not owned. `repairing_a_structure_costs_and_heals`,
  `network_repair_command_heals_remote_structure` and the network command
  round-trip cover it.
- TD building repair: **R** repairs the selected building to full health, paying
  a cost proportional to the missing health; full buildings report "No repair
  needed". `repairing_a_building_costs_and_heals` and
  `td_repair_selected_building_costs_and_notices` cover the library and UI.
- TD building selling: clicking one of your buildings (when no unit is picked)
  selects it with a gold border; **X** refunds half its cost, marks it sold
  (health zero, `is_selling`, removed from the house) and shows a
  "Building sold (+N credits)" notice. `selling_a_building_refunds_half` and
  `td_sell_selected_building_refunds_and_notices` cover the library and UI
  paths.
- Queue readouts: both views show the player's front production item and
  percent (TD sidebar, RA HUD "| queue PowerPlant 45%" or "empty"), sharing
  `production_percent`.
- TD queue readout: the sidebar shows the player's front production item and
  its percent ("queue PowerPlant 45%", or "empty") instead of only the total
  queue count; `production_percent` is covered by
  `production_percent_handles_progress`.
- TD placement preview: when a completed structure is waiting to be placed,
  the cursor cell shows a green border when `can_place_building` accepts it and
  a red border otherwise, matching the RA placement feedback.
- TD minimap: the game view draws a bottom-left minimap (bottom-right would
  cover the build menu) with terrain/tiberium colors, own units/structures in
  cool colors, enemies in red and a camera viewport box that follows pan/zoom.
  Uses the same 150 px / step-capped scaling as the RA minimap; clicking it
  also recenters the camera on the picked cell.
- TD pause menu: Esc pauses the simulation (no updates while paused) and opens
  Resume / Save game / Load game / Abort mission (Up/Down, Enter, Esc resumes).
  Save/load share the F5/F9 quicksave path and report "Game saved"/"Game
  loaded"/failures through the same four-second HUD notice line. Abort returns
  to the campaign mission selector when one exists and to the launcher main
  menu for a local skirmish. `td_pause_menu_resumes_and_returns_to_selector`
  covers resume and return paths.
- Campaign advance: both TD and RA now jump to the first incomplete mission
  after the current one (wrapping), instead of blindly selecting the next index;
  finishing every mission still opens the completion screen.
  `next_incomplete_skips_and_wraps` covers skipping, wrapping and all-done.
- TD campaign completion: winning the last uncleared mission opens a
  "CAMPAIGN COMPLETE" screen with cleared count and total best time; Enter/Esc
  returns to the mission selector. `td_campaign_completion_shows_final_screen`
  covers the transition and dismissal.
- TD mission finish: when a mission ends, the HUD says "press Enter for mission
  select". Enter records the result (best time kept) and returns to the
  selector, advancing to the next mission after a win and staying on the same
  mission after a defeat; other keys are ignored once the outcome is decided.
  `td_mission_finish_returns_to_selector_and_advances` covers both paths.
- TD spawn preview: each selector entry captures objective/trigger counts and
  every unit/structure spawn from the mission's scripts and triggers. The
  selected mission draws a right-hand "SPAWN OVERVIEW" panel on a scaled grid
  (GDI blue, Nod red, squares for structures) and notes that mission INIs carry
  no terrain. Verified by screenshot/OCR showing "2 objectives 3 triggers",
  the marker legend and the no-terrain note.

## Network Session Harness - DONE (library)

- `LiveSession` (cnc-tiberian-dawn::network): combines the command queue, mesh
  relay and host migration for a live session. It deduplicates relayed datagrams,
  reports forward targets, builds per-frame packets, queues remote commands and
  refreshes the host address after an election/claim.
- `NetworkConfig`: bind port, advertised NAT address and player-count validation
  (ephemeral port 0 supported) ahead of a launcher network UI.
- Integration tests cover relay-once forwarding, duplicate dropping, frame packet
  building and host-address updates after migration.
- Launcher harness: `cargo run -p cnc-launcher --bin cnc-net-lobby -- --host
  --port 47800 --players 3` plus two `--join 127.0.0.1:47800` peers runs the live
  UDP loop with a status window, heartbeat commands, mesh relay and H-triggered
  host-loss events.
- Host/join discovery: joining peers send JOIN, the host assigns the next free
  player, replies WELCOME, shares the existing peer list and broadcasts the new
  peer; PEER lines fill the mesh automatically (no manual `--peer` needed).
- Three-peer verified run: host `rx=23 relayed=21 dup=20`, joins relayed 23/24,
  all processes exited 0. NAT/port settings are shown in the lobby header and
  validated through `NetworkConfig` (ephemeral port + advertised address).

## RA Game-View Network Play - DONE

- `--net-role host --net-port 47840 --net-peer 127.0.0.1:47841` and
  `--net-role join --net-join 127.0.0.1:47840 --net-port 47841` (with
  `--game ra`) start a two-player network skirmish directly in the game view;
  bind/advertise settings are validated through `NetworkConfig`.
- Network play forces the local house (host 0 / join 1), disables the skirmish
  AI, registers the peers in a `LiveSession`, and shows role, applied/sent
  packet counts and peer count in the HUD.
- Right-click orders, successful structure placements, production keys
  (Space/1/2/3), structure sales and MCV deploys are applied locally and queued
  through `LiveSession::queue_local`; every 250 ms the session advances, sends
  its frame packet to every peer, and remote `Move`, `Attack`, `Build`,
  `Produce`, `Sell` and `Deploy` commands are applied through
  `GameCommand`.
  Right-clicking an enemy unit/structure within 18 px (closest object wins)
  turns the order into an attack on that object id; otherwise it is a move.
  Duplicate datagrams (mesh relay echoes) are filtered by `NetCommand::hash`,
  so each command applies exactly once.
- Verified with two processes on ports 47840/47841 (240 frames): both exited 0,
  sent/received counters advanced and the lockstep frame counts matched. Unit
  test `network_peers_exchange_move_commands` proves a real UDP move reaches
  the peer's game and orders a path; it also sends the packet twice and checks
  it still applies once.
- Remote builds: `network_build_command_places_remote_structure` places a
  Power Plant from a world position (rounding to a cell), rejects the same cell
  on a replay, and rejects a unit class; `network_commands_place_structures_once`
  covers the same invariants inside the RA library.
- Remote production: production keys route through the same local pending path
  and broadcast a `Produce` command. `network_produce_command_queues_remote_unit`
  checks the remote house queue grows (and unknown houses are rejected),
  `ra_produce_unit_queues_locally` checks the local pending/drain path, and
  `network_commands_queue_units` verifies up-front cost charging in the library.
- Right-click attacks: `network_commands_attack_only_listed_units` proves an
  attack targets a unit and a structure by shared id while unlisted units keep
  their orders (unknown ids are ignored); `enemy_target_picks_enemy_units_only`
  covers the cursor picker and the fact that friendly units are never targets.
- Structure selling: left-clicking one of your structures selects it (gold
  border), **X** sells it for half its cost, and the sale is broadcast as a
  `Sell` command. `selling_a_structure_refunds_half` covers refunds, repeat
  sells and wrong owners; `network_sell_command_removes_remote_structure` and
  `selected_structure_sells_locally` cover the remote and local paths.
- MCV deploy: selecting a `VehicleE7` MCV and pressing **D** replaces it with a
  Construction Yard at its cell (rejected for wrong owners, non-MCVs or an
  occupied cell) and broadcasts a `Deploy` command.
  `mcv_deploys_into_a_construction_yard`, `network_deploy_command_replaces_remote_mcv`
  and `deploy_selected_mcv_queues_locally` cover the library, remote and local
  paths.
- Network lobby screen: from the RA setup screen press **N** (or launch with
  `--net-lobby`) to choose Host/Join, edit the peer/host address in place
  (digits, dots and colons; Backspace), step the local port with `[`/`]` and
  start with Enter. Bind/address errors are shown inline and keep the lobby
  open; Esc returns to setup. Defaults are Host on 47870 with peer
  `127.0.0.1:47871`, Join on 47871 pointing at `127.0.0.1:47870`, so two
  instances on one machine connect with two Enters.
- Lobby rendering verified by screenshot/OCR (`--net-lobby --frames 900`,
  wgpu): role, editable address with caret, port controls and start/back hints
  all present. Unit test `network_lobby_edits_and_starts_both_roles` covers
  editing, invalid-address errors, host start (house 0) and join start (house 1).
- LAN discovery: a network host broadcasts `CNCRALAN 1 <game_port> RA` on UDP
  47890 every two seconds (limited broadcast plus the loopback subnet broadcast).
  Press **L** in the lobby to listen; discovered hosts appear as a sorted,
  de-duplicated list, Up/Down selects one and Enter joins it (address and Join
  role are filled in automatically). Esc stops scanning before it leaves the
  lobby. A host that started without `--net-peer` learns the joining peer from
  its first packet, so a discovered join needs no pre-shared address.
- LAN path verified with a live host process: an independent UDP listener on
  47890 received `CNCRALAN 1 47882 RA` from the host's game port; a host started
  with zero peers learned the dial-in peer and sent 43 packets. Beacon parsing,
  de-duplication and selection stability are covered by
  `lan_beacons_parse_and_deduplicate`, and the dynamic-peer path by
  `network_peers_exchange_move_commands`.
- Host-loss handling: every peer datagram refreshes a per-player last-seen
  timestamp; after 8 s of silence the loss is reported once, a migration
  election runs, and the winner broadcasts its claim (migration control frames
  are handled on receipt). The HUD and exit line show `lost`/`mig` counters.
  Live check: host on 47884, join exits after 150 frames, host logs
  `lost=1 mig=0` and keeps running. Tests cover single loss reporting on the
  host and a joiner claiming host after the host times out.
- Match replay: network commands are logged as they leave the frame queue and,
  when the match exits (frame limit or abort), written as a CNCR replay to
  `--net-replay PATH` or `saves/ra_network_replay.rep`. The log is
  deduplicated per command and decodes with `Replay::load`;
  `network_replay_saves_command_log` covers save + reload.
- Replay playback: `--replay FILE` loads a CNCR replay into a fresh 64x64
  skirmish (AI off, player house 0) and applies commands on the recorded frames
  at a 250 ms cadence; Space pauses/resumes, Esc quits, and the HUD shows the
  frame/command counters plus END. Playback input is gated so live controls
  cannot disturb the replay. `replay_playback_applies_frames_in_order` covers
  frame ordering and completion.
- Reconnection: joins send `CNCRAJOIN 1 <player>` once at startup and every
  two seconds; a host that receives it registers (or re-registers after a loss)
  that player at the sending address, revives the migration slot and clears the
  mesh-relay duplicate cache so the reconnecting peer's restarted sequence
  numbers are not dropped. Later packets also re-register a lost peer by id.
  The exit line adds `rej`. Live check: join exits, host reports the loss, a
  second join from a different port reconnects and the host ends with
  `lost=2 ... rej=1` while the reconnecting client reports no host loss.
  `network_lost_peer_rejoins_from_a_new_port` and
  `relay_clear_accepts_replayed_packets` cover the paths.
- Text chat: in network games **Y** opens an input line (Enter sends, Esc
  cancels, Backspace edits, 120-char cap). Messages travel as
  `CNCCHAT 1 <player> <text>` datagrams outside the lockstep stream, are sent to
  every known peer, and the last four show in the bottom-left of the HUD.
  `chat_lines_parse_and_log` covers parsing, empty/bad lines and local logging.
- Status notices: losses ("Peer N lost - election started"), reconnects
  ("Peer N reconnected") and host migration show as a four-second orange HUD
  line under the NET counters; tests assert the loss and reconnect notices.

## Distributed Game Directory - DONE

New standalone crate `cnc-net-directory` (binary `cnc-directory`): a
decentralised lobby directory with no central server, so any surviving node
serves the whole game list.

- **Replication**: UDP gossip with versioned bincode datagrams chunked to the
  MTU; last-writer-wins on `(updated_at_ms, origin, revision)`, withdrawal
  tombstones retained for one hour, per-record TTL expiry, deterministic
  registry digest, push deltas and anti-entropy pulls.
- **Discovery**: literal bootstrap addresses, DNS seeds, LAN multicast
  `239.255.42.99:47770`, peer-of-peer exchange, and a BitTorrent fallback
  (plain JSON URL, BEP-19 web seed, or magnet/.torrent fetched through the
  qBittorrent Web API). `cnc-directory torrent publish|fetch`.
- **Health**: two silent windows -> suspect, four -> dead, backoff retries,
  pinned bootstrap peers, live mesh view at `/api/v1/nodes`.
- **API/UI**: JSON API for publish, browse/filter, heartbeat, withdraw and
  stats, plus an embedded browser at `/`. CLI `run`, `browse` (multi-node
  failover), `publish` (heartbeat loop) and `torrent`.
- **Verified live**: three processes replicated A->B->C in under a second,
  withdrawal propagated, A was killed while B/C kept serving and accepted new
  lobbies, a fresh node pulled the full state from B, and a web-seed torrent
  round trip returned the node list. Tests: 47 unit + 5 integration + 1 doc.

## Next Steps
1. Network polish: support more than two players (the lobby/session is
   two-player; LiveSession itself supports up to eight). Rejoin, host-loss
   detection, migration, status notices and all
   move/attack/build/produce/sell/deploy routing are complete.
2. Map editor polish: multi-object property editing (toolbar grouping, palette
   side hints, extended placed search, scene metadata, Map Info and Agent
   Studio check-all are complete).
3. Optional campaign expansion: further TD/RA missions (nine TD and ten RA
   missions, the
   mission-select spawn preview and the TD selector are complete).