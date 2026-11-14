# PATH/FILE
original/TIBERIANDAWN/SIDEBARGlyphx.CPP

## Purpose
GlyphX sidebar integration for Tiberian Dawn remastered. Handles sidebar rendering with production data export to GlyphX client. Supports multiple columns for buildings vs other object types. Windows-only implementation with C++ class hierarchy extending SidebarClass.

## Where it fits in the game
Sidebar infrastructure - renders player sidebar UI with production progress, tech buttons, and object selections. Two-column layout: Column 0 for buildings, Column 1 for units/other. Integrated with `STARTUP.CPP` game initialization and `INIT.CPP` scenario loading. Global `SidebarGlyphx` instance.

## Classes, structs, enums and important data

### SidebarGlyphxClass
**GlyphX sidebar handler** - Extended sidebar with production tracking.

**Inherits from:** `SidebarClass` (base sidebar UI class)

**Key data members:**
| Field | Purpose |
|-------|---------|
| `SidebarPlayerPtr` (HouseClass*) | Pointer to player house owning this sidebar |
| `Column[2]` (SidebarColumnClass[2]) | Two sidebar columns for object organization |
| `IsSidebarActive` (bool) | Whether sidebar is currently visible |
| `IsRepairActive` (bool) | Repair mode active flag |
| `IsUpgradeActive` (bool) | Upgrade mode active flag |
| `IsDemolishActive` (bool) | Demolish mode active flag |

### SidebarColumnClass
**Sidebar column** - One of two vertical columns in the sidebar.

**Key methods:**
- `Factory_Link(factory, type, id)` - Link factory production progress
- `Set_Parent_Sidebar(sidebar)` - Set parent sidebar reference

### RTTIType enum
| Constant | Type | Purpose |
|----------|------|---------|
| `RTTI_BUILDINGTYPE` | enum | Building type identifier |
| `RTTI_BUILDING` | enum | Building instance identifier |

## Functions and methods

### SidebarGlyphxClass::SidebarGlyphxClass()
**Constructor:**
1. Initializes `SidebarPlayerPtr = NULL`
2. Disables repair/upgrade/demolish flags (commented out)
3. Sets up default sidebar state

### SidebarGlyphxClass::Init_Clear(player_ptr)
**Reset sidebar to known state:**
1. Sets `SidebarPlayerPtr = player_ptr`
2. Resets column flags (repair/upgrade/demolish - commented)
3. Calls `Column[0].Set_Parent_Sidebar(this)`
4. Calls `Column[1].Set_Parent_Sidebar(this)`

### SidebarGlyphxClass::Init_IO()
**Add buttons to button list:**
- If game loaded and sidebar enabled: `Activate(1)`
- Currently disabled (commented out)

### SidebarGlyphxClass::Which_Column(type)
**Determine column for object type:**
- `type == RTTI_BUILDINGTYPE` or `type == RTTI_BUILDING` → Column 0
- Otherwise → Column 1

### SidebarGlyphxClass::Factory_Link(factory, type, id)
**Link factory to sidebar strip:**
1. Delegates to `Column[Which_Column(type)].Factory_Link(factory, type, id)`
2. Returns bool success/failure

### SidebarGlyphxClass::Add(object)
**Add game object to sidebar:**
1. Handles duplicate detection (ignores already-added items)
2. Returns bool success/failure

### SidebarColumnClass::Factory_Link(factory, type, id)
**Link factory to column:**
- Tracks factory production progress
- Updates sidebar display with current build state

## DDE Protocol Details (Sidebar Integration)

### Sidebar initialization
- `STARTUP.CPP` → `SidebarGlyphx.Init_Clear(player)`
- `INIT.CPP` → `SidebarGlyphx.Init_IO()`
- Multiplayer: one sidebar per player house

### Production tracking
- Factory buildings linked via `Factory_Link()`
- Production progress visible in sidebar column
- Column assignment based on `Which_Column(type)`

### Column layout
- Column 0: Buildings (Command Center, Barracks, Factory, etc.)
- Column 1: Units and other object types
- Dynamic: columns adjust based on object type RTTI

## Inputs, outputs and side effects

**State mutations:**
- `SidebarPlayerPtr` assignment
- `IsSidebarActive` flag toggle
- Column factory links and production state
- Button activation state

**External systems affected:**
- `STARTUP.CPP` game initialization
- `INIT.CPP` scenario loading
- `SIDEBAR.CPP` base sidebar rendering
- Multiplayer per-player sidebar instances

## Calls into / called by

**Called by:**
- `STARTUP.CPP` → `SidebarGlyphx.Init_Clear()`, `Init_IO()`
- `INIT.CPP` → sidebar setup during scenario load
- `SIDEBAR.CPP` → base sidebar rendering routines
- Column classes → production tracking callbacks

**Calls into:**
- `SidebarClass` base methods (inherited)
- `Which_Column()` for column assignment
- Column-specific Factory_Link implementations

## Related original files
- **SidebarGlyphx.H** - Class declarations
- **SidebarClass.H** - Base sidebar class
- **STARTUP.CPP** - `SidebarGlyphx.Init_Clear()`, `Init_IO()`
- **INIT.CPP** → sidebar setup during load
- **SIDEBAR.CPP** - Base sidebar rendering logic
- **ColumnClass** - Sidebar column implementations (left/right)

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/ui/sidebar_glyphx.rs` - SidebarGlyphxClass
- `cnc-tiberian-dawn/src/ui/sidebar_column.rs` - SidebarColumnClass production tracking

**Not yet ported:**
- Windows UI integration (`SidebarClass` hierarchy)
- Dual-column layout management
- Factory production progress tracking
- RTTI type checking for column assignment
- GlyphX client export protocol
- Sidebar button activation logic
- Per-player sidebar instances in multiplayer

## Edge cases and compatibility notes

1. **Windows-only**: Sidebar UI is Windows-specific GlyphX integration
2. **Two-column layout**: Rigid Column[0]/Column[1] split (buildings vs others)
3. **Factory linking**: Must call `Factory_Link()` for production progress to display
4. **Duplicate handling**: `Add()` silently ignores already-added objects
5. **Flags disabled**: Repair/upgrade/demolish flags currently commented out
6. **Multiplayer**: One sidebar per player house; `SidebarPlayerPtr` tracks owner
7. **RTTI-based dispatch**: `Which_Column()` uses `RTTIType` for column selection
8. **Init sequence**: `Init_Clear()` → `Init_IO()` is the standard initialization path
9. **Player pointer**: `SidebarPlayerPtr` can default to `PlayerPtr` if NULL
10. **Commented-out features**: Repair/upgrade/demolish modes not currently active

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SIDEBARGlyphx.CPP at f1f0d42
