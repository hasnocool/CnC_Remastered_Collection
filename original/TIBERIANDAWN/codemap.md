# TIBERIANDAWN/ — Tiberian Dawn Game Engine (C++)

## Responsibility
Core game engine for **Command & Conquer: Tiberian Dawn (C&C1)**. Implements the original 1995 Westwood RTS simulation including GDI/NOD faction logic, Tiberium harvesting, base building, combat, and mission scripting. Compiled as `TiberianDawn.dll` consumed by the GlyphX client frontend.

Shares architectural DNA with `REDALERT/` but diverges in unit rosters, balance, campaign structure, and certain engine subsystems (e.g., no naval vessels, different superweapons, simpler AI).

## Design Patterns

### Class Hierarchy (Parallel to Red Alert)
```
AbstractClass
  ├── BuildingClass (Construction Yard, Power Plant, Refinery, Barracks, etc.)
  ├── TechnoClass
  │     ├── UnitClass (Humvee, Medium Tank, Mammoth Tank, etc.)
  │     ├── InfantryClass (Minigunner, Grenadier, Flamethrower, Engineer, Commando)
  │     └── AircraftClass (Helicopter, A-10, Transport)
  ├── BulletClass (projectiles)
  └── AnimClass (effects)
```
**Key differences from Red Alert:**
- No `VesselClass` (no naval warfare in TD)
- `float` bias values vs Red Alert's `fixed` (16.16) — TD uses single-precision floats for handicaps
- `HouseTypeClass const * const Class` (raw pointer) vs Red Alert's `CCPtr<HouseTypeClass>` — simpler ownership
- `USE_RA_AI` conditional compilation imports Red Alert AI improvements (FootClass, FactoryClass)

### Manager Singletons
| Manager | File | Notes |
|---------|------|-------|
| `HouseClass` | `HOUSE.H/CPP` | Faction state, AI, economy — **simpler than RA** (no GPS, no nuke silo logic) |
| `CellClass`/`MapClass` | `CELL.H/CPP` | Identical grid/zone/shroud system |
| `CommQueueClass` | `COMQUEUE.H/CPP` | Identical networking stack |
| `TriggerClass` | `TRIGGER.H/CPP` | Mission scripting — **original TD trigger set** |
| `AudioClass`/`AnimClass` | `AUDIO.H`/`ANIM.H` | Presentation — shared with RA |

### Deterministic Lockstep
Identical to Red Alert: frame-synchronous, fixed-time-step, command queue broadcast, state serialization with pointer fixup (`Code_Pointers`/`Decode_Pointers`).

### Memory Management
- `ALLOC.CPP` — Custom allocator with debug tracking (unique to TD)
- Object pools for bullets, particles, cells
- `operator new`/`delete` overrides on `HouseClass`, `BuildChoiceClass`

## Data & Control Flow

### Game Loop
Identical structure to Red Alert:
```
Initialize: Load TD INI (rules.ini, art.ini), create HouseClass[GDI/NOD/Civilian], MapClass
Main Loop:
  1. Windows messages
  2. Network: CommQueueClass receive
  3. Input → Command queue send
  4. HouseClass::AI() — expert system (GDI/NOD distinct behaviors)
  5. Object updates (TechnoClass, BuildingClass, BulletClass)
  6. Pathfinding: RegionClass A* on zones
  7. TriggerClass::Process() — TD mission scripts
  8. Audio/Anim
  9. Render (GlyphX)
  10. FrameTimerClass sleep
```

### House AI (Expert System)
`HouseClass::Expert_AI()` — **TD-specific logic**:
- GDI: favors armor, air power, defensive posture
- NOD: favors stealth, infantry swarms, offensive harassment
- No superweapon recharge logic (Ion Cannon / Nuke handled via triggers)
- Simpler base building: no `ZoneInfo` defense analysis (added in RA)

### Faction/Unit Roster (Key Differences)
| Category | GDI | NOD |
|----------|-----|-----|
| **Infantry** | Minigunner, Grenadier, Rocket Soldier, Engineer, Commando | Minigunner, Grenadier, Flamethrower, Engineer, Chem Warrior |
| **Vehicles** | Humvee, APC, Medium Tank, Mammoth Tank, MLRS, Mobile HQ | Buggy, Light Tank, Stealth Tank, Flame Tank, Artillery, SSM Launcher |
| **Aircraft** | Orca Helicopter, A-10 Strike, Chinook Transport | Apache Helicopter, Chinook Transport |
| **Structures** | Advanced Guard Tower, Ion Cannon (superweapon) | Obelisk of Light, Temple of Nod (Nuke superweapon) |
| **Harvesting** | Standard Harvester + Refinery | Identical |

### Asset Pipeline
- **INI-driven**: `rules.ini`, `art.ini`, `sound.ini` (TD-specific values)
- **Westwood formats**: `.mix`, `.shp`, `.vxl`, `.aud` — **same parsers as RA**
- **RESOURCE/**: TD campaign maps, mission INIs, briefing videos (`.vqa`)
- **WIN32LIB/**: Identical platform abstraction layer

## Integration Points

### Exports (TiberianDawn.dll)
- `Init_Game()`, `Start_Game()`, `Run_Frame()`, `Shutdown_Game()`
- `HouseClass` array access for GlyphX UI
- Save/Load serialization
- Multiplayer session management

### Dependencies
| Dependency | Purpose |
|------------|---------|
| `WIN32LIB/` | Video, input, audio, file, registry, DirectDraw |
| MFC | Dialog resources, `CString` |
| Windows 8.1 SDK | `windows.h`, DirectX headers |
| `RESOURCE/` | Runtime maps, missions, campaigns |

### Consumers
- **GlyphX Client**: Rendering, UI, Steam integration
- **CnCTDRAMapEditor** (`../CnCTDRAMapEditor/TiberianDawn/`): Reads TD data formats for editing

## Subdirectory Maps
| Directory | Responsibility |
|-----------|----------------|
| `RESOURCE/` | TD game data: maps (`.map`), missions (`.ini`), campaigns, `.vqa` videos |
| `WIN32LIB/` | Shared Win32 platform layer (identical to REDALERT/WIN32LIB/) |

---

## Key Files Reference
| File | Role |
|------|------|
| `HOUSE.H/CPP` | Faction state, AI, economy — **TD-specific expert system** |
| `CELL.H/CPP` | Map grid, terrain, shroud, pathfinding — **shared with RA** |
| `COMQUEUE.H/CPP` | Network reliability — **shared with RA** |
| `TECHNO.H/CPP` | Mobile object base — **shared architecture** |
| `BUILDING.H/CPP` | Structures — **TD structure roster** |
| `BULLET.H/CPP` | Projectiles — **TD warhead/types** |
| `AIRCRAFT.H/CPP` | Air units — **helicopters, A-10, no jets** |
| `TRIGGER.H/CPP` | Mission scripting — **TD trigger set** |
| `ALLOC.CPP` | **TD-unique** custom memory allocator with debug |
| `TYPE.H` | Core types: `float` biases, `COORDINATE`, `CELL` |
| `COORDA.H` | Coordinate math (TD-specific helper) |

---

## Divergence Notes (vs REDALERT)
| Aspect | Tiberian Dawn | Red Alert |
|--------|---------------|-----------|
| **Factions** | GDI vs NOD | Allies vs Soviet |
| **Naval** | None | Full vessel subsystem |
| **Superweapons** | Trigger-only (Ion Cannon, Nuke) | Rechargeable (GPS, Iron Curtain, Chronosphere) |
| **AI Complexity** | Simpler base building, no zone defense | Zone-based defense, paranoid logic |
| **Math** | `float` handicaps | `fixed` (16.16) handicaps |
| **Smart Pointers** | Raw const pointers | `CCPtr<T>` ref-counted |
| **RA AI Backport** | `USE_RA_AI` conditional | Native |
| **Campaign** | 15 missions × 2 sides | 14 missions × 2 sides + Counterstrike/Aftermath |