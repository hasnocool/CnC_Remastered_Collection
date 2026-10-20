# cnc-classic

> **Status: ROADMAP PLACEHOLDER — not a working release mode.** This crate is a scaffold: `src/lib.rs` is 8 lines exposing `classic_version() -> "0.1.1-classic"` and `src/main.rs` is 3 lines; there is no RTS runtime, no rules engine and **0 tests** (verified with `cargo test -p cnc-classic`). The scope statement below is the intended destination, not the current state — do not cite it as parity. *Marked as a roadmap placeholder 2026-10-20 (re-verified 2026-10-20 against the live tree — `.omo/evidence/task-6-parity-100.log`; the workspace test ledger books cnc-classic at 0 tests).*

Pure RTS classic release mode for the CnC Remastered Collection (planned).

- Workspace version: 0.1.1
- SemVer: inherits workspace version
- Scope: core RTS rules only (no bench, viewer, or editor dependencies)

## Build

```bash
cd RUST
cargo build -p cnc-classic
```
