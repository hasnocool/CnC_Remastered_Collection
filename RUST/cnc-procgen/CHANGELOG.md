## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### Unreleased
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- `src/tileset/` — logic-faithful Rust port of the CNC32 procedural tileset
  generator (`cnc32_asset_generator_v5.py` v5.5.0 from the v6.2 asset pack):
  47-mask eight-neighbour terrain blob autotiles (20 directed pairs = 940
  transitions), terrain bases, resources, decorations, faction buildings,
  16-mask roads/shorelines/cliffs/walls, bridges, the full unit
  animation/condition/turret matrices (2176 + 448 + 144 sprites) and 23
  effect frames — same seed layout, IDs, metadata keys and validation
  gates as the Python pack; deterministic within Rust (pinned SplitMix64)
  but not byte-identical to the Python output
- `src/tileset/pack/` pack builder: writes `sprites/`, 16-column category
  atlases, 27 QA preview sheets (contact sheets, unit damage review grid,
  12×8 composed continuity map), `manifest.json` (schema 1.2.0),
  `validation_report.json` (contrast audit, road-continuity audit,
  duplicate-frame check, directed-pair coverage, strict integrity
  validation of every PNG and atlas cell) with staged-directory promotion
- `cnc-tileset-pack` CLI mirroring the Python `main()`: `--out`, `--seed`,
  `--force`, `--dry-run`, `--contrast-threshold`; exit codes 0/1/2
- `src/tileset/canvas.rs` software rasterizer (PIL-flavored point/line/
  rect/ellipse/polygon/arc, alpha composite, masked paste) and
  `src/tileset/font.rs` built-in 5×7 label font; WCAG contrast auditing in
  `src/tileset/pack/audit.rs`
- 102+ crate tests plus `tests/tileset_pack.rs` end-to-end full-pack build
  with determinism (identical manifest across rebuilds) and coverage gates

#### Fixed
- v5.9 sprite registration order now matches the shipped
  `cnc32_assets_v6_2_1` manifest (reusable art — props, portraits, cards,
  atlas — before the scene frames/storyboards; the archived generator
  script's call order contradicts the shipped pack, which is the parity
  target)
- v5.9/v6.0 timeline notes now match the shipped pack's 21 entries: the
  v5.9 layer emits the short note variants and the v6.0 layer adds both
  long variants in the shipped order
- the v6.0 overview preview (`previews/v6_campaign_overview.png`) is no
  longer listed as a timeline supporting asset (Python renders it after
  the timeline write; the shipped timeline lists 368 assets)

#### Added
- `src/expansion/` — the pack's five v6.x expansion layers as a
  logic-faithful Rust port of `cutscene_generator_v5_8.py` and
  `campaign_expansion_v5_9/6_0/6_1.py` + `asset_expansion_v6_2.py`:
  cinematic story sequences (269 sprites + the shared cinematics
  timeline), campaign art and effect animations (125 sprites, five
  campaign acts), tactical briefings/cards/personnel (123 sprites,
  manifest schema 1.3.0 + timeline `briefings` index) and the v6.2
  portraits/UI/controls/mockups (151 sprites, generator version 6.2.1);
  per-layer default seeds 58017/59017/60017/61021/62021 and standalone
  build reports
- `src/tileset/pack/expansion.rs` — expansion runner: executes the five
  layers over the staged base pack in the Python order, merges each
  layer's manifest/timeline contributions in place (Python merge
  semantics, incl. the `cinematics` counter recompute from the merged
  timeline), verifies every expansion sprite (file, SHA-256, size) and
  appends an `expansion` summary block to the validation report
- Full-pack output: `build()` now produces the complete
  `cnc32_assets_v6_2_1` pack — 4577 sprites, 20 atlases, manifest schema
  1.3.0, generator version 6.2.1 — with the base `validation_report.json`
  kept base-scoped (3909/14) exactly like the shipped pack; dry-run plan
  reports both base and pack totals
- `tests/tileset_pack.rs` extended to the full pack: category histogram,
  sprite/atlas order, timeline contents, expansion artefacts and
  byte-identical rebuild determinism against the shipped pack's numbers

#### Added
- `src/briefing.rs` static briefing art: `BriefingFaction` (Gdi/Nod/Allies/Soviet, built via `from_td_player`/`from_ra_player`), `generate_briefing_emblem` (faction shield + sigil + mission plaque) and `generate_briefing_portrait` (faction bust with headgear), both at `BRIEFING_ART_SIZE = 48`, palette-snapped through `Palette::find_closest` with `PlayerRemap` accents and no I/O
- 8 `briefing.rs` unit tests plus `tests/procgen.rs` briefing determinism/palette-boundedness coverage (42 crate tests); original VQA briefing movies stay out of reach (licensed decoders are stubs)
