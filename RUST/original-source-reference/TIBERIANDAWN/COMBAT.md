# PATH/FILE
original/TIBERIANDAWN/COMBAT.CPP

## Purpose
Core combat damage system for Tiberian Dawn. Implements the damage modification pipeline (armor vs warhead) and explosion area-of-effect damage distribution.

## Where it fits in the game
Core combat system - all damage in the game flows through `Modify_Damage()` for armor/warhead interactions, and `Explosion_Damage()` for area-of-effect damage. Used by all weapons, bullets, explosions, and special weapons.

## Classes, structs, enums and important data

### WarheadType enum (key values)
| Warhead | Description |
|---------|-------------|
| `WARHEAD_NONE` | No damage |
| `WARHEAD_SA` | Small arms (standard bullets) |
| `WARHEAD_HE` | High explosive (double vs aircraft) |
| `WARHEAD_AP` | Armor piercing |
| `WARHEAD_FIRE` | Fire/flame |
| `WARHEAD_SPORE` | Tiberium spore |
| `WARHEAD_HE` | High explosive |
| `WARHEAD_FIRE` | Fire/flame |
| `WARHEAD_SPORE` | Tiberium spore |

### ArmorType enum (key values)
| Armor | Description |
|-------|-------------|
| `ARMOR_NONE` | No armor (infantry) |
| `ARMOR_LIGHT` | Light armor |
| `ARMOR_HEAVY` | Heavy armor (tanks) |
| `ARMOR_STEEL` | Steel (buildings) |
| `ARMOR_WOOD` | Wood |
| `ARMOR_ALUMINUM` | Aluminum (aircraft) |
| `ARMOR_NONE` | Infantry/no armor |
| `ARMOR_WOOD` | Wood |
| `ARMOR_LIGHT` | Light vehicles |
| `ARMOR_HEAVY` | Heavy armor |
| `ARMOR_STEEL` | Buildings/heavy |
| `ARMOR_ALUMINUM` | Aircraft |

### WarheadTypeClass (static)
**Static warhead data** - defines damage behavior per warhead type.

**Key data members:**
- `Modifier[ArmorType]` - Damage multiplier vs each armor type
- `SpreadFactor` (int) - Distance falloff rate
- `IsWallDestroyer` (bool) - Can destroy walls
- `IsWoodDestroyer` (bool) - Can destroy wooden walls
- `IsAntiAircraft` (bool) - Anti-air weapon
- `IsAntiGround` (implied) - Anti-ground weapon
- `IsHigh` (bool) - High altitude detonation

### Constants
- `ICON_LEPTON_W` - Icon width in leptons
- `ICON_LEPTON_W >> 1` - Half icon width

## Functions and methods

### Modify_Damage(damage, warhead, armor, distance) → int
**Core damage modification** - applies armor/warhead interaction and distance falloff.

**Algorithm:**
1. **Early exit:** Returns 0 if `Special.IsInert`, no damage, or `WARHEAD_NONE`
2. **Armor modifier:** `damage = Fixed_To_Cardinal(damage, whead->Modifier[armor])`
   - Uses warhead's `Modifier[armor]` table (fixed-point multiplication)
3. **Distance falloff:**
   - `distance >>= whead->SpreadFactor` (right shift by spread factor)
   - `distance = Bound(distance, 0, 16)` (clamp 0-16)
   - `damage >>= distance` (right shift by distance)
4. **Minimum damage:** Returns at least 1 if any damage remains

**SpreadFactor:** Controls how quickly damage falls off with distance. Higher = faster falloff.

**Armor modifier:** Fixed-point multiplication via `Fixed_To_Cardinal(damage, Modifier[armor])`.

**Distance falloff:** Right-shifts damage by `distance >> SpreadFactor`, clamped to 16.

**Minimum damage:** Always returns ≥1 if any damage remains.

### Explosion_Damage(coord, strength, source, warhead)
**Area-of-effect damage** - distributes explosion damage to objects in radius.

**Algorithm:**
1. **Early exit:** Returns if no strength, inert, or `WARHEAD_NONE`
2. **Range calculation:** `range = ICON_LEPTON_W + (ICON_LEPTON_W >> 1)` (1.5 icon widths)
3. **Cell scan:** Center cell + 8 adjacent cells (FACING_NONE to FACING_NW)
4. **Object collection:** Builds array of up to 32 objects from cells
   - Skips `IsToDamage` flagged objects
   - Skips source object
   - Max 32 objects
5. **Damage application:**
   - For each object: calculates distance
   - Buildings hit on any cell: distance = 0 (direct hit)
   - Range check: `distance < range`
   - **Anti-air bonus:** `WARHEAD_HE` vs aircraft → double damage
   - Calls `object->Take_Damage(damage, distance, warhead, source)`
6. **Wall destruction:**
   - Reduces tiberium: `cellptr->Reduce_Tiberium(strength / 10)`
   - Wall check: `OverlayTypeClass::IsWallDestroyer` or `IsWoodDestroyer` + `IsWooden`
   - Calls `Reduce_Wall(strength)`

**Damage distribution:**
- Radius: 1.5 icon widths (`ICON_LEPTON_W + ICON_LEPTON_W/2`)
- Max 32 objects per explosion
- Direct hit on building = distance 0
- Range check: `distance < range`
- HE vs aircraft: 2x damage

## Inputs, outputs and side effects

**Modify_Damage:**
- Input: raw damage, warhead, armor, distance
- Output: modified damage (≥1 if >0)
- Side effects: None (pure function)

**Explosion_Damage:**
- Input: coordinate, strength, source, warhead
- Output: None (applies damage via `Take_Damage`)
- Side effects:
  - Calls `Take_Damage` on affected objects
  - Reduces tiberium: `cellptr->Reduce_Tiberium(strength/10)`
  - Destroys walls via `Reduce_Wall()`
  - Object `Take_Damage` handles destruction, kill credit

## Calls into / called by

**Called by:**
- BulletClass::AI() - projectile impact
- TechnoClass::Take_Damage() - damage pipeline
- AnimClass::AI() - explosion animations (ANIM_ATOM_BLAST, etc.)
- Special weapons: Nuke, Ion Cannon, Air Strike
- Building/Unit/Infantry Take_Damage()

**Calls into:**
- WarheadTypeClass::Modifier[armor] - armor table
- Fixed_To_Cardinal() - fixed-point math
- Distance() - coordinate distance
- ObjectClass::Take_Damage() - applies damage
- CellClass::Reduce_Tiberium() - tiberium reduction
- CellClass::Reduce_Wall() - wall destruction
- OverlayTypeClass::IsWallDestroyer/IsWoodDestroyer - wall checks

## Related original files
- **COMBAT.H** - Function declarations
- **WARHEAD.H** - WarheadTypeClass, WarheadType enum
- **ARMOR.H** - ArmorType enum
- **FUNCTION.H** - Core includes, Fixed_To_Cardinal, Distance, Bound
- **OBJECT.CPP/.H** - ObjectClass::Take_Damage
- **BULLET.CPP** - Bullet impact
- **ANIM.CPP** - Explosion animations
- **TECHNO.CPP** - TechnoClass::Take_Damage
- **BUILDING.CPP** - BuildingClass::Take_Damage
- **AIRCRAFT.CPP** - AircraftClass::Take_Damage

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/combat/damage.rs` - Modify_Damage, Explosion_Damage
- `cnc-core/src/combat/warhead.rs` - WarheadTypeClass, ArmorType
- `cnc-tiberian-dawn/src/combat/td_combat.rs` - TD-specific warheads
- `cnc-core/src/combat/explosion.rs` - Explosion_Damage

**Not yet ported:**
- Warhead armor modifier table (2D array)
- Distance falloff with SpreadFactor
- Explosion radius (1.5 icon widths)
- HE vs aircraft 2x damage
- Wall destruction logic (IsWallDestroyer/IsWoodDestroyer)
- Tiberium reduction on explosion
- Max 32 objects per explosion

## Edge cases and compatibility notes

1. **Minimum damage:** Always ≥1 if any damage gets through modifiers.

2. **Distance falloff:** Right-shift by `distance >> SpreadFactor`, clamped 0-16, then `damage >>= distance`.

3. **HE vs aircraft:** `WARHEAD_HE` deals 2x damage to aircraft (`RTTI_AIRCRAFT`).

4. **Building direct hit:** `impacto == object` sets distance=0 for building damage.

5. **Tiberium reduction:** `Reduce_Tiberium(strength/10)` on explosion cell.

5. **Tiberium:** Reduced by `strength/10` on explosion cell.

6. **Wall destruction:** `WarheadTypeClass::IsWallDestroyer` or `IsWoodDestroyer` + `IsWooden`.

6. **Wall destruction:** Requires `IsWallDestroyer` or (`IsWoodDestroyer` + `IsWooden`).

7. **Max objects:** 32 objects per explosion (fixed array).

7. **Max objects:** 32 objects per explosion limit.

8. **Range:** `ICON_LEPTON_W + (ICON_LEPTON_W >> 1)` = 1.5 icon widths.

8. **Range:** 1.5 icon widths (center + adjacent cells).

9. **Inert check:** `Special.IsInert` disables all damage.

9. **Inert:** `Special.IsInert` disables all damage.

10. **Inert check:** `Special.IsInert` returns 0 damage immediately.

10. **Inert:** Global inert flag disables all damage.

11. **Distance calculation:** `Distance(coord, object->Center_Coord())` in leptons.

11. **Distance:** Leptons from explosion center to object center.

12. **Building hit:** Any cell of building = direct hit (distance=0).

12. **Building hit:** Any occupied cell = direct hit.

13. **SpreadFactor:** Controls falloff speed. Higher = faster falloff.

13. **SpreadFactor:** Higher = faster distance falloff.

14. **Minimum damage:** `return(damage)` ensures ≥1 if >0.

14. **Min damage:** Always ≥1 if >0 after modifiers.

15. **TOW missile:** Special case in BulletClass - extra 33% vs air.

15. **TOW:** Extra 33% vs air (handled in BulletClass, not here).

16. **Source tracking:** `source` parameter for kill credit.

16. **Source:** `TechnoClass* source` for kill credit.

17. **TOW bonus:** Handled in BulletClass::AI(), not Modify_Damage.

17. **TOW bonus:** Handled in BulletClass, not here.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/COMBAT.CPP at f1f0d42
