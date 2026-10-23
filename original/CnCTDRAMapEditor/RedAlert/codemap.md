# CnCTDRAMapEditor/RedAlert/ — Red Alert Game Plugin Implementation

## Responsibility
`IGamePlugin` implementation for **Command & Conquer: Red Alert**. Provides type registries, map I/O, INI parsing, and theater/tileset data specific to Red Alert. Mirrors the engine's `REDALERT/` data structures in C#.

## Design Patterns

### GamePlugin (Singleton-like)
**`GamePlugin.cs`** — Implements `IGamePlugin`:
- `GameType.RedAlert`
- Loads `rules.ini`, `art.ini` via `Utility/INI.cs`
- Populates static type dictionaries (see below)
- `LoadMap()`/`SaveMap()` → delegates to `Model/Map` with RA-specific validation

### Type Registries (Static Dictionaries)
| Registry | Source | Key Fields |
|----------|--------|------------|
| `BuildingTypes` | `BuildingTypes.cs` | `BuildTime`, `Power`, `Prerequisites`, `TechLevel`, `Owner` |
| `UnitTypes` | `UnitTypes.cs` | `Speed`, `Armor`, `Weapon`, `Turret`, `CarryCapacity` |
| `InfantryTypes` | `InfantryTypes.cs` | `Speed`, `Weapon`, `Deployable`, `CrateGoodie` |
| `AircraftTypes` | `UnitTypes.cs` (shared) | `Speed`, `TurnRate`, `ReloadTime`, `Ammo` |
| `VesselTypes` | `UnitTypes.cs` (shared) | Naval-specific: `Submerged`, `Sonar` |

Each type implements `ITechnoType` + game-specific interfaces.

### Constants & Enums (Game-Specific)
| File | Contents |
|------|----------|
| `Constants.cs` | RA-specific limits: `MAX_HOUSES`, `MAX_TEAM_TYPES`, cell limits |
| `DirectionTypes.cs` | 8-direction + RA-specific facings |
| `TheaterTypes.cs` | `Temperate`, `Snow`, `Urban`, `NewUrban`, `Desert`, `Winter` |
| `TerrainTypes.cs` | Template/Overlay/Smudge enums matching `REDALERT/TERRAIN.H` |
| `OverlayTypes.cs` | Concrete, walls, fences, rails, rivers |
| `SmudgeTypes.cs` | Scorch, craters, debris |
| `ActionTypes.cs` / `EventTypes.cs` / `MissionTypes.cs` | Trigger system enums |
| `TeamMissionTypes.cs` | AI team scripts (attack, patrol, guard, etc.) |
| `HouseTypes.cs` | `Allies`, `Soviet`, `Civilian`, `Special` — color, IQ, sidebar |

### Map I/O
- `LoadMap(path)` → Parses `.map` INI → constructs `Model.Map` with RA sections
- `SaveMap(map, path)` → Validates RA-specific rules (naval units need water, etc.) → writes INI
- **Theater detection**: Reads `[Basic] Theater=` → selects tileset

## Data & Control Flow

### Initialization
```
GamePlugin.ctor()
  → INI.Parse(rules.ini) → BuildingTypes, UnitTypes, InfantryTypes, AircraftTypes, VesselTypes
  → INI.Parse(art.ini) → Icon indices, voxel/anims references
  → Theaters populated from [Theaters] section
```

### Type Lookup (Used by Tools/Controls)
```
TypeComboBox.Populate(BuildingTypes.Values)
  → Each ITechnoType → displays Icon + Name
  → Filter by Category (Building/Unit/Infantry/Aircraft/Vessel)
```

### Map Validation (RA-Specific)
- Naval vessels (`VesselTypes`) only placeable on water cells (`TemplateType.Water`)
- Aircraft require `Airfield` or `Helipad` building owned by same house
- Superweapon buildings (Chronosphere, Iron Curtain, GPS) limited to 1 per house
- Base defense zones validated against `ZoneInfo` in `HouseClass`

## Integration Points

### Dependencies
- `Interface/IGamePlugin.cs` — Contract
- `Interface/ITechnoType.cs` — Type base
- `Model/` — `Map`, `Building`, `Unit`, `Infantry`, `House`, `Trigger`, `TeamType`
- `Utility/` — `INI.cs`, `Megafile.cs`, `TilesetManager.cs`, `TextureManager.cs`

### Consumers
- `MainForm.cs` — Instantiates via `Activator.CreateInstance<RedAlert.GamePlugin>()`
- `Tools/*Tool.cs` — Access type registries for placement palettes
- `Controls/*Properties.cs` — Bind to type properties

## Key Files
| File | Role |
|------|------|
| `GamePlugin.cs` | `IGamePlugin` implementation — entry point |
| `BuildingTypes.cs` | Building registry — 100+ types |
| `UnitTypes.cs` | Vehicle/naval/air registry — 80+ types |
| `InfantryTypes.cs` | Infantry registry — 30+ types |
| `Constants.cs` | RA engine limits |
| `TheaterTypes.cs` | 6 theaters + tileset mapping |
| `TerrainTypes.cs` | Template/Overlay/Smudge enums |
| `ActionTypes.cs` | Trigger actions (Create Team, Set Local, etc.) |
| `EventTypes.cs` | Trigger events (Entered, Destroyed, Timer, etc.) |
| `HouseTypes.cs` | Faction definitions (Allies/Soviet colors, sidebar) |