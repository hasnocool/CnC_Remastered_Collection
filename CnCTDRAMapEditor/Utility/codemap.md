# CnCTDRAMapEditor/Utility/ — Shared Helpers & Infrastructure

## Responsibility
Cross-cutting utilities used by the editor: file format parsers, archive handling, texture management, undo/redo, Steam integration, and game data access. **No UI dependencies** — pure .NET libraries.

## Design Patterns

### File Format Parsers
| Utility | Format | Purpose |
|---------|--------|---------|
| `INI.cs` | Westwood INI | `rules.ini`, `art.ini`, map `.ini` — section/key/value with inheritance |
| `Megafile.cs` | `.mix` (Westwood archive) | Read game assets: `.shp`, `.vxl`, `.tem`, `.til`, `.aud` |
| `MegafileBuilder.cs` | `.mix` write | Create/modify archives for map packaging |
| `WWCompression.cs` | Westwood LZSS | Decompress game assets (`.shp`, `.vxl` frames) |
| `TGASharpLib.cs` | Truevision TGA | Load/save textures (also in `SCRIPTS/tgautil.py`) |

### Asset Management
| Utility | Role |
|---------|------|
| `TilesetManager.cs` | Loads `.tem` (template) + `.til` (tileset) from `.mix` → provides `Tile` bitmaps per `TemplateType` |
| `TextureManager.cs` | Caches all `Bitmap` resources: tiles, overlays, smudges, unit sprites, icons — LRU eviction |
| `TeamColorManager.cs` | Remaps palette indices in sprites to player colors (remap tables per `HouseType`) |
| `GameTextManager.cs` | Loads `.csf` (string table) for localized UI strings |

### Editing Infrastructure
| Utility | Pattern |
|---------|---------|
| `UndoRedoList.cs` | Command stack — `Push(ICommand)`, `Undo()`, `Redo()`, `CanUndo`, `CanRedo`, `Clear()` |
| `PropertyTracker.cs` | Auto-generates undo commands for `INotifyPropertyChanged` objects |
| `ExtensionMethods.cs` | LINQ-style helpers for collections, geometry, enums |
| `CRC.cs` | CRC32 for map checksums, asset validation |
| `MRU.cs` | Most Recently Used file list (persisted to settings) |

### Steam Workshop
| Utility | Role |
|---------|------|
| `SteamworksUGC.cs` | High-level wrapper: `CreateItem()`, `UpdateItem()`, `PublishItem()`, `QueryItems()`, `DownloadItem()` — wraps `Steamworks.NET` P/Invoke |

## Data & Control Flow

### Asset Loading Pipeline
```
TilesetManager.Load(TheaterType.Temperate)
  → Megafile.Open("temperat.mix")
  → Megafile.Read("temperat.tem") → Template definitions
  → Megafile.Read("temperat.til") → Tile graphics (LZSS compressed)
  → WWCompression.Decompress() → Raw pixel data
  → TextureManager.Cache(tileIndex, Bitmap)
  → Returns Tileset with TemplateType → Tile mapping
```

### Team Color Remapping
```
TextureManager.GetSprite(unitType, facing)
  → TeamColorManager.Apply(sprite, house.RemapTable)
    → LockBits → iterate pixels → replace remappable indices → UnlockBits
  → Returns recolored Bitmap (cached per house)
```

### Undo/Redo
```
Tool mutates model.Property = value
  → PropertyTracker.PropertyChanged fires
    → Creates UndoCommand(oldValue, newValue, setter)
    → UndoRedoList.Push(command)
```

## Integration Points

### Dependencies
- `Steamworks.NET/` — P/Invoke bindings (for `SteamworksUGC`)
- `Newtonsoft.Json` — Settings/Steam metadata serialization
- `Pfim` — Image decoding (DDS, KTX, etc.)
- `System.Drawing` — GDI+ `Bitmap`, `Graphics`

### Consumers
| Consumer | Uses |
|----------|------|
| `Render/MapRenderer.cs` | `TextureManager`, `TilesetManager`, `TeamColorManager` |
| `Tools/*Tool.cs` | `UndoRedoList`, `PropertyTracker`, `TilesetManager` |
| `Model/Map.cs` | `INI`, `Megafile`, `WWCompression` for save/load |
| `RedAlert/TiberianDawn/GamePlugin.cs` | `INI`, `Megafile`, `TilesetManager` |
| `Controls/*Properties.cs` | `PropertyTracker`, `TeamColorManager` |
| `SteamworksUGC.cs` | `Steamworks.NET` callbacks |

## Key Files
| File | Role |
|------|------|
| `INI.cs` | Westwood INI parser (section/key/value, comments, arrays) |
| `Megafile.cs` | `.mix` archive reader (random access, streaming) |
| `MegafileBuilder.cs` | `.mix` writer (for map pak creation) |
| `WWCompression.cs` | LZSS decompressor (Westwood variant) |
| `TGASharpLib.cs` | TGA load/save (RLE, 24/32-bit) |
| `TilesetManager.cs` | Template/tileset loader from game archives |
| `TextureManager.cs` | Bitmap cache with LRU |
| `TeamColorManager.cs` | Palette remap for player colors |
| `UndoRedoList.cs` | Command stack |
| `PropertyTracker.cs` | Auto-undo for property changes |
| `SteamworksUGC.cs` | Workshop publish/update/query |
| `GameTextManager.cs` | `.csf` string table loader |
| `CRC.cs` | Checksums |
| `MRU.cs` | Recent files |