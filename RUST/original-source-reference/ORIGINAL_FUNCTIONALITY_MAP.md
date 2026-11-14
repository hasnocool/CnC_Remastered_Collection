# Original Functionality Map — CnC Remastered Collection (C++ Engines)

**Method:** domain-level feature map checked against the pinned upstream Git tree and the original engine/source guides. The exact tracked-file and language counts are re-measured in §6.2. Tables below group source files by functional domain; they are not a claim that every symbol or every function body has been exhaustively audited. Items inferred from names or codemap summaries are explicitly marked `(inferred)` or `(partially inferred)`.

**Pinned upstream commit:** `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`. The tree contains 1,331 tracked files: `REDALERT/` 628 files (301 `.CPP`, 295 headers, 28 assembly), `TIBERIANDAWN/` 411 files (193 `.CPP`, 184 headers, 28 assembly), and `CnCTDRAMapEditor/` 286 files (257 `.cs`). See §6.2 for the count breakdown and corrections to stale prior figures.

---

## 1. Shared Architecture (both engines)

### 1.1 Deterministic lockstep simulation — the load-bearing contract
| Mechanism | Files | Notes |
|---|---|---|
| Frame-synchronous main loop | `CONQUER.CPP`, `INIT.CPP`, `STARTUP.CPP` | Init → per-frame: network pump → input → House AI → object AI → bullets → triggers → render handoff |
| Fixed-point math | `FIXED.CPP` (RA), `TYPE.H` float-bias (TD) | RA: `fixed` 16.16; TD: `float` handicaps — **do not unify** |
| Deterministic RNG | `RANDOM.CPP`/`RAND.CPP` | Seeded, same-seed-same-sequence; lockstep depends on it |
| Command queue broadcast | `COMQUEUE.CPP`, `COMBUF.CPP` | Player orders queued → broadcast → executed same tick on all peers |
| Frame checksums / desync detect | `COMBUF.CPP`, `PACKET.CPP` | Purity/CRC checks on command frames |
| State serialization + pointer fixup | `SAVELOAD.CPP`, `EXPAND.CPP` | `Code_Pointers`/`Decode_Pointers` swap raw pointers to indices on save/load |
| Custom RTTI | `DEFINES.H` (`RTTIType` enum), `ABSTRACT.CPP` | Enum + `RTTI` member, no C++ RTTI; type-safe downcast |
| Object pools / heaps | `HEAP.CPP`, `ALLOC.CPP` (TD-dedicated), `FixedHeap`/`FixedIHeap`/`TFixed*` | Placement-new via `NoInitClass`; `operator new/delete` overrides on hot classes |

### 1.2 Game object model (custom RTTI type tables)
**TD `RTTIType` (25):** INFANTRY, INFANTRYTYPE, UNIT, UNITTYPE, AIRCRAFT, AIRCRAFTTYPE, BUILDING, BUILDINGTYPE, TERRAIN, ABSTRACTTYPE, ANIM, ANIMTYPE, BULLET, BULLETTYPE, OVERLAY, OVERLAYTYPE, SMUDGE, SMUDGETYPE, TEAM, TEMPLATE, TEMPLATETYPE, TERRAINTYPE, OBJECT, SPECIAL, NONE.

**RA `RTTIType` (32):** all of the above **plus** CELL, FACTORY, HOUSE, HOUSETYPE, TEAMTYPE, TRIGGER, TRIGGERTYPE, **VESSEL, VESSELTYPE** (naval — the headline RA addition).

### 1.3 Class inheritance spine
```
AbstractClass (RTTI, coords, serialization)
├── ObjectClass (occupancy, selection, damage)
│   └── TechnoClass (owner house, targeting, weapons, missions)   [TECHNO.CPP]
│       ├── UnitClass      [UNIT.CPP]  ground vehicles: locomotion, MCV deploy, harvester
│       ├── InfantryClass  [INFANTRY.CPP] squads, prone, capture, saluting, passengers
│       ├── AircraftClass  [AIRCRAFT.CPP] flight physics, landing/rearm, no pathfind grid
│       └── VesselClass    [VESSEL.CPP — RA only] subs (submerge), transports, gunboats
├── BuildingClass  [BUILDING.CPP] production, power, gates, repair bolts, sale, capture
├── BulletClass    [BULLET.CPP] ballistic arcs, homing, fuel/range, warhead impact
└── AnimClass      [ANIM.CPP] visual effects, attached loops, damage fires/smoke
+ FootClass (movement-capable mix-in; exported via USE_RA_AI into TD)
+ RadioClass [RADIO.CPP] inter-object command link (docking, cargo handoff, rearm requests)
+ CargoClass [CARGO.CPP] passenger holds; CrewClass [CREW.CPP]; DriveClass [DRIVE.CPP] vehicle locomotion; DoorClass [DOOR.CPP] (turret/gate hatches); FuseClass [FUSE.CPP] delayed detonation
```
Type-side (`XTypeClass`, INI-driven): `HouseTypeClass`, `UnitTypeClass`, `BuildingTypeClass`, `InfantryTypeClass`, `AircraftTypeClass`, `VesselTypeClass` (RA), `BulletTypeClass`, `WarheadTypeClass` (`WARHEAD.CPP`, verses-armor %), `WeaponTypeClass` (`WEAPON.CPP`), `AnimTypeClass`, `OverlayTypeClass`, `SmudgeTypeClass`, `TerrainTypeClass`, `TemplateTypeClass` (theater tile sets), `RuleType` aggregates.

---

## 2. Functional Domain Map (every `.CPP` assigned)

### 2.1 Combat & weapons
| Capability | Files |
|---|---|
| Damage resolution, warhead vs. armor tables, explosion pipeline | `COMBAT.CPP`, `WARHEAD.CPP` |
| Weapon systems: range, ROF, projectile selection, turret logic | `WEAPON.CPP`, `TURRET.CPP` (independent turret rotation) |
| Projectiles: ballistic arc, homing missiles, instant-hit, fuel | `BULLET.CPP` |
| Superweapons (TD: Ion Cannon/Nuke via triggers; RA: rechargeable GPS/Chronosphere/Iron Curtain + **ChronalVortex** vortex effect) | `SUPER.CPP` (both), `VORTEX.CPP` (RA) |
| Special/crate powerups (money, unit, invulnerability...) — **RA-only system** | `CRATE.CPP` (RA) |

### 2.2 Economy, production & logistics
| Capability | Files |
|---|---|
| House credits, credit display ticking | `CREDITS.CPP` |
| Production queues/factories (RA formalizes `FactoryClass`; TD uses building-integrated queues) | `FACTORY.CPP` (RA) |
| Power grid & low-power effects | `POWER.CPP` |
| Base layout management + placement rules | `BASE.CPP` |
| Reinforcements delivery | `REINF.CPP` |
| Ore/tiberium: harvester logic, storage, refinery dump — via `UnitClass`/`BuildingClass` + `RadioClass` handoff | `UNIT.CPP`, `BUILDING.CPP` |
| Cargo/passengers, transport carry, vehicle drive states, doors/hatches, delayed fuses | `CARGO.CPP`, `DRIVE.CPP`, `DOOR.CPP`, `FUSE.CPP` |

### 2.3 AI & navigation
| Capability | Files |
|---|---|
| House expert AI: weighted urgency checks (attack, build power/defense/offense/income, fire-sale), `BuildChoiceClass` priority heap, TD-vs-RA behavior differences (RA zone defense, paranoid logic; TD simpler) | `HOUSE.CPP` (`Expert_AI`) |
| Unit mission state machine (Move/Attack/Guard/Enter/Unload/Hunt...), guard areas, retaliation | `MISSION.CPP`, `MISSION.H` (`MissionClass`/`MissionControlClass`) |
| A*/zone pathfinding on movement zones, threat cost, formation movement | `FINDPATH.CPP`, `CELL.CPP` (`CellClass` zones), `REGION` (`RegionClass` in `CELL.H`) |
| Trigger-driven team scripting: TeamTypes, mission lists, behavior steps | `TEAM.CPP`, `TEAMTYPE.CPP`, `TEAM` infra (RA adds `TeamMemberClass`, `TeamMissionClass`) |
| Radio-commanded docking/rearm/submerge protocols | `RADIO.CPP` |

### 2.4 Mission scripting & scenario
| Capability | Files |
|---|---|
| Trigger system: events → conditions → actions, per-object attachment | `TRIGGER.CPP`, `TRIGTYPE.CPP` |
| Event/action vocabularies (spawn teams, reveal, win/lose, text, credits...) | `TEVENT.CPP`, `TACTION.CPP` |
| Generic event queue (in-game events, response tables) | `EVENT.CPP`, `EVENT.H` |
| Scenario INI parsing: maps, houses, triggers, teams, waypoints, briefings; victory/defeat evaluation | `SCENARIO.CPP` |
| Campaign intro/ending movie playback hooks | `INTRO.CPP`, `ENDING.CPP` |
| Session/game-mode options (skirmish, MP settings, difficulty) | `SESSION.CPP`, `SPECIAL.CPP`, `OPTIONS.CPP`/`GOPTIONS.CPP` |

### 2.5 Rules & data pipeline (INI-driven everything)
| Capability | Files |
|---|---|
| Rules parsing (RULES.INI → all type tables), difficulty settings | `RULES.CPP`, `DIFFICULTY` (`DifficultyClass`) |
| INI reader/writer core | `INI.CPP`, `CCINI.CPP`, `INIBIN/INICODE` (RA) |
| Per-type rule data tables (AircraftData, BulletData, HouseData, UnitData...) | `ADATA/BDATA/CDATA/HDATA/IDATA/ODATA/SDATA/TDATA/UDATA/VDATA.CPP` *(inferred: XDATA = type-data registration tables)* |
| Profile/registry persistence of options | `PROFILE.CPP` |
| Misc control constants | `CONST.CPP`, `GLOBALS.CPP` |

### 2.6 Persistence (save/load)
| Capability | Files |
|---|---|
| Save-game serialization of every heap object + `Code_Pointers`/`Decode_Pointers` fixup, save slots, load dialog | `SAVELOAD.CPP`, `LOADDLG.CPP` |
| Save-buffer expansion management | `EXPAND.CPP` |

### 2.7 Networking (the deepest RA investment)
| Capability | Files |
|---|---|
| Reliable circular packet queues, ACK/retry/backoff, latency tracking | `COMQUEUE.CPP`, `COMBUF.CPP`, `PACKET.CPP` |
| Connection managers: sequenced / non-sequenced / null-modem / 10-conn (RA) | `SEQCONN/NOSEQCON/NULLCONN/NULLMGR/CONNECT.CPP`, `TENMGR.CPP` (RA) |
| IPX LAN stack | `IPX*.CPP` (IPX, IPXADDR, IPXCONN, IPXGCONN, IPXMGR, IPX95) |
| TCP/UDP internet play (RA) | `TCPIP.CPP`, `WSPROTO/_WSPROTO/WSPIPX/WSPUDP.CPP`, `INTERNET.CPP` |
| Modem/dial-up: modem init, dialing, phone book | `DIAL8.CPP`, `PHONE` (`PhoneEntryClass`), `DDEServerClass`/`CCDDE.CPP` (DDE bridge) |
| Westwood Online (RA): lobby login/chat/game-list/download/options matchmaking | `WOL_MAIN/WOL_LOGN/WOL_CHAT/WOL_CGAM/WOL_GSUP/WOL_OPT/WOL_DNLD/WOLEDIT/WOLSTRNG/WOLAPIOB.CPP` (RA only) |
| Multiplayer game setup: player pools, color/house assignment, spawn (GlyphX bridge reads these) | `MPLAYER.CPP` (TD) / `MPLAYER+MPLIB+MPMGRD+MPMGRW+MPU+MPGSET.CPP` (RA), `MPlayerManClass` |
| Send-file (scenario transfer) | `SENDFILE.CPP` (RA) |

### 2.8 Graphics, UI & interaction
| Capability | Files |
|---|---|
| Screen buffer/composite surface management | `GSCREEN.CPP`, `SURFACE` (RA), `DISPLAY.CPP` |
| Sidebar: build palette, tabs, minimap — with **GlyphX remaster bridge** (`SidebarGlyphxClass`) | `SIDEBAR.CPP`, `SIDEBARGlyphx.CPP` (both engines) |
| Radar map | `RADAR.CPP` |
| Gadget UI framework + widgets: buttons (text/shape/static), checkboxes, checklists, drop lists, edit fields, gauges, sliders, tabs, toggles, tooltips, icon lists, LED, tri-color gauge | `GADGET.CPP`, `CONTROL.CPP`, + `CHECKBOX/CHEKLIST/COLRLIST/DROP/EDIT/GAUGE/ICONLIST/LED/LIST/SLIDER/SHAPEBTN/STATBTN/TAB/TEXTBTN/TXTLABEL/TOGGLE/TOOLTIP/TRICOLOR (TriColorGaugeClass).CPP` |
| Dialogs: confirmation, game options, load, save-edit, sound, visual, message box, net dialog, map-edit dialogs | `CONFDLG/GAMEDLG/LOADDLG/SEDITDLG/SOUNDDLG/VISUDLG/MSGBOX/NETDLG/MAPEDDLG.CPP` |
| Menus & scrolling, help | `MENUS.CPP`, `SCROLL.CPP`, `HELP.CPP` |
| Mouse & keyboard drivers | `MOUSE.CPP`, `KEYBOARD.CPP` + `WWKeyboardClass` (RA) |
| Palettes & color: palette interp (theater fades), RGB/HSV math, palette class | `INTERPAL.CPP`, `PALETTEC.CPP`, `RGB.CPP`, `HSV.CPP`, `ColorListClass` |
| Sprites/facing/staging: 32-angle sprite selection, frame staging | `SPRITE`/`STAGE`/`FACING.CPP`, `FLY.CPP` (fly-over view), `FLASHER.CPP`, `FlasherClass` |
| In-game map editor (both engines): placement/selection/trigger tools | `MAPEDIT.CPP` + `MAPEDPLC/MAPEDSEL/MAPEDTM/MAPSEL.CPP` |
| Scoring/scoreboard screens, end-of-mission stats | `SCORE.CPP` + `ScoreAnim/Creds/Print/Scale/Time`, `STATS.CPP` |
| Monochrome debug console (classic Westwood Mono) | `MONOC.CPP` |
| Message lists (EVA/report lines) | `MSGLIST.CPP`, `MessageListClass` |
| Debug/tracing hooks | `DEBUG.CPP`, `W95TRACE.CPP` (RA), `READLINE` (RA) |
| System probing | `GETCPU.CPP` (RA) |

### 2.9 Audio & video
| Capability | Files |
|---|---|
| Sound/music engine interface | `AUDIO.CPP`, `AudioClass` |
| Music themes & battle/menu switching | `THEME.CPP`, `ThemeClass` |
| ADPCM audio codec | `ADPCM.CPP` (RA) |
| Windows Media playback (briefing movies via MCI) | `MCI.CPP`, `MCIMOVIE.CPP` (RA) |
| Sound options dialog | `SOUNDDLG.CPP` |

### 2.10 File formats, compression & crypto (Westwood "Straw/Pipe" stream architecture)
| Capability | Files |
|---|---|
| Stream primitives: `Straw`/`Pipe` filter chain | `STRAW.CPP`, `PIPE.CPP` + `XPIPE/XSTRAW` |
| **MIX archive** format | `MIXFILE.CPP`, `MixFileClass` |
| File layers: raw file, in-RAM file, CD file, buffered file, expand-file | `RAWFILE.CPP`, `RAMFILE` (RA), `CDFILE.CPP`, `CCFILE.CPP`, `BUFF/BUFFER` |
| Compression: **LCW** (Westwood Lempel-Cohn-Welch), **LZO1X**, **PKWARE** | `LCW+LCWPIPE/LCWSTRAW/LCWUNCMP.CPP`, `LZO1X_C/LZO1X_D/LZOPIPE/LZOSTRAW.CPP`, `PK+PKPIPE+PKSTRAW.CPP` |
| Crypto: **Blowfish** (RA), **SHA-1** (RA), checksums | `BLOWFISH+BLOWPIPE+BLWSTRAW.CPP`, `SHA+SHAPIPE+SHASTRAW.CPP`, `CRC+CRCPIPE+CRCSTRAW.CPP`, `BASE64+B64PIPE+B64STRAW.CPP` |
| RNG / noise (RA) | `RAND.CPP`, `RANDOM.CPP`, `RNDSTRAW.CPP` |

### 2.11 Containers & platform utilities
`VECTOR.CPP`, `DYNAVEC.CPP` (DynamicVector), `LIST.CPP`, `QUEUE.CPP`, `LINK.CPP`, `INDEX.CPP` (RA), `FIELD.CPP`, `HEAP.CPP` + FixedHeap variants, `RECT`, `COORD.CPP`, `FACING`, `VERSION.CPP` (RA), `TRACKER.CPP`/`UTRACKER.CPP` (unit selection tracking), `TARGET.CPP` (TargetClass abstract targeting), `LAYER.CPP` (render/logic layers), `LOGIC.CPP` (LogicClass tick), `JSHELL.CPP` (window shell), `STUB.CPP`/`WINSTUB.CPP`/`DPMI.CPP`/`DOSSegmentClass` (DOS-era stubs retained), `CARRY.CPP`/`CarryoverClass` (RA: campaign carry-over between missions), `ITABLE/DTABLE` (RA index/data tables *(inferred)*), `EGOS.CPP`/`EgoClass` (RA *(function unverified)*), `CCMPATH`, `2KEYFRAM`/`KEYFRAME` (keyframe interpolation, TD has KEYFRAME) *(inferred)*, `SEditClass`/`SimpleEditDlgClass` (RA).

### 2.12 WIN32LIB/ platform layer (shared, both engines)
| Capability | Files |
|---|---|
| DirectDraw video surfaces, graphics buffer (GBuffer), draw rects | `DDRAW.CPP`, `GBUFFER.CPP`, `DRAWRECT.CPP`, `BUFFER.CPP` |
| Shape/icon loading: IFF, iconsets, PCX write | `IFF.CPP`, `ICONSET.CPP`, `GETSHAPE.CPP`, `WRITEPCX.CPP` |
| Fonts incl. diphthong tables | `FONT.CPP`, `LOADFONT.CPP`, `SET_FONT.CPP`, `DIPTHONG.CPP`, `_DIPTABL.CPP` |
| Palette | `PALETTE.CPP` |
| Input: Westwood mouse | `MOUSEWW.CPP` |
| Timers (system, countdown, init) | `TIMER.CPP`, `TIMERDWN.CPP`, `TIMERINI.CPP` |
| Random | `IRANDOM.CPP` |
| Memory + window glue | `ALLOC.CPP`, `WINDOWS.CPP`, `WINHIDE.CPP`, `LOAD.CPP`, `DELAY.CPP` |

### 2.13 RESOURCE/ subdirectories
Game data: scenario `.MAP`/`.INI`, campaign definitions, cursors/icons (RA `RESOURCE/` includes `.rc`, icons, cursors); TD adds `.VQA` video references. Compiled into/next to the DLLs; format parsers live in `MIXFILE.CPP`/`SCENARIO.CPP`/WIN32LIB.

---

## 3. Feature Divergences (RA vs TD) — quick reference

| Feature | TIBERIANDAWN | REDALERT |
|---|---|---|
| Naval units (`VesselClass`, RTTI_VESSEL) | ✗ | ✓ (+ submerge, transports) |
| Formal `FactoryClass` production | ✗ (building queues) | ✓ |
| Crates/powerup system | ✗ | ✓ (`CrateClass`, `CRATE.CPP`) |
| Chronosphere vortex effect | ✗ | ✓ (`ChronalVortexClass`, `VORTEX.CPP`) |
| Campaign carry-over between missions | ✗ | ✓ (`CarryoverClass`) |
| Westwood Online lobby stack | ✗ | ✓ (11 `WOL_*.CPP`) |
| Winsock/TCP internet play | minimal (`TCPIP.CPP`) | full (`WSP*`, `UDPInterface`, `TcpipManager`) |
| Blowfish/SHA crypto for MP | ✗ | ✓ |
| In-game map editor | ✓ | ✓ (+ placement/trigger variants) |
| Custom allocator with debug tracking | ✓ (`ALLOC.CPP` + `HeapClass`) | generic heaps |
| Keyframe interpolation helpers | `KEYFRAME.CPP` | `2KEYFRAM.CPP` |
| Modular arithmetic type | float bias | `fixed` 16.16 |
| Trigger/Team RTTI-persistent types | values only | full RTTI types (TRIGGER/TEAMTYPE/HOUSE as objects) |
| Multiplayer connection managers | 3 (seq/no-seq/null) | + `TenConnManClass`, `IPXGlobalConnClass` |

## 4. GlyphX Client Integration Points (remaster bridge)

- DLL exports: `Init_Game()`, `Start_Game()`, `Run_Frame()`, `Shutdown_Game()`; `HouseClass` array access; save/load; MP session management.
- `SidebarGlyphxClass` (`SIDEBARGlyphx.CPP`, both engines): sidebar rendering delegated to the client.
- `UTRACKER.CPP` (UnitTracker), `WOLAPIOB.CPP` (WOL API object), `GETCPU.CPP`, `W95TRACE.CPP` — remaster-era hooks *(partially inferred)*.
- Rendering itself is client-side; the DLLs hand over composited state each frame.

## 5. Coverage & Confidence Notes

- The pinned source tree has 301 RA and 193 TD `.CPP` translation units. The domain tables group files by subsystem and are not a per-file/per-symbol completeness proof; use the complete file inventory and individual source guides for path-level coverage.
- *(inferred)* marks items identified by naming pattern without direct source reading: `XDATA.CPP` type-data tables, `CCMPATH`, `EGOS`, keyframe helpers, `UTRACKER` role.
- Deeper per-file detail: see the source inventory and individual guides. Validate each class/function against its actual declaration and implementation; grep patterns and naming conventions alone are not sufficient evidence.


---

## 6. Rust port status and remaining implementation work

**Status snapshot date:** 2026-11-14  
**Audited Rust HEAD:** [`8853580f03fb66159143136165ef24077865d780`](https://github.com/hasnocool/CnC_Remastered_Collection/commit/8853580f03fb66159143136165ef24077865d780)  
**Status details:** see [IMPLEMENTED_FUNCTIONALITY_MAP.md](IMPLEMENTED_FUNCTIONALITY_MAP.md).

This map describes what the original games do. The presence of a capability below is not a claim that the Rust port has completed it. The status column is a source-level snapshot against the Rust commit above.

| Original capability | Current Rust mapping | Status at audited HEAD |
|---|---|---|
| Shared object/type hierarchy, ownership, damage and deterministic simulation | `cnc-core/src/entity.rs`, `class_id.rs`, `fixed.rs`; engine `game.rs` modules | **Partial.** Working Rust model and game loops, but not a 1:1 C++ RTTI/inheritance/allocator reproduction. |
| TD units, buildings, economy, combat and projectiles | `cnc-tiberian-dawn/src/{game,unit,building,weapon,td_rules}.rs` | **Partial.** Real game paths/tests; complete original roster and edge-case semantics remain open. |
| RA ground/naval/air units, combat and projectiles | `cnc-red-alert/src/{game,pathfind,superweapon,tesla}.rs` and special-unit modules | **Partial.** Selected roster, armor, missiles, splash, aircraft rearming and naval/carrier behaviors are implemented; full roster parity remains open. |
| Original TD scenario INI/BIN loading | `cnc-tiberian-dawn/src/scenario_loader.rs` | **Implemented for supported data.** Raw/LCW cell data, objects, houses, waypoints and supported trigger records are connected to runtime conversion. Unknown/unmapped records remain explicit gaps. |
| Original RA MPR MapPack/OverlayPack, Houses and Base | `cnc-red-alert/src/original_scenario.rs`, `scenario_mpr.rs` | **Implemented for supported records.** Parser and game-builder tests exist; all parsed fields are not yet proven to affect runtime. |
| TeamTypes, TaskForces and waypoint behavior | `cnc-core/src/entity.rs`; TD `ai.rs`; RA `game.rs`; editor `map.rs` / `tools.rs` | **Partial-to-implemented for tested paths.** Model, parser, team manager and selected objectives exist; all original mission steps and editor dialog parity remain open. |
| House expert AI (urgency, build, offense, defense, income and fire-sale) | `cnc-core/src/house_ai.rs`; TD `ai.rs`; RA `ai.rs` | **Partial; RA has confirmed stubs.** TD adapter has functional decision paths. RA structure queue, attack-wave and sell hooks currently do not perform the claimed action. |
| Mission triggers, event/action tables and per-object attachments | `cnc-core/src/{trigger_apply,trigger_attach,trigger_records}.rs`; engine `mission.rs` / `game.rs` | **Partial.** Supported actions execute; other host actions reject/log. Full original TEVENT/TACTION runtime parity is not present. |
| Rules.ini / type-data consumption | `cnc-core/src/rules.rs`; TD `td_rules.rs`; RA `game.rs` | **Partial.** Several values reach runtime; full original field-by-field consumption is not proven. |
| Original map editor import/export and map model | `cnc-map-editor/src/{map,original}.rs`; `app.rs` | **Implemented for supported TD/RA formats.** UI import and fixture round trips exist. Complete C# dialogs, controls, Steamworks and validation parity remain open. |
| Save/load, replay and pointer restoration | TD `savegame.rs` / `network.rs`; RA `game.rs` / `replay.rs`; core `pointers.rs` | **Partial.** Rust state round-trips and deterministic RA replay exist; original save-file byte compatibility is not claimed. |
| Multiplayer command transport and lobby services | TD `network.rs`; `cnc-net-directory/src/`; launcher lobby | **Modern replacement / partial integration.** UDP lockstep/handshake and modern directory/LFG exist. Original IPX, modem, DDE, WOL and legacy manager protocols are not reproduced. |
| DirectDraw, GlyphX and Win32 presentation/input | `cnc-renderer/src/`, `cnc-launcher/src/` | **Modern replacement.** wgpu/software renderer and winit frontend exist; no DirectDraw/GlyphX ABI compatibility claim. |
| Sound, EVA, music and media playback | `cnc-audio/src/`; `cnc-asset/src/{audio,adpcm,video}.rs`; launcher `movie.rs` | **Partial.** Mixer/queues, audio decoders and VQA frame decoding exist; full original codec/event/MCI coverage remains open. |
| MIX, SHP, palette, font, VQA, VXL and other formats | `cnc-asset/src/` | **Partial by format.** Font parser/rasterizer and VQA/VXL paths exist; archive/format edge coverage varies. CSF string-table loading is a confirmed stub. |

### 6.1 Confirmed implementation gaps

These gaps are anchored in current source at `8853580f03fb66159143136165ef24077865d780`:

1. **Red Alert House AI:** `cnc-red-alert/src/ai.rs` contains explicit placeholder behavior. `queue_structure` can return success without queuing/placing a structure; `launch_attack_wave` can return success without sending attack orders; `sell_structure` returns success without selling. The adapter recreates `HouseAIState` locally inside each update, so persistence of timers and one-time decisions is not established.
2. **Trigger action families:** `cnc-core/src/trigger_apply.rs` supplies default reject paths. The 2026-10-24 parity-gap report recorded 12 RA action families as documented-only; re-count against the current tree and verify TD/RA separately before closing them.
3. **CSF strings:** `cnc-asset/src/string_table.rs` currently returns an empty map from `CsfFile::load`.
4. **Rules and scenarios:** parse coverage exceeds proven runtime consumption. Audit each field from parser through scenario/game construction to a tested gameplay effect.
5. **Full roster and rare behavior:** complete unit/building/weapon/special behavior matrices remain to be compared against the pinned original source.
6. **Original editor parity:** map formats are imported/exported, but every legacy tool, dialog, control, validation path and Steam publishing flow is not reproduced.
7. **Legacy persistence/platform/network:** modern Rust serializers, wgpu/winit and UDP/directory services are not substitutes for byte-compatible saves, DirectDraw/GlyphX ABI, IPX/modem/DDE/WOL without explicit compatibility work.
8. **Media and audio edge cases:** all original audio variants, EVA emitters, movie and VQA edge paths need source-derived fixtures and integration tests.

### 6.2 Source and inventory counts corrected against pinned upstream tree

The pinned upstream Git tree contains **1,331 tracked files** (1,375 tree entries including directory objects), not just the engine-file subtotals. Verified subtree counts:

| Upstream subtree | All files | `.CPP` | Headers | Assembly |
|---|---:|---:|---:|---:|
| `REDALERT/` | 628 | 301 | 295 | 28 |
| `TIBERIANDAWN/` | 411 | 193 | 184 | 28 |
| `CnCTDRAMapEditor/` | 286 | 0 | 0 | 0 |

The editor subtree contains 257 `.cs` files. There are 6 other top-level files in the pinned tree. The prior figures “519 RA files,” “298 TD files,” and “953 headers” are stale and have been corrected. The 301 + 193 = **494** C++ translation units agrees with the separate 494-translation-unit inventory.

The domain tables in this document are a functional guide, not a line-by-line allocation of every C++ symbol. Use the complete file inventory for path coverage and the individual source guides for exact declarations and behavior.

### 6.3 Evidence and confidence rule

- **Original functionality** answers “what does the pinned upstream source do?”
- **Rust implementation status** answers “what does the current Rust HEAD actually do?”
- **Source-documentation status** answers “how completely has the upstream file been explained?”

Do not merge these three states. A fully documented C++ feature may still be absent from Rust; a Rust helper may exist without an integrated game behavior. A feature is complete only after tracing original symbols → Rust implementation → runtime call path → focused test/fixture. Record the command/date/commit for any test result. The November 14 audit inspected code and test definitions but did not run Cargo gates.
