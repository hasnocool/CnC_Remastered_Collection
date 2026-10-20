# REDALERT/ — Red Alert Game Engine (C++)

## Responsibility
Core game engine for **Command & Conquer: Red Alert**. Implements the complete simulation layer including unit/building logic, combat, pathfinding, AI, networking, audio, animation, and resource management. Compiled as `RedAlert.dll` consumed by the GlyphX client frontend.

## Design Patterns

### Class Hierarchy (Inheritance-Driven)
All game objects derive from a common RTTI-enabled base:
```
AbstractClass (RTTI, serialization, damage state)
  ├── BuildingClass (structures, production, power)
  ├── TechnoClass (mobile entities with movement/orders)
  │     ├── UnitClass (ground vehicles)
  │     ├── InfantryClass (foot soldiers)
  │     ├── AircraftClass (air units, distinct physics)
  │     └── VesselClass (naval units)
  ├── BulletClass (projectiles, instant/homing)
  └── AnimClass (visual effects, non-interactive)
```
- **RTTI system**: Custom `RTTIType` enum + `RTTI` member enables type-safe downcasting without C++ RTTI
- **Smart pointers**: `CCPtr<T>` (reference-counted) used for cross-references (triggers, targets)
- **NoInitClass pattern**: Placement-new friendly constructors for object pools/serialization

### Manager Singletons (Global Systems)
| Manager | File | Responsibility |
|---------|------|----------------|
| `HouseClass` | `HOUSE.H/CPP` | Player/faction state, AI, economy, production, diplomacy |
| `CommQueueClass` | `COMQUEUE.H/CPP` | Network packet queues (send/recv), ACK/retry, latency tracking |
| `CellClass`/`MapClass` | `CELL.H/CPP` | Grid cells, terrain, shroud, pathfinding zones, waypoints |
| `AudioClass` | `AUDIO.H/CPP` | Sound/music playback, mixing, 3D positioning |
| `AnimClass` | `ANIM.H/CPP` | Animation playback, particle effects |
| `TriggerClass` | `TRIGGER.H/CPP` | Mission scripting, event/condition/action system |

### Deterministic Lockstep Simulation
- **Frame-synchronous**: All game logic advances in discrete ticks (`FrameTimerClass`)
- **Fixed-point math**: `fixed` type (16.16) for deterministic cross-platform physics
- **Command queue**: Player actions queued → broadcast → executed on same tick (see `COMQUEUE`)
- **State serialization**: `Save`/`Load` + `Code_Pointers`/`Decode_Pointers` for pointer fixup

### Memory Management
- Custom allocators: `ALLOC.CPP` (Tiberian Dawn) / similar in Red Alert
- Object pools for high-frequency types (bullets, particles, cells)
- Manual `operator new`/`delete` overrides on major classes (`HouseClass`, `BuildChoiceClass`)

## Data & Control Flow

### Game Loop (Simplified)
```
WinMain / GlyphX entry
  → Initialize: Load INI, create HouseClass[], MapClass, managers
  → Main Loop:
      1. Process Windows messages
      2. Network: CommQueueClass::Queue_Receive/UnQueue_Receive
      3. Input → Command queue (CommQueueClass::Queue_Send)
      4. HouseClass::AI() — per-house expert system (timed via AITimer)
      5. Object updates: TechnoClass::AI(), BuildingClass::AI(), BulletClass::Move()
      6. Pathfinding: RegionClass A* on movement zones (CellClass::Zones)
      7. TriggerClass::Process() — mission script evaluation
      8. Audio/Anim updates
      9. Render (delegated to GlyphX client)
      10. FrameTimerClass::SleepUntilNextFrame()
```

### Network Architecture
- **Peer-to-peer lockstep**: `CommQueueClass` implements reliable UDP-like semantics over DirectPlay/IPX/TCP
- **Circular buffers**: Send/Receive queues with sequence numbers, ACK tracking, exponential backoff retry
- **Latency compensation**: `Add_Delay()` / `Avg_Response_Time()` for adaptive timeouts
- **Determinism guarantee**: Identical simulation state on all peers enforced by fixed-point + same tick order

### House AI (Expert System)
`HouseClass::Expert_AI()` evaluates weighted `UrgencyType` checks:
- `Check_Attack` → `AI_Attack` (team formation, targeting)
- `Check_Build_Power/Defense/Offense/Income` → production decisions
- `Check_Fire_Sale` → sell buildings when losing
- Build choices queued in `TFixedIHeapClass<BuildChoiceClass>` (priority heap)

### Asset Pipeline
- **INI-driven**: `rules.ini`, `art.ini`, `sound.ini` define all type data (`HouseTypeClass`, `UnitTypeClass`, etc.)
- **Westwood formats**: `.mix` archives, `.shp` sprites, `.vxl` voxels, `.aud` audio
- **RESOURCE/** subdirectory: scenario maps (`.map`), mission scripts (`.ini`), campaign data
- **WIN32LIB/**: Platform abstraction (video, input, file I/O, registry, DirectDraw wrappers)

## Integration Points

### Exports (RedAlert.dll)
- `Init_Game()`, `Start_Game()`, `Run_Frame()`, `Shutdown_Game()`
- `HouseClass` array access for GlyphX UI (sidebar, radar, scores)
- Save/Load game state serialization
- Multiplayer session management (host, join, sync)

### Dependencies
| Dependency | Purpose |
|------------|---------|
| `WIN32LIB/` | Video mode setup, DirectDraw surfaces, mouse/keyboard, timer, file I/O |
| MFC | Dialog resources, `CString`, collection classes (limited use) |
| Windows 8.1 SDK | `windows.h`, DirectX 7/9 headers |
| `RESOURCE/` | Runtime data (maps, INI, campaigns) |

### Consumers
- **GlyphX Client** (external, not in repo): Rendering, UI, input handling, Steam integration
- **CnCTDRAMapEditor** (`../CnCTDRAMapEditor/RedAlert/`): Reads same INI/data formats for map editing

## Subdirectory Maps
| Directory | Responsibility |
|-----------|----------------|
| `RESOURCE/` | Game data: maps (`.map`), missions (`.ini`), campaigns, briefing videos |
| `WIN32LIB/` | Win32 platform layer: video, input, audio, file, registry, DirectDraw |

---

## Key Files Reference
| File | Role |
|------|------|
| `HOUSE.H/CPP` | Player/faction, AI, economy, production, diplomacy — **central coordinator** |
| `CELL.H/CPP` | Map grid, terrain, shroud, pathfinding zones — **spatial foundation** |
| `COMQUEUE.H/CPP` | Network packet queues, reliability, latency — **multiplayer backbone** |
| `TECHNO.H/CPP` | Mobile object base: movement, orders, targeting, damage |
| `BUILDING.H/CPP` | Structures: production, power, repair, capture, sell |
| `BULLET.H/CPP` | Projectiles: ballistics, homing, area damage, warheads |
| `AIRCRAFT.H/CPP` | Air units: flight physics, landing, reloading |
| `TRIGGER.H/CPP` | Mission scripting: events, conditions, actions |
| `AUDIO.H/CPP` / `ANIM.H/CPP` | Presentation layer (sound, visual effects) |
| `TYPE.H` | Core type definitions: `fixed`, `COORDINATE`, `CELL`, enums |