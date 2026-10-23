# TIBERIANDAWN/RESOURCE/ — Tiberian Dawn Game Data Assets

## Responsibility
Runtime game data for Tiberian Dawn: **maps, missions, campaigns, briefing videos, and localized strings**. Consumed directly by `TiberianDawn.dll` engine at runtime. Identical structure to `REDALERT/RESOURCE/` but with TD-specific content.

## Contents

| Category | Format | Description |
|----------|--------|-------------|
| **Maps** | `.map` | Multiplayer/skirmish maps — INI-based, 3 theaters (Temperate, Arctic, Desert) |
| **Missions** | `.ini` | Campaign missions (15 per faction) — triggers, briefings, map+script |
| **Campaigns** | `.cam` | GDI/NOD campaign definitions — mission order, videos |
| **Briefings** | `.vqa` | Westwood VQA videos — iconic EVA/CABAL briefings |
| **Strings** | `.csf` | String tables — localized UI, unit names, mission text |
| **Save Games** | `.sav` | Serialized game state |

## Key Differences from Red Alert
- **3 theaters** vs 6 (no Urban, NewUrban, Winter)
- **2 campaigns** (GDI, NOD) vs Allies/Soviet + expansion
- **Briefing videos** feature EVA (GDI) / CABAL (NOD) — distinct from RA's mission briefings
- **Tiberium** vs Ore/Gems resource types in map data

## Data Flow
Identical to Red Alert — engine loads via `CCFileClass`, `CCINIClass`, `MapClass`.

## Integration Points
- **Engine**: `TIBERIANDAWN/` C++ code
- **Map Editor**: `CnCTDRAMapEditor/TiberianDawn/GamePlugin.cs` reads same formats
- **Map Editor**: `Utility/Megafile.cs` packages for distribution