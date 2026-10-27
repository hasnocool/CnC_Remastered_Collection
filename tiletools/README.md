# tiletools

Python terrain-tile matching + procedural map generation for the C&C
Remastered Collection, working directly on the original shipped assets
(texture MEGs from the Steam install, classic MIX archives, DOS `.TEM` art).

See `DESIGN.md` for the research findings and the matching-method rationale.

## Usage

```bash
cd tiletools
# 48x48 temperate TD map, seed 42, screenshot + seam metric:
python3 -m tiletools --game td --theater temperate --seed 42 --size 48

# comparison sheet (naive vs matched) + smaller map:
python3 -m tiletools --size 32 --seed 7 --compare

# Red Alert, snow theater:
python3 -m tiletools --game ra --theater snow --seed 3
```

Outputs go to `--out` (default `RUST/target/scratch-tiletools/`):

| File             | What it is                                            |
|------------------|-------------------------------------------------------|
| `map.png`        | Screenshot at native scale (96px/cell by default)     |
| `map_preview.png`| Same, downscaled to ≤1600px for vision QA             |
| `map_cells.json` | `(template, icon)` per cell (row-major)               |
| `seams.json`     | Seam metric (holes = real defects, transitions = coastlines) |
| `tiles.json`     | Per-tile classification (edge kinds, arc flips, clutter) |
| `tile_atlas.png` | Labeled contact sheet of every vocabulary tile        |
| `zoom_X0_Y0_X1_Y1.png` | With `--zoom-crop`: high-zoom seam inspection   |
| `compare.png`    | With `--compare`: naive vs matched sheet + seam stats |

## Requirements

Python 3.10+, numpy, Pillow. The Steam C&C Remastered install must be
present (auto-detected; override with `--install-root`).
