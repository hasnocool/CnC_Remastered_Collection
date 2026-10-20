# RTS 16×16 Tileset Pack Rendering

The map editor and the game front-ends (Red Alert, Tiberian Dawn) draw terrain
from the in-repo tileset pack by default instead of flat colored quads.

## Pack location

`assets/packs/rts_16x16_tileset/` (read-only generated art):

- Palettes: `td_inspired/` and `ra_inspired/`, each with `atlas_manifest.json`
  (per-category row → family metadata) and eight 256×256 category PNGs
  (`terrain`, `transitions`, `water_edges`, `cliffs`, `roads_bridges`,
  `walls_fences`, `decorations`, `resources_fx`).
- `catalog.json` at the pack root describes both palettes.
- Tiles are addressed `tile_id = row * 16 + column`, row-major, 16 px, no
  gutters.

## Default palettes

| Application      | Default palette |
|------------------|-----------------|
| Map editor       | `ra_inspired`   |
| Launcher         | `ra_inspired` (RA screens) / `td_inspired` (TD screens) |
| Red Alert        | `ra_inspired`   |
| Tiberian Dawn    | `td_inspired`   |

## Editor flags

- `--tileset <dir>` — pack root (default
  `../assets/packs/rts_16x16_tileset` relative to the crate manifest, falling
  back to `assets/packs/rts_16x16_tileset` in the current directory).
- `--tileset-palette <name>` — palette directory name (default `ra_inspired`).
- `--no-tileset` — force the original flat colored quads.

## Environment overrides (games)

- `CNC_TILESET_DIR` — pack root directory.
- `CNC_TILESET_PALETTE` — palette directory name (editor defaults: `ra_inspired`
  for Red Alert, `td_inspired` for Tiberian Dawn).

Both are read once per process; the pack is loaded lazily on first render and
shared by all subsequent frames.

## Asset coverage

Every consumer draws from the same manifest rows (both palettes are
structurally identical; tile ids are `row * 16 + column` per category):

| Category | Where it renders |
|---|---|
| terrain (all 16 families) | base tiles everywhere; `tiberium_ground`/`ore_ground` under resource cells; new editor terrain types (mud, crater, scorched, swamp, ice, concrete, forest/jungle floor) |
| water_edges | shore side/corner/river families from land-adjacency (RA river, editor water, TD reserved water codes `terrain_type >= 0xa0`), reef_rocks on rock-near-water, ice sheets/edges for Snow |
| transitions | terrain-pair edges in the editor (grass/dirt/sand/rock/snow/rough adjacency, decoded from the generator's mask columns), RA Beach → `sand_shore`, grass↔rock/sand in RA |
| cliffs | editor: persisted `[Elevation]` section drives cliff faces, corners, ramps and plateau tops |
| roads_bridges | editor: new `Road` overlay with neighbour auto-connect (paved/dirt tracks, curves, T-junctions, bridges) |
| walls_fences | editor: new `Wall` overlay with neighbour auto-connect (concrete runs/corners/T, sandbags, barbed wire, gate) |
| decorations | editor Trees overlay → tree/bush/dead-tree variants; player starts → flag sprite |
| resources_fx | tiberium stage art (sprout/cluster/bloom, blue variant), ore/ gems by density, blast explosion/fire/smoke in TD and RA (RA blasts land later with sim state), ore/ tiberium markers in launcher previews |

Diligent leftovers (no data drives them yet): RA only has a 4-value terrain
enum so several transition pairs do not apply in RA, RA blast art waits for a
blast state, and radiation/oil-well families have no producer anywhere.

## Fallback behaviour

When the pack directory is missing, a palette is unknown or the atlas upload
fails, the affected application logs a warning once and keeps the original
flat colored quads — rendering never fails because of the pack. The procedural
`SpriteAtlas` for unit/building icons (`assets/atlas.png` / `atlas.json`) coexists
with the pack: `cnc-renderer` keeps a small multi-slot texture array
(slot 0 = unit atlas via `set_atlas`, slot 1 = tileset via
`set_atlas_texture(cnc_renderer::TILESET_TEXTURE_ID, …)`), sampled per sprite
through `Sprite::texture_id`. Sprites outside these slots, or slots whose
atlas was never uploaded, keep the flat colored-quad behaviour. The launcher
game shells load both palettes and upload the one their screens need at
startup, reusing it for terrain previews and minimaps.

Note for Tiberian Dawn / Red Alert: terrain renders from this pack in all
paths, including while the procedural unit/building atlas is bound — the
former single-slot fallback (flat terrain under an active unit atlas) is
gone.
