# Catalog build status

- [x] Pinned source revision recorded.
- [x] Complete 1,331-path upstream file manifest generated from the Git tree.
- [x] Source groups classified (Red Alert, Tiberian Dawn, map editor, tools/build/resources).
- [x] Function contract template defined.
- [ ] Run the extractor against a checkout containing the pinned source files.
- [ ] Review every extracted declaration/definition pair and callable macro.
- [ ] Document each function's responsibility, inputs, outputs, state effects, and call relationships.
- [ ] Map original contracts to current Rust call paths and mark gaps.
- [ ] Add source-derived fixtures and parity tests for each contract.

The manifest is complete; the symbol and behavior catalogs are not yet claimed complete until the generator has run and extracted symbols are reviewed.
