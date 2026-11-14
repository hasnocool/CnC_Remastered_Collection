# PATH/FILE
original/TIBERIANDAWN/ANIM.CPP

## Purpose
Animation system for Tiberian Dawn. Manages all visual effects (explosions, fire, smoke, ion cannon, napalm, craters, flames) with frame-based rendering, attachment to objects, damage over time, and chaining to subsequent animations.

## Where it fits in the game
Visual effects subsystem - handles all non-unit animation rendering and logic. Runs once per frame per active animation (AnimClass::AI), integrates with map rendering (Mark/Refresh_Cells), and triggers collateral effects (scorches, craters, screen shake).

## Classes, structs, enums and important data

### AnimClass
**Core animation object** - pooled allocation via `new`/`Delete_This`.

**Key data members:**
- `Class` (AnimTypeClass const*) - Animation type definition
- `Object` (ObjectClass*) - Attached object (or NULL)
- `SortTarget` (TARGET) - Forced render sort above target
- `OwnerHouse` (HousesType) - Owner for damage attribution
- `KillTime` (unsigned long long) - Wall-clock expiry time
- `Delay` (unsigned char) - Frames before start
- `Loops` (signed char) - Remaining loops (-1 = infinite)
- `Stage` (current frame index)
- `Accum` (unsigned char) - Damage accumulator (0-255)
- `IsToDelete` / `IsBrandNew` / `IsAlternate` / `IsInvisible` (flags)
- `VirtualAnim` (AnimClass*) - Chained virtual animation
- `VisibleFlags` (unsigned int) - Render visibility mask

### AnimTypeClass (referenced via `Class`)
**Animation type definition** from ART.INI/RULES.INI

**Key fields:**
- `Type` (AnimType) - Animation category
- `Stages` (int) - Total frames
- `Start` / `LoopStart` / `LoopEnd` - Frame ranges
- `Delay` (int) - Frame delay (ticks)
- `Loops` (int) - Default loop count
- `Damage` (unsigned char) - Damage per frame
- `Biggest` (int) - Frame index for Middle() effects
- `IsGroundLayer` / `IsSticky` / `IsScorcher` / `IsCraterForming` / `IsFlameThrower` / `IsWhiteTrans` / `IsTranslucent` / `IsNormalized` (flags)
- `VirtualAnim` / `ChainTo` (AnimType) - Chaining
- `Sound` (SoundType) - Sound effect
- `VirtualStages` / `VirtualScale` - Virtual rendering

### Animation types (AnimType enum, partial)
| Type | Description |
|------|-------------|
| `ANIM_ION_CANNON` | Ion cannon strike |
| `ANIM_ATOM_BLAST` | Nuclear explosion |
| `ANIM_NAPALM1/2/3` | Napalm spread |
| `ANIM_FIRE_SMALL/MED/MED2` | Fire variants |
| `ANIM_SMOKE_M` | Medium smoke |
| `ANIM_LZ_SMOKE` | Landing zone smoke |
| `ANIM_MOVE_FLASH` | Unit move flash |
| `ANIM_BEACON_VIRTUAL` | Virtual beacon |
| `ANIM_FLAG` | Flag animation |
| `ANIM_ATOM_DOOR` | Temple door |
| `ANIM_FLAME_N`... | Flamethrower directions |
| `ANIM_RAPT_DIE` etc | Dinosaur deaths |
| `ANIM_TRIC_DIE` etc | |

### Special animation types
- `ANIM_NONE` - Invalid
- `ANIM_LZ_SMOKE` - Reveals map (Sight_From radius 4)
- `ANIM_ATOM_BLAST` - Nuke: screen shake, palette fade, massive damage, scorches
- `ANIM_ION_CANNON` - Ion cannon: 600 damage, WARHEAD_PB
- `ANIM_NAPALM*` - Spawns fire/smoke at scatter offsets
- `ANIM_FIRE_MED/MED2` - Spawns ANIM_FIRE_SMALL attached
- `ANIM_FLAME_*` - Flamethrower: spawns fire/smoke at offset angles
- `ANIM_*_DIE` - Unit death anims

## Functions and methods

### AnimClass::AI()
**Per-frame animation processor** - Called once per frame per active animation.

**Logic:**
1. **Ground layer refresh**: If `Class->IsGroundLayer`, `Map.Refresh_Cells()` for overlap list
2. **LZ smoke cleanup**: If building placed on LZ smoke → `IsToDelete = true`
3. **KillTime check**: Wall-clock expiry via `GetSystemTimeAsFileTime()`
4. **Immediate delete**: If `IsToDelete` → `Delete_This()`, return
5. **Brand new skip**: First frame skips processing (prevents frame skip)
6. **Lazy init**: Loads `Stages`/`LoopEnd` from SHP if -1 (save/load)
7. **Delay countdown**: Decrements `Delay`, calls `Start()` when 0
8. **Active processing**:
   - `Mark(MARK_CHANGE)` - Always marks for redraw
   - `StageClass::Graphic_Logic()` - Frame advance gate
   - **Damage over time**: If attached + `Class->Damage` + frame < `Stages`:
     - Accumulates `Accum += Damage`, applies per 256 units
     - `Object->Take_Damage(damage, 0, WARHEAD_FIRE)`
     - If destroyed → deletes virtual anim + self
   - **Middle effects**: At `Biggest` frame → `Middle()`
   - **Loop/chain logic**:
     - If `(Loops <= 1 && stage >= Stages) || (Loops > 1 && stage >= LoopEnd-Start)`:
       - Decrement `Loops`, if >0 → `Set_Stage(LoopStart)`
       - Else if `VirtualAnim` → `Make_Invisible()`, chain or delete
       - Else if `stage >= VirtualStages` → chain or delete

### AnimClass::Start()
**Initial side effects** - Called when delay expires or immediately.

1. `Mark()` for initial render
2. `Sound_Effect(Class->Sound, Coord)`
3. If `Biggest == 0` → `Middle()` (immediate collateral)
4. **Sticky attachment**: If `!Object && Class->IsSticky && in radar`:
   - Attaches to gunboat at cell (`ANIM_LZ_SMOKE` etc)

### AnimClass::Middle()
**Collateral effects at biggest frame** - Type-specific:

| Type | Effects |
|------|---------|
| `ANIM_ATOM_BLAST` | Finds owner building (prefers Temple), radius 3/4, damage 200/1000, WARHEAD_FIRE, screen shake, palette fade to white/back, spawns scorch smudges |
| `ANIM_ION_CANNON` | Finds owner building (prefers Eye), `Explosion_Damage` 600, WARHEAD_PB |
| `ANIM_NAPALM*` | Spawns `ANIM_FIRE_SMALL/MED` at scattered coords |
| `ANIM_FIRE_MED/MED2` | Spawns `ANIM_FIRE_SMALL` attached to same object |
| `IsScorcher` | Spawns scorch smudge at center |
| `IsCraterForming` | Reduces Tiberium by 6, spawns `SMUDGE_CRATER1` |
| `IsFlameThrower` | Spawns fire/smoke at offset angles (32° increments), scorches |

### AnimClass::Chain()
**Metamorphosis to next animation** - Changes `Class` to `ChainTo`, resets rate/stage.

### AnimClass constructor `AnimClass(animnum, coord, timedelay, loop, alt)`
1. Sets `Class`, `Object=NULL`, `SortTarget=TARGET_NONE`, `OwnerHouse=HOUSE_NONE`, `KillTime=0`
2. Lazy-loads `Stages` from SHP frame count if -1
3. Lazy-loads `LoopEnd = Stages` if -1
4. Sets rate: `Options.Normalize_Delay(Class->Delay)` if `IsNormalized`
5. `Adjust_Coord(coord)` - Special offsets (e.g., ATOM_DOOR: -1,-36 pixels)
6. `Unlimbo(coord)` - Places on map
7. `VisibleFlags = -1` (all visible)
7. **LZ smoke reveal**: `Map.Sight_From(PlayerPtr, cell, 4, false)`
8. `Delay = timedelay`
9. `Loops = MAX(loop, 1) * Class->Loops` (min 1), or `Class->Loops` if negative
10. Flags: `IsToDelete=false`, `IsBrandNew=true`, `IsAlternate=alt`, `IsInvisible=false`
11. If `!Delay` → `Start()`
12. If `Class->VirtualAnim != ANIM_NONE` → creates `VirtualAnim` (invisible)

### AnimClass::~AnimClass()
1. If attached to `Object`:
   - Removes from display layer
   - Scans for other anims on same object
   - If last → `Object->Fire_Out()`, `IsAnimAttached=false`, `Mark(OVERLAP_UP/DOWN)`
   - Transfers coord to world space: `Coord = Coord_Add(Object->Center_Coord(), Coord)`
2. `Limbo()` - Removes from map

### AnimClass::Render(forced) → bool
- Returns false if `Delay > 0`
- Sets `IsToDisplay = true`
- Delegates to `ObjectClass::Render(forced)`

### AnimClass::Draw_It(x, y, window)
**Complex rendering with special cases:**
- Legacy render (`render_legacy`): Draws SHP frame via `CC_Draw_Shape`
- Virtual render (`render_virtual`): Draws `VirtualAnim` at same coords
- **Special fixups per type:**
  - `ION_CANNON`: Vertical offset, `SHAPE_BOTTOM` in virtual
  - `*_DIE` / `ATOM_BLAST`: Uses `Map.UnitShadow` translucent table
  - `FLAG`: Icon-relative offset, shadow table
  - `BEACON_VIRTUAL`: Fixed 29x39, `SHAPE_BOTTOM|SHAPE_COMPACT`
- **Translucency**: `IsWhiteTrans` → `WhiteTranslucentTable`; `IsTranslucent` → `TranslucentTable`
- **Alternate/owner recolor**: `SHAPE_FADING` + house remap table
- **Ghost flag**: If translucent table set → `SHAPE_GHOST`
- **Virtual frame guard**: Skips legacy if `window == WINDOW_VIRTUAL` or frame >= `Stages`

### AnimClass::Mark(mark) → bool
- Calls `ObjectClass::Mark(mark)`
- If true: `Map.Refresh_Cells(Coord_Cell(Center_Coord()), Overlap_List())`

### AnimClass::Overlap_List() → CellStruct*
- Returns `Class->Overlap` for ground layer
- `CellStruct::ZeroCellList` for air layer

### AnimClass::Occupy_List() → CellStruct*
- Returns `Class->Occupy` for ground layer
- `CellStruct::ZeroCellList` for air layer

### AnimClass::Adjust_Coord(coord) → COORDINATE
- `ANIM_ATOM_DOOR`: Returns `coord + (-1, -36)` pixels
- Others: Returns unchanged

### AnimClass::Sort_Above(target)
- Sets `SortTarget = target` for render sorting

### AnimClass::In_Which_Layer() → LayerType
- `LAYER_GROUND` if attached or `Class->IsGroundLayer`
- `LAYER_AIR` otherwise

### AnimClass::As_Target() → TARGET
- Returns `Build_Target(KIND_ANIMATION, Anims.ID(this))`

### AnimClass::Detach(target, all)
- If `all` and virtual/target matches → clears virtual, marks delete if invisible
- If `all` and object matches → removes from layer, clears object, marks delete

### AnimClass::Set_Owner(owner) / Set_Visible_Flags(flags)
- Propagates to `VirtualAnim`

### Shorten_Attached_Anims(obj)
- Iterates all anims, if `anim.Object == obj` → `anim.Loops = 0` (accelerates completion)

## Inputs, outputs and side effects

**State mutations:**
- Creates/deletes AnimClass instances (pooled)
- Modifies map cells (Refresh_Cells, Sight_From)
- Applies damage to attached objects (WARHEAD_FIRE)
- Spawns collateral anims (fire, smoke, craters)
- Screen shake / palette fade (ATOM_BLAST)
- Tiberium reduction (craters)

**External systems affected:**
- Map rendering (Mark, Refresh_Cells, display lists)
- Object damage system (Take_Damage)
- Smudge system (scorches, craters)
- Sound system (Sound_Effect)
- Fog of war (Sight_From for LZ smoke)

## Calls into / called by

**Called by:**
- Game main loop: `AnimClass::AI()` per frame
- Weapon impacts: `new AnimClass(...)` for explosions
- Building destruction: Death animations
- Unit movement: `ANIM_MOVE_FLASH`
- Flamethrower: `ANIM_FLAME_*` chain
- Ion cannon/Nuke: Superweapon triggers

**Calls into:**
- `AnimTypeClass::As_Reference()` - Type data
- `ObjectClass::Render/Mark/Unlimbo/Limbo`
- `Map.Refresh_Cells/Sight_From/Cell_Building/Cell_Unit/Closest_Free_Spot`
- `CC_Draw_Shape` - Low-level SHP rendering
- `Sound_Effect` - Audio
- `Explosion_Damage` - Area damage
- `SmudgeClass` - Ground decals
- `Shake_The_Screen/Fade_Palette_To` - Screen effects
- `Take_Damage` - Object damage
- `Options.Normalize_Delay` - Frame rate normalization

## Related original files
- **ANIM.H** - Class declarations, AnimType enum
- **ANIMTYPE.H/.CPP** - AnimTypeClass definitions
- **OBJECT.H/.CPP** - ObjectClass base, RTTI
- **MAP.H/.CPP** - MapClass, CellClass
- **SMUDGE.H/.CPP** - SmudgeClass (scorches/craters)
- **BUILDING.H/.CPP** - BuildingClass (Temple/Eye for superweapons)
- **SOUND.H/.CPP** - Sound_Effect
- **OPTIONS.H/.CPP** - Normalize_Delay
- **SHAPE.H/.CPP** - CC_Draw_Shape, ShapeFlags
- **SPECIAL.H/.CPP** - GameToPlay, ModernBalance
- **LOGIC.H** - Logic array for owner finding

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-render`

**Mapped modules:**
- `cnc-render/src/animation.rs` - AnimClass, AnimTypeClass
- `cnc-tiberian-dawn/src/animation/td_anim.rs` - TD-specific types
- `cnc-core/src/effects/anim_processor.rs` - Per-frame AI logic
- `cnc-render/src/anim_draw.rs` - Draw_It with special cases

**Not yet ported:**
- Pooled allocation (`new`/`Delete_This` via `Anims` pool)
- Virtual animation chaining (`VirtualAnim`)
- Frame normalization (`Options.Normalize_Delay`)
- Complex Draw_It special cases (ION_CANNON offset, death anims, flag, beacon)
- Translucency tables (white/normal/ghost)
- Alternate recoloring (SHAPE_FADING + house remap)
- LZ smoke fog reveal (Sight_From)
- Wall-clock KillTime expiry
- Brand-new frame skip
- Damage accumulation per 256 ticks (WARHEAD_FIRE)
- Middle() collateral effects per type (nuke, ion, napalm, fire, crater, flame)
- ChainTo metamorphosis
- Adjust_Coord per-type offsets
- SortTarget render ordering
- Sticky attachment to gunboats
- Overlap/Occupy lists for ground layer
- Shorten_Attached_Anims for accelerated cleanup

## Edge cases and compatibility notes

1. **Pooled allocation**: `Anims` pool manages all AnimClass instances; `Delete_This()` returns to pool
2. **Virtual animations**: Separate AnimClass instance for virtual layer; created in constructor if `Class->VirtualAnim != ANIM_NONE`
3. **Frame normalization**: `Options.Normalize_Delay()` scales delay for game speed
4. **Lazy SHP loading**: `Stages`/`LoopEnd` loaded from SHP on first use or save/load (`-1` sentinel)
5. **Damage accumulator**: 8-bit `Accum` wraps at 256; applies `damage = accum >> 8` per frame
6. **LZ smoke**: Reveals fog of war radius 4 at creation; deleted if building placed on cell
7. **Nuke (ATOM_BLAST)**: Modern balance = radius 4, damage 1000; legacy = radius 3, damage 200; screen shake + palette fade
8. **Ion cannon**: Finds Temple of Nod (prefers) or any owner building; 600 damage WARHEAD_PB
9. **Napalm**: Spawns multiple fire/smoke anims at random scatter offsets (0x40-0xA0 leptons)
10. **Flamethrower**: Spawns at 32° offsets (type - ANIM_FLAME_N << 5), creates fire/smoke + scorch
11. **Crater**: Reduces Tiberium by 6, spawns SMUDGE_CRATER1
12. **Fire medium**: Spawns small fire attached to same object
13. **SortTarget**: Forces render above specific target (explosions)
14. **Sticky**: Attaches to gunboat at cell on Start()
15. **Adjust_Coord**: Only ATOM_DOOR has custom offset (-1, -36 pixels)
16. **ChainTo**: Changes animation type mid-lifecycle, resets frame/rate
17. **KillTime**: Wall-clock expiry via GetSystemTimeAsFileTime (FILETIME)
18. **Brand new skip**: First AI call after creation skips processing
19. **VirtualStages guard**: Chains only after virtual anim completes its stages
20. **Ground layer refresh**: IsGroundLayer anims force cell refresh every frame

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/ANIM.CPP at f1f0d42
