# SCRIPTS/ — Build & Asset Utilities (Python)

## Responsibility
Python utility scripts for asset pipeline operations, workspace management, and optional third-party asset fetching. Contains:
- **tgautil.py** — TGA (Truevision TGA) texture archive tool for cropping, packing, and unpacking texture atlases
- **download_openra_assets.py** — Optional OpenRA asset downloader (CC-BY-SA/GPL) for reference art
- **backup_workspace.py** / **restore_workspace.py** — Workspace backup/restore helpers

## Design Patterns

### CLI Command Pattern (tgautil.py)
`argparse` with subcommands:
- `tgautil.py z <directory>` — **Pack**: Crop transparent borders from TGA files, store crop metadata, write ZIP archive
- `tgautil.py u <archive>` — **Unpack**: Extract ZIP, restore original dimensions using crop metadata

### Image Processing Pipeline (tgautil.py)
```
Pack (z):
  For each TGA in directory:
    1. Load via PIL.Image to convert('RGBA')
    2. Scan alpha channel for non-transparent bounds (top, bottom, left, right)
    3. Crop to content bounds
    4. Save cropped TGA to in-memory buffer
    5. Serialize crop metadata (original size, crop rect) as JSON to .meta file
    6. Write both to ZIP (DEFLATED)

Unpack (u):
  For each entry in ZIP:
    1. Pair .tga + .meta by basename
    2. Load cropped TGA
    3. Resize to crop rect dimensions
    4. Paste onto transparent canvas of original size at crop offset
    5. Save restored TGA to output directory
```

### Metadata Format (JSON) (tgautil.py)
```json
{
  "size": [original_width, original_height],
  "crop": [left, top, right, bottom]
}
```
Stored as `.meta` files alongside TGAs in the ZIP. Enables lossless round-trip.

### Asset Fetching Pattern (download_openra_assets.py)
- **Manifest-driven**: `manifest.json` records sha256 + metadata for each fetched file
- **Idempotent**: Skips files with matching sha256 on re-runs
- **Atomic writes**: Stream to `.tmp` then rename; manifest updated atomically
- **Integrity gate**: sha256 computed while streaming; advisory byte count logged but not fatal (upstream drift on bleed branch)
- **HTTPS only**: Refuses non-HTTPS URLs
- **License tracking**: Records license per entry (GPL-3.0, CC-BY-SA-3.0/4.0)

### Workspace Backup/Restore (backup_workspace.py, restore_workspace.py)
- Simple wrapper scripts delegating to `tar` for archive creation/extraction
- `backup_workspace.py`: Recommends GraphFlow first, then `tar -czf backup/CnC.tar.gz --exclude=.git .`
- `restore_workspace.py`: `tar -xzf archive.tar.gz`

## Data & Control Flow

### tgautil.py Entry Points
```python
if __name__ == '__main__':
    parser = argparse.ArgumentParser(...)
    subparsers = parser.add_subparsers()
    # ... define 'z' and 'u' subcommands
    args = parser.parse_args()
    args.func(args)  # Dispatches to zip() or unzip()
```

### download_openra_assets.py Flow
1. Parse args (`--dest`, `--list`, `--dry-run`, `--force`)
2. Load `manifest.json` from destination
3. For each entry in ENTRIES:
   - If `--list`: print entry info and exit
   - If `--dry-run`: print plan without fetching
   - If sha256 matches manifest and not `--force`: skip
   - Else: stream download to `.tmp`, compute sha256, atomic rename, update manifest
4. Save manifest atomically

### Dependencies
| Package | Purpose |
|---------|---------|
| `PIL` (Pillow) | Image loading, conversion, cropping, saving (TGA support) |
| `zipfile` | ZIP archive creation/extraction (DEFLATE compression) |
| `json` | Crop metadata serialization, manifest |
| `argparse` | CLI parsing (stdlib) |
| `hashlib` | sha256 integrity verification |
| `urllib.request` | HTTPS downloads (stdlib) |
| `pathlib` | Path manipulation (stdlib) |
| `io`, `os`, `sys` | Stdlib utilities |

### Integration with Codebase
- **Consumed by**: `CnCTDRAMapEditor/Utility/TGASharpLib.cs` — C# port of similar TGA handling logic
- **Asset Pipeline**: Used to prepare texture atlases for `.mix` archives consumed by `REDALERT/` and `TIBERIANDAWN/` engines
- **Map Editor**: Preview images, terrain tiles, unit sprites all use TGA format
- **Optional Assets**: `download_openra_assets.py` fetches reference art to `ASSETS/OpenRA/` for procedural generation comparison
- **Workspace**: Backup/restore scripts for development environment snapshots

## Key Files
| File | Role |
|------|------|
| `tgautil.py` | TGA crop/pack/unpack utility for texture atlas management |
| `download_openra_assets.py` | Optional OpenRA asset downloader (CC-BY-SA/GPL) with manifest verification |
| `backup_workspace.py` | Workspace backup helper (tar + GraphFlow recommendation) |
| `restore_workspace.py` | Workspace restore helper (tar extraction) |

## Usage Examples
```bash
# Pack a directory of TGA textures into a cropped ZIP archive
python tgautil.py z ./textures -y

# Unpack a ZIP archive, restoring original dimensions
python tgautil.py u textures.ZIP -y

# List available OpenRA assets
python SCRIPTS/download_openra_assets.py --list

# Download OpenRA assets to default location (ASSETS/OpenRA)
python SCRIPTS/download_openra_assets.py

# Dry-run to see what would be downloaded
python SCRIPTS/download_openra_assets.py --dry-run

# Force re-download all assets
python SCRIPTS/download_openra_assets.py --force
```

## Notes
- **Python 2 syntax** detected in tgautil.py (`print >> sys.stderr`, `raw_input()`, `StringIO.StringIO`) — may require `2to3` or manual port for Python 3
- `download_openra_assets.py` is Python 3 only (type hints, `pathlib`, f-strings)
- No test suite present
- Single-file scripts with no modular decomposition (except tgautil.py internal functions)
