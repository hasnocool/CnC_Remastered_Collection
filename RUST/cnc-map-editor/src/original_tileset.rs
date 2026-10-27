//! Original-art tileset: renders the map from the shipped Remaster textures.
//!
//! [`PackTileset`](super::PackTileset) composites the generated 32px pack; this
//! module composites the ORIGINAL icon art from `TEXTURES_TD_SRGB.MEG` into the
//! same combined-atlas shape, so the editor's render loop, `CellTile` shape and
//! `sprite_rect` addressing stay unchanged — only the source of the pixels and
//! the semantic family rows differ.
//!
//! Template names (`CLEAR1`, `SH1`, `S01`, ...) come from the original
//! `TemplateTypeClass` table in `original/TIBERIANDAWN/CDATA.CPP`, ported in
//! [`cnc_asset::original_templates`]. Rows follow the same semantic families the
//! draw code queries (`terrain.grass`, `water_edges.deep_water`, ...), each row
//! holding that theater's own art.

use super::pack_tileset::CellTile;
use cnc_asset::original_assets::{Game, OriginalAssets, Theater};
use cnc_asset::tile_join::{self, JoinMaterial};
use cnc_core::Rect;

/// Combined-atlas geometry.
///
/// The shipped icons are 128x128 per tile, so the atlas keeps them at native
/// resolution instead of crushing to 16px: `NATIVE_TILE` is the edge length of
/// one icon, and `SHEET`/`COLUMNS`/`ROWS` are sized for the resulting scale
/// (a 16x16 grid of 128px icons is 2048px, far past any reasonable atlas, so
/// the grid is trimmed to the tiles actually composited).
const NATIVE_TILE: u32 = 128;
/// Rows of tiles per atlas sheet (the semantic families used, 16 at most).
const SHEET_ROWS: u32 = 16;
/// Tiles per row: one column per variant, and the original has few.
const SHEET_COLS: u32 = 4;
const SHEET: u32 = SHEET_COLS * NATIVE_TILE;
const COLUMNS: u32 = SHEET_COLS;

/// (category, family, original template) triples in slot order — the same
/// semantic rows `TilesetPack::load_original` composites in `cnc-asset`.
/// Icon index inside a shore template for a 4-bit land mask (N=1, E=2, S=4, W=8).
///
/// Measured from the shipped art (`examples/shore_geom.rs` prints the beach
/// coverage of every edge of every icon): each entry is the icon whose beach
/// band lies on the land sides and whose open edges lie on the water sides.
/// Choosing per-cell from this table is what makes adjacent shore cells line
/// up: two neighbouring shore cells share an edge only when both selected icons
/// agree that edge is water (or both that it is beach).
///
/// Index = mask (0..15); value = icon index (icon 4/5/6 of `SH1` etc. carry no
/// directional band and are used where the mask demands no dominant side).
const FAMILIES: &[(&str, &str, &[&str])] = &[
    ("terrain", "grass", &["P01", "P02", "P03"]),
    ("terrain", "dirt", &["P03", "P01", "P04"]),
    ("terrain", "sand", &["P04", "P02", "P03"]),
    ("terrain", "rock", &["P07", "P08", "P04"]),
    ("terrain", "rough", &["P08", "P13", "P14"]),
    ("terrain", "mud", &["P13", "P14", "P08"]),
    ("terrain", "concrete", &["P14", "P13", "P08"]),
    ("water_edges", "deep_water", &["W1"]),
    ("water_edges", "shallow_water", &["W2", "W1"]),
    // Shore cells resolve per-cell from the cell's own template icon when the
    // map carries one (original maps do — see `cell_draw_masked`); the
    // mask-driven table below serves generated maps, whose cells have no
    // authored icon.
    ("water_edges", "shore", &["SH1", "SH01", "SH1.TEM"]),
    ("transitions", "grass_rock", &["S01", "S03", "S09"]),
    ("transitions", "grass_sand", &["S02", "S01", "S04"]),
    ("transitions", "dirt_rock", &["S03", "S01", "S09"]),
    ("transitions", "sand_rock", &["S04", "S02", "S01"]),
    ("roads_bridges", "paved_ns", &["D01", "D03", "D26"]),
    ("roads_bridges", "paved_ew", &["D02", "D04", "D36"]),
    ("cliffs", "cliff_interior", &["RV01", "RV02", "RV05"]),
];

/// Shore `(template, icon)` per 4-bit land mask (N=1, E=2, S=4, W=8).
///
/// MEASURED, not guessed: `cnc-asset/examples/shore_geom.rs` decodes every one
/// of the 170 `(template, icon)` pairs the temperate theater ships and reports
/// the fraction of each edge that is water-blue. A pair is a candidate for a
/// mask when the set of its water edges is exactly the mask's water sides.
/// Each row below is the argmin of that join penalty over all 170 pairs.
///
/// 12 of the 16 masks reach penalty 0.000 — the neighbour's land edge meets
/// beach and its water edge meets open water, so the shared edge matches and
/// the seam is invisible. The four remaining masks (0x02, 0x05, 0x08, 0x0A)
/// score 0.250 because the shipped art holds no pair with that exact 4-side
/// profile; every alternative differs in at least one edge. That is a property
/// of the 1996 art, not of this table.
///
/// The original game never computes a mask: `original/TIBERIANDAWN/CELL.CPP`
/// `Draw_It` draws `icon = TIcon` straight from the cell. Seamless terrain in
/// shipped maps is AUTHORED data (`MAP.CPP` stores `(TemplateType, TIcon)` per
/// cell), which is why `cell_draw_masked` prefers a cell's own icon when the
/// map carries one. This table only serves generated maps, whose cells have no
/// authored icon.
const SHORE_BY_MASK: [(&str, u32); 16] = [
    ("SH11", 7), // 0x00 join penalty 0.000
    ("SH2", 6),  // 0x01 join penalty 0.000
    ("SH12", 6), // 0x02 join penalty 0.250
    ("SH11", 6), // 0x03 join penalty 0.000
    ("SH10", 4), // 0x04 join penalty 0.000
    ("SH1", 0),  // 0x05 join penalty 0.250
    ("SH12", 6), // 0x06 join penalty 0.000
    ("SH14", 5), // 0x07 join penalty 0.000
    ("SH2", 4),  // 0x08 join penalty 0.250
    ("SH17", 2), // 0x09 join penalty 0.000
    ("SH11", 4), // 0x0A join penalty 0.250
    ("SH1", 6),  // 0x0B join penalty 0.000
    ("SH2", 4),  // 0x0C join penalty 0.000
    ("SH1", 0),  // 0x0D join penalty 0.000
    ("SH11", 4), // 0x0E join penalty 0.000
    ("SH1", 1),  // 0x0F join penalty 0.000
];
/// Family key a shore mask resolves to.
fn shore_variant_name(mask: u8) -> String {
    format!("shore_mask{mask}")
}

/// Spellings to try for a template, game-aware.
///
/// RA zero-pads two-digit shoreline families (`SH01` where TD writes `SH1`);
/// both orders are tried, so one table serves both games.
fn candidates(template: &str, ra: bool) -> Vec<String> {
    let padded = template
        .strip_prefix("SH")
        .map(|num| format!("SH{num:0>2}"))
        .unwrap_or_else(|| template.to_string());
    if ra {
        vec![padded, template.to_string()]
    } else {
        vec![template.to_string(), padded]
    }
}

/// One mask-driven shore variant, resolved at build time.
#[derive(Clone)]
struct MaskVariant {
    template: String,
    icon: u32,
}

/// `SHORE_BY_MASK` resolved against the archive for this game.
fn mask_variants(assets: &OriginalAssets, theater: Theater) -> [Option<MaskVariant>; 16] {
    let ra =
        matches!(theater, Theater::Snow | Theater::Interior) && assets.game() == Game::RedAlert;
    let mut out: [Option<MaskVariant>; 16] = std::array::from_fn(|_| None);
    for (mask, (template, icon)) in SHORE_BY_MASK.iter().enumerate() {
        let options = candidates(template, ra);
        if let Some(hit) = options
            .iter()
            .find(|tpl| assets.icon_bytes(tpl, *icon, theater, 0).is_ok())
        {
            out[mask] = Some(MaskVariant {
                template: hit.clone(),
                icon: *icon,
            });
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Atlas composition
// ---------------------------------------------------------------------------

/// Atlas sheet a join material is composited into.
fn category_for_material(material: JoinMaterial) -> &'static str {
    match material {
        JoinMaterial::Water | JoinMaterial::Shore | JoinMaterial::River => "water_edges",
        JoinMaterial::Road | JoinMaterial::Slope => "roads_bridges",
        JoinMaterial::Rock | JoinMaterial::Patch | JoinMaterial::Land => "terrain",
    }
}

/// Combined original atlas: semantic families plus explicitly named template
/// icons, all at the shipped 128px tile size.
pub struct OriginalTileset {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    /// (category, family) → row.
    rows: std::collections::HashMap<(String, String), u8>,
    /// (category, "<template>#<icon>") → row for per-cell authored icons.
    icon_rows: std::collections::HashMap<(String, String), u8>,
    /// category → sheet origin (x, y).
    origins: std::collections::HashMap<String, (u32, u32)>,
    /// Source archive (theater it was built for).
    assets: OriginalAssets,
    theater: Theater,
    /// Measured edge profiles of every icon that ships, for join scoring.
    join: cnc_asset::tile_join::JoinTable,
    /// Registered icon list per canonical template name, filled while the
    /// atlas is composed. Drives the interior scatter pick that keeps
    /// same-terrain interiors from rendering as one repeated icon.
    icons_of_template: std::collections::HashMap<String, Vec<u32>>,
}

impl OriginalTileset {
    /// Composite TD's temperate art (kept for callers that do not pick a game).
    pub fn load(theater: Theater) -> Option<Self> {
        Self::load_game(Game::TiberianDawn, theater)
    }

    /// Composite one game's original art for a theater.
    pub fn load_game(game: Game, theater: Theater) -> Option<Self> {
        let assets = OriginalAssets::open_default(game).ok()?;
        Self::from_assets(assets, theater, game)
    }

    /// Composite from an already-open archive.
    pub fn from_assets(assets: OriginalAssets, theater: Theater, game: Game) -> Option<Self> {
        // One sheet per category, stacked vertically; each sheet holds
        // `SHEET_ROWS` rows of NATIVE_TILE px tiles.
        let mut cats: Vec<&str> = Vec::new();
        for (cat, _, _) in FAMILIES {
            if !cats.contains(cat) {
                cats.push(cat);
            }
        }
        // `roads_bridges` needs the most rows (roads + slopes ship ~1120 icons
        // over the 4 available columns, and the join picker can select any of
        // them). One sheet size for all categories keeps addressing simple;
        // the atlas is allocated once up front.
        // Every preloaded icon occupies one full row of its category's sheet
        // (they are blit at `ox`, the sheet's left edge), so the row count is
        // the number of icons that sheet needs. Measure it: count the icons
        // each category ships and size for the largest.
        let mut sheet_rows_per_cat = 16u32;
        for cat in cats.iter().copied() {
            let mut rows_for_cat = 16u32;
            for (prefix, count) in tile_join::PREFIX_COUNT {
                for n in 0..=*count {
                    for name in [format!("{prefix}{n:02}"), format!("{prefix}{n}")] {
                        let Some(mat) = tile_join::material_for(&name) else {
                            continue;
                        };
                        if category_for_material(mat) != cat {
                            continue;
                        }
                        for icon in 0..8u32 {
                            if assets.icon_rgba(&name, icon, theater).is_ok() {
                                rows_for_cat += 1;
                            }
                        }
                    }
                }
            }
            sheet_rows_per_cat = sheet_rows_per_cat.max(rows_for_cat);
        }
        let sheet_height = u64::from(sheet_rows_per_cat) * u64::from(NATIVE_TILE);
        let _ = SHEET_ROWS; // documented for the semantic-row layout above
        let width = u64::from(COLUMNS * SHEET);
        let height = sheet_height * cats.len() as u64;
        let mut rgba = vec![0u8; (width * height * 4) as usize];
        let (width, height) = (width as u32, height as u32);

        let mut rows = std::collections::HashMap::new();
        let mut icon_rows = std::collections::HashMap::new();
        let mut icons_of_template: std::collections::HashMap<String, Vec<u32>> =
            std::collections::HashMap::new();
        let mut origins = std::collections::HashMap::new();
        let mut next_row = std::collections::HashMap::new();

        for (slot, cat) in cats.iter().enumerate() {
            let ox = 0u32;
            let oy = slot as u32 * (sheet_height as u32);
            origins.insert((*cat).to_string(), (ox, oy));
            next_row.insert((*cat).to_string(), 0u32);

            let ra = assets.game() == Game::RedAlert;
            let mut row = 0u32;
            for (_, family, cand) in FAMILIES.iter().filter(|(c, _, _)| c == cat) {
                if *family == "shore" {
                    // 16 rows, one per land mask.
                    for (mask, variant) in mask_variants(&assets, theater).iter().enumerate() {
                        let Some(variant) = variant else {
                            row += 1;
                            continue;
                        };
                        let Ok((pixels, icon_w, _)) =
                            assets.icon_rgba(&variant.template, variant.icon, theater)
                        else {
                            row += 1;
                            continue;
                        };
                        blit_icon(
                            &mut rgba,
                            width,
                            &pixels,
                            icon_w,
                            ox,
                            oy + row * NATIVE_TILE,
                        );
                        rows.insert(
                            ((*cat).to_string(), shore_variant_name(mask as u8)),
                            row as u8,
                        );
                        // Also register the icon itself, so a cell that carries
                        // its own `(template, icon)` resolves to this variant.
                        icon_rows.insert(
                            (
                                (*cat).to_string(),
                                Self::icon_id(&variant.template, variant.icon),
                            ),
                            row as u8,
                        );
                        row += 1;
                    }
                    continue;
                }
                let options = candidates(cand[0], ra);
                let hit = options
                    .iter()
                    .find_map(|tpl| assets.icon_rgba(tpl, 0, theater).ok());
                if let Some((pixels, icon_w, _)) = hit {
                    blit_icon(
                        &mut rgba,
                        width,
                        &pixels,
                        icon_w,
                        ox,
                        oy + row * NATIVE_TILE,
                    );
                    rows.insert(((*cat).to_string(), (*family).to_string()), row as u8);
                }
                row += 1;
            }
            // Preload every icon of every template that ships for this
            // category, so the join picker and any authored icon resolve
            // without decoding at draw time. The set comes from the measured
            // join table, which covers every family in the theater.
            for (prefix, count) in tile_join::PREFIX_COUNT {
                for n in 0..=*count {
                    let names = [format!("{prefix}{n:02}"), format!("{prefix}{n}")];
                    for tpl in &names {
                        // Only templates in this category's sheet.
                        let Some(mat) = tile_join::material_for(tpl) else {
                            continue;
                        };
                        if category_for_material(mat) != *cat {
                            continue;
                        }
                        for icon in 0..8u32 {
                            if row >= sheet_rows_per_cat {
                                break;
                            }
                            let Ok((pixels, icon_w, _)) = assets.icon_rgba(tpl, icon, theater)
                            else {
                                continue;
                            };
                            blit_icon(
                                &mut rgba,
                                width,
                                &pixels,
                                icon_w,
                                ox,
                                oy + row * NATIVE_TILE,
                            );
                            icon_rows
                                .insert(((*cat).to_string(), Self::icon_id(tpl, icon)), row as u8);
                            icons_of_template
                                .entry((*tpl).clone())
                                .or_default()
                                .push(icon);
                            row += 1;
                        }
                    }
                }
            }
        }

        Some(Self {
            rgba,
            width,
            height,
            rows,
            icon_rows,
            origins,
            join: tile_join::measure(game, theater),
            icons_of_template,
            assets,
            theater,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn rgba(&self) -> &[u8] {
        &self.rgba
    }

    /// Registered icon list per canonical template name, filled while the
    /// atlas is composed. Drives the interior scatter pick that keeps
    /// same-terrain interiors from rendering as one repeated icon.
    /// Sprite id for an authored `(template, icon)` pair.
    pub fn icon_id(template: &str, icon: u32) -> String {
        format!("{template}#{icon}")
    }

    /// Atlas rect for a semantic id (`"<category>/<family>"`) or an authored
    /// icon id (`"<category>/<template>#<icon>"`).
    pub fn sprite_rect(&self, sprite_id: &str) -> Option<Rect> {
        let (cat, rest) = sprite_id.split_once('/')?;
        let (ox, oy) = *self.origins.get(cat)?;
        let row = if let Some((template, icon)) = rest.split_once('#') {
            let icon: u32 = icon.parse().ok()?;
            self.icon_row(cat, template, icon)?
        } else {
            *self.rows.get(&(cat.to_string(), rest.to_string()))?
        };
        Some(Rect::new(
            ox as i32,
            (oy + u32::from(row) * NATIVE_TILE) as i32,
            NATIVE_TILE as i32,
            NATIVE_TILE as i32,
        ))
    }

    /// Row of an authored `(template, icon)` pair, composited at build time.
    ///
    /// Returns `None` when that pair does not ship for this theater.
    fn icon_row(&self, category: &str, template: &str, icon: u32) -> Option<u8> {
        let key = (category.to_string(), Self::icon_id(template, icon));
        self.icon_rows.get(&key).copied()
    }

    /// Per-cell pick without neighbour context.
    pub fn cell_draw(&self, terrain_id: u8) -> Option<CellTile> {
        self.cell_draw_masked(terrain_id, 0x0F, None)
    }

    /// Per-cell pick driven by the measured join table, for EVERY family.
    ///
    /// `neighbours` lists the material on each side of the cell, north
    /// clockwise from `'N'` (`N`, `E`, `S`, `W`, then the diagonals `NE`, `SE`,
    /// `SW`, `NW`). A side is included when it is inside the map; the picker
    /// scores every candidate icon of `material` by how well its measured edge
    /// profiles agree with what each neighbour demands and returns the best.
    ///
    /// This is the general form of the shore-only table: [`crate::tile_join`]
    /// measures all ~1700 icons the theater ships, so roads, slopes, rivers,
    /// patches, brush and shore all join by the same rule.
    pub fn cell_draw_joined(
        &self,
        material: JoinMaterial,
        neighbours: &[(JoinMaterial, char)],
    ) -> Option<CellTile> {
        let pick = self.join.select(material, neighbours)?;
        // The atlas holds each family under its own category sheet.
        let cat = match material {
            JoinMaterial::Water | JoinMaterial::Shore | JoinMaterial::River => "water_edges",
            JoinMaterial::Road | JoinMaterial::Slope => "roads_bridges",
            JoinMaterial::Rock | JoinMaterial::Patch | JoinMaterial::Land => "terrain",
        };
        Some(CellTile {
            base: Some(format!("{cat}/{}#{}", pick.template, pick.icon)),
            layers: Vec::new(),
        })
    }

    /// The measured join table (for evidence and tests).
    pub fn join_table(&self) -> &cnc_asset::tile_join::JoinTable {
        &self.join
    }

    /// Icons the atlas registered for a template, in insertion order.
    ///
    /// Empty when the template does not ship, so callers fall back to the
    /// join picker instead of guessing.
    pub fn icons_of(&self, template: &str) -> &[u32] {
        self.icons_of_template
            .get(template)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Join pick restricted to one template family.
    ///
    /// The original engine stores `TemplateType` per cell (`MAP.CPP:678`), and
    /// the template is what decides the base material: `P01` is clear grass,
    /// `P02` a slightly rougher patch, `S01` grass-to-cliff slope, `RV01`
    /// river water, and so on. Picking only by material collapses 80 patch
    /// variants into whichever one scores best, so a whole map becomes one
    /// tile. This overload keeps the `template` constraint and lets the join
    /// score choose the icon inside it.
    pub fn cell_draw_from(
        &self,
        template: &str,
        neighbours: &[(JoinMaterial, char)],
    ) -> Option<CellTile> {
        let material = tile_join::material_for(template)?;
        let restricted = self.join.restricted(material, template);
        let pick = restricted.select(material, neighbours)?;
        let cat = category_for_material(material);
        Some(CellTile {
            base: Some(format!("{cat}/{}#{}", pick.template, pick.icon)),
            layers: Vec::new(),
        })
    }

    /// Per-cell pick aware of the neighbouring land mask.
    ///
    /// `land_mask` uses the editor's bit order (N=1, E=2, S=4, W=8); a bit is
    /// set when that neighbour is NOT water.
    pub fn cell_draw_masked(
        &self,
        terrain_id: u8,
        land_mask: u8,
        icon_hint: Option<u32>,
    ) -> Option<CellTile> {
        use super::generator::TerrainType;

        // A cell that carries its own template icon wins: that is the original
        // engine's per-cell storage, and the art is authored to line up inside
        // a template set. Prefer any shore candidate whose icon exists.
        if let Some(icon) = icon_hint {
            for (cat, _, cand) in FAMILIES.iter() {
                for tpl in cand.iter() {
                    if self.assets.icon_bytes(tpl, icon, self.theater, 0).is_ok() {
                        return Some(CellTile {
                            base: Some(format!("{cat}/{tpl}#{icon}")),
                            layers: Vec::new(),
                        });
                    }
                }
            }
        }

        if TerrainType::from_id(terrain_id) == Some(TerrainType::Water) {
            if land_mask & 0x0F == 0 {
                // Interior water with no land neighbour: deep water, not a
                // shore variant (mask 0's shore icon is an all-water tile, but
                // `deep_water` is the correct semantic and its art is the
                // engine's own water texture).
                return Some(CellTile {
                    base: Some("water_edges/deep_water".to_string()),
                    layers: Vec::new(),
                });
            }
            let fam = shore_variant_name(land_mask & 0x0F);
            if self
                .rows
                .contains_key(&("water_edges".to_string(), fam.clone()))
            {
                return Some(CellTile {
                    base: Some(format!("water_edges/{fam}")),
                    layers: Vec::new(),
                });
            }
        }

        let (cat, fam) = match TerrainType::from_id(terrain_id)? {
            TerrainType::Grass | TerrainType::ForestFloor => ("terrain", "grass"),
            TerrainType::Water => ("water_edges", "deep_water"),
            TerrainType::Sand => ("terrain", "sand"),
            TerrainType::Dirt | TerrainType::Mud | TerrainType::Swamp => ("terrain", "dirt"),
            TerrainType::Rock | TerrainType::Crater => ("terrain", "rock"),
            TerrainType::Snow => ("water_edges", "shallow_water"),
            TerrainType::Rough | TerrainType::Scorched => ("terrain", "rough"),
            TerrainType::Ice => ("water_edges", "shallow_water"),
            TerrainType::Concrete => ("terrain", "concrete"),
            TerrainType::JungleFloor => ("terrain", "grass"),
        };
        if !self.rows.contains_key(&(cat.to_string(), fam.to_string())) {
            return None;
        }
        Some(CellTile {
            base: Some(format!("{cat}/{fam}")),
            layers: Vec::new(),
        })
    }
}

/// Copy one shipped icon into the atlas at native resolution.
///
/// The original tiles are 128x128 and the atlas holds them as-is (no
/// downsampling), so zooming the editor shows the shipped pixel art rather than
/// a 16px crush of it. Icons larger than a native tile are cropped to their
/// top-left NATIVE_TILE square; smaller icons are centred.
fn blit_icon(
    atlas: &mut [u8],
    atlas_width: u32,
    icon: &[u8],
    icon_side: u32,
    dst_x: u32,
    dst_y: u32,
) {
    let (icon_side, icon) = if icon_side > NATIVE_TILE {
        (
            NATIVE_TILE,
            &icon[..((NATIVE_TILE * NATIVE_TILE) * 4) as usize],
        )
    } else {
        (icon_side, icon)
    };
    let inset = (NATIVE_TILE - icon_side) / 2;
    let ox = dst_x + inset;
    let oy = dst_y + inset;
    for sy in 0..icon_side {
        let s = ((sy * icon_side) * 4) as usize;
        let d = ((oy + sy) * atlas_width + ox) as usize * 4;
        let len = (icon_side * 4) as usize;
        if s + len <= icon.len() && d + len <= atlas.len() {
            atlas[d..d + len].copy_from_slice(&icon[s..s + len]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temperate_pack() -> Option<OriginalTileset> {
        OriginalTileset::load_game(Game::TiberianDawn, Theater::Temperate)
    }

    /// Every measured icon that ships must be reachable in the atlas, because
    /// the join picker can select any of them for a cell.
    #[test]
    fn every_measured_icon_resolves_in_the_atlas() {
        let Some(pack) = temperate_pack() else {
            return;
        };
        let mut measured = 0usize;
        let mut unresolved = Vec::new();
        for (prefix, count) in cnc_asset::tile_join::PREFIX_COUNT {
            for n in 0..=*count {
                for name in [format!("{prefix}{n:02}"), format!("{prefix}{n}")] {
                    let Some(material) = cnc_asset::tile_join::material_for(&name) else {
                        continue;
                    };
                    let cat = category_for_material(material);
                    for icon in 0..8u32 {
                        if pack.assets.icon_rgba(&name, icon, pack.theater).is_ok() {
                            measured += 1;
                            let id = OriginalTileset::icon_id(&name, icon);
                            if pack.icon_row(cat, &name, icon).is_none() {
                                unresolved.push(id);
                            }
                        }
                    }
                }
            }
        }
        assert!(
            measured > 1000,
            "expected the full temp inventory, got {measured}"
        );
        assert!(
            unresolved.is_empty(),
            "icons missing from the atlas (first 8): {:?}",
            unresolved.iter().take(8).collect::<Vec<_>>()
        );
    }

    /// Adjacent cells that carry the same join material must agree on whether
    /// their facing atlas edge carries material, on both axes.
    ///
    /// This is the seam property the goal asks for, measured from the actual
    /// composited pixels rather than from metadata. A hard material boundary
    /// (water meeting land) is exempt: that boundary is supposed to show.
    #[test]
    fn neighbouring_cells_agree_on_their_shared_edge() {
        let Some(pack) = temperate_pack() else {
            return;
        };
        use crate::ui::TileSource;
        let source = TileSource::Original(std::sync::Arc::new(pack));
        let settings = crate::generator::MapGenSettings::new(42);
        let model = crate::generator::generate(&settings);
        let (px, w) = (source.rgba(), source.width() as usize);
        let carries = |p: &[u8]| p.len() >= 4 && p[3] >= 32;

        let (mut agree, mut disagree) = (0usize, 0usize);
        let mut sample: Vec<String> = Vec::new();
        for y in 0..model.height as i32 {
            for x in 0..model.width as i32 {
                let here = source.cell_draw(&model, x, y).and_then(|t| t.base);
                let east = source.cell_draw(&model, x + 1, y).and_then(|t| t.base);
                let south = source.cell_draw(&model, x, y + 1).and_then(|t| t.base);
                let Some(here) = here else { continue };
                let Some(hr) = source.sprite_rect(&here) else {
                    continue;
                };
                for (other, vertical) in [(east, false), (south, true)] {
                    let Some(other) = other else { continue };
                    if here.split('/').next() != other.split('/').next() {
                        continue; // different family: hard boundary is intended
                    }
                    let Some(or) = source.sprite_rect(&other) else {
                        continue;
                    };
                    let (mut ca, mut cb) = (0usize, 0usize);
                    for i in 0..NATIVE_TILE as usize {
                        let (ao, bo) = if vertical {
                            // This tile's bottom row vs the tile below's top row.
                            (
                                ((hr.y as usize + 127) * w + (hr.x as usize + i)) * 4,
                                ((or.y as usize) * w + (or.x as usize + i)) * 4,
                            )
                        } else {
                            // This tile's right column vs the neighbour's left column.
                            (
                                ((hr.y as usize + i) * w + (hr.x as usize + 127)) * 4,
                                ((or.y as usize + i) * w + (or.x as usize)) * 4,
                            )
                        };
                        if carries(&px[ao..ao + 4]) {
                            ca += 1;
                        }
                        if carries(&px[bo..bo + 4]) {
                            cb += 1;
                        }
                    }
                    if (ca > 64) == (cb > 64) {
                        agree += 1;
                    } else {
                        disagree += 1;
                        if sample.len() < 6 {
                            sample.push(format!("({x},{y}) {} {ca} vs {} {cb}", here, other));
                        }
                    }
                }
            }
        }
        // The audit walks both axes over the whole map; requiring a few hundred
        // pairs catches the picker silently returning nothing for most cells.
        assert!(agree > 500, "audit found too few pairs: {agree}");
        assert!(
            disagree == 0,
            "seam mismatches: {disagree} of {} pairs; first few: {sample:?}",
            agree + disagree
        );
    }

    #[test]
    fn interior_water_draws_deep_water_not_a_shore() {
        let Some(pack) = temperate_pack() else {
            return; // Steam install absent
        };
        let water = crate::generator::TerrainType::Water.id();
        let cell = pack.cell_draw_masked(water, 0x00, None).expect("cell");
        assert_eq!(cell.base.as_deref(), Some("water_edges/deep_water"));
    }

    #[test]
    fn shore_mask_selects_the_matching_variant() {
        let Some(pack) = temperate_pack() else {
            return;
        };
        let water = crate::generator::TerrainType::Water.id();
        // Land at north only: the shore must be a per-mask variant, not deep
        // water and not a land family.
        let cell = pack.cell_draw_masked(water, 0b0001, None).expect("cell");
        let id = cell.base.clone().unwrap_or_default();
        assert!(id.starts_with("water_edges/shore_mask"), "got {id}");
        assert!(pack.sprite_rect(&id).is_some(), "{id} has no atlas rect");
    }

    #[test]
    fn authored_template_icons_resolve_to_native_size() {
        let Some(pack) = temperate_pack() else {
            return;
        };
        // Every template ships at least one icon for temperate TD; which ones
        // exist depends on the theater, so probe every category.
        // CLEAR1 is the translucent overlay: correctly not part of any semantic
        // family, so it gets no row.
        for tpl in ["P01", "SH8", "S01", "D01", "RV01", "W1", "W2"] {
            let mut found = 0;
            for cat in [
                "terrain",
                "water_edges",
                "transitions",
                "cliffs",
                "roads_bridges",
            ] {
                for icon in 0..8u32 {
                    if let Some(rect) = pack.sprite_rect(&format!("{cat}/{tpl}#{icon}")) {
                        assert_eq!(
                            (rect.width, rect.height),
                            (NATIVE_TILE as i32, NATIVE_TILE as i32),
                            "{tpl}#{icon} must render at native resolution"
                        );
                        found += 1;
                    }
                }
            }
            assert!(found > 0, "{tpl} composited no icons");
        }
    }

    #[test]
    fn semantic_families_resolve_for_temperate() {
        let Some(pack) = temperate_pack() else {
            return;
        };
        for fam in [
            "terrain/grass",
            "terrain/dirt",
            "terrain/sand",
            "terrain/rock",
            "terrain/rough",
            "terrain/mud",
            "terrain/concrete",
            "water_edges/deep_water",
            "water_edges/shallow_water",
            "transitions/grass_rock",
            "transitions/grass_sand",
            "roads_bridges/paved_ns",
            "cliffs/cliff_interior",
        ] {
            assert!(
                pack.sprite_rect(fam).is_some(),
                "family {fam} did not resolve"
            );
        }
    }

    #[test]
    fn interior_water_is_blue_and_land_is_warm() {
        let Some(pack) = temperate_pack() else {
            return;
        };
        let rgba = pack.rgba();
        let w = pack.width() as usize;
        let sample = |id: &str| -> (u8, u8, u8) {
            let rect = pack.sprite_rect(id).expect(id);
            let mut sx: usize = 0;
            let mut sy: usize = 0;
            let mut sn: usize = 0;
            for y in 0..NATIVE_TILE {
                for x in 0..NATIVE_TILE {
                    let o =
                        ((rect.y as usize + y as usize) * w + (rect.x as usize + x as usize)) * 4;
                    if rgba.get(o).copied().unwrap_or(0) > 8 {
                        sx += x as usize;
                        sy += y as usize;
                        sn += 1;
                    }
                }
            }
            if sn == 0 {
                return (0, 0, 0);
            }
            let o = ((sy / sn) * w + (sx / sn)) * 4;
            (rgba[o], rgba[o + 1], rgba[o + 2])
        };
        let water = sample("water_edges/deep_water");
        let land = sample("terrain/rock");
        assert!(water.2 > water.0, "water must be blue-dominant: {water:?}");
        assert!(
            u16::from(land.0) + u16::from(land.1) + u16::from(land.2) > 30,
            "rock tile must have texture: {land:?}"
        );
    }
}
