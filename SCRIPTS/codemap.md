# SCRIPTS/ — Build & Asset Utilities (Python)

## Responsibility
Python utility scripts for asset pipeline operations. Currently contains **`tgautil.py`** — a TGA (Truevision TGA) texture archive tool for cropping, packing, and unpacking texture atlases used by the game engines and map editor.

## Design Patterns

### CLI Command Pattern
`argparse` with subcommands:
- `tgautil.py z <directory>` — **Pack**: Crop transparent borders from TGA files, store crop metadata, write ZIP archive
- `tgautil.py u <archive>` — **Unpack**: Extract ZIP, restore original dimensions using crop metadata

### Image Processing Pipeline
```
Pack (z):
  For each TGA in directory:
    1. Load via PIL.Image → convert('RGBA')
    2. Scan alpha channel for non-transparent bounds (top, bottom, left, right)
    3. Crop to content bounds
    4. Save cropped TGA to in-memory buffer
    5. Serialize crop metadata (original size, crop rect) as JSON → .meta file
    6. Write both to ZIP (DEFLATED)

Unpack (u):
  For each entry in ZIP:
    1. Pair .tga + .meta by basename
    2. Load cropped TGA
    3. Resize to crop rect dimensions
    4. Paste onto transparent canvas of original size at crop offset
    5. Save restored TGA to output directory
```

### Metadata Format (JSON)
```json
{
  "size": [original_width, original_height],
  "crop": [left, top, right, bottom]
}
```
Stored as `.meta` files alongside TGAs in the ZIP. Enables lossless round-trip.

## Data & Control Flow

### Entry Points
```python
if __name__ == '__main__':
    parser = argparse.ArgumentParser(...)
    subparsers = parser.add_subparsers()
    # ... define 'z' and 'u' subcommands
    args = parser.parse_args()
    args.func(args)  # Dispatches to zip() or unzip()
```

### Dependencies
| Package | Purpose |
|---------|---------|
| `PIL` (Pillow) | Image loading, conversion, cropping, saving (TGA support) |
| `zipfile` | ZIP archive creation/extraction (DEFLATE compression) |
| `json` | Crop metadata serialization |
| `argparse` | CLI parsing (stdlib) |
| `io`, `os`, `sys` | Stdlib utilities |

### Integration with Codebase
- **Consumed by**: `CnCTDRAMapEditor/Utility/TGASharpLib.cs` — C# port of similar TGA handling logic
- **Asset Pipeline**: Used to prepare texture atlases for `.mix` archives consumed by `REDALERT/` and `TIBERIANDAWN/` engines
- **Map Editor**: Preview images, terrain tiles, unit sprites all use TGA format

## Key Files

| File | Role |
|------|------|
| `tgautil.py` | Sole script — TGA crop/pack/unpack utility for texture atlas management |

## Usage Examples
```bash
# Pack a directory of TGA textures into a cropped ZIP archive
python tgautil.py z ./textures -y

# Unpack a ZIP archive, restoring original dimensions
python tgautil.py u textures.ZIP -y
```

## Notes
- **Python 2 syntax** detected (`print >> sys.stderr`, `raw_input()`, `StringIO.StringIO`) — may require `2to3` or manual port for Python 3
- No test suite present
- Single-file script with no modular decomposition