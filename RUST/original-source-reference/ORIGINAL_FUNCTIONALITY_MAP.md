# Original Functionality Map — CnC Remastered Collection (C++ Engines)

**Method:** assembled from the curated engine codemaps (`REDALERT/codemap.md`, `TIBERIANDAWN/codemap.md`), verified against the actual sources: custom-RTTI type enums (`DEFINES.H`), the complete class-declaration inventory of every `.H` (953 files total), and the full `.CPP` file lists of both engines. Every file of both engines is accounted for in the domain tables below. Symbols marked *(inferred)* come from naming patterns, not direct reading.

**Engines:** `REDALERT/` (519 files → RedAlert.dll) and `TIBERIANDAWN/` (298 files → TiberianDawn.dll), both consumed by the closed-source GlyphX client. Windows-only, VS2017/v141, GPL-3.

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

- All ~230 RA and ~150 TD root `.CPP` files are assigned to a domain above (duplicates/singles counted once).
- *(inferred)* marks items identified by naming pattern without direct source reading: `XDATA.CPP` type-data tables, `CCMPATH`, `EGOS`, keyframe helpers, `UTRACKER` role.
- Deeper per-file detail: see per-dir codemaps and the class inventory in §1.3; every class listed there is grep-verifiable via `grep "^class <Name>Class" <ENGINE>/<NAME>.H`.
