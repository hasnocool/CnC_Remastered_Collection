# REDALERT/RESOURCE/ — Red Alert Game Data Assets

## Responsibility
Runtime game data for Red Alert: **maps, missions, campaigns, briefing videos, and localized strings**. Consumed directly by `RedAlert.dll` engine at runtime. Not compiled — loaded via Westwood file formats.

## Contents

| Category | Format | Description |
|----------|--------|-------------|
| **Maps** | `.map` | Multiplayer/skirmish maps — INI-based, references theater, tileset, starting positions |
| **Missions** | `.ini` | Campaign missions — trigger scripts, briefing references, map + script combined |
| **Campaigns** | `.cam` | Campaign meta-files — mission ordering, briefing videos, faction availability |
| **Briefings** | `.vqa` | Westwood VQA video format — mission briefings/debriefings |
| **Strings** | `.csf` | String tables — localized UI, mission text, unit names |
| **Save Games** | `.sav` | Serialized game state (engine `Save`/`Load`) |

## Data Flow
```
RedAlert.dll (engine)
  → FileSystem.Open("RESOURCE/map.map")
  → INIParser.Parse() → Map data
  → HouseClass.Load() / MapClass.Load() / TriggerClass.Load()
  → Game starts
```

## Integration Points
- **Engine**: `REDALERT/` C++ code loads via `CCFileClass`, `CCINIClass`, `MapClass`
- **Map Editor**: `CnCTDRAMapEditor/RedAlert/GamePlugin.cs` reads same formats for editing
- **Map Editor**: `Utility/Megafile.cs` can package into `.mix` for distribution

## Notes
- **Read-only at runtime** — engine never writes here (saves go to user directory)
- **Case-insensitive** filenames (Windows heritage)
- **No subdirectories** — flat file list per Westwood convention