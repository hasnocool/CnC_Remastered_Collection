# PATH/FILE
original/TIBERIANDAWN/SAVELOAD.CPP

## Purpose
Save/load game system for Tiberian Dawn. Implements complete game state serialization including map, all game objects, houses, teams, triggers, layers, and miscellaneous game state. Handles pointer encoding/decoding for save/load cycles.

## Where it fits in the game
Persistence system - enables saving and loading complete game states. Handles the complex pointer relocation required for C++ object graphs with virtual functions and inter-object references.

## Classes, structs, enums and important data

### Key constants
- `SAVEGAME_VERSION` - Version code: `DESCRIP_MAX + 0x01000003 + sum of all class sizes`
- `DESCRIP_MAX` - Description buffer size
- `SAVEGAME_VERSION` includes sizes of all serializable classes

### Global functions

#### Save_Game(id, descr) → bool
Saves game to `SAVEGAME.%03d` file.

**Process:**
1. **Pointer encoding:** `Code_All_Pointers()` - converts all pointers to TARGET values
2. **DLL save:** `DLLSave()` if running as DLL
3. **Header:** Writes description (with CR/LF + Ctrl-Z), scenario #, house, version
4. **Map save:** `Map.Save()` - saves theater, cells, triggers
5. **Object arrays:** Saves all `TFixedIHeap` arrays:
   - Houses, TeamTypes, Teams, Triggers
   - Aircraft, Anims, Buildings, Bullets
   - Infantry, Overlays, Smudges, Templates
   - Terrains, Units, Factories
6. **Layers:** Logic + Map layers (Ground, Air, Top)
7. **Score:** `Score.Save()`
8. **AI Base:** `Base.Save()`
8. **Misc values:** `Save_Misc_Values()`

#### Load_Game(id/filename) → bool
Loads game from `SAVEGAME.%03d` file.

**Process:**
1. **DLL load:** `DLLLoad()` if DLL
2. **Header read:** Description, scenario, house
3. **Clear scenario:** `Clear_Scenario()` - resets all systems
7. **Version check:** Verifies `SAVEGAME_VERSION`
8. **CD check:** Verifies required CD for scenario
9. **Map load:** `Map.Load()` - loads theater, mixfiles, cells
11. **Object arrays:** Loads all heaps in order
12. **Layers:** Logic + Map layers
13. **Score:** `Score.Load()`
13. **AI Base:** `Base.Load()`
13. **Misc values:** `Load_Misc_Values()`
13. **Pointer decoding:** `Decode_All_Pointers()`
14. **IO init:** `Map.Init_IO()`, `Map.Flag_To_Redraw()`
14. **Scenario fixup:** `Fixup_Scenario()`
14. **Unit trackers:** Re-initializes house unit trackers
15. **DEMO check:** Validates scenario for demo

#### Save_Misc_Values(file) → bool
Saves global game state:
- Frame counter, scenario, random seed
- Game mode, theater, difficulty
- Player house, difficulty, map dimensions
- Special flags (TGrowth, TSpread, TFast, etc.)
- PlayerPtr (encoded as TARGET)
- CurrentObject selections
- Colors, remap tables, game options
- Scores, stats, timestamps

#### Load_Misc_Values(file) → bool
Loads all miscellaneous variables in same order.

### Pointer encoding/decoding

#### Code_All_Pointers()
Encodes all pointers to TARGET values for save:
1. Map cells: `Code_Pointers()`
2. Logic layer: `Code_Pointers()`
2. Map layers: `Code_Pointers()`
2. Object arrays: `Code_Pointers()`
3. Houses, TeamTypes, Teams, Triggers, etc.
3. Layers: `Code_Pointers()`
3. Score, Base
4. DLL data

#### Decode_All_Pointers()
Decodes all TARGETs back to pointers:
1. Map: `Decode_Pointers()`
2. Houses (first - needed for other objects)
3. DLL data
4. ArrayOf's: TeamTypes, Teams, Triggers, Aircraft, Anims, Buildings, Bullets, Infantry, Overlays, Smudges, Templates, Terrains, Units, Factories
4. Layers: Logic, Map layers
4. Score, Base
4. PlayerPtr: Converts TARGET to HouseClass*
5. CurrentObject selections: Converts TARGETs to ObjectClass*
5. Map.PendingObject fixup

### Pointer encoding details

**TARGET encoding:** `Build_Target(kind, id)` - encodes kind + ID into 32-bit value
**Pointer encoding:** `Code_Pointers()` converts all raw pointers to TARGETs
**Pointer decoding:** `Decode_Pointers()` converts TARGETs back to pointers

**Key functions:**
- `Build_Target(kind, id)` - Creates TARGET from kind + ID
- `Target_Kind(target)` - Extracts kind
- `Target_Value(target)` - Extracts ID
- `As_Object(target)` - Converts TARGET → ObjectClass*
- `As_Techno(target)` - Converts TARGET → TechnoClass*

### Save file format

**Header:**
- Description (DESCRIP_MAX + CR/LF + Ctrl-Z)
- Scenario number (unsigned)
- House (HousesType)
- Version (SAVEGAME_VERSION)

**Body (sequential):**
1. Map (theater, cells, triggers)
2. Object arrays (Houses, TeamTypes, Teams, Triggers, Aircraft, Anims, Buildings, Bullets, Infantry, Overlays, Smudges, Templates, Terrains, Units, Factories)
3. Layers (Logic + Map layers)
4. Score
5. AI Base
6. Misc values

### Save game version

`SAVEGAME_VERSION = DESCRIP_MAX + 0x01000003 + sum of sizeof(all serializable classes)`

Includes sizes of: AircraftClass, AircraftTypeClass, AnimClass, AnimTypeClass, BuildingClass, BuildingTypeClass, BulletClass, BulletTypeClass, HouseClass, HouseTypeClass, InfantryClass, InfantryTypeClass, OverlayClass, OverlayTypeClass, SmudgeClass, SmudgeTypeClass, TeamClass, TeamTypeClass, TemplateClass, TemplateTypeClass, TerrainClass, TerrainTypeClass, UnitClass, UnitTypeClass, MouseClass, CellClass, FactoryClass, BaseClass, LayerClass, BriefingText, Waypoint

### INI Save/Load (HouseClass, TeamClass, UnitClass, etc.)

**HouseClass::Read_INI/Write_INI** - Scenario INI format
- House control settings: TechLevel, MaxBuilding/Unit/Infantry/Vessel, InitialCredits, IQ, Edge, PlayerControl, Allies
- Color remap, ActLike, Credits

**TeamClass::Read_INI/Write_INI**
- Team name, event, action, data, house, team, persistence

**UnitClass::Read_INI/Write_INI**
- Format: `Housename, Typename, Strength, Coord, Facing, Missionname, Triggername`

**TriggerClass::Read_INI/Write_INI**
- Format: `Triggername = Eventname, Actionname, Data, Housename, TeamName, IsPersistant`

### Custom allocator support

**Pool allocation:** All serializable classes use custom `operator new/delete` with pool allocators (`Houses.Allocate()`, `Teams.Allocate()`, etc.)

**Read_Object/Write_Object:** Handle vtable pointer correctly (skips 4-byte vtable pointer at offset base_size-4)

### DLL integration

**DLLSave/DLLLoad:** Saves/loads DLL-specific state first (version check)

**RunningAsDLL** flag controls DLL save/load path

### CD verification

**RequiredCD logic:**
- Scenarios 20-59 (TD campaign): CD 2
- Scenarios 60+ (bonus): CD -1 (gateway)
- GDI campaign: CD 0
- NOD campaign: CD 1

### Version compatibility

**SAVEGAME_VERSION** includes sum of all class sizes - any class size change invalidates saves

### Multiplayer considerations

**Load_Misc_Values:** Handles `PlayerPtr` decode, `MPlayerID`, `MPlayerNames`, `MPlayerTColors`

### DEMO mode restrictions

**Load_Game:** Blocks scenarios not in {1, 6, 10} for demo

### Fixup after load

**Fixup_Scenario():** Post-load scenario fixes
**Init_Unit_Trackers():** Rebuilds house unit trackers
**Decode_All_Pointers:** Converts all TARGETs to pointers
**Map.Init_IO():** Initializes map I/O
**Map.Flag_To_Redraw(true):** Forces redraw

### Object serialization

**Read_Object/Write_Object:** Handle vtable pointer correctly
- Skips 4-byte vtable pointer at `base_size - 4`
- Reads/writes `class_size - vtable_adjust` bytes
- `has_vtable` parameter controls vtable handling

### Error handling

**Validation:** Version check, size verification, pointer validation
**Rollback:** `Decode_All_Pointers()` on failure before returning false
**CD check:** `Force_CD_Available()` before loading

### Key constants

- `SAVEGAME_VERSION` - Includes all class sizes for versioning
- `DESCRIP_MAX` - Description buffer size
- `SAVEGAME_VERSION` formula ensures save compatibility
- `PIXELSTACK` - Radar pixel stack size

## Inputs, outputs and side effects

**State mutations:**
- Complete game state persisted/restored
- Pointer encoding/decoding
- File I/O

**External systems affected:**
- All game systems (map, objects, houses, teams, etc.)
- File system (save/load files)
- CD drive verification

## Calls into / called by

**Called by:**
- Main menu: Save/Load game
- Scenario editor: Save/Load scenarios
- Game loop: Auto-save (if implemented)

**Calls into:**
- All major systems: Map, Houses, Teams, Units, Buildings, etc.
- File I/O: `FileClass` (RawFileClass)
- DLL: `DLLSave`, `DLLLoad`
- Pointer encoding: `Code_All_Pointers`, `Decode_All_Pointers`
- CD: `Force_CD_Available`, `RequiredCD`

## Related original files
- **SAVELOAD.H** - Function declarations
- **FUNCTION.H** - Core includes, FileClass
- **MAP.CPP/.H** - MapClass save/load
- **HOUSE.CPP/.H** - HouseClass save/load
- **TEAM.CPP/.H** - TeamClass save/load
- **ALL OBJECT CLASSES** - Each has Read/Write methods
- **FUNCTION.H** - Core includes, FileClass, Code_All_Pointers
- **SIDEBAR.CPP** - Sidebar state
- **SCORE.CPP** - Score save/load
- **BASE.CPP** - AI Base save/load

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/savegame.rs` - Save/load core
- `cnc-tiberian-dawn/src/savegame/td_save.rs` - TD-specific
- `cnc-core/src/serialization.rs` - Pointer encoding/decoding
- `cnc-core/src/savegame/version.rs` - Version handling
- `cnc-tiberian-dawn/src/savegame/td_format.rs` - TD save format

**Not yet ported:**
- Pointer encoding/decoding (TARGET system)
- Class size versioning (SAVEGAME_VERSION)
- Multi-heap object serialization (TFixedIHeap)
- Pointer relocation (Code_All_Pointers/Decode_All_Pointers)
- DLL state save/load
- CD verification
- DEMO mode restrictions
- Post-load fixup (Fixup_Scenario, Init_Unit_Trackers)
- Custom object serialization (Read_Object/Write_Object with vtable handling)
- INI serialization for all classes
- Map binary format (Read_Binary/Write_Binary)

## Edge cases and compatibility notes

1. **Version fragility:** Any class size change breaks save compatibility. `SAVEGAME_VERSION` includes sum of all class sizes.

2. **Pointer encoding:** All pointers converted to TARGETs (kind + ID). Requires all objects to have stable IDs.

3. **Decode order matters:** Houses must decode first (other objects reference houses).

4. **DLL state:** Must save/load DLL state first for version compatibility.

4. **House decode first:** Other objects reference houses via pointers.

5. **DEMO restrictions:** Blocks non-demo scenarios in demo mode.

5. **Demo mode:** Only allows scenarios 1, 6, 10.

6. **CD check:** Verifies correct CD before loading campaign missions.

6. **CD check:** Campaign missions require correct CD.

7. **Post-load fixup:** `Fixup_Scenario()`, `Init_Unit_Trackers()`, `Decode_All_Pointers()`, `Map.Init_IO()`

7. **Post-load:** Fixup, tracker init, pointer decode, map I/O init.

8. **Version includes class sizes:** Any class layout change breaks saves.

8. **Version fragility:** Class size sum in version breaks on any layout change.

9. **Custom allocators:** Pool allocation means object addresses aren't stable across saves.

9. **Pool allocators:** Object addresses not stable; IDs used instead.

10. **VTable handling:** Read/Write_Object skip 4-byte vtable at `base_size - 4`.

10. **VTable skip:** 4-byte vtable at `base_size - 4` skipped in serialization.

11. **Multiplayer save:** PlayerPtr, MPlayerID, MPlayerNames saved/restored.

11. **MP state:** PlayerPtr, MPlayerID, names saved.

12. **INI format:** House/Team/Unit/Trigger INI serialization for editor.

12. **INI format:** House/Team/Unit/Trigger INI serialization.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SAVELOAD.CPP at f1f0d42
