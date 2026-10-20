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

## Palette faithfulness

All colors snap through `cnc_core::palette::Palette::find_closest`, staying inside the game's 256-color model; player accents flow through `cnc_core::color::PlayerRemap` exactly like real sprites. Pixel writers emit opaque `image::RgbaImage`.

## Flow

Terrain: `generate_terrain_texture(params, palette)` → `generate_heightmap` (fBm) → per-pixel band/slope classify → `find_closest` snap → RGBA image. Sprites: `generate_sprite_for_class(class_id, pose, palette, remap)` → `family_of` → per-family draw using remapped accent entries. Briefing: `generate_briefing_emblem(faction, mission_index, seed, palette, remap)` → shield profile + rivet dither (`hash_f64`) → sigil polygon → digit plaque; `generate_briefing_portrait(faction, seed, palette, remap)` → frame/torso/head ellipses → seeded shading → faction headgear → accent collar. No I/O: both return `image::RgbaImage` values only.

## Integration

- Deps: `cnc-core` (`palette`, `color`, `class_id`) + `image 0.25`.
- Consumed by `cnc-sfx` (`prng::Rng`/`Seed` for noise shaping) and, transitively, `cnc-music`. No other in-workspace dependents.
- Tests: per-module unit tests + integration `tests/procgen.rs` (determinism, sizing, palette-bounded output, pose sensitivity, briefing-art determinism + palette boundedness).
