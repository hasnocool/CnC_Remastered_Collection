# RUST/cnc-procgen/

## Responsibility

Deterministic procedural asset generation for the C&C Rust port (per its Cargo.toml description). Synthesizes **optional** replacement art for the original Westwood binaries, which are not redistributable: terrain heightmaps + biome textures, noise/Tiberium textures, and parametric 32×32 placeholder unit sprites. Everything is a pure function of seed + parameters — the same input yields byte-identical output — so assets bake offline and can be committed or cached without build-time flakiness (lib.rs docs). Touches no game simulation state.

## Modules

- `prng.rs` — pinned SplitMix64 `Rng`/`Seed` plus stateless lattice hashes `hash_coord`/`hash_f64`. Own PRNG on purpose (no `rand`): determinism is load-bearing, the algorithm is pinned and never version-drifts; a known-vector test locks the output.
- `math.rs` — tiny local helpers (`clamp01`, `mix`, `smoothstep`, `remap`), kept out of `cnc-core` so the simulation-facing core API does not grow surface just for asset generation.
- `terrain.rs` — value-noise fBm over a wrapping lattice (seamless tiling); `TerrainParams` (defaults + `with_seed`), `generate_heightmap`, `generate_terrain_texture` producing water/sand/grass/rock bands.
- `texture.rs` — `generate_noise_texture` (grayscale two-octave ramp) and `generate_tiberium_texture` (thresholded two-tone green patches with dirt gaps).
- `sprites.rs` — parametric `SpriteFamily` (Infantry/Vehicle/Aircraft/Building), `SpritePose`, `family_of(class_id)`, `generate_unit_sprite`, `generate_sprite_for_class` at `SPRITE_SIZE = 32`.
- `briefing.rs` — static briefing art: `BriefingFaction` (Gdi/Nod/Allies/Soviet, built via `from_td_player` / `from_ra_player`), `generate_briefing_emblem` (faction shield + sigil + two-digit mission plaque) and `generate_briefing_portrait` (faction bust with seeded shading + player-accent collar), both at `BRIEFING_ART_SIZE = 48`. Static only: the original VQA briefing movies stay out of reach (licensed decoder stubs), so text panels plus this art are the whole briefing story.
- `expansion/` — logic-faithful ports of the pack's five additive expansion scripts, one module per Python script, each with its own unit tests mirroring the script's assertions: `cutscene_v58.rs` (6 cinematic story sequences: 233 frames + storyboards + UI overlays, creates `sprites/cinematics/timeline.json`), `campaign_v59.rs` (12 campaign-art scenes, 3 acts, first campaign art atlas), `campaign_v60.rs` (10 more scenes, acts 4/5, 48 effect-animation frames, v6 atlases), `briefing_v61.rs` (8 tactical briefing/debrief scenes, cards, personnel portraits; stamps manifest schema 1.3.0 and the timeline `briefings` index), `ui_v62.rs` (commander/NPC portraits, 110 UI icons, control components, screen mockups/wireframes, contact sheets; stamps generator version 6.2.1). Shared contract in `mod.rs`: `PackWriter` (atomic staged writes), `LayerOutput`/`TimelineContribution`/`SpriteSpec` (layer → builder contributions). Layers run with their own default seeds (58017/59017/60017/61021/62021).
- `tileset/` — logic-faithful Rust port of the CNC32 procedural tileset generator (`cnc32_asset_generator_v5.py` v5.5.0, the engine behind the v6.2 asset pack). Submodules:
  - `tileset/mod.rs` — pack palette (direct RGBA, deliberately not 256-color-snapped; documented deviation), `stable_seed` (SHA-256[:8] BE, matches Python), `TileRngExt` (Python-`random`-shaped helpers on the crate PRNG), `TILE = 32`.
  - `tileset/canvas.rs` — PIL-flavored software rasterizer over `image::RgbaImage`: line/rect/ellipse/polygon/arc, alpha composite, masked paste; clipping is silent like Pillow.
  - `tileset/font.rs` — built-in 5×7 bitmap font for preview labels only.
  - `tileset/terrain.rs` — `terrain_base` (5 materials), `blob_masks()` (canonical 47-mask eight-neighbour set), `terrain_transition` (940 directed-pair blob autotiles with jagged seeded edges).
  - `tileset/connections.rs` — `connected_road` (16 cardinal masks), `connected_shore`, `cliff`, `wall`, `bridge`, plus `audit_road_continuity` (the Python pack's own QA gate).
  - `tileset/units.rs` — `unit_frame` (direction-aware, 7 units × states × 8 facings), `unit_condition_frame` (intact/damaged/critical/destroyed full frames), `independent_turret_frame` (8 aims × 3 recoil), `effect_frame` (explosion/smoke/muzzle/destruction).
  - `tileset/pack/` — pack builder: `mod.rs` (`build()`/`BuildOptions`/`PackError` + `dry_run_plan`; staged-directory generation with backup-and-restore promotion), `generators.rs` (resources/decorations/buildings), `manifest.rs` (manifest + validation-report serde shapes incl. the `ExpansionSummary` block), `previews.rs` (contact sheets, unit-damage review grid, 12×8 composed continuity map), `audit.rs` (WCAG contrast audit), `expansion.rs` (runs the five v6.x expansion layers in Python order over the staged base pack: re-reads `manifest.json`/`timeline.json` after each layer, merges the layer's contributions in place — sprites upsert-by-id, atlases by file, timeline sequences/campaigns by id, notes/assets deduped, `cinematics` key-merged with recomputed sequence/frame/campaign-scene counters — then verifies every expansion sprite's file/hash/size).
  - `bin/cnc-tileset-pack.rs` — CLI mirroring the Python `main()` (`--out --seed --force --dry-run --contrast-threshold`, exit codes 0/1/2).
  - Integration: `tests/tileset_pack.rs` builds the complete 4577-sprite / 20-atlas pack (schema 1.3.0) end-to-end and gates on report pass, base coverage counts, the expansion summary, the shipped pack's category histogram, manifest record/atlas order, timeline contents, expansion artefacts and byte-identical rebuild determinism.

## Palette faithfulness

All colors are snapped through `cnc_core::palette::Palette::find_closest`, staying inside the game's 256-color model; player accents flow through `cnc_core::color::PlayerRemap` exactly like real sprites. Pixel writers emit opaque `image::RgbaImage`. **Exception:** the `tileset` module emits the pack's own straight-RGBA palette directly (the Python pack is plain RGBA output); engine consumers may snap on load.

## Flow

Terrain: `generate_terrain_texture(params, palette)` → `generate_heightmap` (fBm) → per-pixel band/slope classify → `find_closest` snap → RGBA image. Sprites: `generate_sprite_for_class(class_id, pose, palette, remap)` → `family_of` → per-family draw using remapped accent entries. Briefing: `generate_briefing_emblem(faction, mission_index, seed, palette, remap)` → shield profile + rivet dither (`hash_f64`) → sigil polygon → digit plaque; `generate_briefing_portrait(faction, seed, palette, remap)` → frame/torso/head ellipses → seeded shading → faction headgear → accent collar. No I/O: both return `image::RgbaImage` values only.

## Integration

- Deps: `cnc-core` (`palette`, `color`, `class_id`) + `image 0.25`.
- Consumed by `cnc-sfx` (`prng::Rng`/`Seed` for noise shaping) and, transitively, `cnc-music`. No other in-workspace dependents.
- Tests: per-module unit tests + integration `tests/procgen.rs` (determinism, sizing, palette-bounded output, pose sensitivity, briefing-art determinism + palette boundedness).
