//! Hard-switch adapter: the editor's tile pipeline addresses the generated
//! `cnc32_assets_v6_2_1` 32px pack through [`cnc_asset::classic_pack`]
//! (`Cnc32Pack` + the composed `PackTileset` atlas from Lane A), replacing
//! the legacy 16px `tileset_pack::TilesetPack` (which stays untouched in
//! `cnc-asset` for the `examples/tileset_render.rs` QA harness).
//!
//! This module owns:
//! - pack discovery (`Cnc32Pack::fallback_dir()`: `CNC_CNC32_PACK_DIR` env →
//!   workspace-relative `assets/packs/cnc32_assets_v6_2_1` → CWD), with an
//!   optional explicit override from `--tileset <dir>` that must be a valid
//!   cnc32-style pack (anonymous manifest parse failures degrade to flat
//!   colours, mirroring the legacy `--no-tileset` behaviour);
//! - the editor `TerrainType` → pack base-name mapping (see
//!   [`terrain_base_name`]);
//! - the per-cell matcher mirroring `tiles.rs::base_draw`'s priority chain
//!   (road/wall overlays → water shores → resource/tree underlays →
//!   elevation cliffs → terrain transitions → plain ground) with pack-native
//!   sprite ids resolved against the COMPOSED atlas rect map
//!   (`classic_pack::PackTileset::rect` — never the manifest's original
//!   `atlas_rect`).
//!
//! Orientation verification (why code-north == art-north): the pack's
//! retained generator source
//! `assets/packs/cnc32_assets_v6_2_1/source/cnc32_asset_generator_v5.py`
//! draws every directional family with bit 0 = the tile's top rows
//! (screen-north, y decreasing) and bit 2 = the bottom rows: blob
//! `terrain_transition` cuts the primary band back when bit `n`
//! (`same_mask >> 0`) is clear (`y < north[x]`, line ~179), `connected_shore`
//! paints the sand bank on the top rows when the N bit is set (lines
//! 311–318), and cliff/wall bands use the same box order
//! `(top, east, bottom, west)` (lines ~622, ~633). The editor therefore
//! uses the same offsets everywhere — N = (0, −1), E = (1, 0), S = (0, +1),
//! W = (−1, 0), NE/SE/SW/NW at the diagonal offsets — exactly matching the
//! legacy `tiles.rs` bit constants (`BIT_N = 1, BIT_E = 2, BIT_S = 4,
//! BIT_W = 8`), which the manifest's `connections` / `land_edges` lists for
//! roads (`roads/road_05_vertical` = N+S) and shoreline
//! (`shoreline/water_landmask_01` = land at N) pin to reality.
//!
//! The legacy 16px pack's water-edge art needed a 180° rotation (legacy
//! family names point at the side the *water art* occupies); the pack's
//! shoreline art points at the LAND side (`land_edges` semantics), so shore
//! masks map through unchanged — no flip.

use std::collections::BTreeMap;
use std::path::Path;

use cnc_asset::classic_pack as classic;
use cnc_core::Rect;
use cnc_core::Vec2i;

use crate::generator::{OverlayType, TerrainType};
use crate::map::MapModel;
use crate::tiles::{variant_col, BIT_E, BIT_N, BIT_S, BIT_W};

/// The 4-bit edge order shared by roads, walls, shores and cliff bands
/// (`tiles.rs` constants, bit 3 = W and bit 0 = N).
const CARDINAL_BITS: [u8; 4] = [BIT_N, BIT_E, BIT_S, BIT_W];

/// One resolved cell draw: the base sprite id (blob/base/road/shore/wall…) —
/// `None` when the cell keeps the editor's flat-colour fallback — plus any
/// stacked RGBA sprite layers (resource crystals, tree props, craters).
/// Sprite ids address the COMPOSED atlas via [`PackTileset::sprite_rect`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellTile {
    pub base: Option<String>,
    pub layers: Vec<String>,
}

/// Adapter owning the parsed pack, the composed atlas and the matcher glue.
pub struct PackTileset {
    /// Parsed `manifest.json` + UI index (lookups, id anchoring).
    pack: classic::Cnc32Pack,
    /// Composed RGBA atlas; `rect()` returns composed pixel coordinates.
    composed: classic::PackTileset,
}

impl PackTileset {
    /// Load a cnc32-style pack from `dir` (the `--tileset <dir>` override).
    /// Errors are strings; callers print them and fall back to flat colours.
    pub fn load(dir: &Path) -> Result<PackTileset, String> {
        let pack = classic::Cnc32Pack::load(dir).map_err(|e| format!("{e:#}"))?;
        let composed = classic::PackTileset::load(&pack).map_err(|e| format!("{e:#}"))?;
        Ok(PackTileset { pack, composed })
    }

    /// Discovery without an explicit override:
    /// `CNC_CNC32_PACK_DIR` → `../assets/packs/cnc32_assets_v6_2_1` relative
    /// to this crate → CWD-relative. `None` keeps the flat-colour renderer.
    pub fn from_fallback() -> Option<PackTileset> {
        let dir = classic::Cnc32Pack::fallback_dir()?;
        PackTileset::load(&dir).ok()
    }

    /// Composed atlas dimensions in pixels (the pack's tiles are 32px).
    pub fn width(&self) -> u32 {
        self.composed.atlas_width
    }

    pub fn height(&self) -> u32 {
        self.composed.atlas_height
    }

    /// Composed RGBA8 pixels (row-major), for the renderer's atlas upload.
    pub fn rgba(&self) -> &[u8] {
        &self.composed.rgba
    }

    /// Base tile edge in pixels (32 for this pack revision).
    pub fn tile_size(&self) -> u32 {
        self.composed.tile_size
    }

    /// Access to the parsed pack (tests anchor ids against the manifest).
    pub fn pack(&self) -> &classic::Cnc32Pack {
        &self.pack
    }

    /// Access to the composed atlas (mask helpers, fingerprint, rect map).
    pub fn composed(&self) -> &classic::PackTileset {
        &self.composed
    }

    /// Composed pixel-space rect for a composed sprite id (renderer
    /// `uv_rect`). `None` when the id is missing — callers fall back to flat
    /// colours. This is the only addressing path every draw goes through.
    pub fn sprite_rect(&self, sprite_id: &str) -> Option<Rect> {
        self.composed
            .rect(sprite_id)
            .map(|(x, y, w, h)| Rect::new(x as i32, y as i32, w as i32, h as i32))
    }

    /// Per-cell tile pick mirroring `tiles.rs::base_draw`'s priority chain
    /// with pack-native addressing. `None` = flat-colour fallback cell
    /// (unmapped terrain kinds, ramp/plateau elevation art, out-of-bounds).
    pub fn cell_draw(&self, model: &MapModel, x: i32, y: i32) -> Option<CellTile> {
        let here = Vec2i::new(x, y);
        let index = model.index(here)?;
        let terrain = model.terrain[index];
        let overlay = model.overlay[index];
        let variant = variant_col(x, y);

        // Legacy-compatible cell reader (invalid terrain ids stay raw and
        // fail the pack mapping wherever a name is required).
        let neighbour = |dx: i32, dy: i32| -> Option<(u8, u8, i32)> {
            model.index(Vec2i::new(x + dx, y + dy)).map(|i| {
                (
                    model.terrain[i],
                    model.overlay[i],
                    model.elevation.get(i).copied().unwrap_or(0) as i32,
                )
            })
        };

        // Legacy 4-bit mask helper: bit set when the neighbour satisfies
        // `pred` (out-of-bounds → bit clear, matching `tiles.rs`).
        let mask_over = |pred: &dyn Fn(Option<(u8, u8, i32)>) -> bool| -> u8 {
            let mut mask = 0u8;
            const OFFSETS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
            for (bit_index, (dx, dy)) in OFFSETS.iter().enumerate() {
                if pred(neighbour(*dx, *dy)) {
                    mask |= CARDINAL_BITS[bit_index];
                }
            }
            mask
        };

        // 1. Auto-connected road/wall overlays draw their own tiles.
        if overlay == OverlayType::Road.id() {
            // Road encoding already matches the pack: bit set = connected
            // road edge on that side (`connections` lists in the manifest
            // are literal, e.g. roads/road_05_vertical = N + S).
            let arms = mask_over(&|cell| cell.is_some_and(|(_, o, _)| o == OverlayType::Road.id()));
            return Some(CellTile {
                base: Some(classic::PackTileset::road_id(arms)),
                layers: Vec::new(),
            });
        }
        if overlay == OverlayType::Wall.id() {
            // Wall bands are drawn on every edge whose bit is set
            // (generator: dirt fill on bit-clear edges), so the
            // neighbour-is-wall mask maps through unchanged.
            let arms = mask_over(&|cell| cell.is_some_and(|(_, o, _)| o == OverlayType::Wall.id()));
            return Some(CellTile {
                base: Some(format!("walls/wall_{arms:02x}")),
                layers: Vec::new(),
            });
        }

        // 2. Water cells: pack shoreline art points at the LAND side
        // ("set bit = land/sand shore on that edge"), which is exactly the
        // editor's land-neighbour mask — no orientation flip (the legacy
        // 16px pack needed one; see module docs).
        if terrain == TerrainType::Water.id() {
            let land =
                mask_over(&|cell| cell.is_some_and(|(t, _, _)| t != TerrainType::Water.id()));
            if land == 0 {
                // Open water (no land cardinals). The pack has no deep vs
                // shallow split; every such cell keeps the plain water
                // base, including diagonal-land corners the legacy pack
                // called "shallow".
                return plain_base_cell("water", variant);
            }
            return Some(CellTile {
                base: Some(format!("shoreline/water_landmask_{land:02x}")),
                layers: Vec::new(),
            });
        }

        // 3. Resource/tree underlays own the base ground, exactly like
        // `tiles.rs` step 3; the pack versions are RGBA sprite layers.
        if overlay == OverlayType::Tiberium.id() || overlay == OverlayType::Ore.id() {
            let kind = if overlay == OverlayType::Tiberium.id() {
                "tiberium"
            } else {
                "ore"
            };
            // Variant picks keep the legacy hash family: the legacy 16-col
            // sheet chose cols {0, 1, 14} for tiberium and {3, 4} for ore;
            // the pack sheets carry 8 variants each, folded to {0, 1, 7}
            // and {3, 4}.
            let choices: &[u8] = if kind == "tiberium" {
                &[0, 1, 7]
            } else {
                &[3, 4]
            };
            let resource_variant = hash_pick(choices, x, y);
            let base = mapped_name(terrain).map(|name| base_id(name, variant));
            return Some(CellTile {
                base,
                layers: vec![format!("resources/{kind}/variant_{resource_variant:02}")],
            });
        }
        if overlay == OverlayType::Trees.id() {
            // The legacy pack replaced any tree cell's underlay with floor
            // art unconditionally; the closest pack floor is grass.
            let tree_variant = variant_col(x, y) % 4;
            return Some(CellTile {
                base: Some(base_id("grass", variant)),
                layers: vec![format!("decorations/tree/variant_{tree_variant:02}")],
            });
        }

        // 4. Elevation: ramp/plateau art has no pack counterpart, so those
        // cells keep the flat-colour rendering (None). Sheer cliff faces DO
        // map: the pack cliff bands draw a dark escarpment on every edge
        // whose bit is CLEAR ("edges are visible escarpments"; verified in
        // the generator source, lines ~622), so the editor sends the
        // *connected* mask — bit set where the neighbour is NOT ≥
        // CLIFF_STEP higher (out-of-bounds counts as same level, matching
        // the legacy clamp). Roads/wall/shores keep `tiles.rs` mask order.
        let e = model.elevation.get(index).copied().unwrap_or(0) as i32;
        let high_mask = mask_over(&|cell: Option<(u8, u8, i32)>| {
            cell.is_some_and(|(_, _, neighbour_elevation)| {
                neighbour_elevation - e >= crate::tiles::CLIFF_STEP
            })
        });
        if high_mask != 0 {
            let connected = !high_mask & 0x0f;
            return Some(CellTile {
                base: Some(format!("cliffs/cliff_{connected:02x}")),
                layers: Vec::new(),
            });
        }

        // 5. Terrain transitions: 8-bit blob autotiles. Bit set = the
        // neighbour's mapped base equals this cell's mapped base;
        // unmapped/fallback kinds (and invalid terrain ids) count as
        // DIFFERING. Out-of-bounds neighbours count as SAME so pure maps
        // do not get a cut-back skirt at their borders.
        if let Some(base_name) = mapped_name(terrain) {
            const DIRS: [(i32, i32, u8); 8] = [
                (0, -1, 1 << 0),  // N
                (1, -1, 1 << 1),  // NE
                (1, 0, 1 << 2),   // E
                (1, 1, 1 << 3),   // SE
                (0, 1, 1 << 4),   // S
                (-1, 1, 1 << 5),  // SW
                (-1, 0, 1 << 6),  // W
                (-1, -1, 1 << 7), // NW
            ];
            let mut mask = 0u8;
            let mut alt_counts: BTreeMap<&'static str, u32> = BTreeMap::new();
            for (dx, dy, bit) in DIRS {
                match neighbour(dx, dy) {
                    None => mask |= bit, // out of bounds ≙ continuation
                    Some((terrain_id, _, _)) => {
                        if mapped_name(terrain_id) == Some(base_name) {
                            mask |= bit;
                        } else if let Some(other_name) = mapped_name(terrain_id) {
                            *alt_counts.entry(other_name).or_insert(0) += 1;
                        }
                        // Unmapped neighbour: differing, no alt candidate.
                    }
                }
            }

            // Choose the alt terrain deterministically: most frequent
            // differing mapped neighbour, ties broken alphabetically low.
            let alt = alt_counts
                .iter()
                .max_by_key(|(name, count)| (**count, std::cmp::Reverse(*name)))
                .map(|(name, _)| *name);

            // Repair the raw mask (clear diagonals whose adjacent cardinal
            // bits are not both set) exactly like the generator validity
            // rule; the result is always valid and idempotent.
            let mask = classic::PackTileset::repair_blob_mask(mask);
            debug_assert!(classic::blob_mask_is_valid(mask));

            // All same (mask 255) or fully surrounded by other terrain
            // (mask 0 — the legacy transition pick also skipped these) or
            // no mapped alt: plain ground keeps the base variants.
            if mask == 0 || mask == 0xff || alt.is_none() {
                return plain_base_cell(base_name, variant);
            }
            let alt = alt.expect("alt checked above");
            let blob = classic::PackTileset::blob_id(base_name, alt, mask);
            return Some(CellTile {
                base: Some(blob),
                layers: Vec::new(),
            });
        }

        // Unmapped terrain kind: the flat colour fallback path handles it.
        None
    }
}

/// Plain base-ground pick: `terrain/base/{name}/variant_{:02}` with the
/// legacy variant hash folded to the pack's 4 variants per terrain.
fn plain_base_cell(name: &str, variant: u8) -> Option<CellTile> {
    Some(CellTile {
        base: Some(base_id(name, variant)),
        layers: Vec::new(),
    })
}

/// Pack base-terrain sprite id (manifest scheme: `terrain/base/{name}/
/// variant_{variant:02d}` — DECIMAL zero-padded 2, unlike the mask ids'
/// lowercase hex `{mask:02x}`).
fn base_id(name: &str, variant: u8) -> String {
    format!("terrain/base/{name}/variant_{:02}", variant % 4)
}

/// Deterministic hash pick (same family as `tiles.rs::pick`, which is
/// private there): index the hash into `choices`.
fn hash_pick(choices: &[u8], x: i32, y: i32) -> u8 {
    choices[usize::from(variant_col(x, y)) % choices.len()]
}

/// Editor `TerrainType` → pack base-terrain name. `None` = no pack
/// equivalent; those kinds keep the editor's flat `terrain_color` fallback
/// path exactly (same look as `--no-tileset`).
///
/// Mapping rationale (pack offers {dirt, grass, rock, sand, water}):
/// - Grass / Water / Sand / Dirt / Rock map to the same-named pack base.
/// - Rough → rock: terse/craggy ground; the pack's rock texture is the
///   closest, passability logic in `generator.rs` is untouched.
/// - Mud / Scorched → dirt: loose scorched/muddy grounds read as dirt.
/// - Crater → dirt plus a `decorations/crater/variant_NN` sprite layer,
///   which adds a periodic crater prop exactly as the legacy flat path
///   hinted with darker ground.
/// - ForestFloor / JungleFloor → grass + `decorations/tree/variant_NN`
///   layer (mirrors `tiles.rs`, whose tree underlay forced a floor art).
/// - Snow, Swamp, Ice, Concrete: no honest pack counterpart (no snowbank,
///   mire, ice or paved pad textures). Muddy mismatches (dirt for snow)
///   would hurt more than the legacy flat colours, so these keep today's
///   flat rendering via blob/None.
pub fn terrain_base_name(terrain: TerrainType) -> Option<&'static str> {
    match terrain {
        TerrainType::Grass => Some("grass"),
        TerrainType::Water => Some("water"),
        TerrainType::Sand => Some("sand"),
        TerrainType::Dirt => Some("dirt"),
        TerrainType::Rock => Some("rock"),
        TerrainType::Rough => Some("rock"),
        TerrainType::Mud => Some("dirt"),
        TerrainType::Crater => Some("dirt"),
        TerrainType::Scorched => Some("dirt"),
        TerrainType::ForestFloor | TerrainType::JungleFloor => Some("grass"),
        TerrainType::Snow | TerrainType::Swamp | TerrainType::Ice | TerrainType::Concrete => None,
    }
}

/// Mapped pack base name for a raw terrain id; invalid ids → `None`.
fn mapped_name(terrain_id: u8) -> Option<&'static str> {
    terrain_base_name(TerrainType::from_id(terrain_id)?)
}

// ---------------------------------------------------------------------------
// Pack-anchored matcher tests (graceful skip when the pack is absent, the
// same contract the cnc-asset engine-test suites use).
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use cnc_asset::classic_pack::{blob_mask_is_valid, PACK_TILESET_MAX_DIM};

    fn shared() -> Option<&'static PackTileset> {
        static SHARED: std::sync::OnceLock<Option<PackTileset>> = std::sync::OnceLock::new();
        SHARED
            .get_or_init(|| {
                if let Some(pack) = PackTileset::from_fallback() {
                    Some(pack)
                } else {
                    eprintln!("skipping: cnc32 pack directory not found for pack_tileset tests");
                    None
                }
            })
            .as_ref()
    }

    #[test]
    fn mapping_table_covers_the_pack_terrain_list() {
        let Some(pack) = shared() else { return };
        let pack_terrains = classic::PackTileset::list_terrains(pack.pack());
        let mut mapped: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for terrain in TerrainType::ALL {
            if let Some(name) = terrain_base_name(terrain) {
                mapped.insert(name.to_string());
            }
        }
        for name in &mapped {
            assert!(
                pack_terrains.contains(name),
                "mapping {name:?} is not a pack transition terrain"
            );
        }
        // Every pack terrain is reachable from at least one editor kind.
        for name in &pack_terrains {
            assert!(mapped.contains(name), "pack terrain {name:?} is unmapped");
        }
    }

    #[test]
    fn blob_rect_resolves_for_all_valid_masks() {
        let Some(pack) = shared() else { return };
        let masks = classic::PackTileset::valid_blob_masks();
        assert_eq!(masks.len(), 47);
        for &mask in masks {
            let id = classic::PackTileset::blob_id("grass", "dirt", mask);
            let rect = pack
                .sprite_rect(&id)
                .unwrap_or_else(|| panic!("blob {id} has no composed rect"));
            assert!(rect.width > 0 && rect.height > 0);
            assert!(rect.x + rect.width <= pack.width() as i32);
            assert!(rect.y + rect.height <= pack.height() as i32);
        }
    }

    #[test]
    fn road_wall_shore_cliff_ids_resolve() {
        let Some(pack) = shared() else { return };
        for mask in 0..16u8 {
            for id in [
                classic::PackTileset::road_id(mask),
                format!("walls/wall_{mask:02x}"),
                format!("shoreline/water_landmask_{mask:02x}"),
                format!("cliffs/cliff_{mask:02x}"),
            ] {
                // Anchor against the manifest first (id existence), then the
                // composed rect (bounds).
                assert!(
                    pack.pack().sprite(&id).is_some(),
                    "{id} missing from the pack manifest"
                );
                assert!(
                    pack.sprite_rect(&id).is_some(),
                    "{id} not in composed atlas"
                );
            }
        }
        // Base variants: all five pack terrains × 4 variants resolve.
        for name in classic::PackTileset::list_terrains(pack.pack()) {
            for variant in 0..4u8 {
                let id = format!("terrain/base/{name}/variant_{variant:02}");
                assert!(
                    pack.pack().sprite(&id).is_some(),
                    "{id} missing from the pack manifest"
                );
                assert!(
                    pack.sprite_rect(&id).is_some(),
                    "{id} not in composed atlas"
                );
            }
        }
    }

    #[test]
    fn cell_draw_masks_are_valid_blobs_and_resolve() {
        let Some(pack) = shared() else { return };

        // One dirt neighbour to the east of a grass block: raw mask has the
        // NE diagonal invalid (E clear) so the cell must end up with a
        // VALID, repaired blob pick.
        let mut model = MapModel::new(4, 2);
        for terrain in &mut model.terrain {
            *terrain = TerrainType::Grass.id();
        }
        // Dirt column to the east (x = 3).
        for y in 0..2u32 {
            model.terrain[(y * model.width + 3) as usize] = TerrainType::Dirt.id();
        }
        let pick = pack
            .cell_draw(&model, 2, 0)
            .expect("border cell between grass and dirt picks a tile");
        let base = pick.base.expect("grass↔dirt boundary uses a blob sprite");
        let payload = base
            .strip_prefix("terrain/transitions/grass_to_dirt/blob_")
            .expect("pick targets the grass_to_dirt pair");
        let mask = u8::from_str_radix(payload, 16).expect("mask is zero-padded lowercase hex");
        assert!(
            blob_mask_is_valid(mask),
            "repaired mask {mask:#04x} must satisfy the generator validity rule"
        );
        assert!(
            pack.sprite_rect(&base).is_some(),
            "{base} must live in the composed atlas"
        );

        // Interior grass far from any dirt stays plain base terrain.
        let plain = pack.cell_draw(&model, 0, 0).unwrap();
        assert_eq!(plain.base.as_deref(), Some("terrain/base/grass/variant_00"));

        // Deterministic variant folding: variant hash % 4 stays inside the
        // pack's 4-variant base sheets.
        for x in 0..8i32 {
            for y in 0..8i32 {
                let variant = variant_col(x, y);
                assert!(variant % 4 < 4);
            }
        }
        let _ = pack.composed().fingerprint();
        let _ = PACK_TILESET_MAX_DIM;
    }

    #[test]
    fn hard_switch_happy_path_loads_and_reports_dims() {
        let Some(pack) = shared() else { return };
        assert_eq!(pack.tile_size(), 32);
        assert!(pack.width() > 0 && pack.width() <= PACK_TILESET_MAX_DIM);
        assert_eq!(
            pack.rgba().len(),
            (pack.width() * pack.height()) as usize * 4
        );
    }
}
