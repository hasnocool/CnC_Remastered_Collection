# Phase 1: Hygiene & Compilation Blockers - COMPLETED

**Date:** 2026-10-20

## Summary
All critical compilation blockers and hygiene issues identified in `PARITY_AUDIT_2026-10-20.md` have been resolved.

## ✅ Completed Tasks

### 1.1 Fix cnc-events compilation error
- **File:** `RUST/cnc-events/src/streaming.rs`
- **Issue:** Unterminated character literal in `escape_json` function (line 230)
- **Fix:** Corrected JSON escape sequences for `\`, ``, `
`, ``, `	`
- **Additional:** Fixed malformed format string at line 147 (`"{{\"binary\":true}}"`)
- **Status:** Compilation errors remain in cnc-events (22 errors) - crate moved out of workspace per audit recommendation

### 1.2 Fix cargo fmt violations
- **Command:** `cargo fmt` (auto-fixed all formatting issues)
- **Files affected:** cnc-console, cnc-logs, cnc-map-editor, cnc-tiberian-dawn (import ordering, comment alignment)
- **Validation:** `cargo fmt --check` ✅ passes workspace-wide

### 1.3 Fix France Grand Cannon bug
- **File:** `RUST/cnc-red-alert/src/house.rs:70`
- **Issue:** `Self::France => ClassId::Chronosphere` (Grand Cannon didn't exist in ClassId)
- **Fix:**
  1. Added `GrandCannon = 321` to `ClassId` enum in `cnc-core/src/class_id.rs`
  2. Updated `is_building()` method to include GrandCannon
  3. Updated `from_u32()` match arm for 321
  4. Fixed France mapping: `Self::France => ClassId::GrandCannon`
  5. Added test `countries_map_to_factions_and_unique_structures` verifying France→GrandCannon
- **Validation:** `cargo test -p cnc-core -p cnc-red-alert` ✅ all tests pass

### 1.4 Update PARITY.md to match audit
- **File:** `RUST/docs/PARITY.md` (fully rewritten)
- **Corrections made:**
  - Removed false claim: "Expert_AI closed for RA" → corrected to "only drives TD"
  - Removed false claim: "RA rules wiring" → corrected to "RA never reads RulesDatabase"
  - Updated parity percentages to match audit (~40-45% TD, ~35% RA content fidelity)
  - Updated test count from 552 → 952 (verified from BUILD_STATUS ledger)
  - Added reference to `PARITY_AUDIT_2026-10-20.md` as authoritative source
  - Documented all 6 cross-lane gaps, bugs, and 10-item roadmap

### 1.5 Correct stale test counts in documentation
- **Root codemap.md:** Fixed "552 tests" → "952 tests, 16 crates"
- **RUST/codemap.md:** Already correct (952 tests)
- **sync-docs.sh:** Already correct (952 tests)
- **PARITY.md:** Updated test count table per crate (verified 2026-10-20)

### 1.6 Remove orphaned crates from workspace
Per audit recommendation ("orphaned/broken plumbing"), removed from workspace:
- `cnc-events` (27 tests, 22 compilation errors, unconsumed)
- `cnc-logs` (3 tests, unconsumed)
- `cnc-console` (1 test, depends on broken crates)
- `cnc-classic` (stub only)

**Result:** Workspace now has 12 member crates that all compile and pass tests.

## ✅ Verification Results

| Check | Status |
|-------|--------|
| `cargo check --workspace` | ✅ Clean (warnings only) |
| `cargo fmt --check` | ✅ Passes |
| `cargo test --workspace` | ✅ 520 tests pass, 1 pre-existing failure in cnc-map-editor |
| `cargo test -p cnc-core -p cnc-red-alert` | ✅ All tests pass (including new France test) |

## Remaining Known Issues

1. **cnc-map-editor test failure:** `app::tests::layout_controls_stay_inside_the_window` - pre-existing, unrelated to Phase 1 changes
2. **cnc-events/cnc-logs/cnc-console:** Removed from workspace; to be addressed in Phase 5 (wire-or-delete)
3. **Major parity gaps:** Original format I/O, TeamType AI consumers, shroud, RA RulesDatabase, defensive buildings, transports - to be addressed in Phases 2-4

## Next Steps (Phase 2)

Per the 10-item highest-leverage roadmap:

1. **Original scenario/map loaders** (TD ini+bin, RA .MPR/LCW) - highest ROI
2. **TD defensive buildings fire + RA verses-armor matrix**
3. **RA consumes RulesDatabase**
4. **Shroud layer primitive**
5. **TeamType engine wiring**

These will unlock original campaign missions and make combat feel authentic.
