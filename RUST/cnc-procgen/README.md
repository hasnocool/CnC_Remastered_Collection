# cnc-procgen

This crate follows strict semantic versioning (version.workspace = true, edition = 2021, SemVer 2.0.0).

## Scope and limits

cnc-procgen is the built-in, free asset path: it deterministically generates
terrain heightmaps with biome band textures, noise/Tiberium textures, and
parametric placeholder unit sprites — no original game files or downloads
needed. The terrain and texture generators are the working core; the sprite
lane is a foundation-grade set of 32px placeholders rather than final art,
with deeper per-class sprite profiles left as future work.

The `tileset` module is a logic-faithful Rust port of the CNC32 procedural
tileset generator from the v6.2 asset pack. It covers the v5.5.0 base
(`cnc32_asset_generator_v5.py`: 940 terrain blob transitions, roads,
shorelines, cliffs, walls, bridges, decorations, buildings, the full unit
animation/condition/turret matrices, effects — mask conventions, seed
layout, manifest schema and validation gates) plus the five additive
expansion layers as separate modules under `expansion/`, run by the pack
builder in the Python scripts' order:

- `cutscene_v58` — six cinematic story sequences (233 frames, storyboards,
  UI overlays) with `sprites/cinematics/timeline.json`
- `campaign_v59` / `campaign_v60` — campaign art (75 sprites), effect
  animations (48 frames) and five campaign acts
- `briefing_v61` — tactical briefing/debrief scenes, cards, personnel
  (123 sprites), stamping manifest schema 1.3.0
- `ui_v62` — commander portraits, NPC roster, UI icons, controls, mockups
  and wireframes (151 sprites), stamping generator version 6.2.1

A full build writes the complete 4577-sprite / 20-atlas pack (schema
1.3.0) with `sprites/`, `atlases/`, `previews/`, `ui/`, the cinematics
timeline, per-layer build reports, `manifest.json` and
`validation_report.json`. Pixels are rendered with this crate's pinned
SplitMix64 PRNG, so output is deterministic within Rust but not
byte-identical to the Python pack.

On top of the v5 base the builder also ports the pack's five additive
expansion scripts and runs them in the Python order after the base
manifest is written, reproducing the complete `cnc32_assets_v6_2_1` pack
(4577 sprites, 20 atlases, manifest schema 1.3.0):

1. `cutscene_v58` — six cinematic story sequences (233 frames,
   storyboards, overlay UI) and the shared cinematics timeline
   (`sprites/cinematics/timeline.json`);
2. `campaign_v59` — twelve campaign-art scenes across three acts plus the
   first campaign art atlas;
3. `campaign_v60` — ten more scenes (acts 4/5), 48 effect animation
   frames and the v6 campaign/effect atlases;
4. `briefing_v61` — eight tactical briefing/debrief scenes, reusable
   props/icons, campaign cards and personnel portraits (stamps manifest
   schema 1.3.0 and the timeline `briefings` index);
5. `ui_v62` — commander/NPC portraits, 110 UI icons, control components,
   screen mockups, wireframes and contact sheets (stamps generator
   version 6.2.1).

Each layer keeps its own default seed (58017, 59017, 60017, 61021, 62021)
and writes its standalone build report; the manifest and cinematics
timeline are merged in place exactly like the Python scripts' in-place
updates. The base `validation_report.json` stays base-scoped (3909/14,
like the shipped pack) and carries an additional `expansion` summary
block describing the full pass.

## Usage

```sh
# Dry-run: planned asset counts
cargo run -p cnc-procgen --bin cnc-tileset-pack -- --dry-run

# Build the full pack (4577 sprites, 20 atlases: v5 base + 5 expansion layers)
cargo run -p cnc-procgen --release --bin cnc-tileset-pack -- \
    --out ./cnc32_assets --seed cnc32 --force

# From Rust
# cnc_procgen::build_tileset_pack(&TilesetPackOptions { .. })
```

## Documentation
- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## CI / Git
- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
