//! RTS 16×16 tileset pack loader.
//!
//! Packs live under `assets/packs/rts_16x16_tileset/<palette>/` and ship eight
//! 256×256 category sheets (terrain, transitions, water_edges, cliffs,
//! roads_bridges, walls_fences, decorations, resources_fx) with no gutters.
//! Tiles are addressed as `tile_id = row * 16 + column`, row-major, inside each
//! sheet. Opaque sheets are RGB; decoration and effects sheets are RGBA.
//!
//! [`TilesetPack::load`] composites one palette's sheets into a single RGBA8
//! atlas of `4 × sheet` by `2 × sheet` pixels (1024×512 for the shipped packs),
//! filling slots in alphabetical category order, row-major:
//!
//! ```text
//! slot 0 (0,0)   slot 1 (256,0)   slot 2 (512,0)   slot 3 (768,0)
//! slot 4 (0,256) slot 5 (256,256) slot 6 (512,256) slot 7 (768,256)
//! ```
//!
//! That layout matches the atlas size accepted by
//! `cnc_renderer::Renderer::set_atlas_texture` on the tileset texture slot
//! (`cnc_renderer::TILESET_TEXTURE_ID`); sprite UV rectangles are expressed in
//! atlas pixels and can be obtained with [`TilesetPack::tile_px_rect`].

use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::Deserialize;

/// Tile edge length in pixels inside a category sheet.
pub const TILE_SIZE: u16 = 16;
/// Number of tiles per row/column of a category sheet (`tile_id = row*16 + col`).
pub const GRID: u16 = 16;
/// Edge length of one category sheet in pixels.
pub const SHEET_SIZE: u32 = 256;
/// Category sheets composited along the combined atlas X axis.
pub const SHEET_COLUMNS: u32 = 4;
/// Category sheets composited along the combined atlas Y axis.
pub const SHEET_ROWS: u32 = 2;
/// Tiles per row along a 256px sheet at `TILE_SIZE` (16x16 grid).
pub const SHEET_TILES: u32 = 16;

/// Atlas metadata as written in the pack's `catalog.json`.
#[derive(Deserialize)]
struct Catalog {
    #[serde(default)]
    games: HashMap<String, CatalogPalette>,
}

#[derive(Deserialize)]
struct CatalogPalette {
    atlases: HashMap<String, CatalogAtlas>,
}

#[derive(Deserialize)]
struct CatalogAtlas {
    file: String,
}

/// Row → family metadata as written in the palette's `atlas_manifest.json`.
#[derive(Deserialize)]
struct PaletteManifest {
    #[serde(default)]
    atlases: HashMap<String, ManifestAtlas>,
}

#[derive(Deserialize)]
struct ManifestAtlas {
    #[serde(default)]
    rows: Vec<ManifestRow>,
}

#[derive(Deserialize)]
struct ManifestRow {
    row: u8,
    #[serde(default)]
    family: String,
}

/// A complete RTS tileset pack for one palette, composited as a single RGBA8
/// atlas with per-category metadata.
#[derive(Clone, Debug)]
pub struct TilesetPack {
    palette: String,
    tile_size: u16,
    width: u32,
    height: u32,
    rgba: Vec<u8>,
    /// Categories with their origin inside the combined atlas (slot order).
    categories: Vec<(String, (u32, u32))>,
    /// category → family → row index, sourced from the palette manifest.
    families: HashMap<String, HashMap<String, u8>>,
}

impl TilesetPack {
    /// Load one palette of a tileset pack.
    ///
    /// Reads `<pack_dir>/catalog.json` to find the palette entry and its category
    /// files, parses `<pack_dir>/<palette>/atlas_manifest.json` for row
    /// metadata, then composites every category sheet into the combined atlas.
    pub fn load(pack_dir: &Path, palette: &str) -> Result<TilesetPack> {
        let palette_dir = pack_dir.join(palette);
        let catalog_path = pack_dir.join("catalog.json");
        let catalog_text = std::fs::read_to_string(&catalog_path).with_context(|| {
            format!(
                "failed to read tileset pack catalog {}",
                catalog_path.display()
            )
        })?;
        let catalog: Catalog = serde_json::from_str(&catalog_text)
            .with_context(|| format!("failed to parse {}", catalog_path.display()))?;
        let entry = catalog.games.get(palette).ok_or_else(|| {
            anyhow::anyhow!("pack {} has no palette '{}'", pack_dir.display(), palette)
        })?;
        if entry.atlases.is_empty() {
            bail!(
                "palette '{}' in {} declares no category sheets",
                palette,
                pack_dir.display()
            );
        }

        let manifest_path = palette_dir.join("atlas_manifest.json");
        let families = match std::fs::read_to_string(&manifest_path) {
            Ok(text) => {
                let manifest: PaletteManifest = serde_json::from_str(&text)
                    .with_context(|| format!("failed to parse {}", manifest_path.display()))?;
                palette_families(&manifest)
            }
            Err(_) => HashMap::new(),
        };

        // Deterministic slot order: alphabetical categories, row-major.
        let mut sheets: Vec<(&String, &CatalogAtlas)> = entry.atlases.iter().collect();
        sheets.sort_by_key(|(a, _)| *a);

        let width = SHEET_COLUMNS * SHEET_SIZE;
        let height = SHEET_ROWS * SHEET_SIZE;
        let mut pack = TilesetPack {
            palette: palette.to_string(),
            tile_size: TILE_SIZE,
            width,
            height,
            rgba: vec![0; (width * height * 4) as usize],
            categories: Vec::with_capacity(sheets.len()),
            families,
        };

        for (slot, (category, atlas)) in sheets.into_iter().enumerate() {
            let slot_x = ((slot as u32) % SHEET_COLUMNS) * SHEET_SIZE;
            let slot_y = ((slot as u32) / SHEET_COLUMNS) * SHEET_SIZE;
            let path = palette_dir.join(&atlas.file);
            let image = image::open(&path)
                .with_context(|| format!("failed to load tileset sheet {}", path.display()))?
                .to_rgba8();
            let (sheet_w, sheet_h) = image.dimensions();
            if sheet_w != SHEET_SIZE || sheet_h != SHEET_SIZE {
                bail!(
                    "tileset sheet {} is {}x{}, expected {}x{}",
                    path.display(),
                    sheet_w,
                    sheet_h,
                    SHEET_SIZE,
                    SHEET_SIZE
                );
            }
            blit_sheet(
                &mut pack.rgba,
                width,
                image.as_raw(),
                slot_x,
                slot_y,
                sheet_w,
                sheet_h,
            );
            pack.categories.push((category.clone(), (slot_x, slot_y)));
        }

        Ok(pack)
    }

    /// Palette directory name this pack was loaded from.
    pub fn palette(&self) -> &str {
        &self.palette
    }

    /// Tile edge length in pixels of one grid cell inside a category sheet.
    pub fn tile_size(&self) -> u16 {
        self.tile_size
    }

    /// Combined atlas width in pixels (1024 for the shipped packs).
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Combined atlas height in pixels (512 for the shipped packs).
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Combined atlas RGBA8 bytes, `width * height * 4` long.
    pub fn rgba(&self) -> &[u8] {
        &self.rgba
    }

    /// Categories present in this pack, in compositing (slot) order.
    pub fn categories(&self) -> impl Iterator<Item = &str> {
        self.categories.iter().map(|(name, _)| name.as_str())
    }

    /// Origin (top-left pixel) of a category sheet inside the combined atlas.
    pub fn category_origin(&self, category: &str) -> Option<(u32, u32)> {
        self.categories
            .iter()
            .find(|(name, _)| name == category)
            .map(|(_, origin)| *origin)
    }

    /// Pixel-space tile rectangle `(x, y, w, h)` of one tile in the combined
    /// atlas. `row`/`col` follow the pack's `tile_id = row * 16 + col` layout.
    pub fn tile_px_rect(&self, category: &str, row: u8, col: u8) -> Option<(u32, u32, u32, u32)> {
        let tile_size = u32::from(self.tile_size);
        let grid = GRID;
        if u16::from(row) >= grid || u16::from(col) >= grid {
            return None;
        }
        let (origin_x, origin_y) = self.category_origin(category)?;
        let x = origin_x + u32::from(col) * tile_size;
        let y = origin_y + u32::from(row) * tile_size;
        Some((x, y, tile_size, tile_size))
    }

    /// Row index of a semantic family inside a category (e.g. `"terrain"` +
    /// `"snow"`), as declared by the palette's atlas manifest.
    pub fn family_row(&self, category: &str, family: &str) -> Option<u8> {
        self.families.get(category)?.get(family).copied()
    }

    /// All semantic families of one category as `(name, row)` pairs.
    pub fn families_of(&self, category: &str) -> Vec<(&str, u8)> {
        self.families
            .get(category)
            .map(|m| m.iter().map(|(n, &r)| (n.as_str(), r)).collect())
            .unwrap_or_default()
    }

    /// Load a tileset pack built natively from the shipped Remaster textures.
    ///
    /// Reads the Steam `TEXTURES_*_SRGB.MEG` through
    /// [`crate::original_assets::OriginalAssets`], decodes each template icon of
    /// `theater` to RGBA, and composites the icons into the same combined-atlas
    /// layout [`TilesetPack::load`] produces — so every consumer (map editor,
    /// launcher) renders the original art with no other code change.
    ///
    /// Family rows follow the semantic families the draw code queries
    /// (`terrain.grass`, `water_edges.deep_water`, ...); each family's column
    /// holds the theater's own art.
    ///
    /// Returns `None` when the Steam install is absent or a member fails to
    /// decode.
    pub fn load_original(
        game: crate::original_assets::Game,
        theater: crate::original_assets::Theater,
    ) -> Option<TilesetPack> {
        let assets = crate::original_assets::OriginalAssets::open_default(game).ok()?;
        build_original_pack(&assets, theater)
    }
}

/// Copy one sheet's RGBA rows into the combined atlas at `(slot_x, slot_y)`.
fn blit_sheet(
    atlas: &mut [u8],
    atlas_width: u32,
    sheet: &[u8],
    slot_x: u32,
    slot_y: u32,
    sheet_w: u32,
    sheet_h: u32,
) {
    let atlas_stride = atlas_width as usize * 4;
    let bytes_per_sheet_row = sheet_w as usize * 4;
    for row in 0..sheet_h as usize {
        let src = row * bytes_per_sheet_row;
        let dst = (slot_y as usize + row) * atlas_stride + slot_x as usize * 4;
        atlas[dst..dst + bytes_per_sheet_row]
            .copy_from_slice(&sheet[src..src + bytes_per_sheet_row]);
    }
}

/// Flatten `category → family → row` out of a palette manifest.
fn palette_families(manifest: &PaletteManifest) -> HashMap<String, HashMap<String, u8>> {
    let mut families = HashMap::new();
    for (category, atlas) in &manifest.atlases {
        let mut rows = HashMap::new();
        for row in &atlas.rows {
            if !row.family.is_empty() {
                rows.insert(row.family.clone(), row.row);
            }
        }
        if !rows.is_empty() {
            families.insert(category.clone(), rows);
        }
    }
    families
}

/// Directory of the in-repo RTS 16×16 pack, resolved without CLI flags.
///
/// Tries `../assets/packs/rts_16x16_tileset` relative to this crate (which is
/// the workspace layout for every member crate), then
/// `assets/packs/rts_16x16_tileset` relative to the current directory.
pub fn default_pack_dir() -> Option<PathBuf> {
    let from_crate = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets")
        .join("packs")
        .join("rts_16x16_tileset");
    if from_crate.is_dir() {
        return Some(from_crate);
    }
    let from_cwd = PathBuf::from("assets")
        .join("packs")
        .join("rts_16x16_tileset");
    if from_cwd.is_dir() {
        return Some(from_cwd);
    }
    None
}

/// Composite the original theater tiles into a `TilesetPack`.
///
/// Template families and their semantic rows:
/// - `terrain`: CLEAR1 (grass), P01 (dirt/rough/mud/... patches), B1 (rock)
/// - `water_edges`: W1 (deep), SH1 icons (shore)
/// - `transitions`: S01.. (rock slopes), rendered from their slope art
/// - `cliffs`: RV01.. river banks (elevation look-alikes)
/// - `roads_bridges`: D01.. roads
fn build_original_pack(
    assets: &crate::original_assets::OriginalAssets,
    theater: crate::original_assets::Theater,
) -> Option<TilesetPack> {
    /// (category, family, template, manifest row, icon index)
    // Base ground comes from the PTT patch family: the original engine draws a
    // solid base under `CLEAR1`, which is itself a translucent clear-overlay
    // (28% coverage). See `pack_slots` for the verified family list.
    const SLOTS: &[(&str, &str, &str, u8, u32)] = &[
        ("terrain", "grass", "P01", 0, 0),
        ("terrain", "dirt", "P03", 1, 0),
        ("terrain", "sand", "P04", 2, 0),
        ("terrain", "rock", "P07", 4, 0),
        ("terrain", "rough", "P08", 9, 0),
        ("terrain", "mud", "P13", 7, 0),
        ("terrain", "concrete", "P14", 14, 0),
        ("water_edges", "deep_water", "W1", 0, 0),
        ("water_edges", "shallow_water", "W2", 1, 0),
        ("water_edges", "shore_north", "SH1", 2, 0),
        ("water_edges", "shore_east", "SH1", 3, 1),
        ("water_edges", "shore_south", "SH1", 4, 2),
        ("water_edges", "shore_west", "SH1", 5, 3),
        ("water_edges", "shore_nw", "SH1", 6, 4),
        ("water_edges", "shore_ne", "SH1", 7, 5),
        ("water_edges", "shore_sw", "SH1", 8, 6),
        ("water_edges", "shore_se", "SH1", 9, 7),
        ("transitions", "grass_rock", "S01", 2, 0),
        ("transitions", "grass_sand", "S02", 1, 0),
        ("transitions", "dirt_rock", "S03", 6, 0),
        ("transitions", "sand_rock", "S04", 9, 0),
        ("roads_bridges", "paved_ns", "D01", 0, 0),
        ("roads_bridges", "paved_ew", "D02", 1, 0),
        ("cliffs", "cliff_interior", "RV01", 8, 0),
    ];

    // Slot layout must be deterministic: group by category, alphabetical.
    let mut cats: Vec<&str> = SLOTS.iter().map(|s| s.0).collect();
    cats.sort_unstable();
    cats.dedup();

    let width = SHEET_COLUMNS * SHEET_SIZE;
    let height = SHEET_ROWS * SHEET_SIZE;
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    let mut categories: Vec<(String, (u32, u32))> = Vec::new();
    let mut families: HashMap<String, HashMap<String, u8>> = HashMap::new();

    for (slot, category) in cats.iter().enumerate() {
        let slot_x = ((slot as u32) % SHEET_COLUMNS) * SHEET_SIZE;
        let slot_y = ((slot as u32) / SHEET_COLUMNS) * SHEET_SIZE;
        categories.push(((*category).to_string(), (slot_x, slot_y)));

        let rows: Vec<&(&str, &str, &str, u8, u32)> =
            SLOTS.iter().filter(|s| s.0 == *category).collect();
        let mut fam_rows: HashMap<String, u8> = HashMap::new();
        for (_cat, family, template, row, icon) in rows {
            let (pixels, w, _h) = match assets.icon_rgba(template, *icon, theater) {
                Ok(v) => v,
                Err(_) => continue,
            };
            // Original families have no per-variant art: one icon stands for the
            // whole family. Fill every column of the row with it, because both
            // the editor and launcher pick the column as a deterministic
            // 0..15 variant and expect art there.
            for column in 0..SHEET_TILES {
                blit_original_tile(
                    &mut rgba,
                    width,
                    &pixels,
                    w,
                    slot_x + column * u32::from(TILE_SIZE),
                    slot_y + u32::from(*row) * u32::from(TILE_SIZE),
                );
            }
            fam_rows.insert((*family).to_string(), *row);
        }
        if !fam_rows.is_empty() {
            families.insert((*category).to_string(), fam_rows);
        }
    }

    Some(TilesetPack {
        palette: "original".to_string(),
        tile_size: TILE_SIZE,
        width,
        height,
        rgba,
        categories,
        families,
    })
}

/// Downsample one shipped icon (64 or 128 px) into a 16x16 cell.
///
/// The original icons are sparse: only ~1/3 of the pixels carry texture, the
/// rest is transparent gap. A naive every-8th-pixel pick lands on gaps and the
/// cell comes out empty, so each cell samples every source pixel in its block
/// and averages the opaque ones, falling back to the first opaque pixel seen.
fn blit_original_tile(
    atlas: &mut [u8],
    atlas_width: u32,
    icon: &[u8],
    icon_side: u32,
    dst_x: u32,
    dst_y: u32,
) {
    let step = icon_side / u32::from(TILE_SIZE);
    if step == 0 {
        return;
    }
    for ty in 0..u32::from(TILE_SIZE) {
        for tx in 0..u32::from(TILE_SIZE) {
            let mut sum = [0u32; 4];
            let mut opaque = 0usize;
            let mut last_opaque = 0usize;
            for sy in (ty * step)..((ty + 1) * step) {
                for sx in (tx * step)..((tx + 1) * step) {
                    let o = ((sy * icon_side + sx) * 4) as usize;
                    if o + 4 > icon.len() {
                        continue;
                    }
                    last_opaque = o;
                    if icon[o + 3] > 8 {
                        for c in 0..4 {
                            sum[c] += u32::from(icon[o + c]);
                        }
                        opaque += 1;
                    }
                }
            }
            let d = ((dst_y + ty) * atlas_width + dst_x + tx) as usize * 4;
            if d + 4 > atlas.len() {
                continue;
            }
            match NonZeroUsize::new(opaque) {
                Some(count) => {
                    for c in 0..3 {
                        atlas[d + c] = u8::try_from(sum[c] / count.get() as u32).unwrap_or(u8::MAX);
                    }
                    atlas[d + 3] = u8::try_from(sum[3] / count.get() as u32).unwrap_or(u8::MAX);
                }
                None => {
                    // No opaque sample in the block: leave the cell transparent
                    // (the last sampled pixel's colour is kept unlit).
                    if last_opaque + 4 <= icon.len() {
                        atlas[d..d + 4].copy_from_slice(&icon[last_opaque..last_opaque + 4]);
                        atlas[d + 3] = 0;
                    }
                }
            }
        }
    }
}

/// Load the default pack with process-wide environment overrides.
///
/// `CNC_TILESET_DIR` overrides the pack directory and `CNC_TILESET_PALETTE`
/// overrides the palette (which otherwise defaults to `default_palette`).
/// Returns `None` when the pack is missing or unreadable; callers fall back to
/// flat-coloured terrain rendering in that case.
pub fn load_default(default_palette: &str) -> Option<TilesetPack> {
    // `CNC_ORIGINAL_TILES=1` composes the shipped Remaster art for the
    // palette's game (ra_inspired → Red Alert, everything else → Tiberian
    // Dawn) instead of the generated pack.
    //
    // Red Alert's art uses different member naming and template families, so the
    // original composite only builds when its semantic families actually
    // resolve; otherwise this falls through to the generated pack.
    let original = std::env::var("CNC_ORIGINAL_TILES").is_ok_and(|v| v == "1");
    if original {
        use crate::original_assets::{Game, Theater};
        let (game, required) = if default_palette.starts_with("ra") {
            (Game::RedAlert, ["grass", "deep_water"])
        } else {
            (Game::TiberianDawn, ["grass", "deep_water"])
        };
        if let Some(pack) = TilesetPack::load_original(game, Theater::Temperate) {
            let complete = required
                .iter()
                .any(|f| pack.family_row("terrain", f).is_some())
                && required
                    .iter()
                    .any(|f| pack.family_row("water_edges", f).is_some());
            if complete {
                return Some(pack);
            }
        }
    }
    let dir = match std::env::var("CNC_TILESET_DIR") {
        Ok(dir) if !dir.trim().is_empty() => PathBuf::from(dir),
        _ => default_pack_dir()?,
    };
    let palette = match std::env::var("CNC_TILESET_PALETTE") {
        Ok(palette) if !palette.trim().is_empty() => palette,
        _ => default_palette.to_string(),
    };
    TilesetPack::load(&dir, &palette).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Paint one 16×16 tile inside a 256×256 RGBA image.
    fn paint_tile(image: &mut image::RgbaImage, color: [u8; 4], row: u8, col: u8) {
        for y in u32::from(row) * 16..u32::from(row) * 16 + 16 {
            for x in u32::from(col) * 16..u32::from(col) * 16 + 16 {
                image.put_pixel(x, y, image::Rgba(color));
            }
        }
    }

    fn write_mini_pack(dir: &Path) -> Result<()> {
        let palette_dir = dir.join("mini_palette");
        std::fs::create_dir_all(&palette_dir)?;

        let mut alpha_sheet =
            image::RgbaImage::from_pixel(256, 256, image::Rgba([40, 40, 40, 255]));
        paint_tile(&mut alpha_sheet, [255, 0, 0, 255], 3, 5);
        alpha_sheet.save(palette_dir.join("alpha.png"))?;

        let mut beta_sheet = image::RgbaImage::from_pixel(256, 256, image::Rgba([90, 90, 90, 255]));
        paint_tile(&mut beta_sheet, [0, 255, 0, 128], 1, 0);
        beta_sheet.save(palette_dir.join("beta.png"))?;

        let manifest = json!({
            "display_name": "Mini",
            "atlases": {
                "alpha": {
                    "rows": [
                        { "row": 0, "family": "grass", "label": "Grass" },
                        { "row": 1, "family": "dirt", "label": "Dirt" }
                    ]
                },
                "beta": {
                    "rows": [ { "row": 0, "family": "deep_water", "label": "Water" } ]
                }
            }
        });
        std::fs::write(
            palette_dir.join("atlas_manifest.json"),
            serde_json::to_string(&manifest)?,
        )?;

        let catalog = json!({
            "title": "Mini pack",
            "tile_size": [16, 16],
            "atlas_size": [256, 256],
            "grid": [16, 16],
            "games": {
                "mini_palette": {
                    "display_name": "Mini",
                    "atlases": {
                        "alpha": {
                            "file": "alpha.png",
                            "mode": "RGB",
                            "size_px": [256, 256],
                            "grid": [16, 16],
                            "tile_size_px": [16, 16],
                            "tile_count": 256,
                            "rows": []
                        },
                        "beta": {
                            "file": "beta.png",
                            "mode": "RGBA",
                            "size_px": [256, 256],
                            "grid": [16, 16],
                            "tile_size_px": [16, 16],
                            "tile_count": 256,
                            "rows": []
                        }
                    }
                }
            }
        });
        std::fs::write(dir.join("catalog.json"), serde_json::to_string(&catalog)?)?;
        Ok(())
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "cnc_asset_tileset_{}_{}_{}",
            std::process::id(),
            tag,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn synthetic_pack_loads_and_addresses_tiles() -> Result<()> {
        let dir = temp_dir("synthetic");
        let _guard = TempDirGuard(&dir);
        write_mini_pack(&dir)?;

        let pack = TilesetPack::load(&dir, "mini_palette")?;
        assert_eq!(pack.palette(), "mini_palette");
        assert_eq!(pack.tile_size(), 16);
        assert_eq!(pack.width(), 1024);
        assert_eq!(pack.height(), 512);
        assert_eq!(pack.rgba().len(), 1024 * 512 * 4);
        // Alphabetical category order, row-major slots.
        assert_eq!(pack.categories().collect::<Vec<_>>(), vec!["alpha", "beta"]);
        assert_eq!(pack.category_origin("alpha"), Some((0, 0)));
        assert_eq!(pack.category_origin("beta"), Some((256, 0)));
        assert_eq!(pack.category_origin("missing"), None);

        // tile_id = row*16 + col, tile 16px.
        assert_eq!(pack.tile_px_rect("alpha", 3, 5), Some((80, 48, 16, 16)));
        assert_eq!(pack.tile_px_rect("beta", 1, 0), Some((256, 16, 16, 16)));
        assert_eq!(pack.tile_px_rect("missing", 0, 0), None);
        assert_eq!(pack.tile_px_rect("alpha", 16, 0), None);
        assert_eq!(pack.tile_px_rect("alpha", 0, 16), None);

        // The painted tiles land exactly at their declared rects.
        let rgba = pack.rgba();
        let at = |x: u32, y: u32| {
            let index = (y as usize * pack.width() as usize + x as usize) * 4;
            [
                rgba[index],
                rgba[index + 1],
                rgba[index + 2],
                rgba[index + 3],
            ]
        };
        assert_eq!(at(80, 48), [255, 0, 0, 255], "alpha tile (3,5) is red");
        assert_eq!(
            at(250, 250),
            [40, 40, 40, 255],
            "sheet filler is carried over"
        );
        assert_eq!(at(256, 16), [0, 255, 0, 128], "beta tile keeps its alpha");

        // Family metadata is read from the palette manifest.
        assert_eq!(pack.family_row("alpha", "grass"), Some(0));
        assert_eq!(pack.family_row("alpha", "dirt"), Some(1));
        assert_eq!(pack.family_row("beta", "deep_water"), Some(0));
        assert_eq!(pack.family_row("beta", "grass"), None);
        Ok(())
    }

    #[test]
    fn synthetic_pack_rejects_unknown_palette() -> Result<()> {
        let dir = temp_dir("unknown-palette");
        let _guard = TempDirGuard(&dir);
        write_mini_pack(&dir)?;
        let error = TilesetPack::load(&dir, "no_such_palette");
        assert!(error.is_err(), "unknown palette must fail, not be ignored");
        Ok(())
    }

    #[test]
    fn real_pack_composites_all_eight_categories() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("assets")
            .join("packs")
            .join("rts_16x16_tileset");
        if !dir.is_dir() {
            eprintln!("skipping: in-repo pack not present at {}", dir.display());
            return;
        }
        for palette in ["ra_inspired", "td_inspired"] {
            let pack = TilesetPack::load(&dir, palette).expect("shipped palettes must load");
            assert_eq!(pack.palette(), palette);
            assert_eq!(pack.width(), 1024);
            assert_eq!(pack.height(), 512);
            let categories = pack.categories().count();
            assert_eq!(categories, 8, "{palette} must expose all eight categories");
            assert_eq!(pack.category_origin("terrain"), Some((0, 256)));
            assert!(pack.category_origin("water_edges").is_some());
            assert!(pack.rgba().iter().any(|byte| *byte != 0));
        }
    }

    /// Removes a temp directory when the test scope ends.
    struct TempDirGuard<'a>(&'a Path);

    impl Drop for TempDirGuard<'_> {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(self.0);
        }
    }
}
