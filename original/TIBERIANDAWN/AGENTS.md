# AGENTS.md — TIBERIANDAWN/

**Generated:** 2026-10-09 · **Commit:** f1f0d42 · **Branch:** ?

## OVERVIEW
Tiberian Dawn (C&C1) game engine (C++): 411 tracked files (193 .CPP, 184 headers, 28 assembly), same layout as REDALERT/. Core game logic for Command & Conquer: Tiberian Dawn. Shares architectural patterns with Red Alert but implements distinct unit rosters, campaigns, and balance. Includes house/ faction logic, mission scripting, and legacy Westwood file format handlers.

## STRUCTURE


## Rust parity workflow

When auditing a Tiberian Dawn feature against the port, use `RUST/original-source-reference/ORIGINAL_FUNCTIONALITY_MAP.md` for the original behavior and `RUST/original-source-reference/IMPLEMENTED_FUNCTIONALITY_MAP.md` for current Rust evidence. Trace original symbols → Rust implementation → actual game/scenario/command call path → focused test. TD original INI/BIN loading and cell-trigger support exist for covered records; do not continue to call them wholly missing. Keep unmapped records and unsupported trigger vocabulary explicit. Record exact commit and test execution status; never infer test passes from historical logs.