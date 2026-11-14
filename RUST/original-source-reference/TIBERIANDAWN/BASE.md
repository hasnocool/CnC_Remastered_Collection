# PATH/FILE
original/TIBERIANDAWN/BASE.CPP

## Purpose
AI base layout system for Tiberian Dawn. Defines `BaseClass` which manages a list of planned building positions (`BaseNodeClass`) for computer players, with INI persistence, save/load serialization, and build status tracking.

## Where it fits in the game
AI subsystem - Each computer house has a `BaseClass` instance (global `Base` in GLOBALS.CPP). Used by AI building logic to determine what to build next and where. Reads/writes scenario INI `[Base]` section.

## Classes, structs, enums and important data

### BaseNodeClass
**Single planned building position** - Type + coordinate.

**Fields:**
- `Type` (StructType) - Building type (STRUCT_POWER, STRUCT_BARRACKS, etc.)
- `Coord` (COORDINATE) - World coordinate of building placement

**Operators:**
- `operator==` - Compares Type and Coord
- `operator!=` - Inverse
- `operator>` - Always returns true (placeholder for sorting)

### BaseClass
**AI base plan** - Ordered list of BaseNodeClass.

**Key data members:**
- `Nodes` (DynamicVectorClass<BaseNodeClass>) - Planned buildings in build order
- `House` (HousesType) - Owner house (GDI/NOD)

**Global instance:** `Base` (GLOBALS.CPP)

## Functions and methods

### BaseClass::Read_INI(buffer)
**Loads base plan from scenario INI** (`[Base]` section).

**INI format:**
```ini
[Base]
Count=5
000=STRUCT_POWER,123456
001=STRUCT_BARRACKS,234567
...
```

**Process:**
1. Reads `[BASIC] Player` to determine human house
2. Sets `House` to opposite (AI house)
3. Reads `Count` from `[Base]`
4. Loops 0..Count-1: reads `###=TYPE,COORD`, parses with `strtok`
5. `BuildingTypeClass::From_Name()` for type, `atol()` for coord
6. `Nodes.Add(node)` in order

### BaseClass::Write_INI(buffer)
**Saves base plan to scenario INI.**

1. Clears `[Base]` section
2. Writes `Count=Nodes.Count()`
3. Loops nodes: writes `###=IniName,Coord`

### BaseClass::Load(file) → bool
**Save game deserialization.**

1. Reads class size (validates `sizeof(*this)`)
2. Reads `House`, `num_struct`
3. Loops `num_struct`: reads `BaseNodeClass`, `Nodes.Add()`
4. Returns true on success

### BaseClass::Save(file) → bool
**Save game serialization.**

1. Writes class size, `House`, `num_struct`
2. Loops nodes: writes each `BaseNodeClass`
3. Returns true on success

### BaseClass::Is_Built(index) → bool
**Checks if planned building exists** - Calls `Get_Building(index) != NULL`

### BaseClass::Get_Building(index) → BuildingClass*
**Finds built building at planned position.**

1. `cell = Coord_Cell(Nodes[index].Coord)`
2. Checks `Map[cell].Cell_Building()` and 3 `Overlapper` slots
3. Matches: `obj->Coord == node.Coord`, `RTTI_BUILDING`, `Class->Type == node.Type`
4. Returns building or NULL

### BaseClass::Is_Node(building) → bool
**Checks if building is in plan** - Calls `Get_Node(building) != NULL`

### BaseClass::Get_Node(building) → BaseNodeClass*
**Finds plan entry for existing building.**

1. Loops `Nodes`: matches `obj->Class->Type == node.Type && obj->Coord == node.Coord`
2. Returns pointer or NULL

### BaseClass::Next_Buildable(type) → BaseNodeClass*
**Finds next unbuilt planned building.**

- If `type == STRUCT_NONE`: Returns first unbuilt of any type
- Else: Returns first unbuilt matching `type`
- Returns NULL if none (AI must place freely)

## INI section name
`INI_Name()` returns `"Base"` (from BASE.H)

## Inputs, outputs and side effects

**State mutations:**
- `Nodes` vector (Add during Read_INI/Load)
- `House` assignment

**External systems affected:**
- AI building logic (uses `Next_Buildable`)
- Scenario editor (Read/Write_INI)
- Save/load system

## Calls into / called by

**Called by:**
- Scenario load → `Read_INI`
- Scenario save/editor → `Write_INI`
- Save game → `Load`/`Save`
- AI building logic → `Next_Buildable`, `Is_Built`, `Get_Building`

**Calls into:**
- `WWGetPrivateProfileString/Int` (INI parsing)
- `BuildingTypeClass::From_Name/As_Reference`
- `DynamicVectorClass::Add/Count`
- `FileClass::Read/Write`
- `Map` cell access (`Cell_Building`, `Overlapper`)
- `Coord_Cell`

## Related original files
- **BASE.H** - Class declarations, INI_Name()
- **BUILDING.H/.CPP** - BuildingTypeClass, BuildingClass
- **HOUSE.H/.CPP** - HouseTypeClass
- **MAP.H/.CPP** - MapClass, CellClass
- **INI.H/.CPP** - INI functions
- **SCENARIO.H/.CPP** - Scenario loading
- **SAVELOAD.H/.CPP** - Save/Load
- **GLOBALS.CPP** - `BaseClass Base;` global

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/base.rs` - BaseClass, BaseNodeClass
- `cnc-tiberian-dawn/src/ai/td_base.rs` - TD-specific building types
- `cnc-core/src/ai/ai_builder.rs` - AI building logic consumer

**Not yet ported:**
- DynamicVectorClass for Nodes
- INI parsing with `strtok` format
- Save/load binary serialization with size validation
- Map cell overlapper check (4 slots per cell)
- House auto-assignment (opposite of human player)
- Next_Buildable with type filtering
- STRUCT_NONE wildcard

## Edge cases and compatibility notes

1. **Single base per scenario**: INI assumes one `[Base]` section
2. **Build order matters**: Nodes must be read/written in same order (000, 001, ...)
3. **House assignment**: Automatic based on human player (GoodGuy → AI=Bad)
4. **Overlapper check**: Buildings can be in Cell_Building or 3 Overlapper slots
5. **Next_Buildable NULL**: AI falls back to free placement
6. **Save/load size check**: Validates `sizeof(*this)` for version safety
7. **Static fake node**: Comment mentions creating fake node for placement (not implemented)
8. **Operator> always true**: Placeholder for potential sorting

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/BASE.CPP at f1f0d42
