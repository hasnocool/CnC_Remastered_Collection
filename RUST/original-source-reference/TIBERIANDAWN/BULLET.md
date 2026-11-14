# PATH/FILE
original/TIBERIANDAWN/BULLET.CPP

## Purpose
Projectile/bullet system for Tiberian Dawn. Implements all projectile types (bullets, missiles, flames, grenades, napalm, nukes) with physics, homing, ballistics, damage application, and rendering.

## Where it fits in the game
Projectile system - instantiated when weapons fire. Manages projectile lifecycle from launch to impact/detonation. Handles ballistics, homing, gravity, damage application, and visual effects. Core combat system component.

## Classes, structs, enums and important data

### BulletClass
**Core projectile class** - represents individual projectile instances.

**Key data members:**
- `Class` (BulletTypeClass*) - Static type data pointer
- `Payback` (ObjectClass*) - Firing object (for kill credit)
- `IsToAnimate` (bool) - Animation toggle (flame puffs)
- `Altitude` (int) - Current height in leptons (0 = ground)
- `Riser` (int) - Vertical velocity (leptons/frame)
- `TarCom` (TARGET) - Target for homing projectiles
- `Strength` (int) - Current damage potential
- `IsLocked` (bool) - Target locked flag
- `IsInaccurate` (bool) - Accuracy deviation flag
- `IsToAnimate` (bool) - Animation toggle (flames)
- `Altitude` (int) - Height in leptons (0 = ground)
- `Riser` (int) - Vertical velocity
- `TarCom` (TARGET) - Homing target
- `Strength` (int) - Damage potential
- `IsLocked` (bool) - Target lock
- `IsInaccurate` (bool) - Inaccuracy flag
- `IsToAnimate` (bool) - Animation toggle
- `Altitude` (int) - Height in leptons
- `Riser` (int) - Vertical velocity
- `TarCom` (TARGET) - Target
- `Strength` (int) - Damage
- `IsLocked` (bool) - Locked
- `IsInaccurate` (bool) - Inaccurate flag
- `IsToAnimate` (bool) - Animation toggle
- `Payback` (ObjectClass*) - Firing object
- `TarCom` (TARGET) - Target
- `Strength` (int) - Damage potential
- `IsLocked` (bool) - Locked
- `IsInaccurate` (bool) - Inaccurate
- `IsToAnimate` (bool) - Animation toggle
- `Altitude` (int) - Height in leptons
- `Riser` (int) - Vertical velocity

**BulletTypeClass (static)**
- `IsArcing` - Ballistic arc trajectory
- `IsDropping` - Dropping projectile (bombs)
- `IsHoming` - Homing missile
- `IsDropping` - Dropping (bombs)
- `IsHoming` - Homing missile
- `IsHigh` - High altitude
- `IsArcing` - Ballistic arc
- `IsDropping` - Dropping
- `IsHoming` - Homing
- `IsFlameEquipped` - Flame effect
- `IsInvisible` - Invisible projectile
- `IsFaceless` - No facing sprite
- `IsHigh` - High altitude
- `IsAntiAircraft` - Anti-air
- `IsFueled` - Fueled weapon
- `IsDropping` - Dropping
- `IsHoming` - Homing
- `IsArcing` - Ballistic arc
- `IsInvisible` - Invisible
- `IsFaceless` - No facing
- `IsHigh` - High altitude
- `IsAntiAircraft` - Anti-air
- `IsFueled` - Fueled
- `IsDropping` - Dropping
- `IsHoming` - Homing
- `IsArcing` - Ballistic
- `IsInvisible` - Invisible
- `IsFaceless` - No facing
- `IsHigh` - High altitude
- `IsAntiAircraft` - Anti-air
- `IsFueled` - Fueled
- `MaxSpeed` (MPHType) - Max speed
- `Range` (int) - Range (0 = auto-calc)
- `ROF` (int) - Rate of fire
- `Attack` (int) - Damage
- `Warhead` (WarheadType) - Damage type
- `Arming` (int) - Arming delay
- `Explosion` (AnimType) - Impact animation
- `IsInvisible` - Invisible
- `IsFaceless` - No facing
- `IsHigh` - High altitude
- `IsAntiAircraft` - Anti-air
- `IsFueled` - Fueled
- `IsDropping` - Dropping
- `IsHoming` - Homing
- `IsArcing` - Ballistic
- `Explosion` (AnimType) - Impact anim
- `Warhead` (WarheadType) - Damage type
- `Arming` (int) - Arming delay
- `ROF` (int) - Rate of fire
- `Attack` (int) - Damage
- `Range` (int) - Range
- `MaxSpeed` (MPHType) - Max speed
- `Fires` (BulletType) - Fired bullet type

**BulletType enum (key values):**
- `BULLET_BULLET` - Small arms (instant/hitscan)
- `BULLET_FLAME` - Flamethrower
- `BULLET_NAPALM` - Napalm (tumbles 6 frames)
- `BULLET_GRENADE` - Grenade (tumbles 8 frames)
- `BULLET_NUKE_UP/DOWN` - Nuke missile (large splash 48)
- `BULLET_TOW` - TOW missile (extra damage vs air)
- `BULLET_FLAME` - Flame (animates smoke puffs)

**Occupy_List return values:**
- Flame: 25-cell spillage
- Nuke: 48-cell spillage
- Altitude > 0: 5-cell at altitude + 5-cell at ground
- Default: 10-cell spillage

## Functions and methods

### Constructors/Destructor/Allocation

**BulletClass::BulletClass()** - Default constructor
- Initializes to unusable state
- Sets Class=0, Payback=NULL, IsToAnimate=false, Altitude=0, Riser=0, TarCom=NONE, Strength=0, IsLocked=true, IsInaccurate=false, IsToAnimate=false, Payback=0, Riser=0, Strength=0, IsLocked=true, IsInaccurate=false, IsToAnimate=false

**BulletClass::BulletClass(BulletType id)** - Parameterized constructor
- Sets Class to BulletTypeClass::As_Reference(id)
- Initializes: Altitude=0, IsInaccurate=false, IsLocked=true, IsToAnimate=false, Payback=0, Riser=0, Strength=Class->MaxStrength, TarCom=NONE

**BulletClass::operator new()** - Pool allocation
- Allocates from Bullets pool
- Calls Set_Active() on allocated object

**BulletClass::operator delete()** - Pool deallocation
- Marks inactive, returns to Bullets pool

**BulletClass::Init()** - System initialization
- Frees all bullets from pool
- Creates dummy instance to capture VTable pointer

**BulletClass::Validate()** (CHEAT_KEYS)
- Validates index in Bullets array (0 to BULLET_MAX-1)

### Physics & Motion

**BulletClass::AI()** - **Core projectile logic** (called once per tick)
- **Ballistic arc (IsArcing)**: Altitude += Riser; if Altitude <= 0 → forced detonation; Riser -= GRAVITY (3) if > -100
- **Dropping (IsDropping)**: Altitude += Riser; if Altitude <= 0 → forced; Riser -= 1 if > -100
- **Homing (IsHoming)**: Every other frame (Frame & 0x01), faces target if target legal
- **Physics()**: Moves projectile via Physics(coord, facing) → returns IMPACT_EDGE, IMPACT_NONE, or IMPACT_NORMAL
- **Flame animation**: Alternates IsToAnimate, spawns ANIM_SMOKE_PUFF every other frame
- **Body rotation**: PrimaryFacing.Rotation_Adjust(Class->ROT) every frame

**Impact handling:**
- **IMPACT_EDGE**: Mark, Delete_This (explodes off-map)
- **IMPACT_NONE/NORMAL**: Mark(), check cell overlay for high terrain, check anti-air bonus vs aircraft (TOW gets +33%, others +50%), check fuse (Fuse_Checkup/Fuse_Target)
- **Fuse_Checkup**: Returns true if fuse expired/impact
- **Fuse_Target**: Returns true if reached target

**Impact damage:**
- Ground targets: `Explosion_Damage(Coord, Strength, Payback, Class->Warhead)`
- Aircraft targets (SAM): If distance < 128 leptons, `AircraftClass->Take_Damage(Strength, 0, Warhead, Payback)`
- TOW missile vs air: Strength += Strength/3
- Other anti-air: Strength += Strength/2
- Invisible projectiles: Scatter impact coord by 0x20

**Explosion effects:**
- Creates AnimClass(Class->Explosion, Coord)
- Sorts above target
- Nuke fix: Sets animation owner to Payback's house for kill tracking

**Fuse/Arm logic:**
- `Arm_Fuse(start, target, range, arming)` - Arms fuse
- `Fuse_Checkup(coord)` - Checks if reached target/range
- `Fuse_Target()` - Returns true if at target
- `Arm_Fuse(start, target, range, arming)` - Arms fuse

### Unlimbo & Launch

**BulletClass::Unlimbo(coord, dir)** → bool - **Launch projectile**
- Calls ObjectClass::Unlimbo(coord)
- **Homing**: Keeps specified dir, doesn't override facing
- **Inaccuracy**: Adds scatter to target:
  - Homing/arcing: CEP scatter (distance/3, max 0x200)
  - Grenade: distance/4, max 0x80
  - High-speed: overshoot target by 0x100
- **Range calculation**: 
  - If no Class->Range: (Distance / MaxSpeed) + 4 frames
  - Else Class->Range
- **Speed**: Class->MaxSpeed (MPH_LIGHT_SPEED → MPH_IMMOBILE)
- **Arching**: Speed += Distance/32, min 25
- **Fly_Speed(255, speed)**
- **Arm_Fuse(start, target, range, arming)**
- **Ballistic vertical**: Altitude=1, Riser = (Distance/2)/(speed+1) * GRAVITY, min 10
- **Dropping**: Altitude=24px leptons, Riser=0
- **Invisible + light speed**: Instant teleport to target

### Occupancy & Rendering

**BulletClass::Occupy_List()** → short const*
- Flame: `Coord_Spillage_List(Coord, 25)`
- Nuke up/down: `Coord_Spillage_List(Coord, 48)`
- Altitude > 0: 5-cell at altitude + 5-cell at ground (offset adjusted)
- Default: `Coord_Spillage_List(Coord, 10)`

**BulletClass::Mark(mark)** → bool
- Calls ObjectClass::Mark()
- If !IsInvisible: `Map.Refresh_Cells(Coord_Cell(Coord), Occupy_List())`

**BulletClass::Draw_It(x, y, window)** - Rendering
- Invisible projectiles: return
- No shapeptr: return
- Shape selection: `BodyShape[facing]` (or faceless=0)
- Tumbling: Napalm (Frame%6), Grenade (Frame%8)
- Altitude > 0: Draw shadow (SHAPE_PREDATOR|FADING), y -= Lepton_To_Pixel(Altitude)
- Main body: SHAPE_NORMAL or SHAPE_GHOST (if translucent), SHAPE_CENTER|SHAPE_WIN_REL

### Targeting & Homing

**BulletClass::AI()** - Homing logic
- Every other frame (Frame & 0x01): `PrimaryFacing.Set_Desired(Direction256(Coord, As_Coord(TarCom)))`
- Only if Class->IsHoming && Target_Legal(TarCom)

**Anti-air bonus** (in AI):
- Anti-air weapon + aircraft target within 128 leptons:
  - TOW: Strength += Strength/3
  - Other: Strength += Strength/2

**Inaccuracy handling** (in Unlimbo):
- Homing/arcing: CEP scatter (distance/3, max 512 leptons)
- Grenade: distance/4, max 128 leptons
- High-speed: overshoot by 256 leptons
- Range limit: clipped to firer's weapon range

### Detach & Cleanup

**BulletClass::Detach(target, all)**
- Clears Payback if matches
- Clears TarCom if all && target == TarCom

**BulletClass::Detach(target, all)**
- Clears Payback if matches
- Clears TarCom if all && target == TarCom

**BulletClass::Detach(target, all)**
- Clears Payback if matches
- Clears TarCom if all && target == TarCom

### Allocation

**BulletClass::operator new()** - Pool allocation
- `Bullets.Allocate()`, calls Set_Active()

**BulletClass::operator delete(ptr)** - Pool deallocation
- Sets IsActive=false, `Bullets.Free()`

**BulletClass::Init()** - System init
- `Bullets.Free_All()`
- Creates dummy to capture VTable

### Target Conversion

**BulletClass::As_Target()** → TARGET
- `Build_Target(KIND_BULLET, Bullets.ID(this))`

### Occupancy

**BulletClass::Occupy_List()** → short const*
- Flame: 25-cell spillage
- Nuke: 48-cell spillage
- Altitude: 5-cell at altitude + 5-cell ground (offset)
- Default: 10-cell spillage

**BulletClass::Mark(mark)** → bool
- ObjectClass::Mark()
- If !IsInvisible: Map.Refresh_Cells(cell, Occupy_List())

### Rendering

**BulletClass::Draw_It(x, y, window)**
- Invisible: return
- No shape: return
- Shape: BodyShape[facing] (faceless=0)
- Tumbling: Napalm (Frame%6), Grenade (Frame%8)
- Altitude: Draw shadow (PREDATOR|FADING), y -= Altitude pixels
- Main: SHAPE_NORMAL or GHOST (translucent), CENTER|WIN_REL

## Inputs, outputs and side effects

**State mutations:**
- Position (Coord), Altitude, Riser
- Target (TarCom), Strength, IsLocked
- Animation state (IsToAnimate)
- Facing (PrimaryFacing)

**External systems affected:**
- Map: Refresh_Cells, Refresh_Cells for occupancy
- Explosion_Damage: Ground damage
- AircraftClass::Take_Damage: Air damage
- AnimClass: Explosion animations
- Map: Refresh_Cells for occupancy
- AnimClass: Explosion animations

## Calls into / called by

**Called by:**
- Weapon firing: BulletClass::new(), Unlimbo()
- Logic system: AI() per tick
- Rendering: Draw_It()
- Map: Mark(), Occupy_List()
- Damage system: Explosion_Damage(), AircraftClass::Take_Damage()

**Calls into:**
- ObjectClass: AI(), Mark(), Unlimbo(), Mark(), Detach()
- MapClass: Refresh_Cells(), Coord_Spillage_List(), Coord_Cell(), Coord_Scatter()
- MapClass: Refresh_Cells, Coord_Spillage_List
- AnimClass: ANIM_SMOKE_PUFF, explosion animations
- Explosion_Damage(): Ground damage
- AircraftClass::Take_Damage(): Air damage
- Weapon system: Get_Image_Data(), Warhead, ROF, etc.
- Physics(): Motion calculation
- Fuse_Checkup(), Fuse_Target(), Arm_Fuse()
- Explosion_Damage()
- AircraftClass::Take_Damage()
- AnimClass: Explosion animations

## Related original files
- **BULLET.H** - Class declarations, BulletTypeClass, BulletType enum
- **WEAPON.CPP/.H** - WeaponTypeClass, weapon definitions
- **WARHEAD.CPP/.H** - WarheadTypeClass, damage modifiers
- **AIRCRAFT.CPP/.H** - AircraftClass (air targets)
- **FUNCTION.H** - Core includes, Distance(), Random_Pick(), Explosion_Damage()
- **ANIM.CPP/.H** - AnimClass (explosions, smoke)
- **OBJECT.CPP/.H** - Base ObjectClass
- **BULLET.H** - BulletTypeClass, BulletType enum, bullet definitions

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/combat/bullet.rs` - BulletClass core
- `cnc-core/src/combat/projectile.rs` - Projectile physics
- `cnc-tiberian-dawn/src/combat/td_bullet.rs` - TD-specific bullets
- `cnc-core/src/combat/damage.rs` - Explosion_Damage, Warhead system
- `cnc-core/src/entity/projectile.rs` - Projectile entity
- `cnc-core/src/combat/homing.rs` - Homing logic
- `cnc-core/src/combat/ballistics.rs` - Ballistic arc, gravity
- `cnc-core/src/combat/explosion.rs` - Explosion effects
- `cnc-core/src/render/bullet.rs` - Bullet rendering

**Not yet ported:**
- Ballistic arc physics (gravity, riser, altitude)
- Homing missile logic (every-other-frame facing)
- Inaccuracy/CEP scatter system
- Projectile pooling (new/delete pool)
- Fuse/arming system
- Invisible/light-speed projectiles
- Anti-air damage bonuses
- TOW missile special handling
- Napalm/grenade tumbling animation
- Flame thrower smoke puffs
- Projectile pooling (new/delete from pool)
- VTable capture for RTTI

## Edge cases and compatibility notes

1. **Gravity constant**: `GRAVITY = 3` (leptons/frame²)

2. **Ballistic arc**: Altitude starts at 1, Riser calculated from distance/speed * GRAVITY, min 10. Vertical motion simulated crudely.

3. **Dropping projectiles**: Altitude = 24 pixels (leptons), Riser = 0. Straight down.

4. **Homing**: Updates facing every other frame (Frame & 0x01). Uses Direction256 for 256-direction precision.

4. **Inaccuracy**: 
   - Homing/arcing: CEP scatter (distance/3, max 512 leptons)
   - Grenade: distance/4, max 128 leptons
   - High-speed: overshoot by 256 leptons
   - Range limited to firer's weapon range

5. **Anti-air bonus**: Within 128 leptons (0.5 cells):
   - TOW: +33% damage
   - Other: +50% damage

6. **Invisible + light speed**: Instant teleport to target (no flight)

6. **Explosion damage**: Ground targets use Explosion_Damage(). Aircraft use direct Take_Damage().

7. **Invisible projectiles**: Impact scattered by 0x20 leptons for visual effect.

8. **Nuke fix (2020)**: Animation owner set to Payback's house for kill tracking (TDRA-6610).

9. **Flame animation**: Alternates IsToAnimate, spawns ANIM_SMOKE_PUFF every other frame.

9. **Explosion owner fix**: Animation owner set to Payback's house for kill credit.

10. **Tumbling projectiles**: Napalm (Frame%6), Grenade (Frame%8) frame cycling.

11. **Flamethrower**: Alternates smoke puffs every frame.

12. **High-speed projectiles**: Overshoot target by 256 leptons (no homing).

13. **Fuse system**: Arm_Fuse(start, target, range, arming). Fuse_Checkup(coord) checks range. Fuse_Target() checks target reached.

13. **Fuse**: Arm_Fuse(start, target, range, arming). Fuse_Checkup(coord), Fuse_Target().

14. **Range calculation**: Auto = Distance/MaxSpeed + 4 frames. Explicit = Class->Range.

15. **Speed**: Class->MaxSpeed. LIGHT_SPEED → IMMOBILE. Arching: +Distance/32, min 25.

16. **Dropping**: Altitude=24px leptons, Riser=0.

17. **Invisible + LIGHT_SPEED**: Instant teleport to target.

18. **Occupancy**: Flame=25, Nuke=48, Altitude=5+5, Default=10 cell spillage.

19. **Tumbling**: Napalm (6 frames), Grenade (8 frames).

20. **Flame**: Alternates smoke puffs (ANIM_SMOKE_PUFF).

21. **VTable**: Captured in Init() via dummy allocation.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/BULLET.CPP at f1f0d42
