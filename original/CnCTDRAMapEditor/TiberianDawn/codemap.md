# CnCTDRAMapEditor/TiberianDawn/ — Tiberian Dawn Game Plugin Implementation

## Responsibility
`IGamePlugin` implementation for **Command & Conquer: Tiberian Dawn (C&C1)**. Provides type registries, map I/O, INI parsing, and theater/tileset data specific to Tiberian Dawn. Mirrors the engine's `TIBERIANDAWN/` data structures in C#.

## Design Patterns

### GamePlugin (Singleton-like)
**`GamePlugin.cs`** — Implements `IGamePlugin`:
- `GameType.TiberianDawn`
- Loads `rules.ini`, `art.ini` (TD versions) via `Utility/INI.cs`
- Populates static type dictionaries
- **No `VesselTypes`** — TD has no naval warfare
- `LoadMap()`/`SaveMap()` → TD-specific validation

### Type Registries
| Registry | Source | Notes |
|----------|--------|-------|
| `BuildingTypes` | `BuildingTypes.cs` | GDI/NOD structures: ConYard, Power, Refinery, Barracks, War Factory, Tech Center, Silo, Guard Tower, Obelisk, Temple, Ion Cannon |
| `UnitTypes` | `UnitTypes.cs` | Humvee, APC, Medium Tank, Mammoth, MLRS, Buggy, Light Tank, Stealth Tank, Flame Tank, Artillery, SSM |
| `InfantryTypes` | `InfantryTypes.cs` | Minigunner, Grenadier, Rocket Soldier, Flamethrower, Engineer, Commando, Chem Warrior |
| `AircraftTypes` | `UnitTypes.cs` (shared) | Helicopter (Orca/Apache), A-10, Chinook |
| `VesselTypes` | **Empty** | Not used in TD |

### Constants & Enums (TD-Specific)
| File | Contents |
|------|----------|
| `Constants.cs` | TD limits: `MAX_HOUSES=3` (GDI, NOD, Civilian), no naval |
| `DirectionTypes.cs` | 8-direction (same as RA) |
| `TheaterTypes.cs` | `Temperate`, `Arctic`, `Desert` (3 theaters vs RA's 6) |
| `TerrainTypes.cs` | Template/Overlay/Smudge enums matching `TIBERIANDAWN/TERRAIN.H` |
| `OverlayTypes.cs` | Concrete, walls, tiberium veins |
| `SmudgeTypes.cs` | Tiberium bloom, scorch, craters |
| `ActionTypes.cs` / `EventTypes.cs` / `MissionTypes.cs` | TD trigger system (simpler than RA) |
| `TeamMissionTypes.cs` | AI scripts (hunt, guard, patrol) |
| `HouseTypes.cs` | `GDI`, `NOD`, `Civilian` — gold/red/gray, distinct sidebars |

### Map I/O
- `.map` format compatible with `TiberianDawn.dll` engine
- Theater detection from `[Basic] Theater=`
- **No naval validation** (no water templates for vessels)
- Superweapons: Ion Cannon (GDI), Nuclear Missile (NOD) — trigger-only, not buildable

## Data & Control Flow

### Initialization
```
GamePlugin.ctor()
  → INI.Parse(rules.ini) → TD unit/building stats
  → INI.Parse(art.ini) → SHP/VXL references, icons
  → Theaters: Temperate, Arctic, Desert
```

### Key Differences from RedAlert Plugin
| Aspect | TiberianDawn | RedAlert |
|--------|--------------|----------|
| Factions | GDI / NOD / Civilian | Allies / Soviet / Civilian |
| Naval | None | Full vessel subsystem |
| Theaters | 3 | 6 |
| Superweapons | Trigger-only | Buildable + recharge |
| Aircraft | Helicopters, A-10, Transport | Jets (Yak, Mig), Helicopters, Transport |
| Harvesting | Tiberium only | Ore + Gems |

## Integration Points
Same as RedAlert plugin — consumers are identical (`MainForm`, `Tools`, `Controls`), only type data differs.

## Key Files
| File | Role |
|------|------|
| `GamePlugin.cs` | `IGamePlugin` implementation for TD |
| `BuildingTypes.cs` | GDI/NOD building registry |
| `UnitTypes.cs` | Vehicle/aircraft registry |
| `InfantryTypes.cs` | Infantry registry |
| `Constants.cs` | TD engine limits |
| `TheaterTypes.cs` | 3 theaters |
| `TerrainTypes.cs` | TD terrain enums |
| `HouseTypes.cs` | GDI (gold), NOD (red), Civilian (gray) |