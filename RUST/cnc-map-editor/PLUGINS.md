# Map Editor Plugin System — Analysis & Proposal

_Analysis of the current plugin architecture of `cnc-map-editor`, the extension gaps, and a
categorized catalog of useful plugins we could include — grounded in what proven RTS map editors
actually do. Generated 2026-10-19._

---

## Table of Contents

1. [Current plugin architecture](#1-current-plugin-architecture)
2. [How plugins are consumed](#2-how-plugins-are-consumed)
3. [Cross-crate integration (cnc-ai)](#3-cross-crate-integration-cnc-ai)
4. [Extension gaps](#4-extension-gaps)
5. [Cleanest extension points (ranked)](#5-cleanest-extension-points-ranked)
6. [Plugin idea catalog](#6-plugin-idea-catalog)
7. [Proposed trait design](#7-proposed-trait-design)
8. [Prioritized roadmap](#8-prioritized-roadmap)
9. [References](#9-references)

---

## 1. Current plugin architecture

All references are relative to `RUST/`.

### 1.1 The `GamePlugin` trait — `cnc-map-editor/src/plugin.rs:31-81`

The editor already has a plugin seam: a `Send + Sync` trait with **9 methods (2 required)**:

| Method | plugin.rs | Returns | Status |
|---|---|---|---|
| `name` (required) | :32 | `&str` display name | ✅ used |
| `available_terrains` (required) | :33 | `Vec<String>` theater names | ⚠️ only feeds derived `theaters()` |
| `available_objects` (required) | :34 | `Vec<ClassId>` — flat buildable catalog; order = palette order | ✅ used |
| `theaters` | :36-47 | `Vec<Theater>` — derived by string-matching `available_terrains()` | ❌ dead in UI |
| `default_credits` | :50 | `i32` (default 10_000) | ❌ dead outside tests |
| `supports_naval` | :55 | `bool` (default false) | ❌ dead outside tests |
| `trigger_templates` | :60-66 | `Vec<(&str, &str, &str)>` — `(name, condition, action)` triples | ✅ used |
| `validate_object` | :69-71 | `(&MapModel, &MapObject) -> bool` (default: bounds + valid owner) | ✅ used |
| `get_object_name` | :73-75 | `ClassId -> String` — default is Rust Debug text (no human name table) | ✅ used, weak |
| `object_side` | :78-80 | `ClassId -> Option<&'static str>` — side-lock hint for the palette | ✅ used |

The object catalog type is `cnc_core::ClassId` (`cnc-core/src/class_id.rs:14-133`), a flat
RTTI-style enum of ~100 variants. Name parsing (`parse_class_id`, `cnc-map-editor/src/map.rs:592-632`)
matches lowercase Debug names — **not** plugin-driven. Categories are a separate hardcoded
`ObjectCategory` enum (`app.rs:133-177`) keyed to `ClassId` numeric ranges.

### 1.2 The two shipped plugins — `plugin.rs:84-191`

Both are zero-sized structs; they are the only two `impl GamePlugin` in the workspace.

**`TiberianDawnPlugin`** (:84-126)
- Theaters: Temperate, Snow, Desert (no Urban)
- Catalog: 16 objects (E1-E3 infantry, E1/E2/E3/E6 vehicles, Orca, 8 buildings)
- `default_credits` = 5_000; side-locked: `VehicleE2`/`AircraftE1 → "GDI"`
- Known limitation (test :199-213): `VehicleE1` can't be side-locked — Jeep/Buggy share a class.

**`RedAlertPlugin`** (:129-191)
- Theaters: Temperate, Snow, Urban (no Desert)
- Catalog: 23 objects (adds E8/E9 infantry, MCV, naval ShipE1/E2/E4, Airfield, TeslaCoil,
  GapGenerator, Chronosphere, IronCurtain)
- `supports_naval` = true; `default_credits` = 10_000
- Side-locked: Chronosphere/GapGenerator → "Allies"; IronCurtain/TeslaCoil → "Soviet"
- Own trigger templates including a Chronosphere timer template (:184-190).

Unit tests for both plugins live at plugin.rs:193-252.

---

## 2. How plugins are consumed

`cnc-map-editor/src/app.rs` (6460 lines):

- **Storage/switching**: not a stored `Box<dyn>` — the active plugin is `plugin_index: usize`
  (app.rs:494, initialized 0 at :569). `pub fn plugin(&self) -> Box<dyn GamePlugin>` (:635-640)
  **constructs a fresh boxed plugin on every call** from a 2-arm match.
- **Toggle**: `Action::TogglePlugin` (:235) → handler :2644-2648 flips the index, resets the
  trigger template index, and posts a status message. Toolbar button label "Game: TD" / "Game: RA"
  (:4054-4060). Covered by test `trigger_templates_follow_the_active_plugin` (:5731-5752).
- **Consumer sites** (11 `self.plugin()` calls):
  - **Palette**: `visible_objects()` :1102-1124 filters the catalog by `ObjectCategory` + search
    text (search also matches `get_object_name` and `object_side`); buttons :2860-2871; side tags
    `"E2 [GDI]"` :4623-4630; header "OBJECTS (n/m)" :4152-4158.
  - **Trigger templates**: :1164-1166 → feeds `TriggerTool` (:2656-2662) and the trigger UI.
  - **Validation tab**: `validation_issues()` :1485-1500 = `map.validate()` + per-object
    `plugin.validate_object()`.
  - **Agent Studio**: :1792-1808 warns when an agent script references a class absent from the
    active plugin catalog.

**Verdict**: *moderately pluggable on the UI surface, rigid underneath.* Every consumer already
funnels through `self.plugin()`, so the trait is a clean seam — but major systems (terrain/overlay
catalogs, palette categories, faction lists, the procedural generator, minimap rendering, and all
INI serialization) sit entirely outside the plugin boundary. Adding a third game today means
touching `app.rs` in ~6 places. This matches the crate's broader dyn-trait style
(`Box<dyn EditCommand>`, `Box<dyn Tool>`, `dyn Renderer`), so deepening `GamePlugin` fits house
conventions.

---

## 3. Cross-crate integration (cnc-ai)

The one deep cross-crate integration is the **Agent Studio**, not map objects:

- `MapModel.agents: Vec<AgentDefinition>` (map.rs:62); serialized as `[AIAgents]` +
  per-agent `[AIAgentN]` sections (map.rs:334-345, parsed :461+).
- Studio lifecycle in app.rs: edit state `from_agent`/`to_agent` (:326-340), open/close
  (:1623/:1648), click routing (:2216), drawing (:2268), script compile + inline diagnostics
  (:1784, :1832, :1866, error lines :2411), oracle advice via `observation_prompt`/`parse_response`
  (:1944-1965).
- Cross-crate contract: `AgentDefinition::validate(catalog: &[String])`
  (`cnc-ai/src/agent.rs:42-60`) takes the **plugin's** catalog to check `class_references`;
  `agent.owner` defaults to the plugin's `owner_house()` (app.rs:1709). Doctrines: 5 constants
  (agent.rs:9).

`cnc-procgen` is unrelated (it synthesizes replacement art for `cnc-sfx`/`cnc-music`, per its
codemap). The in-crate procedural generator (`src/generator.rs`) is **not plugin-able** today:
`TerrainType` (:25-41, 15 variants), `OverlayType` (:113-120), `MapGenSettings` (:154-190) and
`generate()` (:202+) touch `Theater` only — never `GamePlugin`.

---

## 4. Extension gaps

Things currently hardcoded that could plausibly become plugin-driven:

1. **Theater list** — `const THEATERS` (app.rs:38) drives `TheaterNext` (:2637-2642); the trait's
   `theaters()` (plugin.rs:36) is derived correctly but **dead in the UI**.
2. **`default_credits` / `supports_naval`** — exposed by the trait, consumed nowhere.
3. **Houses/factions** — `owner_house()` (:650-656), the enemy house (:1077-1081) and
   `owner_options()` (:1348-1353) are hardcoded 4-way matches; `ClassId` has only 4 houses.
4. **Terrain & overlay palettes** — `TerrainType::ALL` / `OverlayType::ALL` (generator.rs:44/:123)
   drive palette buttons and the `terrain_color`/`overlay_color` tables; a new game cannot add
   terrain kinds or colors.
5. **`ObjectCategory`** (app.rs:133-177) — hardcoded enum keyed to ClassId ranges.
6. **Trigger semantics** — templates are plugin strings, but the condition/action *dialect* is free
   text evaluated only in `cnc-red-alert/src/game.rs` (:1602-1616); no plugin schema for
   conditions/actions.
7. **Minimap** (app.rs:3959-4036) — fixed terrain colors and object colors.
8. **Object display names** — `get_object_name` default is Debug text; ClassId doc-comment
   nicknames ("Light Tank") are unreachable at runtime.
9. **Auto-placement / resources** — the generator hardcodes Tiberium + Trees + MCV auto-placement;
   `OverlayType::Ore` exists but is never placed, and `supports_naval` doesn't affect generation.
10. **Export/serialization** — a single INI dialect with no plugin hooks (see §8 note).

---

## 5. Cleanest extension points (ranked)

1. **`GamePlugin` trait itself** (plugin.rs:31) — add defaulted methods (terrain/overlay catalogs,
   faction list, minimap colors, display names); consumers already route through `self.plugin()`.
2. **`EditorState::plugin()`** (app.rs:635) — replace `plugin_index: usize` with a plugin registry
   (`&'static [&'static dyn GamePlugin]`) so >2 games need no call-site changes; fold
   `plugin_name`/`owner_house`/enemy-house into trait methods to kill the hardcoded matches.
3. **`plugin.theaters()`** (plugin.rs:36) — already derived correctly; wire `TheaterNext`
   (app.rs:2637) to it instead of `const THEATERS`.
4. **`validate_object`** (plugin.rs:69) — per-game rule slot already merged into
   `validation_issues` (app.rs:1485); grow it into a full lint registry.
5. **`get_object_name`** (plugin.rs:73) — single-point upgrade to human-readable names
   (consumers: app.rs:1114, :4625).
6. **`AgentDefinition::validate(catalog)`** (cnc-ai/agent.rs:42) — already parameterized by the
   plugin catalog; new games get agent validation for free.
7. **`generate`/`MapGenSettings`** (generator.rs:202/:154) — accept a `&dyn GamePlugin` (or a gen
   profile) to make resources/overlays/auto-placement game-aware (Ore for RA, naval starts when
   `supports_naval`).
8. **Keep serialization plugin-free** — the INI dialect is the cross-crate contract with
   `cnc-red-alert/src/scenario.rs:374`. Any plugin-owned sections need an *additive* `[Section]`
   hook, not a redesign.

---

## 6. Plugin idea catalog

Evidence base: StarCraft II Galaxy Editor & WC3 World Editor (module/data-pack design), Tiled
(script extensions + AutoMapping), OpenRA (`IEditorBrush`, `ILintPass`, `OpenRA.Utility` commands,
map format versioning), FinalAlert 2 (Westwood's own editor), Sentient Sketchbook / WFC / AI
playtest research. Tags: **[Catalog]** plugin supplies data · **[LintRule]** validation rule ·
**[TriggerTpl]** trigger template · **[Tool]** editor tool/brush · **[Codec]** import/export ·
**[Gen]** generator · **[Sim]** simulation · **[Assist]** AI assist.

### Generation
| # | Idea | What / why |
|---|---|---|
| G1 | **WFC terrain packer** [Gen] | Learn tile adjacency per theater, fill seeded regions with coherent terrain (proven in Bad North / Caves of Qud); run pathability lint after. No hand-written rule maps needed. |
| G2 | **Sketch mode → detailed map** [Gen/Assist] | Sentient-Sketchbook port: draw a coarse layer (bases / ore / impassable blobs), get live playability checks and evolved alternative completions — makes our existing generator *interactive*. |
| G3 | **Automapping rule packs** [Gen/Tool] | Tiled-style input/output rule maps for C&C shapes: beach transitions, cliff runs, roads, tiberium-field edges. Rules ship as data files — community-extensible without code. |
| G4 | **Symmetry / mirror tool** [Tool/Gen] | Mirror terrain + ore + waypoints + starts across point/rotational/diagonal axes with a balance check. SC2/WC3 famously *lack* this; #1 symmetric-map request. |
| G5 | **Prefab / stamp library** [Tool] | Save terrain+actors+waypoints selections as reusable stamps (base layouts, neutral tech, scenery). OpenRA's copy/paste brush and FA2's base nodes validate demand. |
| G6 | **Constraint ore-field placer** [Gen/Tool] | Paint ore zones constrained: equal harvestable value per start, min distance from spawns, chokepoint corridors kept clear. |

### Placement / editing aids
| # | Idea | What / why |
|---|---|---|
| P1 | **Foundation-aware building placement** [Tool] | Slope/occupancy preview under the ghost before drop; FA2 shows silent-failure placement bugs belong in the brush. |
| P2 | **Random scatter brush with preview** [Tool] | Trees/rocks/props with density + cluster parameters and a live preview (FA2's improved random tree tool). |
| P3 | **Squad / team composer** [Tool/TriggerTpl] | Select N placed units → auto-generate TaskForce + Script + Team + spawn waypoint chain (FA2's painful-to-hand-author workflow). |
| P4 | **Waypoint route editor** [Tool] | Draw patrol/reinforcement routes, auto-number waypoints, search by ID, warn on impassable crossings. |
| P5 | **Base-node layout editor** [Tool/Catalog] | Place AI base layouts as editable node graphs consumed by Agent Studio scripts (FA2's base nodes formalized). |

### Validation / linting
| # | Idea | What / why |
|---|---|---|
| V1 | **Lint-pass registry** [LintRule] | Named checks emitting error/warning with severities at save/export/CI — exactly OpenRA's `ILintPass` shape. Checks: spawn reachability, size divisibility, cordon cells, min spawn distance. |
| V2 | **Catalog cross-reference linter** [LintRule/Catalog] | Every trigger/actor must reference an existing class/house/weapon in the active pack (OpenRA `CheckTraitPrerequisites` model). |
| V3 | **Pathability & chokepoint analysis** [LintRule/Assist] | Walkable regions + Voronoi chokepoints (BWTA-style) as an overlay; feeds generators (connectivity) and AI auto-annotation. |
| V4 | **Packaging validator** [LintRule] | Enforce export rules: preview PNG dimensions, size ceilings, file-size budget for online play (a real multiplayer constraint in FA2). |
| V5 | **Trigger graph linter** [LintRule] | Dangling waypoints, event→action chains that can never fire, unset variable pairs — the classic broken-mission-logic sources. |

### Import / Export
| # | Idea | What / why |
|---|---|---|
| E1 | **Legacy TD/RA INI importer** [Codec] | `--import-td-map` / `--import-ra-map` equivalents with tileset-ID remap tables between games (OpenRA documents the exact byte-level conversion). |
| E2 | **Format upgrade pipeline** [Codec] | Versioned map schema + migration rules + `--upgrade-map`/`--refresh-map` that rewrites old files and regenerates previews. |
| E3 | **Multi-target codec registry** [Codec] | Plugins register importers/exporters: `.oramap`, TD/RA scenario INI, JSON, image snapshots (Tiled `mapFormats` pattern). |
| E4 | **Asset pipeline utils** [Codec/Assist] | MIX listing, palette-aware SHP→PNG + remap (OpenRA `--list-mix`/`--png`/`--remap`) so previews can use original art. |

### Simulation / Playtest
| # | Idea | What / why |
|---|---|---|
| S1 | **AI-vs-AI auto-playtest harness** [Sim] | Our editor already compiles AI scripts: run K short self-play games, collect win rates / time-to-first-contact / engagement maps (SC bot ladders + Borovikov playtest agents prove the pattern). |
| S2 | **Balance metrics + auto-tune** [Sim/Assist] | Metric dashboard (ore per start, shortest attack path, choke count, symmetry index) with optional evolutionary/Bayesian search over layouts to minimize win-rate disparity. |

### AI / Assist
| # | Idea | What / why |
|---|---|---|
| A1 | **Palette-aware sprite upscaler** [Assist] | Real-ESRGAN + auto background removal/defringe for theater previews and HD thumbnails; 24px gameplay pixels stay authoritative. |
| A2 | **Auto-annotation** [Assist] | Derive region labels, chokepoints, expansion spots from terrain (BWTA approach) and inject as named waypoints + AI hints. |

### Rendering / Preview
| # | Idea | What / why |
|---|---|---|
| R1 | **Lighting & ambience preview** [Tool] | Per-map lighting editor with live preview incl. negative lights (FA2's lighting editor) — cheap, big mission-mood win. |
| R2 | **Framework / blueprint overlay** [Tool] | Toggleable structural overlay hiding art detail (FA2 CTRL-F) to check geometry/occupancy/pathing. |
| R3 | **Animated tile preview + bulk animation setup** [Tool] | Preview water/fire/ambient anim sets in-editor; bulk-author animation frames from a tileset range. |

### Workflow
| # | Idea | What / why |
|---|---|---|
| W1 | **User-scripted batch commands** [Tool] | Registerable menu actions that batch-edit maps (rename triggers, shift waypoints, strip previews). Tiled's JS extensions + FA2 "user scripts" are the most-replicated plugin form; Rust equivalent: a `Command` trait registry or Rhai/Wasm sandbox. |
| W2 | **Data-pack / dependency switching** [Catalog] | Loadable catalog packs (GDI/Nod, Allies/Soviet, expansion units, total conversions) selected per map, validated against placed objects (SC2 dependency packs + FA2 mod toggle + FinalOmega precedent). |

---

## 7. Proposed trait design

The evidence maps **24 of the 29 ideas** onto five core traits, plus one optional event trait.
Generators and simulators don't need UI traits — they work as headless commands validated by
lint rules and registered like OpenRA's `OpenRA.Utility` commands (with seeds/checkpoints for
determinism, matching the workspace's determinism policy).

```rust
/// 1. Catalog + game data (existing GamePlugin, extended)
trait CatalogProvider: Send + Sync {
    // existing: name, available_objects, trigger_templates, validate_object,
    //           get_object_name, object_side
    // new: faction list, terrain/overlay catalogs, minimap colors,
    //      default_credits, supports_naval  (currently dead trait methods)
}

/// 2. Validation (OpenRA ILintPass shape)
trait ValidationRule {
    fn run(&self, emit_error: &mut dyn FnMut(String),
                 emit_warning: &mut dyn FnMut(String),
                 map: &MapModel, catalog: &dyn CatalogProvider);
}

/// 3. Trigger templates (existing string triples, promoted to a type)
trait TriggerTemplate { fn name/condition/action(&self) -> &str; }

/// 4. Editor tools (OpenRA IEditorBrush shape: cursor + map -> undoable ops)
trait EditorTool {
    fn id(&self) -> &str;
    fn on_press/on_drag/on_release(&self, map: &mut MapModel, cursor: Cell, mods: Modifiers)
        -> Vec<Box<dyn EditCommand>>;   // one undo step per gesture, per house rules
}

/// 5. Map codecs (Tiled mapFormats pattern)
trait MapCodec {
    fn id(&self) -> &str;
    fn import(&self, path: &Path) -> Result<MapModel>;
    fn export(&self, map: &MapModel, path: &Path) -> Result<()>;
}

/// Optional: live feedback surface (Sentient-Sketchbook-style)
trait OnMapChanged { fn on_map_changed(&self, map: &MapModel); }
```

Notes:
- **Plugin selection** should become a registry (`&'static [&'static dyn CatalogProvider]`)
  replacing `plugin_index: usize`; `plugin_name`/`owner_house`/enemy-house fold into the catalog
  trait.
- **Serialization stays plugin-free** by default (the INI dialect is the cross-crate contract with
  `cnc-red-alert`); codecs are the sanctioned additive hook.
- Determinism: generators/simulators take explicit seeds and produce deterministic output, per
  workspace policy.

---

## 8. Prioritized roadmap

Demand-backed priority (each item is directly evidenced in multiple proven editors; the first
three make the existing trigger/catalog/validation plumbing immediately community-extensible):

| Phase | Items | Rationale |
|---|---|---|
| 1 — Foundation | Registry replacing `plugin_index`; wire dead trait methods (`theaters`, `default_credits`, `supports_naval`); human-readable `get_object_name` | Unblocks everything else; kills ~6 hardcoded app.rs sites |
| 2 — Lint registry | V1 lint-pass registry + V2 catalog cross-reference + V5 trigger graph linter | OpenRA-proven shape; `validate_object` already merged into `validation_issues` |
| 3 — Editing aids | G4 mirror tool, G5 prefab stamps, P3 squad composer, P2 scatter brush | Highest user-visible value; FA2/OpenRA demand evidence |
| 4 — Interop | E1 legacy importer, E2 upgrade pipeline, E3 codec registry | Opens the library of classic C&C maps; versioned schema protects saves |
| 5 — Simulation | S1 AI-vs-AI playtest harness, V3 pathability analysis, S2 balance metrics | Leverages the existing cnc-ai Agent Studio + deterministic engines |
| 6 — Assist/Polish | G1/G2/G3 generators, A1/A2 AI assists, R1-R3 previews | Biggest differentiators, built on a stable plugin core |

---

## 9. References

**In-repo**
- `cnc-map-editor/src/plugin.rs` — `GamePlugin` trait, TD/RA plugins, tests
- `cnc-map-editor/src/app.rs` — plugin consumption (`plugin_index`, 11 call sites)
- `cnc-map-editor/src/map.rs` — `MapModel`, INI serialization, `parse_class_id`
- `cnc-map-editor/src/generator.rs` — procedural generator (in-crate, non-plugin)
- `cnc-core/src/class_id.rs` — `ClassId` catalog enum
- `cnc-ai/src/agent.rs` — `AgentDefinition::validate(catalog)` contract
- `cnc-red-alert/src/scenario.rs` — cross-crate consumer of the editor's INI dialect
- `RUST/codemap.md`, `RUST/BUILD_STATUS.md` — workspace overview and verification ledger

**External (evidence for the idea catalog)**
- StarCraft II Galaxy Editor module docs; WC3 World Editor & `war3map` file spec (WC3MapSpecification)
- Tiled scripting + AutoMapping docs; `mapeditor/tiled-extensions`
- OpenRA: Mapping wiki, map format, `OpenRA.Utility` docs, `LintInterfaces.cs` (ILintPass family),
  `IEditorBrush` implementations
- FinalAlert 2 official readme (CnCNet); C&C Labs trigger tutorial (TaskForce → Script → Team → Trigger)
- Sentient Sketchbook (Liapis et al.); WaveFunctionCollapse (Gumin); Borovikov et al. 2019
  playtest agents; Lara-Cabrera evolutionary RTS balancing; RuleSmith (arXiv 2026)
- BWTA chokepoint analysis; Real-ESRGAN sprite upscaling pipelines
