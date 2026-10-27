# tiletools — Python map generator with original-asset tile matching

Debug harness + procedural map generator for C&C terrain tiles. Non-Rust code
lives at the repo root per project convention.

## Ground truth (from research, 2026-10-26)

### Container formats
- `Data/TEXTURES_TD_SRGB.MEG` / `TEXTURES_RA_SRGB.MEG` (Steam install at
  `~/.local/share/Steam/steamapps/common/CnCRemastered`): header magic
  `0xFFFFFFFF`/`0x3F7D70A4`, name table at 0x18 (u16-prefixed), then 20-byte
  records `{offset, ?, ?, 1-based index, ?}`; member bytes run from one
  record's `offset` to the next record's `offset`. Some shipped slots hold
  ZIP/BMP authoring debris instead of DDS — decode-fail and skip them.
- Terrain texture members are DXT5/DXT1 DDS with a 124-byte header plus a
  4-byte NVTT marker (pixel data at 128). Pillow reads data at
  `header_size + 4` — the marker stays, only the linear-size field needs
  patching to the level-0 size. `tiletools/dds.py` decodes BC1/BC2/BC3
  directly with numpy (bit-exact with Pillow, mean abs diff 0.0).
- Member naming: `<TPL>.<EXT>-<icon>-<frame>.DDS` inside
  `DATA\ART\TEXTURES\SRGB\<TIBERIAN_DAWN|RED_ALERT>\TERRAIN\<THEATER>\<FAM>\`.
  `icon` = DOS SHP frame index (= template cell), verified against DOS
  frame counts: W1.TEM ships 1 frame → MEG field A ∈ {0000}; W2.TEM 4 →
  {0000..0003}; CLEAR1.TEM 16 → {0000..0015}. The second field varies
  (00..07) at constant 128×128 — animation frames or re-authored variants.
- Classic MIX archives (`TEMPERAT.MIX` etc. under `Data/CNCDATA/<GAME>/CD1/`):
  header `{short count; long payload_size}`, count × 12-byte records
  `{crc, offset, size}`, Westwood rol1+add CRC of the uppercased filename
  (`MiscAsm.cpp` `Calculate_CRC`). `tiletools/mix.py` ports it.
- DOS `.TEM` art: `{u16 w; u16 h; u32 frames; u32 data_start}`; frame data =
  raw palette indices (24×24 each), palette = `TEMPERAT.PAL` in the same MIX.

### Tile semantics
- The engine draws authored `(TType, TIcon)` per cell (TD `CELL.CPP:913`).
  Verified in the compiled `TiberianDawn.dll` via REA/Rizin: template-name
  table (`CLEAR1`, `SH01..SH63`, `RV01..RV24`, …) in `.rdata` at
  `0x100c74ac`, registered by ctor `push` sequences at `0x100085d0`.
- Slope tables (`CDATA.CPP:47`) affect passability only, never the icon.
- The remaster re-authored terrain art (bright, 128×128/cell) while the DOS
  art is 24×24 dark-palette. TD tileset metadata is COMPILED into
  `CDATA.CPP` (216 templates, ported in `RUST/cnc-asset/src/original_templates.rs`);
  RA's tileset metadata is data-driven (`THEATER.INI` in its theater MIXes).
- Semantic ground truth (QA'd in `RUST/cnc-asset/src/tileset_pack.rs`):
  grass/dirt/sand/rock/rough/mud/concrete = `P01/P03/P04/P07/P08/P13/P14`;
  water = `W1`/`W2`; shore = `SH*` (TD `SH1` icons 0-7 by direction, RA
  `SH01..SH08` one family per direction); `CLEAR1` is a translucent overlay
  drawn UNDER a base.
- `RUST/cnc-map-editor/src/original_tileset.rs` hardcodes a TD
  `SHORE_BY_MASK[16]` table in which 4 of 16 masks carry a 0.250 join
  penalty — those are the matching defects this project targets.

## Matching method decision — measured edge-profile scoring (no WFC)

For every `(template, icon)` the theater ships, classify each edge's outermost
pixel run into `transparent / opaque / water` fractions. A cell's material
field (land / water / shore) dictates, per side, whether the edge must carry
material (same-material neighbour) or yield (different-material neighbour).
Pick the candidate with lowest disagreement; tie-break toward the most solid
tile; deterministic name+icon tie-break last. Diagonals weigh 0.5× for corner
continuity. Shore cells additionally restrict candidates to shore variants
consistent with their 4-bit land mask (`SHORE_BY_MASK` knowledge merged with
the measured table, eliminating the four 0.25-penalty masks where the art
offers a better pick).

Why not WFC: the constraint is one material field, candidate sets are small,
and material-based scoring is complete. WFC adds contradiction handling and
nondeterminism without visual benefit. Kept as a possible future extension.

## Final method and results (2026-10-26)

The working matcher is a **greedy scanline Wang-tile placement over the
shipped art's edge class-runs** (`tiletools/matcher.py: scanline_assign`):

1. Field: seeded value-noise fBm + island falloff → land/water; water cells
   with a land 4-neighbour become shore (beach arcs live on the water side).
2. Icons: DOS `.TEM` art (24×24, palette-indexed) decoded from the theater
   MIX; water/grass index sets learned from the pure templates (W1/W2 vs
   CLEAR1) — the DOS palette's dark dithers are index-disjoint between
   grass and water, so classification is exact.
3. Per-cell pick: the North and West neighbours are already placed
   (row-major scan), so their facing edge class-runs are known exactly and
   the candidate's N/W edge runs are compared pixel-for-pixel; E/S and
   diagonals score by fractional demands (`join.edge_demand`). Ties break
   seeded-random among near-equal, so shorelines vary.
4. Small non-border water regions (≤4 cells) render with the pond
   templates (SH32-35, water ringed by sand); everything else uses
   CLEAR1 + W1 + SH10-13.

Metric (`seam.py`): **0 holes / 1984 edges** (transparency mismatches — the
real seam class) for both naive and matched; class-change *transitions*
(289 matched vs ~scattered for naive) are coastlines and arc notches, which
the DOS art produces by design. The naive-vs-matched difference is
qualitative: naive scatters beach teeth through grass; matched hugs coasts.

### Known limits / next steps
- Shore arcs repeat per cell (scalloped "wallpaper") instead of forming
  template-scale curves — full fidelity needs template-level placement:
  chain 3x3 shore templates along coasts the way the original editor
  authors them (SH11 = land W, SH12 = land E, SH13 = land SW; masks are
  encoded in the `_slope<mask>` array names in CDATA.CPP).
- Roads (D*), slopes (S*), rivers (RV*), FALLS/FORD are decoded and
  measured (they're in the join table) but not yet placed by the
  generator; the same run-matching machinery applies.
- The remaster's re-authored MEG art is unreachable without the closed
  client's member-name logic (see `tiletools/art.py` STATUS note); if that
  mapping is ever recovered, `art.py` slots in behind the same interface.
- The DOS palette is dark; `--gamma` (default 2.2) brightens for testing.
  Gamma 1.0 = the exact DOS look.

## Architecture

```
tiletools/
├── DESIGN.md            this file
├── README.md            usage
└── tiletools/
    ├── meg.py           texture-MEG indexer (REMASTER art; experimental)
    ├── art.py           MEG template-icon enumerate/decode (experimental)
    ├── mix.py           classic Westwood MIX reader (CRC-indexed)
    ├── cdata.py         template table parsed from CDATA.CPP
    ├── format80.py      LCW decompression
    ├── dos.py           DOS .TEM icon sets + theater palettes
    ├── dds.py           DXT1/DXT3/DXT5 (BC1/2/3) decoding
    ├── join.py          edge profiles, materials, demand model, join table
    ├── terrain.py       seeded fBm material field + shore/pool labelling
    ├── matcher.py       naive + scanline Wang-tile assignment
    ├── render.py        PNG compositing + method comparison sheet
    ├── seam.py          hole/transition seam metric
    └── __main__.py      CLI
```

Outputs per run (`--out` dir, default `RUST/target/scratch-tiletools/`):
`map.png` (screenshot at native 128px/cell), `map_preview.png` (≤1600px, for
vision QA), `map_cells.json` (`(template, icon)` per cell), `seams.json`
(metric), and with `--compare`, one sheet pitting naive vs matched selection.
