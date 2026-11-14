# Unit Visuals Roadmap — Original TD/RA Assets and Animation

**Status:** Established roadmap; implementation not started by this document  
**Scope:** Original Tiberian Dawn and Red Alert unit visuals in the Rust workspace  
**Last updated:** 2026-11-14  
**Primary implementation crates:** `cnc-asset`, `cnc-renderer`, `cnc-tiberian-dawn`, `cnc-red-alert`, `cnc-launcher`, and later `cnc-map-editor`

## 1. Purpose

Deliver accurate, data-driven unit visuals for both games using the original Remastered Collection assets and the behavior documented in the original C++ engines. This includes asset discovery and decoding, unit definitions, directional body/turret rendering, animation timing, team colours, damage states, effects, and visual regression tests.

This is a dedicated visual implementation roadmap. It complements, but does not replace:

- [Feature parity and gap analysis](PARITY.md)
- [Original asset format evidence](original-asset-format.md)
- [Original functionality map](../original-source-reference/ORIGINAL_FUNCTIONALITY_MAP.md)
- [Implemented functionality map](../original-source-reference/IMPLEMENTED_FUNCTIONALITY_MAP.md)

The roadmap is not evidence that a milestone is implemented. Each milestone must be updated only after its acceptance criteria have been checked against the live code and, where applicable, executed tests or captured output.

## 2. Current baseline and known gaps

### 2.1 Existing generated-art integration

The evidence report [unit-assets-apply.md](../../.omo/evidence/unit-assets-apply.md), dated 2026-10-25, records that generated CNC32 unit atlases are integrated into TD and RA rendering paths and uploaded by the launcher/editor.

That report describes:

- A generated animation atlas with 2,176 frames and a condition atlas with 448 sprites.
- `cnc-asset/src/unit_atlas.rs` indexing animation frames by unit, state, facing, and frame, and condition sprites by unit, condition, and facing.
- TD choosing generated unit kinds and selecting idle versus move based on movement state.
- RA resolving generated unit kinds through its own class mapping.
- Launcher/editor upload of the generated unit atlas.

The same evidence report identifies two important limitations:

1. Render paths select the first animation frame rather than advancing frames according to frame duration.
2. Condition sprites are indexed but not yet used to render damaged, critical, or destroyed states.

These are generated-pack capabilities. They must not be described as proof that original Remastered unit art is loaded or that original unit visual behavior is reproduced.

### 2.2 Existing original-art foundation

The existing [original asset format report](original-asset-format.md) documents a reverse-engineered Remastered texture-MEG path used for terrain. It establishes that the shipped terrain texture archive is not the same archive format as the map editor's older `Megafile.cs` implementation.

The current terrain integration is useful infrastructure, but it does not establish that original unit assets have been discovered, decoded, or rendered. Unit assets may use different archive members, formats, metadata, or rendering conventions. Do not assume the terrain member naming scheme or parser applies to unit resources without evidence.

### 2.3 Known architectural considerations

- `cnc-asset/src/unit_atlas.rs` already exposes frame metadata including `duration_ms` and `loop_`; build on it where appropriate.
- `cnc-asset/src/animation.rs` already contains a tick-oriented `AnimationPlayer`. Prefer extending/reusing the existing animation system over creating another independent animation engine.
- Animation state must advance deterministically from simulation ticks or an explicitly defined presentation clock. It must not advance as a side effect of rendering, because draw frequency and render order must not change animation state.
- Keep presentation cursors and GPU handles separate from serialized gameplay state unless there is a demonstrated compatibility requirement.
- `cnc-asset/src/shape.rs` requires a compatibility audit before it is trusted for retail original SHP assets. Its current simplified assumptions are not proof of full retail-format support.
- `cnc-asset/src/vxl.rs` documents a synthetic VXL contract, not a Westwood VXL reader. TD and RA1 unit visuals do not require VXL/HVA. Do not prioritize VXL/HVA unless scope expands to a later game that needs them.
- The original C# map editor's texture/archive handling and the Remastered terrain texture-MEG reader are distinct format paths. Keep adapters separate until byte-level fixtures prove compatibility.
- Original-source behavior differs between TD and RA. Share decoding and rendering infrastructure where justified, but keep game-specific unit definitions and behavior explicit.

## 3. Goals and non-goals

### Goals

- Discover actual original TD and RA unit asset locations and metadata from supported installed game data.
- Resolve every referenced unit visual resource or report a specific, actionable missing-resource reason.
- Preserve image dimensions, alpha, crop/opaque bounds, offsets, palette/team-colour metadata, and frame timing.
- Render body and turret layers independently where the source assets and original behavior require it.
- Support facing, idle, movement, turning, firing, death, and damage visuals where original data provides them.
- Keep animation deterministic and independent of render frequency.
- Use bounded, thread-safe background work for file reading/decoding and perform GPU uploads on the renderer-owning thread.
- Retain generated CNC32 art as an optional fallback, with procedural/flat fallback last.
- Verify TD and RA independently, using reproducible fixtures, contact sheets, tests, and screenshots.
- Keep proprietary original game assets out of the repository.

### Non-goals for this roadmap

- Rewriting the renderer wholesale.
- Replacing the simulation or changing lockstep semantics merely to support animation.
- Implementing VXL/HVA for TD/RA1.
- Committing extracted proprietary game art or derived bulk asset dumps.
- Claiming pixel-perfect parity from a successful parser test alone.
- Combining TD and RA metadata into a single assumed-identical catalog.

## 4. Roadmap overview

| ID | Milestone | Priority | Depends on | Exit condition |
|---|---|---:|---|---|
| G01 | Original unit asset discovery and inventory | P0 | — | Reproducible TD/RA inventories and unresolved-reference reports |
| G02 | Archive readers and resource resolution | P0 | G01 | Required archive formats read with fixtures and deterministic lookup |
| G03 | Texture decoding and image metadata | P0 | G01–G02 | Supported source images decode with alpha/crop/origin metadata preserved |
| G04 | TD/RA unit catalog and visual definitions | P0 | G01–G03 | Unit IDs map to verified visual resources and game-specific behavior |
| G05 | Original-art atlas, cache, and renderer integration | P0 | G02–G04 | One original unit renders in each game with generated fallback intact |
| G06 | Deterministic facing and animation playback | P0 | G04–G05 | Frame timing, facing, and state transitions work without render-rate dependence |
| G07 | Turrets, weapons, and layered units | P1 | G05–G06 | Body/turret rotation and weapon visuals are independently verified |
| G08 | Team colours and palette transforms | P1 | G03–G05 | Original team-colour rules reproduce known reference outputs |
| G09 | Damage, destruction, and visual effects | P1 | G04–G08 | Damage states and destruction visuals map to real gameplay state |
| G10 | Specialized unit families | P1 | G06–G09 | Infantry, aircraft, naval, harvesters, transports, and special cases covered |
| G11 | Building and structure visuals | P2 | G02–G09 | Building frames, construction, damage, animation, and overlays covered |
| G12 | Projectile and combat presentation | P2 | G06–G09 | Projectiles, muzzle flashes, impacts, and weapon effects verified |
| G13 | Visual regression, performance, and release QA | P0 gate | G01–G12 | Repeatable per-game visual and performance evidence; no regression |

Priority definitions: P0 = foundational/blocking; P1 = core visual fidelity; P2 = broader completeness. G13 is a release gate even though it is listed last.

## 5. Detailed milestone plan

### G01 — Original unit asset discovery and inventory

**Objective:** Establish what the installed original games actually contain before writing assumptions into parsers or catalogs.

**Tasks**

- [ ] Identify supported TD and RA installation roots and the expected user-supplied path/configuration mechanism.
- [ ] Inventory archive files independently for TD and RA; record archive name, size, hash, detected format, and parser decision.
- [ ] Inspect archive member names and metadata without extracting bulk proprietary assets into the repository.
- [ ] Discover unit-related catalogs, XML, texture metadata, animation metadata, and resource references from actual archive contents.
- [ ] Do not hard-code guessed names such as `TD_UNITS.XML` or `RA_UNITS.XML`; identify the real filenames and member paths from the data.
- [ ] Classify discovered resources by unit, body/turret/overlay role, facing, animation/state, damage condition, and game.
- [ ] Record dimensions, frame count, timing/FPS, offsets, crop/opaque bounds, alpha presence, palette/team-colour hints, and source hashes where available.
- [ ] Generate deterministic JSON inventories and contact sheets to a user-selected output directory outside tracked source/assets.
- [ ] Report unresolved references, unsupported formats, duplicate candidates, and ambiguous mappings separately.

**Proposed CLI shape (subject to existing CLI conventions):**

```text
cargo run -p cnc-asset --bin cnc-asset-cli -- original-unit-audit --game td --data-dir <path> --output <path>
cargo run -p cnc-asset --bin cnc-asset-cli -- original-unit-audit --game ra --data-dir <path> --output <path>
cargo run -p cnc-asset --bin cnc-asset-cli -- original-unit-audit --game all --data-dir <path> --output <path>
```

The exact binary name and argument style must be checked against the current crate before implementation; the commands above describe the intended interface, not an existing command.

**Suggested output layout:**

```text
<output>/original-unit-audit/
  td/
    archive_inventory.json
    resource_inventory.json
    unit_catalog.json
    animation_inventory.json
    unresolved_references.json
    contact_sheets/
  ra/
    archive_inventory.json
    resource_inventory.json
    unit_catalog.json
    animation_inventory.json
    unresolved_references.json
    contact_sheets/
  audit_summary.json
```

**Acceptance criteria**

- TD and RA can be audited separately and together.
- Every inventory entry records provenance (archive/member path or loose-file path).
- Repeated runs over identical inputs produce stable ordering and stable machine-readable output.
- Every unresolved reference has a categorized reason; no silent dropping.
- Contact sheets can be regenerated from the same input data.
- No proprietary source assets are added to Git.
- Tests cover malformed archives, missing members, duplicate names, invalid metadata, and deterministic output.

### G02 — Archive readers and resource resolution

**Objective:** Read each required original archive format correctly and resolve resource references without conflating unrelated container formats.

**Tasks**

- [ ] Map every required unit resource to its actual containing archive or loose-file location.
- [ ] Audit the generic MIX reader, the Remastered texture-MEG reader, and the original editor's archive format independently.
- [ ] Implement a distinct adapter for each incompatible format rather than broadening one parser based on an unverified similarity.
- [ ] Add bounds checks, integer-overflow checks, path normalization, duplicate-member policy, and clear errors.
- [ ] Support nested/compressed members only where the source format requires them.
- [ ] Add small synthetic fixtures and legally distributable format fixtures; avoid checking in proprietary content.
- [ ] Define resource precedence when loose files override archive members.

**Acceptance criteria**

- Archive directory/member enumeration is deterministic.
- Every supported format has positive, negative, truncation, and bounds tests.
- A resource can be resolved by game + logical resource key with provenance retained.
- Corrupt or unsupported resources fail explicitly and do not crash the game.

### G03 — Texture decoding and image metadata

**Objective:** Decode the discovered source image formats without losing visual metadata required by the renderer.

**Tasks**

- [ ] Determine actual unit-image formats from G01; implement only formats supported by evidence.
- [ ] Audit and extend SHP decoding only after comparing it with original source behavior and known-good fixtures.
- [ ] Support required TGA, DDS, ZIP-contained image, and metadata variants as discovered.
- [ ] Preserve alpha, dimensions, frame offsets, crop rectangles, opaque bounds, and source origin.
- [ ] Handle palette-indexed assets and palette remapping if the discovered content uses them.
- [ ] Keep decoding off the render path; cache decoded resources with bounded memory behavior.

**Acceptance criteria**

- Each decoder has fixtures for valid, truncated, malformed, and edge-case inputs.
- Decoded dimensions and metadata match independent inspection/reference values.
- Alpha/crop/offset handling is covered by pixel-level tests.
- Repeated decoding does not leak unbounded memory or block the renderer.

### G04 — TD/RA unit catalog and visual definitions

**Objective:** Replace approximate class-to-generated-kind mappings with explicit, source-backed per-game visual definitions.

**Tasks**

- [ ] Discover and parse actual unit definitions and their asset references.
- [ ] Map original engine class/type identifiers to logical visual definitions independently for TD and RA.
- [ ] Represent body, turret, barrel, shadow, muzzle flash, overlay, and other layers where applicable.
- [ ] Represent per-state frame sequences, facing counts, frame durations, loop behavior, offsets, and transition rules.
- [ ] Record which values are source-defined, inferred from fixtures, or fallback defaults.
- [ ] Add validation for missing assets, duplicate IDs, invalid frame ranges, and impossible state definitions.

**Acceptance criteria**

- Every supported unit definition has a provenance trail to original metadata/source behavior.
- TD and RA mappings are tested separately.
- Unknown unit types use a deliberate fallback and emit a diagnosable warning rather than selecting an unrelated sprite.

### G05 — Original-art atlas, cache, and renderer integration

**Objective:** Make original unit art available to both game renderers while preserving the generated-art fallback.

**Tasks**

- [ ] Define an asset registry/resource handle model that supports original and generated assets.
- [ ] Build atlases or GPU texture resources from decoded source images without losing frame rectangles or layer metadata.
- [ ] Use bounded background loading/decoding with thread-safe shared state and cancellation/shutdown behavior.
- [ ] Queue GPU uploads to the renderer-owning thread; do not issue graphics API calls from worker threads.
- [ ] Avoid disk I/O, archive reads, image decoding, or blocking locks in draw calls.
- [ ] Establish explicit fallback order: original asset when available, generated CNC32 asset when configured/available, then procedural or flat fallback.
- [ ] Add instrumentation for cache hits/misses, decode time, upload time, and missing assets.

**Acceptance criteria**

- A selected TD unit and a selected RA unit render original source art in their actual game render paths.
- The game still starts and renders when original data is absent.
- The generated pack remains usable and can be selected as a fallback.
- No disk I/O or image decoding occurs in the render loop.
- Background work is bounded and safely shut down.

### G06 — Deterministic facing and animation playback

**Objective:** Advance visual frames predictably and reproduce the original state/facing selection rules.

**Tasks**

- [ ] Reuse or extend the existing tick-oriented `AnimationPlayer`; do not add a third independent animation engine.
- [ ] Convert source frame durations/FPS to a stable tick representation without cumulative rounding drift.
- [ ] Define animation advancement at a deterministic simulation/presentation boundary, never inside draw calls.
- [ ] Implement facing quantization and direction selection from original TD and RA behavior.
- [ ] Support idle, move, turn, attack/fire, death, and any additional states actually present in source assets.
- [ ] Define state transition, restart, loop, hold-last-frame, and missing-state fallback behavior.
- [ ] Ensure multiple render passes or different frame rates do not advance animation more than once per tick.
- [ ] Keep purely cosmetic timing from altering gameplay checksums or network commands.

**Acceptance criteria**

- Unit tests cover frame boundaries, loops, non-looping sequences, variable durations, state changes, and facing wraparound.
- Identical tick/input sequences produce identical animation frame sequences.
- Rendering the same simulation state multiple times produces the same selected frame.
- No animation update is hidden inside renderer callbacks.

### G07 — Turrets, weapons, and layered units

**Objective:** Reproduce independent body and weapon movement where the original unit design requires it.

**Tasks**

- [ ] Identify units with separate turret/barrel/body layers from original metadata and source.
- [ ] Track body facing and turret facing independently where required.
- [ ] Apply correct layer offsets, pivots, draw order, and mirroring rules.
- [ ] Connect firing state to weapon animation and muzzle flash assets.
- [ ] Validate turret tracking and firing presentation without changing targeting/combat simulation.

**Acceptance criteria**

- A representative turreted vehicle renders body and turret independently.
- Turret rotation and firing visuals do not alter deterministic combat state.
- Layer ordering and pivots are covered by screenshot/pixel tests.

### G08 — Team colours and palette transforms

**Objective:** Match the original team-colour transformation behavior rather than applying a generic tint.

**Tasks**

- [ ] Locate and parse the actual TD and RA team-colour definitions from installed data.
- [ ] Audit original `TextureManager`, `TeamColorManager`, and `TeamColor` behavior.
- [ ] Reproduce the relevant hue ranges, HSV adjustments, and input/output curve behavior where required by the asset pipeline.
- [ ] Determine whether team colours are applied at decode, atlas-build, shader, or draw time; choose based on correctness and performance.
- [ ] Keep source image data immutable so multiple houses/players can share cached assets safely.

**Acceptance criteria**

- Known team-colour inputs produce reference-matched outputs within a documented pixel tolerance.
- TD and RA team-colour definitions are tested independently.
- Alpha and non-team-colour pixels remain unchanged.

### G09 — Damage, destruction, and visual effects

**Objective:** Connect visual damage states to real game state and source-defined visual transitions.

**Tasks**

- [ ] Map health/damage thresholds to original visual condition rules.
- [ ] Use original damaged/critical/destroyed art where available; generated condition sprites remain fallback only.
- [ ] Implement destruction animation and terminal-state behavior.
- [ ] Verify burning, smoke, sparks, and other effects from original source/assets where supported.
- [ ] Ensure destroyed visuals do not accidentally remain selectable/functional due to presentation state.

**Acceptance criteria**

- Damage visual state follows authoritative gameplay health/state.
- State thresholds are source-backed or explicitly marked as approximations.
- Repeated render calls do not mutate damage state.
- Destruction and effect lifetimes have deterministic tests where tied to simulation.

### G10 — Specialized unit families

**Objective:** Cover visual behaviors that do not fit a simple single-sprite vehicle.

**Coverage order**

1. Infantry: directional facings, stance, prone/standing, walking, firing, death, and overlays where present.
2. Harvester: movement, harvesting, unloading, and cargo/state presentation.
3. Aircraft: rotor/propeller animation, shadows, altitude/offset presentation, and landing/takeoff where present.
4. Naval units: directional movement, wake/submerge/transport states where present in RA.
5. Transports and carried passengers.
6. Special units and faction-specific variants.

**Acceptance criteria**

- Each family has an explicit behavior matrix for TD/RA applicability.
- Unsupported states are documented rather than silently approximated.
- Representative unit sequences have deterministic tests and visual captures.

### G11 — Building and structure visuals

**Objective:** Extend the asset pipeline to structures without conflating structure-specific rules with unit rules.

**Tasks**

- [ ] Discover structure art, construction frames, damage stages, bibs, overlays, and animation metadata.
- [ ] Model structure footprint, draw origin, layer ordering, and occupied-cell relationships.
- [ ] Support construction/repair/selling/destruction states.
- [ ] Cover animated structures and faction/theater variants.

**Acceptance criteria**

- Representative structures render with correct footprint, anchor, and animation state.
- TD and RA structure differences are explicit.
- Editor and game views agree on the same asset definitions where they share rendering behavior.

### G12 — Projectile and combat presentation

**Objective:** Reproduce the visual side of weapon behavior while keeping it separate from combat resolution.

**Tasks**

- [ ] Inventory projectile, muzzle flash, impact, explosion, and trail resources.
- [ ] Map weapon/projectile definitions to visual effects.
- [ ] Support projectile animation, orientation, layering, and impact timing.
- [ ] Connect effects to simulation events without changing hit/damage calculations.
- [ ] Add cleanup and resource lifetime rules for transient effects.

**Acceptance criteria**

- Representative weapons show the correct projectile and impact family.
- Visual events are driven by authoritative simulation events.
- Effects do not change combat outcomes or lockstep state.

### G13 — Visual regression, performance, and release QA

**Objective:** Establish repeatable evidence that visual work is correct, stable, and performant.

**Tasks**

- [ ] Create deterministic TD and RA scenes with representative units and known positions/states.
- [ ] Capture reference and Rust screenshots with documented renderer, resolution, zoom, seed, and data version.
- [ ] Generate contact sheets for every supported unit/state/facing family.
- [ ] Add pixel-diff and metadata-diff tooling with documented tolerances.
- [ ] Measure cold/warm load time, decode time, upload time, memory use, cache behavior, and frame-time impact.
- [ ] Test no-original-data mode, missing-resource mode, generated fallback mode, and mixed original/generated mode.
- [ ] Run focused crate tests and workspace quality gates; record exact revision and command results.
- [ ] Update both functionality maps when implementation status changes.

**Acceptance criteria**

- Every claimed feature has original source reference, Rust implementation path, live caller path, and focused executed test or reproducible visual evidence.
- TD and RA are verified independently.
- No original proprietary assets are committed.
- Performance and memory behavior are measured rather than assumed.
- Documentation distinguishes implemented, partial, integration-limited, stub/reject, not implemented, and intentionally replaced states.

## 6. Recommended first vertical slice

Do not attempt every unit family at once. The first end-to-end slice should prove the complete original-art pipeline with two deliberately different cases:

### Slice A — TD infantry

- One representative infantry unit.
- Original resource discovery and decoding.
- Facing selection and at least idle/movement animation if the data supports them.
- Correct alpha, frame bounds, and anchor.
- One reproducible in-game or renderer capture.

### Slice B — TD Medium Tank

- One representative tank with independently rotating body/turret layers if original assets support that split.
- Idle/movement animation and facing.
- Correct layer pivots and draw order.
- One reproducible in-game or renderer capture.

After TD's asset path is proven, add the corresponding RA cases through the same infrastructure but with RA-specific resource definitions and behavior. Do not assume a TD mapping is valid for RA.

The Harvester is the next recommended family because it exercises more than ordinary movement: harvesting/unloading states and associated visual transitions.

## 7. Suggested implementation task breakdown

Use these task IDs in implementation plans, commits, evidence reports, and progress updates.

| Task | Work item | Milestone |
|---|---|---|
| UV-001 | Inspect current CLI conventions and supported data-root configuration | G01 |
| UV-002 | Inventory TD/RA archives and identify actual archive formats | G01 |
| UV-003 | Enumerate unit-related archive members and metadata candidates | G01 |
| UV-004 | Produce deterministic TD/RA JSON inventory and unresolved-reference report | G01 |
| UV-005 | Generate reproducible unit contact sheets | G01 |
| UV-006 | Add fixture-backed archive readers/resource resolver | G02 |
| UV-007 | Audit and implement required original image decoders | G03 |
| UV-008 | Build explicit TD/RA unit visual catalogs | G04 |
| UV-009 | Build original-art atlas/cache and renderer upload path | G05 |
| UV-010 | Implement deterministic frame stepping and facing selection | G06 |
| UV-011 | Implement separate body/turret rendering | G07 |
| UV-012 | Implement original team-colour transforms | G08 |
| UV-013 | Connect damage/destruction visuals to game state | G09 |
| UV-014 | Add specialized unit-family behavior | G10 |
| UV-015 | Add building/structure visual definitions | G11 |
| UV-016 | Add projectile and combat presentation assets | G12 |
| UV-017 | Establish full TD/RA visual regression and performance gates | G13 |

Tasks may be split into smaller issues, but do not mark a task complete merely because a type, parser, or renderer function exists. Confirm the complete call path and observable output.

## 8. Engineering constraints

### Determinism

- Animation updates must have a defined tick/time source and stable conversion from source timing.
- Rendering must be read-only with respect to animation progression and gameplay state.
- Cosmetic presentation must not perturb lockstep checksums, command streams, or save compatibility.

### Concurrency and I/O

- Archive reads and image decoding must not block the render thread.
- Use bounded worker queues and bounded caches; avoid unbounded fan-out when many assets are requested.
- Shared registries/caches must use thread-safe access patterns with short critical sections.
- GPU resource creation/upload must occur on the renderer's owning thread or through its supported upload queue.
- Shutdown must stop accepting work, cancel or drain outstanding jobs safely, and release resources without deadlocks.

### Compatibility and fallback

- Original game data is optional. Startup must remain functional without it.
- Keep generated CNC32 assets available as a fallback and development fixture.
- Keep procedural/flat rendering as the final fallback, with diagnostics that identify which source was selected.
- Do not silently substitute a wrong unit sprite when a mapping is missing.

### Provenance and licensing

- Store source archive/member paths, hashes, and metadata in generated audit output.
- Do not commit extracted original art, bulk dumps, or generated atlases derived from proprietary game data.
- Commit parsers, tests, metadata schemas, and small synthetic/non-proprietary fixtures only where licensing permits.

## 9. Evidence and documentation policy

For every completed milestone or feature, record:

1. Exact Rust repository commit SHA.
2. Exact original source revision used for behavioral comparison.
3. Original file(s) and symbol(s).
4. Rust file(s), symbol(s), and the live caller/call path.
5. Fixture/test names and whether they were actually executed.
6. Commands run and their exit status.
7. Visual artifacts with deterministic scene parameters.
8. Known limitations and unsupported cases.

When implementation status changes, update both:

- `RUST/original-source-reference/IMPLEMENTED_FUNCTIONALITY_MAP.md`
- `RUST/original-source-reference/ORIGINAL_FUNCTIONALITY_MAP.md`

Keep the broader `RUST/docs/PARITY.md` synchronized where the change materially affects its parity summary. This roadmap remains the unit-visuals sequencing document; it is not a replacement for the functionality maps or test evidence.

## 10. Definition of done

The unit-visuals effort is complete only when:

- All supported original TD and RA unit visual references resolve or have documented unsupported/missing reasons.
- Required original resource formats are fixture-tested.
- Body/turret/layer/facing/animation/damage metadata is represented accurately for supported units.
- Animation is deterministic and independent of render frequency.
- Original team-colour behavior is verified for supported assets.
- No archive/disk/image decode work occurs in draw calls.
- Background loading, caches, and GPU uploads are bounded and thread-safe.
- The game remains playable without original asset installation and with generated fallback enabled.
- TD and RA each pass their own visual regression suite.
- Performance, memory, and loading behavior have recorded measurements.
- Documentation and functionality maps match the verified implementation state.

## 11. Immediate next action

Start with **UV-001 through UV-005 (G01)**. Do not begin broad renderer changes or hard-code unit catalog filenames before the original installed data has been inventoried. The output of G01 should determine the exact archive adapters, decoder work, catalog schema, and first asset-backed vertical slice.
