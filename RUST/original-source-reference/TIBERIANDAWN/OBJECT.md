# PATH/FILE
original/TIBERIANDAWN/OBJECT.CPP

## Purpose
Base class implementation for all game objects in Tiberian Dawn. Provides the foundational object lifecycle, state management, damage handling, rendering, and interaction systems that all game entities (units, buildings, infantry, aircraft, etc.) inherit from.

## Where it fits in the game
Core object system - the root of the entire game entity hierarchy. Every tangible game piece (units, buildings, infantry, aircraft, bullets, animations) inherits from ObjectClass or its derivatives (TechnoClass, BuildingClass, etc.). This is the backbone of the game's entity-component system.

## Classes, structs, enums and important data

### ObjectClass
**Core instance class** - represents individual game object instances at runtime.

**Key data members:**
- `Coord` (COORDINATE) - Current map position in leptons (1 lepton = 1/256 cell)
- `Strength` (int) - Current hit points (0 = destroyed, 1 = critical, max = full)
- `IsDown` (bool) - Whether object is placed on map (true) or in limbo/air (false)
- `IsInLimbo` (bool) - Object not in game world (being constructed, dead, or stored)
- `IsToDisplay` (bool) - Flagged for redraw this frame
- `IsInLimbo` (bool) - Object not in game systems
- `IsActive` (bool) - Object participates in game logic
- `IsSelected` / `IsSelectedMask` - Selection state per player (bitmask for MP)
- `CurrentObject` - Per-house selection lists (head/tail pointers)
- `Owner()` / `House` - Owning house/player
- `Class_Of()` - Returns associated ObjectTypeClass

### ObjectTypeClass
**Static type definition class** - shared data for all instances of an object type.

**Key data members:**
- `IsSentient` (bool) - Can receive AI orders (units, infantry)
- `IsFlammable` - Takes fire damage
- `IsCrushable` - Can be crushed by heavy units
- `IsStealthy` - Stealth capable
- `IsSelectable` - Can be player-selected
- `IsLegalTarget` - Can be targeted by weapons
- `IsInsignificant` - Ignored by AI targeting
- `IsImmune` - Immune to all damage
- `Armor` (ArmorType) - Armor classification for damage modification
- `MaxStrength` (int) - Maximum hit points
- `ImageData` - Pointer to SHAPE animation data
- `SelectShapes` / `PipShapes` - Static selection/health UI shapes
- `Armor` (ArmorType) - Damage modification table lookup

### MarkType enum
- `MARK_DOWN` - Place object on map (add to map, logic, threat maps)
- `MARK_UP` - Remove from map (inverse of MARK_DOWN)
- `MARK_CHANGE` - Flag for redraw (visual state changed)
- `MARK_OVERLAP_UP/DOWN` - Overlap list management

### ResultType enum (damage results)
- `RESULT_NONE` - No damage applied
- `RESULT_LIGHT` - Minor damage
- `RESULT_HALF` - Crossed 50% health threshold
- `RESULT_MAJOR` - Reduced to 1 HP
- `RESULT_DESTROYED` - Object destroyed

## Functions and methods

### ObjectTypeClass

**ObjectTypeClass::ObjectTypeClass()** - Constructor
- Initializes all boolean flags, armor, strength
- Called by derived type constructors (TechnoTypeClass, BuildingTypeClass, etc.)
- Parameters: sentient, flammable, crushable, stealthy, selectable, legal_target, insignificant, immune, name, ini, armor, strength

**ObjectTypeClass::Max_Pips()** → int
- Returns maximum health pips for UI display
- Base implementation returns 0 (overridden by TechnoTypeClass)

**ObjectTypeClass::Dimensions(width, height)** → void
- Returns object dimensions in pixels
- Base returns 10x10 (overridden by derived classes)

**ObjectTypeClass::Cost_Of()** → int
- Purchase cost in credits
- Base returns 0 (overridden by purchasable types)

**ObjectTypeClass::Time_To_Build(house)** → int
- Build time in arbitrary time units
- Base returns 0 (overridden by constructible types)

**ObjectTypeClass::Who_Can_Build_Me()** → BuildingClass*
- Returns factory that can build this type
- Base returns NULL (overridden by constructible types)

**ObjectTypeClass::Get_Cameo_Data()** → void const*
- Returns sidebar cameo shape pointer
- Base returns NULL (overridden by sidebar-visible types)

**ObjectTypeClass::One_Time()** - Static initialization
- Loads SELECT.SHP and PIPS.SHP (localized variants: PIPS_F.SHP French, PIPS_G.SHP German)
- Called once at game startup

### ObjectClass

**ObjectClass::ObjectClass()** - Default constructor
- Initializes all member variables to default state
- Object starts in LIMBO state (must call Unlimbo() before use)
- Sets IsInLimbo = true, IsDown = false, Strength = 0

**ObjectClass::Init()** - System initialization
- Clears CurrentObject selection lists
- Called at scenario load

**ObjectClass::Limbo()** → bool
- Removes object from all game systems (map, logic, selection, teams, etc.)
- Calls Detach_All(), Mark(MARK_UP), Map.Remove()
- Sets IsInLimbo = true, IsToDisplay = false
- Returns true if successfully placed in limbo
- **Precondition**: GameActive && !IsInLimbo

**ObjectClass::Unlimbo(coord, dir)** → bool
- Places object into game systems at coordinate
- Validates cell entry (Can_Enter_Cell == MOVE_OK)
- Sets IsInLimbo = false, Coord = Class_Of().Coord_Fixup(coord)
- Calls Mark(MARK_DOWN), Map.Submit(), Logic.Submit() if sentient
- **Precondition**: GameActive && IsInLimbo && !IsDown

**ObjectClass::Detach_All(all)** - Remove from all tracking
- Unselects object (all players or non-owner)
- Map.Detach(), Detach_This_From_All()
- Iterates all houses, teams, units, infantry, aircraft, buildings, bullets, anims
- Calls Detach(target, all) on each

**ObjectClass::Detach_This_From_All(target, all)**
- Sweeps all game object arrays (Houses, Teams, Units, Infantry, Aircraft, Buildings, Bullets, Anims)
- Calls Detach(target, all) on each object

**ObjectClass::Receive_Message(message, ...)** → RadioMessageType
- Handles RADIO_REDRAW → calls Mark(MARK_CHANGE), returns RADIO_ROGER
- Other messages return RADIO_STATIC

**ObjectClass::Take_Damage(damage, distance, warhead, source)** → ResultType
- **Core damage application logic**
- Returns RESULT_NONE if immune, no damage, or no strength
- Modifies damage: `Modify_Damage(damage, warhead, Armor, distance)`
- Damage never below 1 if any damage gets through
- Tracks old strength, applies damage: `Strength = oldstrength - damage`
- **Strength thresholds:**
  - Crosses 50% → RESULT_HALF
  - Reduced to 1 HP → RESULT_MAJOR
  - Strength ≤ 0 → RESULT_DESTROYED (calls Record_The_Kill, Detach_All())
- Triggers EVENT_ATTACKED on source if not destroyed
- Updates selection health bar if selected

**ObjectClass::Mark(MarkType)** → bool
- **Central marking logic** - handles all map state transitions
- **MARK_CHANGE**: Flags redraw if IsDown && !IsToDisplay
- **MARK_OVERLAP_UP/DOWN**: Overlap list management
- **MARK_DOWN**: 
  - Adds to map layer (Map.Submit)
  - Adds to logic if sentient (Logic.Submit)
  - Updates threat map: Map[cell].Adjust_Threat(house, threat)
  - Sets IsDown = true, flags redraw
- **MARK_UP**: 
  - Removes from map layer (Map.Remove via Mark_For_Redraw path)
  - Adjusts threat map: Map[cell].Adjust_Threat(house, -threat)
  - Calls Map.Overlap_Up()
  - Sets IsDown = false
- **Regional threat**: Only for sentient objects in GAME_NORMAL mode

**ObjectClass::Init()** - System initialization
- Clears CurrentObject selection lists

**ObjectClass::Revealed(house)** → bool
- Returns true if valid house pointer
- Base implementation always reveals

**ObjectClass::Set_Selected_By_Player(player)** - Multiplayer selection
- Adds to CurrentObject list (head if leader, tail otherwise)
- Sets bit in IsSelectedMask per house
- Updates IsSelected for local player

**ObjectClass::Set_Unselected_By_Player(player)** - Multiplayer deselection
- Removes from CurrentObject, clears IsSelectedMask bit

**ObjectClass::Is_Selected_By_Player(player)** → bool
- Checks IsSelectedMask bit for house

**ObjectClass::Mark_For_Redraw()**
- Sets IsToDisplay = true, calls Map.Flag_To_Redraw(false)

**ObjectClass::Limbo()** → bool
- Removes from all systems: Unselect, Detach_All, Mark(MARK_UP), Map.Remove, Logic.Delete
- Sets IsInLimbo = true, IsToDisplay = false
- Returns true if was active

**ObjectClass::Unlimbo(coord, dir)** → bool
- Places object at coordinate if Can_Enter_Cell == MOVE_OK
- Calls Mark(MARK_DOWN), Map.Submit, Logic.Submit if sentient
- Returns true if successful

**ObjectClass::Detach_All(all)** - Remove from all tracking
- Unselect (all or non-owner), Map.Detach, Detach_This_From_All

**ObjectClass::Detach_This_From_All(target, all)**
- Iterates all 8 object arrays (Houses, Teams, Units, Infantry, Aircraft, Buildings, Bullets, Anims)
- Calls Detach(target, all) on each

**ObjectClass::Receive_Message(message, ...)** → RadioMessageType
- Handles RADIO_REDRAW → Mark(MARK_CHANGE), returns RADIO_ROGER

**ObjectClass::Take_Damage(damage, distance, warhead, source)** → ResultType
- Returns RESULT_NONE if immune, no damage, or no strength
- Damage modified by: `Modify_Damage(damage, warhead, Armor, distance)`
- Damage clamped to minimum 1
- Strength reduced, thresholds checked:
  - Crosses 50% max → RESULT_HALF
  - Reduced to 1 HP → RESULT_MAJOR
  - Destroyed (≤0) → RESULT_DESTROYED (Record_The_Kill, Detach_All)
- EVENT_ATTACKED trigger on source if not destroyed
- Selection health bar update if selected

**ObjectClass::Mark(MarkType)** → bool
- **Core marking logic** - handles all map state transitions
- MARK_CHANGE: flags redraw if IsDown && !IsToDisplay
- MARK_OVERLAP_UP/DOWN: overlap list management
- MARK_DOWN: Map.Submit, Logic.Submit (if sentient), threat adjustment, IsDown=true
- MARK_UP: Map.Remove, threat adjustment (negative), Overlap_Up, IsDown=false
- Threat map: Map[cell].Adjust_Threat(house, ±threat) for sentient objects

**ObjectClass::Init()** - Clears CurrentObject selection lists

**ObjectClass::Revealed(house)** → bool - Returns true if valid house

**ObjectClass::Set_Selected_By_Player(player)** - MP selection
- Adds to CurrentObject (head if leader, else tail)
- Sets IsSelectedMask bit for house

**ObjectClass::Set_Unselected_By_Player(player)** - MP deselection
- Removes from CurrentObject, clears mask bit

**ObjectClass::Is_Selected_By_Player(player)** → bool - Checks mask bit

**ObjectClass::Mark_For_Redraw()** - Sets IsToDisplay=true, Map.Flag_To_Redraw()

**ObjectClass::Limbo()** → bool - Removes from all systems, sets IsInLimbo=true

**ObjectClass::Unlimbo(coord, dir)** → bool - Places object, validates Can_Enter_Cell

**ObjectClass::Detach_All(all)** - Unselect, Map.Detach, Detach_This_From_All

**ObjectClass::Detach_This_From_All(target, all)** - Sweeps all 8 object arrays

**ObjectClass::Receive_Message(message, ...)** → RadioMessageType
- RADIO_REDRAW → Mark(MARK_CHANGE), RADIO_ROGER

**ObjectClass::Take_Damage(damage, distance, warhead, source)** → ResultType
- Damage pipeline: immune check → Modify_Damage → clamp to ≥1 → apply → threshold checks → destroy/major/half/light → triggers → selection update

**ObjectClass::Mark(MarkType)** → bool
- MARK_CHANGE: redraw flag
- MARK_OVERLAP_UP/DOWN: overlap lists
- MARK_DOWN: Map.Submit, Logic.Submit, threat+, IsDown=true
- MARK_UP: Map.Remove, threat-, Overlap_Up, IsDown=false
- Regional threat only for sentient in GAME_NORMAL

**ObjectClass::Init()** - CurrentObject.Clear_All()

**ObjectClass::Revealed(house)** → bool - true if house != NULL

**Selection MP helpers**: Set_Selected_By_Player, Set_Unselected_By_Player, Is_Selected_By_Player

**Inline helpers** (from header):
- `Occupy_List(placement)` → Class_Of().Occupy_List(placement)
- `Overlap_List()` → Class_Of().Overlap_List()
- `Who_Can_Build_Me(intheory, legal)` → Class_Of().Who_Can_Build_Me(...)
- `Health_Ratio()` → Cardinal_To_Fixed(MaxStrength, Strength)
- `Full_Name()` → Class_Of().Full_Name()

## Inputs, outputs and side effects

**State mutations:**
- Limbo/Unlimbo: IsInLimbo, IsDown, IsToDisplay, map/logic membership
- Mark: IsDown, IsToDisplay, threat maps, map layers, overlap lists
- Take_Damage: Strength, IsDown (if destroyed), selection health bar
- Selection: IsSelectedMask, CurrentObject lists, IsSelected (local)
- Limbo/Unlimbo: IsInLimbo, IsDown, IsToDisplay, map/logic lists, threat maps

**External systems affected:**
- Map: Submit, Remove, Flag_To_Redraw, Overlap_Up/Down, threat maps
- Logic: Submit, Delete
- Teams/Houses/Units/Infantry/Aircraft/Buildings/Bullets/Anims: Detach()
- Trigger system: EVENT_ATTACKED
- Selection UI: health bar updates

## Calls into / called by

**Called by:**
- Scenario loading: Init()
- Object creation: Unlimbo()
- Object deletion: Limbo()
- Damage system: Take_Damage()
- Rendering: Mark_For_Redraw(), Mark()
- Map system: Submit(), Remove(), threat adjustments
- Logic system: Submit(), Delete()
- Selection UI: Set_Selected_By_Player(), Mark_For_Redraw()
- Radio system: Receive_Message()
- Damage system: Take_Damage() → Modify_Damage() → Record_The_Kill()

**Calls into:**
- Map: Submit(), Remove(), Flag_To_Redraw(), Overlap_Up/Down(), Adjust_Threat(), Remove(), Detach()
- Logic: Submit(), Delete()
- MapClass: Overlap_Up(), Overlap_Down(), Adjust_Threat(), [cell].Adjust_Threat()
- Class_Of(): Occupy_List(), Overlap_List(), Who_Can_Build_Me(), MaxStrength, Armor, Full_Name()
- Modify_Damage(), Record_The_Kill(), Detach_All(), Trigger->Spring()

## Related original files
- **OBJECT.H** - Class declarations, inline helpers
- **TECHNO.CPP/.H** - TechnoClass (mobile objects)
- **BUILDING.CPP/.H** - BuildingClass (static structures)
- **UNIT.CPP/.H** - UnitClass (mobile ground units)
- **INFANTRY.CPP/.H** - InfantryClass (foot soldiers)
- **AIRCRAFT.CPP/.H** - AircraftClass (air units)
- **BUILDING.CPP/.H** - BuildingClass
- **BULLET.CPP/.H** - Projectiles
- **ANIM.CPP/.H** - Animations
- **MAP.CPP/.H** - MapClass (map layers, threat, overlap)
- **LOGIC.CPP** - LogicClass (AI processing)
- **TRIGGER.CPP** - Trigger system (EVENT_ATTACKED)
- **FUNCTION.H** - Core includes, Modify_Damage(), Record_The_Kill()
- **RADIO.CPP** - Radio messaging

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/entity/object.rs` - ObjectClass base entity
- `cnc-core/src/entity/object_type.rs` - ObjectTypeClass static data
- `cnc-tiberian-dawn/src/entity/td_object.rs` - TD-specific extensions
- `cnc-core/src/map/map.rs` - MapClass integration (Submit, threat, overlap)
- `cnc-core/src/entity/selection.rs` - CurrentObject, IsSelectedMask
- `cnc-core/src/damage.rs` - Modify_Damage, Take_Damage pipeline
- `cnc-core/src/map/threat.rs` - Threat map Adjust_Threat
- `cnc-core/src/entity/limbo.rs` - Limbo/Unlimbo lifecycle
- `cnc-events/src/event.rs` - EVENT_ATTACKED trigger
- `cnc-core/src/damage.rs` - Modify_Damage, Record_The_Kill
- `cnc-core/src/map/map.rs` - MapClass integration

**Not yet ported:**
- Multiplayer selection mask (IsSelectedMask per-house bitmask)
- Radio messaging (RADIO_REDRAW)
- Limbo/Unlimbo lifecycle coordination with map/logic
- Regional threat map adjustments
- Overlap list management
- Selection health bar updates

## Edge cases and compatibility notes

1. **Limbo state**: Objects always start in limbo. Must call Unlimbo() before participating in game. Limbo() can be called multiple times safely (idempotent check).

2. **Strength = 1 special case**: When damaged to 0, strength is clamped to 1 and RESULT_MAJOR returned. Next damage destroys (RESULT_DESTROYED).

3. **Damage minimum**: Modify_Damage can reduce to 0, but Take_Damage clamps to minimum 1 if any damage gets through.

4. **MARK_CHANGE vs MARK_DOWN/UP**: MARK_CHANGE only flags redraw if already down. MARK_DOWN/MARK_UP manage map/logic membership.

5. **Multiplayer selection**: IsSelectedMask is a bitmask (1 bit per house, max 8 houses). CurrentObject maintains per-house doubly-linked lists. Leader units added to head, others to tail.

6. **Regional threat**: Only adjusted for sentient objects in GAME_NORMAL mode. Threat value from TechnoClass::Risk().

7. **Strength = 1 special**: Object at 1 HP shows as RESULT_MAJOR. Next damage kills (RESULT_DESTROYED).

8. **Radar icons**: Commented out in source (//RadarIcon = NULL). Not implemented in TD.

9. **French/German localization**: PIPS.SHP has PIPS_F.SHP (French) and PIPS_G.SHP (German) variants loaded in One_Time().

10. **RadarIcon**: Commented out in constructor. Not used in TD.

11. **IsImmune**: Completely blocks all damage (RESULT_NONE returned immediately).

12. **IsInsignificant**: Ignored by AI targeting systems.

13. **Can_Enter_Cell validation**: Unlimbo checks MOVE_OK before placing object.

14. **Coord_Fixup**: ObjectTypeClass adjusts coordinate for object footprint.

15. **Radio messaging**: Only RADIO_REDRAW handled at ObjectClass level. Other messages handled by derived classes.

16. **Detach_All multiplayer**: Unselect_All_Players() vs Unselect_All_Players_Except_Owner() for MP safety.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/OBJECT.CPP at f1f0d42
