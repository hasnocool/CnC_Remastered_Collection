# Original Remaster asset integration — evidence (2026-10-26)

## What was proven

Both the map editor and the launcher render terrain from the **shipped Steam
assets**, read natively from `Data/TEXTURES_TD_SRGB.MEG` (no pre-baked pack, no
asset copy in the repo).

| surface | command | evidence |
|---|---|---|
| map editor | `cnc-map-editor --headless --seed 42 --theater Temperate --original-tiles` | `.omo/evidence/editor-original-tiles.png` |
| launcher | `game_screenshots --original-tiles` | `.omo/evidence/launcher-original-tiles.png` |

Generated-pack baselines for comparison: `editor-generated-tiles.png`,
`launcher-generated-tiles.png`.

Pixel difference vs the generated pack: editor 50.5% of cells, launcher 100%.
The editor's lower number reflects identical fallback geometry (menu/labels)
rather than art.

## Container format (reverse-engineered)

`TEXTURES_TD_SRGB.MEG` — the Westwood "MegaFileClass" variant used by
`ClientG.exe`, **not** the map editor's Megafile (`Cnc32Pack` / the C#
`Megafile.cs`), which mis-parses it:

```text
0x00  u32 0xFFFFFFFF   marker
0x04  u32 0x3F7D70A4   magic
0x08  u32 first payload offset (data start)
0x0C  u32 member count (= name count)
0x10  u32 member count (repeat)
0x14  u32 name-table byte size (names span 0x18 .. 0x18 + this)
0x18  N x { u16 len, bytes }   NUL-free paths, packed back to back
names_end + 14
      N x { u32 offset, u32 meta, u32 meta, u32 index(1-based), u32 meta }
```

Facts established by measurement:

- Records carry a **1-based index field**; `index == i + 1` validates a record.
- Names and records pair **positionally** (`names[i]` ↔ `record[i]`).
- Member bytes run from `record[i].offset` to `record[i+1].offset`; the final
  member runs to EOF.
- Members are **raw** DXT5 DDS (no compression on the members themselves).
- Two icon naming forms exist: `<TEMPLATE>.<EXT>-<mip>-<icon>.DDS` (most
  families) and `<TEMPLATE>.<EXT>-<mip>.DDS` (single-icon families). Icon index
  width varies: two digits (`SH1.TEM-0004-00.DDS`) or four
  (`FALLS1.TEM-0000-0000.DDS`).
- RA zero-pads shoreline families (`SH01` vs TD's `SH1`).

Member names look like
`DATA\ART\TEXTURES\SRGB\TIBERIAN_DAWN\TERRAIN\TEMPERATE\SH1\SH1.TEM-0000-00.DDS`
— `TEMPLATE.THEATER-MIP-ICON.DDS`.

## DDS decode gotchas (all handled in `original_assets::normalize_dds`)

1. Pixel data begins at `header_size + 4` (124 + 4 = 128): NVTT writes a 124
   byte header plus a 4-byte marker. Decoders that assume `header_size + 4`
   read the right bytes only when the header says 124; the shipped files do.
2. Members declare 8 mip levels but the chain is **padded into the next payload
   slot**, so a member can be shorter than its declared chain. The largest
   level whose bytes are fully present wins.
3. The `image` crate mis-reads multi-mip DXT5 data offsets (verified: the same
   level-0 bytes decoded wrongly at mips=8 and correctly at mips=1), so multi-mip
   members are normalized to single-level before decoding.

## Template table

216 entries extracted from `original/TIBERIANDAWN/CDATA.CPP`
(`TemplateTypeClass`), including the `AltIcons` slope lists that encode which
icons of a template use the alternate land type. Ported to
`cnc-asset/src/original_templates.rs` with tests:
- shore templates (`SH1`: water + beach-alt icons 0..5),
- rock slopes (`S01`: rock + clear-alt icon 2),
- water (`W1`/`W2`: pure water, no alt icons).

## Verified tile → family mapping

Sampling every TEMPERATE family's icon 0 for alpha coverage showed which cells
are *solid ground* vs decoration overlays. `CLEAR1` is 28% coverage (a
translucent clear-overlay the engine draws over a solid base); `P01`/`P03`/
`P04`/`P07`/`P08`/`P13`/`P14` are 100% opaque patches — the actual base ground.
The pack therefore maps:

| family | template | coverage |
|---|---|---|
| grass | P01 | 100% |
| dirt | P03 | 100% |
| sand | P04 | 100% |
| rock | P07 | 100% |
| rough | P08 | 100% |
| mud | P13 | 100% |
| concrete | P14 | 100% |
| deep_water | W1 | 100% |
| shallow_water | W2 | 100% |
| shore_north..se | SH1 icons 0..7 | 100% |
| grass_rock | S01 | 100% |
| grass_sand | S02 | 100% |
| dirt_rock | S03 | 100% |
| sand_rock | S04 | 100% |
| paved_ns / paved_ew | D01 / D02 | 100% |
| cliff_interior | RV01 | 100% |

## Per-game differences

Both archives share the container and member format but differ in:

| aspect | Tiberian Dawn | Red Alert |
|---|---|---|
| archive | `TEXTURES_TD_SRGB.MEG` | `TEXTURES_RA_SRGB.MEG` |
| theaters | TEMPERATE, WINTER, DESERT | TEMPERATE, SNOW, INTERIOR |
| suffix codes | TEM / WIN / DES | TEM / SNO / INT |
| family directory | `SH1\` | `SH01.TEM\` (code appended) |
| shore template | `SH1` | `SH01` (zero-padded) |
| mip levels | 0..7 for single-icon families | 0..19 for single-icon families |

So a member lookup probes, in order: template and zero-padded template × plain
and `<TPL>.<EXT>` family directory × two- and four-digit icon index × levels 0
up to 20. `MAX_MIP_LEVELS = 20` covers both games.

## Known gaps

- The original compose fills every column of a family row with the one icon,
  because original families have no per-variant art. Per-cell shoreline/cliff
  masks still come from the generated pack's `cell_draw` path in the editor
  (`OriginalTileset` serves base terrain only).
- Only terrain is composited: models, UI, audio and movies are out of scope.
- RA's INTERIOR theater resolves only `CLEAR1` (its base ground), because the
  remaining families ship under SNOW/TEMPERATE. 
