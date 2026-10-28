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

## Final method and results (2026-10-27)

The working matcher is a **three-layer placement over the shipped art**:

1. **Field**: seeded value-noise fBm + island falloff → land/water; 2
   passes of 3×3 majority smoothing (jagged fBm coasts fragment into
   1-2-cell runs where no patch fits); water cells with a land
   4-neighbour become shore; small non-border wet regions (≤4 cells) are
   ponds.
2. **Patches**: whole coast templates chain on a global 3-cell lattice —
   `SH34`/`SH35` (N-S coasts, land W/E), `SH12`/`SH13` alternating (E-W
   coasts, land S), origin may hang off the map edge (clipped). Patches
   validate per cell: full-land art needs field LAND, full-water art
   needs water/shore, arc cells accept either; outer edges must agree
   with the field across them. Plain-water interiors tile with 2×2
   variety patches (`W2`/`SH17`/`SH18`).
3. **Per-cell scanline** (Wang-tile) fills the rest: placed N/W
   neighbours contribute their exact edge runs and smoothed sand-curve
   profiles (5px window over the dithered arc — pixel-exact run matching
   can never join two different dithered arcs); unplaced sides score
   presence-based demands (`join.edge_demand`: a land-facing edge needs
   sand *present*, not full-coverage).

**Classification** (`classify.py`, `tiles.json`, `tile_atlas.png`): every
vocabulary tile gets per-edge kinds (land/water/mixed + arc flip
positions), coverage fractions, and interior clutter (rock/tree detail).
The per-mask fitness table built from it drives **field repair**: coast
shapes no shipped tile fits — near-enclosed inlets (3+ land sides) — are
closed to land so no detached sand squares render. With presence-based
demands every one of the 16 land masks has a fitting tile (max penalty
0.52).

Metric (`seam.py`): **0 holes / 1984 edges**. Vision QA (3 passes, VL
model): pass 1 found floating/blocky coast pieces and missing water
variety → fixed by patch chains + smoothing + water variety patches; pass
2 found border-strip gaps (the strip finder skipped map-edge columns) and
pond bypass gaps → fixed; pass 3: **pass** (atlas classification matches
art; coast continuous; seams clean; water variety confirmed).

### Known limits / next steps
- The DOS temperate set ships exactly one N-S coast template per land
  side (SH34/SH35), so long straight N-S coasts repeat their 3×3 art;
  the E-W coasts alternate SH12/SH13. Wider variety needs the diagonal
  pieces (SH10/SH11) chained at coast turns — next step.
- Roads (D*), slopes (S*), rivers (RV*), FALLS/FORD are decoded and
  measured but not yet placed by the generator; the same machinery
  applies.
- The remaster's re-authored MEG art is unreachable without the closed
  client's member-name logic (see `tiletools/art.py` STATUS note).
- The DOS palette is dark; `--gamma` (default 2.2) brightens for testing.
  Gamma 1.0 = the exact DOS look.

## Shipped-map reverse engineering (2026-10-27, second session)

`tiletools/scenario.py` + `tiletools/learn_maps.py` decode the shipped
TD scenario files and learn the placement conventions empirically:

- Scenario `.BIN` layout (verified against `MAP.CPP:663-700`): sequential
  `{u8 TType; u8 TIcon}` pairs over the full 64×64 cell space; TType 255 =
  none; the INI's `[MAP]` X/Y/Width/Height = the used window. Loose
  `SC*.INI/.BIN` pairs ship under `CNCDATA/TIBERIAN_DAWN/{COMMUNITY,
  CONSOLE_1,CONSOLE_2}`; the main campaign maps are packed inside
  `CD1/CONQUER.MIX` (266 payloads, name-CRC'd with a hash the DOS CRC
  does not match — the remaster repacked them; payload 0 = a real map grid,
  the rest = the same for all campaigns).
- The 17 loose scenarios (17,399 placed cells) yield the empirical tables:
  the shipped maps draw **coast water as plain `W1`** for every shore mask
  (the learned water-side table = `W1:0` across all masks) and **put the
  coast art on the LAND side** — the land-side shelf table shows `CLEAR1`
  at every water mask. I.e. the DOS maps' coast = water cell + grass cell
  directly, with the sand shelf as a *wide* band driven by the island
  shapes, not per-edge arcs.
- `learned_shore_masks.json` carries the full tables for feeding the
  generator's placement rules.
- Campaign CONQUER.MIX crack (2026-10-27): the repacked name hash is the
  **zero-padded Westwood CRC** — the same rol1+add CRC over the uppercased
  name padded to a 4-byte multiple. `MixArchive` now tries both variants
  (`tiletools/tiletools/mix.py`); this also recovered 54 temperate TEMPERAT
  tile sets the raw lookup missed (30 → 84 templates decoded, including the
  whole `S01..S38` slope family). The remaining 78 unresolved names are
  winter-only/unused per CDATA theater masks.
- Corpus stats via the Rust decoder (`RUST/cnc-tiberian-dawn/examples/
  {scan_maps,neighbour_stats}.rs`, 18 maps incl. repacked campaign maps,
  27k cells): shore cells (≥2 land sides) are also **plain `W1`/`W2`** —
  no arcs on water cells anywhere; arc/turn art lives only where land
  adjoins water (`SH17/SH18` all-water 2×2 pieces drawn ON land cells,
  keyed by which side the water is on; the tables live in
  `RUST/target/scratch-tiletools/shipped_rules.json`). The generator
  adopts this: `_place_land_shelf` extends the coast band onto deep-water
  bordering land cells; ponds keep the SH32-35 pockets.

## Tileset fit map (2026-10-28, atlas)

`python3 -m tiletools --atlas` renders every decoded template/icon with a
per-icon NESW fit legend (w = water edge, l = opaque, m = mixed arc, .
= transparent) to `RUST/target/scratch-tiletools/tileset_atlas.png`
(84 templates / 463 icons). Read it to see how the vocabulary joins:

- `W1`/`W2`: all-water; every edge water. Universal fill.
- `CLEAR1`: all-grass; every edge opaque. Universal fill.
- `SH1`, `SH12`, `SH13`: whole-3x3 diagonal beach chunks sweeping
  all-sand -> all-water; use as 3x3 islands, not per-edge.
- `SH34`/`SH35`: the only N-S coast bands (land W / land E resp.);
  chain along straight strips.
- `SH10`/`SH11`: diagonal mixed singles; use at coast turns/ends.
- `SH32`-`SH35` pool pieces: grass-side pockets for small ponds
  (sized by WATER cells only, <= 4).
- `SH17`/`SH18`: all-water 2x2 pieces drawn ON land cells where water
  lies exactly one side away with deep water beyond (shipped-map
  convention).
- `S01`-`S38` slopes, `RV01`-`RV10` rivers, `D01`-`D17` roads, `FORD1/2`,
  `FALLS`: decoded + measured; placement = future work.

## Contour continuity pass (2026-10-28)

Class scheme now 0=transparent, 1=water, 2=grass/rock, 3=sand (warm
palette entries). Contour defects seen and fixed:

1. Stale shore rings: closing wet specks/slivers to LAND left their
   SHORE ring behind -> meandering grey sand chains across solid grass.
   `relabel_stale_shore` reverts SHORE cells with no WATER in their
   8-neighbourhood to LAND.
2. Pools sized by water+shore cells: a 3-water pond weighed 11 and used
   the ocean band instead of SH32-35 pocket art. `label_pools` now
   counts WATER cells only.
3. `_place_land_shelf` painted SH17/SH18 bands that rendered as square
   water patches scattered over grass - removed; the scanline dresses
   coasts instead.
4. Seam comparison normalizes sand(3) -> opaque: sand-to-grass = the
   intended beach look; only water-vs-land materiality = defect.

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
    ├── scenario.py      shipped scenario .BIN reader ([MAP] window parsing)
    ├── learn_maps.py    empirical shore-mask learning from shipped maps
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
