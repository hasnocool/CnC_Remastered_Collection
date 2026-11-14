# PATH/FILE
original/TIBERIANDAWN/ADATA.CPP

## Purpose
Animation type data table and registration for Tiberian Dawn. Defines every static `AnimTypeClass` instance (the type-level description of each animation effect in the game — explosions, flames, smoke, craters, crate pickups, dinosaur deaths, ion cannon, beacon, etc.), builds the `AnimTypeClass::Pointers[ANIM_COUNT]` lookup table that maps `AnimType` enum values to those instances, and implements the constructor plus the `One_Time()` loader that reads each animation's `.SHP` shape file.

## Where it fits in the game
Effects/type-registry subsystem — `AnimTypeClass` is the *type* half of the animation system (parallel to `AnimClass`, the *instance* half in ANIM.CPP). Any gameplay code that spawns an explosion, fire, smoke puff, muzzle flash, crate icon, or dinosaur death sequence resolves an `AnimType` enum to an `AnimTypeClass const &` via `AnimTypeClass::As_Reference(type)` (which dereferences `Pointers[type]`). The table in this file is therefore the single source of truth for animation behavior flags, frame ranges, damage-over-time, sounds, and chain-to-next-animation links.

## Classes, structs, enums and important data

### AnimTypeClass (declared in TYPE.H; constructed here)
**Animation type descriptor** — inherits `ObjectTypeClass` → `AbstractTypeClass`. All instances in this file are `static AnimTypeClass const`, i.e. immutable, file-scope type objects.

**Constructor parameters (in order):**
| Param | Type | Meaning |
|-------|------|---------|
| `anim` | `AnimType` | Enum identifier for this animation kind |
| `name` | `char const *` | `.SHP` file base name (IniName) |
| `size` | `int` | Maximum dimension of shape, edge-to-edge (cell refresh list sizing; keep small for performance) |
| `biggest` | `int` | Frame number where animation is largest; crater/scorch effects are delayed until this frame so they don't "pop" |
| `isnormal` | `bool` | Normalized animation rate (constant apparent speed regardless of game speed) |
| `iswhitetrans` | `bool` | Uses white translucent lookup table |
| `isscorcher` | `bool` | Leaves scorch marks on ground |
| `iscrater` | `bool` | Forms crater |
| `issticky` | `bool` | Attaches to unit in same square |
| `ground` | `bool` | Ground-layer animation (sorted with ground units) |
| `istrans` | `bool` | Translucent colors |
| `isflame` | `bool` | Flame-thrower animation (custom effects while playing: scorch marks + fire pieces) |
| `damage` | `unsigned int` | Fixed-point damage applied per tick to attached object (fire on units) |
| `delaytime` | `int` | Frames between frame advances (overridable by control list) |
| `start` | `int` | Starting frame number |
| `loopstart` | `int` | Loop start frame |
| `loopend` | `int` | Loop end frame (last loop proceeds until `Stages` complete) |
| `stages` | `int` | Number of animation stages; `-1` = use full shape |
| `loops` | `int` | Loop count; stored as `char` |
| `sound` | `VocType` | Sound effect played when animation starts |
| `chainto` | `AnimType` | Follow-up animation type to launch into (`ANIM_NONE` = none) |
| `virtualstages` | `int` | Virtual-renderer-only stage count (4K rendering; ignored by legacy renderer); default `-1` |
| `virtualscale` | `int` | Virtual-renderer scale factor; default `0x100` (1.0 fixed point) |
| `virtualanim` | `AnimType` | Animation type used specifically for virtual rendering; default `ANIM_NONE` |

**Assigned members:** `Biggest`, `ChainTo`, `Damage`, `Delay` (truncated to `unsigned char`), `IsCraterForming`, `IsFlameThrower`, `IsGroundLayer`, `IsNormalized`, `IsScorcher`, `IsSticky`, `IsTranslucent`, `IsWhiteTrans`, `LoopEnd`, `LoopStart`, `Loops` (truncated to `char`), `Size`, `Sound`, `Stages`, `Start`, `Type`, `VirtualStages`, `VirtualScale`, `VirtualAnim`. Base class initialized as `ObjectTypeClass(true, false, false, true, false, false, true, true, TXT_NONE, name, ARMOR_NONE, 0)`.

**Static methods:** `As_Reference(AnimType)` → `*Pointers[type]`; `Init(TheaterType)` → empty; `One_Time()`; `What_Am_I()` → `RTTI_ANIMTYPE`.

### AnimType enum (DEFINES.H, referenced by this table)
`typedef enum AnimType : char` — `ANIM_NONE=-1`, then sequential from `ANIM_FBALL1=0` through `ANIM_BEACON`, plus five `*_VIRTUAL` entries (`ANIM_FIRE_SMALL_VIRTUAL`, `ANIM_FIRE_MED_VIRTUAL`, `ANIM_FIRE_MED2_VIRTUAL`, `ANIM_FIRE_TINY_VIRTUAL`, `ANIM_BEACON_VIRTUAL`), then `ANIM_COUNT` and `ANIM_FIRST=0`. An `#ifdef NEVER` block of `ANIM_E1_ROT_*` … `ANIM_E4_ROT_*` infantry-decay entries exists in the enum but is compiled out.

### Global table
- `AnimTypeClass const * const AnimTypeClass::Pointers[ANIM_COUNT]` — index-ordered pointer table; position in this initializer list must match the `AnimType` enum order exactly.

### Static instances defined in this file (grouped by role)
- **Dinosaur deaths:** `TricDie` (ANIM_TRIC_DIE, "TRIC"), `TRexDie` (ANIM_TREX_DIE, "TREX"), `StegDie` (ANIM_STEG_DIE, "STEG"), `RaptDie` (ANIM_RAPT_DIE, "RAPT") — all `IsNormalized=true`, `IsGroundLayer=true`, `IsTranslucent=true`, delay 2, loops 0.
- **SAM fire (8 directions):** `SAMN`, `SAMNW`, `SAMW`, `SAMSW`, `SAMS`, `SAMSE`, `SAME`, `SAMNE` — all "SAMFIRE" shape, 18 frames each, start frame `18*N`, biggest stage `4+18*N`, delay 1.
- **LZ smoke:** `LZSmoke` ("SMOKLAND"), ground layer, loop 72→91, 127 loops.
- **Flammable burning:** `BurnSmall`/`BurnMed`/`BurnBig` ("BURN-S/M/L") — scorch flag only on BurnBig; damage 0x0008/0x0010/0x0018; loop 0→30..62; 4 loops.
- **Burning that trails into smoke:** `OnFireSmall`/`OnFireMed`/`OnFireBig` — same as Burn* but `ChainTo` = ANIM_SMOKE_M / ANIM_ON_FIRE_SMALL / ANIM_ON_FIRE_MED.
- **Flame thrower (8 directions):** `FlameN/NW/W/SW/S/SE/E/NE` ("FLAME-*") — `IsFlameThrower=true`, delay 1, loops 1, no ground layer.
- **Chem sprayer (8 directions):** `ChemN/NW/W/SW/S/SE/E/NE` ("CHEM-*") — identical shape to Flame group but `IsFlameThrower=false`.
- **Explosions/impacts:** `Grenade` ("VEH-HIT2", VOC_GUN20, crater-forming), `FBall1` ("FBALL1", VOC_XPLOS, crater), `Frag1`/`Frag3` ("FRAG1"/"FRAG3", sticky, VOC_XPLOBIG4/6, virtual stages 29), `VehHit1/2/3` ("VEH-HIT1/2/3", sticky, VOC_XPLOS), `ArtExp1` ("ART-EXP1", crater, VOC_XPLOSML2).
- **Napalm:** `Napalm1/2/3` ("NAPALM1/2/3", scorchers, VOC_FLAMER1).
- **Smoke/puffs:** `SmokePuff` ("SMOKEY", translucent, ground layer), `Piff`, `PiffPiff` ("PIFF"/"PIFFPIFF").
- **Fire with virtual-renderer variants:** `Fire3` (ANIM_FIRE_SMALL), `Fire1` (ANIM_FIRE_MED2), `Fire4` (ANIM_FIRE_TINY), `Fire2` (ANIM_FIRE_MED) — each `ChainTo`/virtual fields point at a matching `*Virtual` instance (`Fire3Virtual`, `Fire1Virtual`, `Fire4Virtual`, `Fire2Virtual`) with `VirtualStages=-1`, `VirtualScale=0x100`; the Virtual instances loop frames 10→21 with 29 stages and damage 0.
- **Oilfield burn:** `OilFieldBurn` ("FLMSPT"), ground layer, loop 33→99, 66 stages, 127 loops.
- **Muzzle flash:** `Gunfire` ("GUNFIRE", ANIM_MUZZLE_FLASH) — note its argument order in the source swaps the `loops`/`stages`/`loopend` positions relative to the comments (commented-out alternates show intent: loops=2, stages=2, loopend=1); virtual stages 10.
- **Mini-gun fire (8 directions):** `GUNN/NW/W/SW/S/SE/E/NE` ("MINIGUN") — start frames 0,6,12,…,42, 6 stages each.
- **Superweapons:** `IonCannon` ("IONSFX", scorch+crater, VOC_ION_CANNON, `ChainTo=ANIM_ART_EXP1`, virtual stages 32, virtual scale 0x200), `AtomBomb` ("ATOMSFX", scorch+crater, VOC_NUKE_EXPLODE, virtual stages 75, virtual scale 0x300), `AtomDoor` ("ATOMDOOR", sticky, scorch+crater).
- **Crate pickups:** `CDeviator`, `CDollar`, `CEarth`, `CEmpulse`, `CInvun`, `CMine`, `CRapid`, `CStealth`, `CMissile` — all normalized, delay 2, 48 size, "DEVIATOR"/"DOLLAR"/"EARTH"/"EMPULSE"/"INVUN"/"MINE"/"RAPID"/"STEALTH2"/"MISSILE2".
- **Misc:** `MoveFlash` ("MOVEFLSH", white-translucent, ground layer), `ChemBall` ("CHEMBALL", VOC_FLAMER1), `Flag` ("FLAGFLY", loops=-1 infinite), `Beacon` ("MOVEFLSH") + `BeaconVirtual` ("BEACON") — beacon pairs legacy shape with virtual-renderer shape.
- **Compiled-out (`#ifdef NEVER`):** `E1RotFire`…`E4RotExp` — 16 infantry rotation/decay animations ("E1ROT"…"E4ROT", delay 30, 4 stages). Also note their constructor calls use a *different argument order* than the current constructor (missing `size`/`biggest` ints, extra `Is a flat on the ground` bool) — evidence of an older constructor signature kept as dead code.

## Functions and methods

### AnimTypeClass::AnimTypeClass(anim, name, size, biggest, isnormal, iswhitetrans, isscorcher, iscrater, issticky, ground, istrans, isflame, damage, delaytime, start, loopstart, loopend, stages, loops, sound, chainto, virtualstages=-1, virtualscale=0x100, virtualanim=ANIM_NONE)
**Constructor.** Initializes the `ObjectTypeClass` base with fixed flags `(true, false, false, true, false, false, true, true, TXT_NONE, name, ARMOR_NONE, 0)`, then assigns every parameter to its corresponding member with two narrowing casts: `Delay = (unsigned char)delaytime` and `Loops = (char)loops`.

### AnimTypeClass::One_Time(void)
**Loads all animation shape data once at game init.**

1. Loops `index` from `ANIM_FIRST` (0) to `ANIM_COUNT-1`.
2. Builds the file name `<IniName>.SHP` via `_makepath(fullname, NULL, NULL, As_Reference(index).IniName, ".SHP")` into `char fullname[_MAX_FNAME+_MAX_EXT]`.
3. Tries `RawFileClass file(fullname)`; if `file.Is_Available()` loads via `Load_Alloc_Data(file)`, otherwise falls back to `MixFileClass::Retrieve(fullname)`. Result is stored into `As_Reference(index).ImageData` through a `void const *&` cast (the field is const-qualified in the type object; the cast strips constness to populate it once at startup).
4. After the loop, manually copies `As_Reference(ANIM_BEACON).ImageData` into `As_Reference(ANIM_BEACON_VIRTUAL).ImageData` — the beacon virtual animation is new in the virtual renderer and shares the legacy beacon shape rather than having its own file.

**Warning (from source):** "This routine should be called ONLY once."

### AnimTypeClass::Pointers[ANIM_COUNT] (static table initializer)
**Index→type mapping.** 68 entries in strict `AnimType` enum order: FBall1, Grenade, Frag1, Frag3, VehHit1..3, ArtExp1, Napalm1..3, SmokePuff, Piff, PiffPiff, FlameN/NE/E/SE/S/SW/W/NW, ChemN/NE/E/SE/S/SW/W/NW, Fire3, Fire2, Fire1, Fire4, Gunfire, [NEVER block: E1..E4 Rot ×4], SmokeM, BurnSmall/Med/Big, OnFireSmall/Med/Big, SAMN/NE/E/SE/S/SW/W/NW, GUNN/NE/E/SE/S/SW/W/NW, LZSmoke, IonCannon, AtomBomb, CDeviator, CDollar, CEarth, CEmpulse, CInvun, CMine, CRapid, CStealth, CMissile, AtomDoor, MoveFlash, OilFieldBurn, TricDie, TRexDie, StegDie, RaptDie, ChemBall, Flag, Beacon, Fire3Virtual, Fire2Virtual, Fire1Virtual, Fire4Virtual, BeaconVirtual.

## Inputs, outputs and side effects

**Inputs:** None at runtime beyond game-init invocation. All data is compile-time constant.

**Outputs / side effects:**
- Static initialization of 68+ `AnimTypeClass` objects before `main` (C++ static initialization order is within this translation unit only; cross-TU ordering is not depended upon because `Pointers` is initialized in the same file after all instances).
- `One_Time()` allocates shape data (`Load_Alloc_Data` / `MixFileClass::Retrieve`) into each type's `ImageData` and writes the beacon-virtual alias.
- `Delay` and `Loops` are narrowed to `unsigned char`/`char`; values like `Loops=127` (LZSmoke, OilFieldBurn) fit, but the narrowing is a latent truncation hazard for future entries.

## Calls into / called by

**Called by:**
- Game initialization sequence → `AnimTypeClass::One_Time()` (source comment: "called by the game initialization process").
- Anywhere an animation is spawned: `AnimTypeClass::As_Reference(AnimType)` (inline in TYPE.H) → `Pointers[type]`.
- `UDATA.CPP` references `ANIM_TRIC_DIE` etc. as unit explosion types; `ANIM.CPP` cases on `ANIM_TREX_DIE`/`ANIM_TRIC_DIE`/`ANIM_ATOM_BLAST` for shadow translucency selection.

**Calls into:**
- `_makepath` (LIBRARY) — path construction.
- `RawFileClass::Is_Available()` / `Load_Alloc_Data` (FILELIB/RAWFILE) — direct file load.
- `MixFileClass::Retrieve` (MIX_FILE) — MIX archive fallback.
- `ObjectTypeClass` base constructor (OBJECT / ABSTRACT type hierarchy).

## Related original files
- **TYPE.H** — `AnimTypeClass` declaration (members, constructor signature, `Pointers`, `As_Reference`, `One_Time`).
- **DEFINES.H** — `AnimType` enum (lines ~1414–1523) and `ANIM_COUNT`/`ANIM_FIRST`.
- **ANIM.CPP / ANIM.H** — `AnimClass`, the instance class that consumes these types at runtime.
- **UDATA.CPP** — unit type table referencing `ANIM_TRIC_DIE` etc. for death explosions.
- **FUNCTION.H** — included header for this translation unit.
- **OBJECT.H / ABSTRACT.H** — `ObjectTypeClass` / `AbstractTypeClass` base classes.
- **MIX_FILE.H, RAWFILE.H, FILELIB.H** — `MixFileClass`, `RawFileClass`, `Load_Alloc_Data`.

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-tiberian-dawn/src/effects/anim_type.rs` (or equivalent) — `AnimTypeClass` as a `struct` with `const` constructors; `AnimType` enum with explicit discriminants including `ANIM_NONE = -1`.
- `cnc-tiberian-dawn/src/effects/anim_type_table.rs` — `Pointers[ANIM_COUNT]` as a `const [&AnimTypeClass; ANIM_COUNT]` array or `phf` map; enum-ordinal indexing must be exact.
- `cnc-tiberian-dawn/src/effects/anim_type.rs::one_time()` — `.SHP` load loop with RawFile → MixFile fallback and the beacon-virtual alias assignment.

**Not yet ported / watch-items:**
- `virtualstages` / `virtualscale` / `virtualanim` triple is legacy-renderer-ignored; Rust port must decide whether virtual-renderer-only fields exist at all.
- `Gunfire` entry's argument order in source disagrees with its trailing comments (loops/stages/loopend swapped, with commented alternates) — port must match *actual* parameter positions, not the comments.
- `#ifdef NEVER` infantry-rotation block and the older constructor signature it uses are dead code; do not port.
- `Delay`/`Loops` narrowing casts (`unsigned char`/`char`) have no direct Rust analogue; use `u8`/`i8` fields and assert ranges at construction.
- Const-cast on `ImageData` (writing into a `const` object) becomes a one-time `OnceCell`/`OnceLock` or interior-mutability pattern in Rust.

## Edge cases and compatibility notes

1. **Table/enum ordering is load-bearing.** `Pointers` is positional; any mismatch between enum order in DEFINES.H and the initializer list silently mis-types every animation. The five `*_VIRTUAL` enum values sit after `ANIM_BEACON` and before `ANIM_COUNT`, and their table entries sit at the end of the list.
2. **`ANIM_NONE = -1`** is a valid "no animation" sentinel, used for `ChainTo`/`VirtualAnim` defaults.
3. **`stages = -1`** means "use all frames in the shape" (e.g. BurnSmall/Med/Big, OnFire*, Flame*, Chem*, Grenade, FBall1, Frag*, VehHit*, ArtExp1, Napalm*, Fire*, Gunfire's virtual stages, AtomBomb/Door, crate anims, MoveFlash, ChemBall, Flag, Beacon).
4. **`loops = -1`** means infinite looping (Flag); `loops = 0` plays once (most explosions); `loops = 127` is the max finite repeat (LZSmoke, OilFieldBurn).
5. **`loopend = -1`** disables mid-loop loopback (single-pass animations); `loopend = 0` with `loopstart = 0` effectively loops from the beginning.
6. **Biggest-frame trick:** crater/scorch effects are deferred until the animation reaches its `Biggest` frame so craters don't visibly "pop" under the growing explosion.
7. **Damage field is fixed-point** (`0x0008` = 8/256 per tick for BurnSmall/Fire3/Fire4; `0x0010` for BurnMed/Fire1/Fire2; `0x0018` for BurnBig) and only applies when `IsSticky` attaches the anim to a unit.
8. **`IsFlameThrower` is not "is fire"** — the Flame*/Chem* direction sets differ only by this flag; flame-thrower anims get special runtime side effects (scorch + fire pieces) in ANIM.CPP.
9. **SAM/MiniGun direction sets** tile one 18- or 6-frame strip per direction using `start = N * frames_per_dir`; `biggest` for SAM is `4 + 18*N` (i.e. the same offset plus 4), a pattern that must be preserved if art changes.
10. **Beacon is special:** `Beacon` uses the "MOVEFLSH" shape, `BeaconVirtual` uses "BEACON"; `One_Time()` manually aliases `ANIM_BEACON_VIRTUAL.ImageData = ANIM_BEACON.ImageData` because the virtual beacon has no separate `.SHP` file — but `BeaconVirtual`'s declared name is "BEACON", so the alias *overwrites* the file load for that entry.
11. **`Gunfire` argument-order anomaly:** source passes `loops=0` where the comment says "Number of times the animation loops", then `stages=1` where the comment says "Number of animation stages", then `loopend=1` — with commented-out `2`s showing the designer's intent. Any port must transcribe the actual positions.
12. **Static-init safety:** all instances are `static const` in this TU and `Pointers` is initialized after them in the same TU, so there is no static-initialization-order fiasco within this file; other TUs only access `Pointers` after `One_Time()`/init.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/ADATA.CPP at f1f0d42
