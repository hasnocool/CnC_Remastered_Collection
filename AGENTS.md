# AGENTS.md — CnC Remastered Collection

**Generated:** 2026-10-09 · **Commit:** f1f0d42 · **Branch:** ?

## OVERVIEW
Source release of the **Command & Conquer Remastered Collection**: two Win32 C++ game-engine DLLs (`TiberianDawn.dll`, `RedAlert.dll`, built from `original/REDALERT/` and `original/TIBERIANDAWN/`) consumed by the closed-source GlyphX client, plus **MobiusEditor**, a C# WinForms map editor for both games (`original/CnCTDRAMapEditor/`). Windows-only, GPL v3 + additional terms, archived — no contributions accepted.

## STRUCTURE
```
/
├── original/           # Original (remastered) source: two Win32 C++ game-engine DLLs + MobiusEditor + scripts
│   ├── REDALERT/       # RA engine, flat dir, 519 C++ files, all CAPS filenames
│   │   ├── RESOURCE/   # .rc, icons, cursors
│   │   └── WIN32LIB/   # Win32 platform glue (keyboard/mouse/sound/timer)
│   ├── TIBERIANDAWN/   # TD (C&C1) engine, same layout, 298 C++ files
│   ├── CnCTDRAMapEditor/   # C# WinForms map editor for both games (.NET Framework 4.6.2, x64 only)
│   ├── SCRIPTS/        # Python asset utilities (tgautil.py)
│   ├── CnCRemastered.sln   # Both engine DLLs (VS2017 / v141 toolset / Win 8.1 SDK)
│   └── CnCTDRAMapEditor.sln  # Map editor solution
├── RUST/               # Rust reimplementation workspace (cnc-* crates)
├── backup/
├── graphflow-out/
├── AGENTS.md
├── codemap.md
├── codemap.md.new
├── file.tree
├── LICENSE.md
├── ORIGINAL_FUNCTIONALITY_MAP.md
├── PHASE1_COMPLETE.md
├── PHASE1_SUMMARY.md
├── README.md
└── WORKFLOWS.md
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| RA game logic | `original/REDALERT/*.CPP` | Uppercase filenames; e.g. `HOUSE.CPP` faction+AI, `COMQUEUE.CPP` netcode |
| TD game logic | `original/TIBERIANDAWN/*.CPP` | Same architecture; divergent subsystems documented in its codemap |
| Map editor model/format | `original/CnCTDRAMapEditor/Model/` | Map.cs sections, CellGrid, undo via `Utility/UndoRedoList.cs` |
| Game-abstraction plugin | `original/CnCTDRAMapEditor/Interface/IGamePlugin.cs` | Implemented per game in `TiberianDawn/`, `RedAlert/` |
| Tooling/build (VS) | `original/*.vcxproj` | v141 toolset, Windows 8.1 SDK, MFC |
| Deep architecture docs | `codemap.md` (root + per-dir) | **Read first** — hierarchy of codemaps exists in most subdirs |

## CONVENTIONS
- **Westwood-era C++**: custom RTTI enum + member (never C++ RTTI), `CCPtr<T>` smart pointers, `NoInitClass` placement-new pattern, manual allocators (`ALLOC.CPP`), object pools, `operator new/delete` overrides on hot classes.
- **Deterministic lockstep simulation**: discrete frames, command queue broadcast, pointer fixup via `Code_Pointers/Decode_Pointers` on save/load. RA uses fixed-point `fixed` (16.16); TD uses `float` bias values — do not unify.
- **Flat directory with ALL-CAPS filenames** in both engines; `.H`/ ".CPP" extensions.
- Editor: plugin pattern (`IGamePlugin`), Command pattern for undo/redo, INI-driven type registries.
- **Visual verification and QA**: for UI artifact review, prefer the `@vision-qa` agent (omo slim custom agent in `~/.config/opencode/oh-my-opencode-slim.json`) — a read-only lane on a free vision-model chain (`nvidia/nemotron-3.5-super-vl` → `nvidia-lindsey|brad/…super-vl` → `…nano-omni` → `opencode/minimax-m3.1-flash-free` → `opencode/nemotron-3.5-super-vl-free`), which reads the PNG natively and returns `VERDICT / ARTIFACT / LAYOUT / TEXT / DEFECTS / CHECKS` without editing anything. Name it `@vision-qa`, not `@visual-qa`: the latter is the heavier multi-reviewer `visual-qa` skill, and the skill name shadows an agent of the same name. Fall back to a subagent pinned to `nvidia/nvidia/nemotron-3.5-super-vl-120b-a12b` (the NVIDIA VL model — see global AGENTS.md §10.5 for the full method: native image read, ground-before-fix, two-pass with session reuse) when a two-pass comparison with session reuse is needed. This repo's artifacts are test-generated screenshots under `RUST/target/scratch-*/` (launcher: `options_modal_writes_a_screenshot_for_visual_qa`; editor: `cargo run -p cnc-map-editor --example tileset_render`). Ground every transcription-level finding against source — the 8x8 bitmap font misreads glyphs (W→M, `-`→`~`), and seeded fixture content (e.g. the Options modal's "Saved renderer 'vulkan' is invalid - using Wgpu." note, deliberately seeded at `RUST/cnc-launcher/src/menu.rs:1515/:1754` and asserted at `main.rs`) is NOT a live defect.

## ANTI-PATTERNS (THIS PROJECT)
- **Do not break ABI/serialization**: saved-game and network-sync formats are load-bearing; struct layout and `Code_Pointers` fixup must stay consistent.
- **Do not "modernize" engine code** wholesale (STL migration, RTTI removal): the DLL must interoperate with the GlyphX client; behavior changes desync multiplayer lockstep.
- Editor: **never mutate the model outside a tool command** — all edits must go through `UndoRedoList` commands or undo breaks.
- `original/CnCTDRAMapEditor/Steamworks.NET/` is vendored+autogenerated — don't restyle it.

## COMMANDS
```
# Engines (VS2017 recommended; newer VS reports SDK header packing mismatch)
#   Open original/CnCRemastered.sln → Build → Batch Build → Select All → Rebuild
# Map editor: open original/CnCTDRAMapEditor.sln (x64)
# Output: root-level bin/
```
CI is enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials — see .github/workflows/rust-ci.yml.

## NOTES
- README pins: Windows 8.1 SDK, MFC, VS 2017. Dev on Linux/WSL = read-only analysis; building requires Windows + MSVC.
- hig file concentration: `original/REDALERT/` (519) and `original/TIBERIANDAWN/` (298)— rely on per-dir codemaps rather than browsing.

## Repository Map

A full codemap is available at `codemap.md` in the project root.

Before working on any task, read `codemap.md` to understand:
- Project architecture and entry points
- Directory responsibilities and design patterns
- Data flow and integration points between modules

For deep work on a specific folder, also read that folder's `codemap.md`.
---
## ENFORCED POLICIES (Updated 2026-10-19)
- **Git / CI** — All builds and commits must go through the local **Forgejo** server (`https://local.lan`) using credentials from `~/APIKEYS.md` (`FORGEJO_API_TOKEN`, `FORGEJO_USER=hyperion`). Push is enforced via `.github/workflows/rust-ci.yml` (and equivalent Forgejo runners) before any merge.
  - **Local CI enforcement**: A `pre-push` git hook (`.git/hooks/pre-push`) runs the full CI pipeline locally before allowing pushes to the Forgejo remote. It verifies:
    - Forgejo credentials from `~/APIKEYS.md`
    - Semantic versioning for all crates
    - README.md and CHANGELOG.md exist for every crate
    - `cargo fmt --check`, `cargo clippy`, `cargo test` all pass
  - Pushes to other remotes (e.g., GitHub `origin`) skip local enforcement.
- **Semantic Versioning** — Every crate in `RUST/` must declare a strict SemVer version (e.g. `1.2.3`) in its `Cargo.toml` (via `version.workspace` or explicit `version`). No non-SemVer versions allowed.
  - Workspace version: `0.1.0` (in `RUST/Cargo.toml`)
  - All crates use `version = { workspace = true }` except `cnc-bench` which uses explicit `version = "0.1.0"`
- **Documentation** — Every crate must maintain:
  - `README.md` (overview, build instructions, documentation links)
  - `CHANGELOG.md` (Keep a Changelog format, linked to SemVer releases)
  - `codemap.md` (architecture reference, already present in most crates)
- See `README.md` and `RUST/Cargo.toml` for workspace-level version rules.


## Original-source / Rust implementation audit workflow

For any C++-to-Rust parity, original-functionality, or port-status task, read these files before changing code or status:

- `RUST/original-source-reference/ORIGINAL_FUNCTIONALITY_MAP.md` — original behavior specification and the current port-status/gap crosswalk.
- `RUST/original-source-reference/IMPLEMENTED_FUNCTIONALITY_MAP.md` — Rust features supported by current source evidence.
- `RUST/original-source-reference/03-rust-port-cross-reference.md` — navigation from original source domains to current Rust paths.
- `RUST/original-source-reference/02-original-file-inventory.md` and the relevant per-file guides — source documentation coverage.

**Required audit loop**
1. Pin the current Rust HEAD and the exact original upstream source commit. Inspect the actual source at those revisions; do not trust an old cross-reference, codemap, TODO list, test count or claimed percentage without rechecking.
2. For each feature, record the original file/symbol, Rust file/symbol, live caller/call path, and focused test or fixture. Check the actual branch and state mutation, not merely the existence of a struct, enum, parser or function.
3. Assign one of these states: implemented with focused evidence; partial; integration-limited; explicit stub/reject; not implemented; or intentionally replaced. State whether a test was merely found or actually executed.
4. Search for stub/success-only paths, default reject handlers, unused parsed fields, uncalled modules, TODO/FIXME markers and runtime branches that never reach the feature.
5. Keep Tiberian Dawn and Red Alert separate where source behavior differs. Parsing a scenario field is not the same as applying it to the game. A modern networking/renderer/save implementation is not legacy protocol/ABI/byte compatibility unless demonstrated.
6. Update both functionality maps in the same change whenever implementation status changes. Keep original-source documentation completeness, Rust implementation status and executed test status as distinct facts.
7. Before reporting package/test counts, run `cargo metadata --no-deps` on the audited checkout. Run relevant crate tests and, where practical, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --no-fail-fast`. Record exact HEAD, command, date and result. If a command cannot be run, say so; never infer a pass from historical logs.
