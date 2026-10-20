//! Engine tests authored in `assets/packs/cnc32_assets_v6_2_1/engine_tests/
//! rust/`, included here via `#[path]` bridging so
//! `cargo test -p cnc-asset` executes them from the pack directory.
//!
//! Each bridged module is a self-contained pack conformance suite:
//! - `manifest_conformance`: manifest.json structure, ids/paths/hex digests,
//!   mask re-packing, gated PNG dimension sampling.
//! - `atlas_integrity`: atlas_rect bounds for every atlas sprite, gated
//!   pixel-exact crop-vs-standalone verification.
//! - `ui_index`: ui/ui_asset_index_v6_2.json version, counts, and referenced
//!   files.
//! - `tileset_matcher`: `PackTileset` composed-atlas determinism, the 47
//!   valid blob masks / `repair_blob_mask` contract, and
//!   blob/road/shore id construction pinned against the manifest.
//!
//! All modules skip gracefully (not fail) when the pack directory is absent;
//! set `CNC_CNC32_PACK_DIR` to point at a copy elsewhere. See the pack's
//! `engine_tests/README.md` for the full run/grading contract.

#[path = "../../assets/packs/cnc32_assets_v6_2_1/engine_tests/rust/atlas_integrity.rs"]
mod atlas_integrity;
#[path = "../../assets/packs/cnc32_assets_v6_2_1/engine_tests/rust/manifest_conformance.rs"]
mod manifest_conformance;
#[path = "../../assets/packs/cnc32_assets_v6_2_1/engine_tests/rust/tileset_matcher.rs"]
mod tileset_matcher;
#[path = "../../assets/packs/cnc32_assets_v6_2_1/engine_tests/rust/ui_index.rs"]
mod ui_index;
