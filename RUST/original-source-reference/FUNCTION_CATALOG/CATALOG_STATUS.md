# Catalog build status

- [x] Pinned source revision recorded.
- [x] Complete 1,331-path upstream file manifest generated from the Git tree.
- [x] Source groups classified (Red Alert, Tiberian Dawn, map editor, tools/build/resources).
- [x] Function contract template defined.
- [x] Run the extractor against a checkout containing the pinned source files. (Generated 2026-11-14: 15,219 symbols from 1,073 files)
- [x] Integrate original-source-reference documentation into generated indexes.
- [ ] Review every extracted declaration/definition pair and callable macro.
- [ ] Document each function's responsibility, inputs, outputs, state effects, and call relationships.
- [ ] Map original contracts to current Rust call paths and mark gaps.
- [ ] Add source-derived fixtures and parity tests for each contract.

## Generation Summary (2026-11-14)

| Group | Manifest files | Files with symbols | Symbols | Files with docs |
|---|---:|---:|---:|---:|
| REDALERT | 628 | 501 | 6,713 | 51 |
| TIBERIANDAWN | 411 | 324 | 4,329 | 159 |
| MAP_EDITOR | 286 | 247 | 4,172 | 56 |
| SCRIPTS_AND_TOOLS | 6 | 1 | 5 | 0 |

Total: 15,219 callable/candidate symbols from 1,073 source files.
Documentation links: 266 callable-source files have documentation entries in RUST/original-source-reference/.
Missing pinned source: 3 (.gitignore, LICENSE.md, README.md — root-level non-code files).

All extracted symbols are marked SIGNATURE_CATALOGUED pending behavioral review via PORTING_CONTRACT_TEMPLATE.md.
