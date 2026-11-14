# PATH/FILE
original/TIBERIANDAWN/RULES.CPP

## Purpose
Game rules and difficulty configuration for Tiberian Dawn. Defines `RulesClass` (AI behavior parameters) and `DifficultyClass` (per-difficulty bias multipliers), both loaded from RULES.INI.

## Where it fits in the game
Global configuration - `RulesClass Rule` and per-house `DifficultyClass` instances are globals (GLOBALS.CPP). Read by AI systems for building ratios, attack timing, unit biases, and difficulty scaling.

## Classes, structs, enums and important data

### DifficultyClass
**Per-difficulty multipliers** - Applied to house stats based on difficulty setting.

| Field | Default | Purpose |
|-------|---------|---------|
| `FirepowerBias` | 1 | Damage multiplier |
| `GroundspeedBias` | 1 | Ground unit speed |
| `AirspeedBias` | 1 | Aircraft speed |
| `ArmorBias` | 1 | Armor effectiveness |
| `ROFBias` | 1 | Rate of fire |
| `CostBias` | 1 | Build cost multiplier |
| `BuildSpeedBias` | 1 | Construction speed |
| `RepairDelay` | 0.02f | Repair tick delay |
| `BuildDelay` | 0.03f | Build tick delay |
| `IsBuildSlowdown` | false | Extra build delay |
| `IsWallDestroyer` | true | Can destroy walls |
| `IsContentScan` | false | Scans crate contents |

### RulesClass
**Global AI behavior rules** - Loaded from RULES.INI `[Basic]` and `[Difficulty]` sections.

| Field | Default | Purpose |
|-------|---------|---------|
| `AttackInterval` | 3 | Minutes between AI attacks |
| `BaseSizeAdd` | 3 | Extra base structures |
| `PowerSurplus` | 50 | Power buffer % |
| `AttackDelay` | 5 | Initial attack delay (min) |
| `PowerEmergencyFraction` | 0xC0 (192/256=75%) | Low power threshold |
| `HelipadRatio` | 0x1E (30/256) | Helipads per factory |
| `HelipadLimit` | 5 | Max helipads |
| `AARatio` | 0x24 (36/256) | AA guns per factory |
| `AALimit` | 10 | Max AA |
| `DefenseRatio` | 0x66 (102/256) | Defenses per factory |
| `DefenseLimit` | 40 | Max defenses |
| `WarRatio` | 0x19 (25/256) | War factories per factory |
| `WarLimit` | 2 | Max war factories |
| `BarracksRatio` | 0x28 (40/256) | Barracks per factory |
| `BarracksLimit` | 2 | Max barracks |
| `RefineryLimit` | 4 | Max refineries |
| `RefineryRatio` | 0x28 (40/256) | Refineries per factory |
| `AirstripRatio` | 0x1E (30/256) | Airstrips per factory |
| `AirstripLimit` | 5 | Max airstrips |
| `MaxIQ` | 5 | Max AI IQ level |
| `IQSuperWeapons` | 4 | IQ for superweapons |
| `IQProduction` | 5 | IQ for production |
| `IQGuardArea` | 4 | IQ for guard area |
| `IQRepairSell` | 1 | IQ for repair/sell |
| `IQCrush` | 2 | IQ for crushing |
| `IQScatter` | 3 | IQ for scattering |
| `IQContentScan` | 4 | IQ for crate scanning |
| `IQAircraft` | 4 | IQ for aircraft |
| `IQHarvester` | 2 | IQ for harvesters |
| `IQSellBack` | 2 | IQ for selling |
| `InfantryReserve` | 3000 | Credits reserved for infantry |
| `InfantryBaseMult` | 1 | Infantry production multiplier |
| `IsComputerParanoid` | true | AI builds extra defenses |
| `AllowSuperWeapons` | true | AI uses superweapons |

**Ratio format**: Fixed-point 8.8 (value/256). E.g., `HelipadRatio=0x1E` = 30/256 ≈ 0.117 helipads per factory.

## Functions and methods

### DifficultyClass::DifficultyClass()
**Default constructor** - Initializes all biases to 1.0, delays to small values, flags to defaults. Expected to be overridden by INI.

### RulesClass::RulesClass()
**Default constructor** - Initializes all AI rules to defaults. Expected to be overridden by INI.

## Inputs, outputs and side effects

**Configuration source**: RULES.INI (parsed elsewhere, likely in HOUSE.CPP or INIT.CPP)

**Consumers**: AI building logic, unit stat calculation, difficulty scaling

## Calls into / called by

**Called by:** (construction only)
- Global `Rule` instance (GLOBALS.CPP)
- House difficulty assignment

**Calls into:** None (pure data)

**Consumed by:**
- `HouseClass` - Applies `DifficultyClass` biases
- AI building logic - Uses `RulesClass` ratios
- Unit stat calculation - Applies `FirepowerBias`, `ArmorBias`, etc.
- Superweapon logic - Checks `AllowSuperWeapons`, `IQSuperWeapons`

## Related original files
- **RULES.H** - Class declarations
- **GLOBALS.CPP** - `RulesClass Rule;` global instance
- **HOUSE.H/.CPP** - `HouseClass` has `DifficultyClass` member
- **INI.H/.CPP** - INI parsing (not shown here)
- **AI building logic** - Various files use ratios

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/rules.rs` - RulesClass, DifficultyClass
- `cnc-tiberian-dawn/src/ai/td_rules.rs` - TD-specific defaults
- `cnc-core/src/config/ini.rs` - INI loading for rules

**Not yet ported:**
- Fixed-point ratio parsing (8.8 format)
- INI section binding (`[Basic]`, `[Difficulty]`)
- Per-house difficulty assignment
- Bias application to unit stats
- IQ-based behavior gating

## Edge cases and compatibility notes

1. **Ratios are 8.8 fixed-point**: Divide by 256 for float; multiply for integer math
2. **Defaults are fallbacks**: RULES.INI expected to override all values
3. **DifficultyClass per-house**: Each house can have different difficulty
4. **CostBias affects factory**: `Balance = Cost * CostBias` in FACTORY.CPP
5. **IQ thresholds**: AI behaviors gated by `House->IQ >= Rules.IQ*`
6. **PowerEmergencyFraction**: 0xC0 = 192/256 = 75% power = emergency
7. **BuildDelay/RepairDelay**: Float seconds per tick (0.03 = ~2 ticks at 15 FPS)
8. **IsComputerParanoid**: Extra defensive building placement
9. **AllowSuperWeapons**: Global toggle for AI superweapon use

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/RULES.CPP at f1f0d42
