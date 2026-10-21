//! Visual-QA preview sheets: contact sheets, the unit-condition review
//! grid and the composed multi-tile test map.
//!
//! Logic-faithful port of the preview section of the Python `build()`
//! (lines 770–866). Sheets use a (28, 28, 30) background, nearest-neighbor
//! upscaled tiles composited with alpha and a (80, 80, 84) per-cell grid
//! outline, saved as RGB PNGs. Labels use the crate's built-in [`font`]
//! instead of Pillow's default bitmap font.

use std::collections::HashMap;
use std::path::Path;

use image::{Rgb, RgbImage};

use super::super::canvas::Canvas;
use super::super::font::draw_text;
use super::super::{color, Rgba};

const SHEET_BG: Rgba = [28, 28, 30, 255];
const GRID: Rgba = [80, 80, 84, 255];
const LABEL: Rgba = [235, 235, 235, 255];

/// A sprite slot on a sheet.
pub struct SheetItem<'a> {
    pub image: &'a Canvas,
}

/// Nearest-neighbor upscale of a 32×32 tile (alpha preserved).
fn upscale(tile: &Canvas, scale: u32) -> Canvas {
    let w = tile.width() * scale;
    let h = tile.height() * scale;
    let mut out = Canvas::transparent(w, h);
    for y in 0..h {
        for x in 0..w {
            if let Some(px) = tile.pixel((x / scale) as i32, (y / scale) as i32) {
                out.set_pixel(x as i32, y as i32, px);
            }
        }
    }
    out
}

fn outline(sheet: &mut Canvas, x: u32, y: u32, w: u32, h: u32) {
    for i in 0..w {
        sheet.set_pixel((x + i) as i32, y as i32, GRID);
        sheet.set_pixel((x + i) as i32, (y + h - 1) as i32, GRID);
    }
    for j in 0..h {
        sheet.set_pixel(x as i32, (y + j) as i32, GRID);
        sheet.set_pixel((x + w - 1) as i32, (y + j) as i32, GRID);
    }
}

/// Drop alpha (composited over the opaque sheet background already).
fn save_rgb(sheet: &Canvas, path: &Path) -> Result<(), image::ImageError> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(image::ImageError::IoError)?;
    }
    let rgba = sheet.as_rgba();
    let (w, h) = rgba.dimensions();
    let mut rgb = RgbImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let px = rgba.get_pixel(x, y).0;
            rgb.put_pixel(x, y, Rgb([px[0], px[1], px[2]]));
        }
    }
    rgb.save(path)?;
    Ok(())
}

/// Generic contact sheet: `cols` tiles per row, nearest-upscaled `scale`×.
pub fn save_contact_sheet(
    path: &Path,
    items: &[SheetItem<'_>],
    cols: usize,
    scale: u32,
) -> Result<(), image::ImageError> {
    let cols = cols.max(1);
    let rows = items.len().div_ceil(cols).max(1);
    let cell = 32 * scale;
    let mut sheet = Canvas::new_filled(cols as u32 * cell, rows as u32 * cell, SHEET_BG);
    for (idx, item) in items.iter().enumerate() {
        let x = (idx % cols) as u32 * cell;
        let y = (idx / cols) as u32 * cell;
        let tile = upscale(item.image, scale);
        sheet.alpha_composite(&tile, (x as i32, y as i32));
        outline(&mut sheet, x, y, cell, cell);
    }
    save_rgb(&sheet, path)
}

/// Unit-damage review grid: one row per faction/unit, one column per
/// condition, labels drawn with the built-in font. Uses facing 2 (east) so
/// damage marks read consistently across rows.
pub fn save_unit_condition_review(
    path: &Path,
    lookup: &HashMap<(String, String, u8, String), &Canvas>,
    unit_order: &[&str],
    factions: &[&str],
    conditions: &[&str],
    scale: u32,
) -> Result<(), image::ImageError> {
    let label_width = 112u32;
    let header_height = 24u32;
    let cell = 32 * scale;
    let rows = factions.len() * unit_order.len();
    let mut sheet = Canvas::new_filled(
        label_width + conditions.len() as u32 * cell,
        header_height + rows as u32 * cell,
        SHEET_BG,
    );
    for (col, condition) in conditions.iter().enumerate() {
        draw_text(
            &mut sheet,
            label_width as i32 + col as i32 * cell as i32 + 5,
            5,
            &condition.to_uppercase(),
            LABEL,
        );
    }
    let mut row_index = 0usize;
    for faction in factions {
        for unit in unit_order {
            let y = header_height + row_index as u32 * cell;
            draw_text(
                &mut sheet,
                5,
                (y + cell / 2 - 4) as i32,
                // Python: f"{faction.upper()} / {unit}" — unit stays
                // lowercase in the string; the 5×7 font renders it via
                // uppercase glyph forms (documented charset deviation).
                &format!("{} / {}", faction.to_uppercase(), unit),
                LABEL,
            );
            for (col, condition) in conditions.iter().enumerate() {
                let x = label_width + col as u32 * cell;
                if let Some(im) = lookup.get(&(
                    faction.to_string(),
                    unit.to_string(),
                    2u8,
                    condition.to_string(),
                )) {
                    let tile = upscale(im, scale);
                    sheet.alpha_composite(&tile, (x as i32, y as i32));
                }
                outline(&mut sheet, x, y, cell, cell);
            }
            row_index += 1;
        }
    }
    save_rgb(&sheet, path)
}

/// Inputs the composed test map needs from the build.
pub struct ComposedMapInputs<'a> {
    /// terrain kind -> variant-0 base tile.
    pub base_tiles: HashMap<&'a str, &'a Canvas>,
    /// road mask -> road tile.
    pub road_tiles: HashMap<u8, &'a Canvas>,
    /// shoreline mask -> shore tile (mask 2 = land on east edge).
    pub shore_tiles: HashMap<u8, &'a Canvas>,
}

fn is_road_rgb(px: Rgba) -> bool {
    let rgb = [px[0], px[1], px[2]];
    rgb == color("road")[..3] || rgb == color("road_dark")[..3] || rgb == color("road_light")[..3]
}

/// Terrain kind at a test-map cell (mirrors the composed-map formula).
fn terrain_at(tx: i32, ty: i32) -> &'static str {
    if tx >= 9 && (2..=6).contains(&ty) {
        "water"
    } else if (ty == 3 || ty == 4) && tx < 9 {
        "dirt"
    } else {
        "grass"
    }
}

/// Road-position predicate for the 12×8 test map: the dirt road band rows
/// plus the north junction stub. Positional only — the Python port evaluates
/// neighbor cells purely by coordinates, even across the water boundary.
fn road_position(tx: i32, ty: i32) -> bool {
    (3..=4).contains(&ty) || ((4..=5).contains(&tx) && (ty == 2 || ty == 5))
}

/// Render the 12×8 multi-tile continuity map and save it at 3× scale.
///
/// Visual QA artifact only — not a claim every terrain transition is
/// seam-free (same caveat as the Python pack).
pub fn save_composed_map(
    path: &Path,
    inputs: &ComposedMapInputs<'_>,
) -> Result<(), image::ImageError> {
    let (cols, rows, tile) = (12i32, 8i32, 32i32);
    let mut map = Canvas::new_filled((cols * tile) as u32, (rows * tile) as u32, [0, 0, 0, 255]);
    for ty in 0..rows {
        for tx in 0..cols {
            let terrain = terrain_at(tx, ty);
            let base = *inputs.base_tiles.get(terrain).expect("base tile");
            let mut tile_img = base.clone();
            // Python places road overlays only off water, but computes the
            // connection mask from positional neighbors alone.
            if terrain != "water" && road_position(tx, ty) {
                let mut mask = 0u8;
                if ty > 0 && road_position(tx, ty - 1) {
                    mask |= 1;
                }
                if tx < cols - 1 && road_position(tx + 1, ty) {
                    mask |= 2;
                }
                if ty < rows - 1 && road_position(tx, ty + 1) {
                    mask |= 4;
                }
                if tx > 0 && road_position(tx - 1, ty) {
                    mask |= 8;
                }
                if let Some(&road) = inputs.road_tiles.get(&mask) {
                    // Isolate road material so the dirt roadbed does not
                    // replace surrounding terrain with square dirt tiles.
                    let mut iso = Canvas::transparent(32, 32);
                    for y in 0..32 {
                        for x in 0..32 {
                            if let Some(px) = road.pixel(x, y) {
                                if is_road_rgb(px) {
                                    iso.set_pixel(x, y, px);
                                }
                            }
                        }
                    }
                    tile_img.alpha_composite(&iso, (0, 0));
                }
            }
            if terrain == "water" && tx == 8 {
                // Preserved from Python: this branch is unreachable there too
                // (tx==8 is never classified water); kept for 1:1 parity.
                if let Some(&shore) = inputs.shore_tiles.get(&2u8) {
                    tile_img.alpha_composite(shore, (0, 0));
                }
            }
            map.alpha_composite(&tile_img, (tx * tile, ty * tile));
        }
    }
    // 3× nearest upscale before saving, matching the Python preview.
    let big = upscale(&map, 3);
    save_rgb(&big, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn red_tile() -> Canvas {
        Canvas::new_filled(32, 32, [255, 0, 0, 255])
    }

    #[test]
    fn contact_sheet_writes_expected_size() {
        let dir = std::env::temp_dir().join(format!("cnc-procgen-sheet-{}", std::process::id()));
        let path = dir.join("sheet.png");
        let tile = red_tile();
        let items: Vec<SheetItem> = (0..5).map(|_| SheetItem { image: &tile }).collect();
        // 5 items, 3 cols -> 2 rows of 64px cells at scale 2.
        save_contact_sheet(&path, &items, 3, 2).unwrap();
        let img = image::open(&path).unwrap().into_rgb8();
        assert_eq!(img.dimensions(), (3 * 64, 2 * 64));
        // The grid outline is stroked over the tile borders (as in Python),
        // so sample just inside the first cell for the upscaled red.
        assert_eq!(img.get_pixel(1, 1), &Rgb([255, 0, 0]));
        assert_eq!(img.get_pixel(63, 0), &Rgb([80, 80, 84])); // grid border
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unit_review_sheet_layout() {
        let dir = std::env::temp_dir().join(format!("cnc-procgen-review-{}", std::process::id()));
        let path = dir.join("review.png");
        let tile = red_tile();
        let mut lookup = HashMap::new();
        lookup.insert(
            (
                "gdi".to_string(),
                "tank".to_string(),
                2u8,
                "intact".to_string(),
            ),
            &tile,
        );
        save_unit_condition_review(
            &path,
            &lookup,
            &["tank"],
            &["gdi", "nod"],
            &["intact", "damaged"],
            1,
        )
        .unwrap();
        let img = image::open(&path).unwrap().into_rgb8();
        assert_eq!(img.dimensions(), (112 + 2 * 32, 24 + 2 * 32));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn composed_map_renders_full_size() {
        let dir = std::env::temp_dir().join(format!("cnc-procgen-map-{}", std::process::id()));
        let path = dir.join("map.png");
        let base_grass = Canvas::new_filled(32, 32, [82, 126, 57, 255]);
        let base_dirt = Canvas::new_filled(32, 32, [139, 99, 59, 255]);
        let base_water = Canvas::new_filled(32, 32, [39, 91, 139, 255]);
        let road = Canvas::new_filled(32, 32, color("road"));
        let shore = Canvas::new_filled(32, 32, color("sand"));
        let mut bases = HashMap::new();
        bases.insert("grass", &base_grass);
        bases.insert("dirt", &base_dirt);
        bases.insert("water", &base_water);
        let mut roads = HashMap::new();
        roads.insert(15u8, &road);
        let mut shores = HashMap::new();
        shores.insert(2u8, &shore);
        save_composed_map(
            &path,
            &ComposedMapInputs {
                base_tiles: bases,
                road_tiles: roads,
                shore_tiles: shores,
            },
        )
        .unwrap();
        let img = image::open(&path).unwrap().into_rgb8();
        assert_eq!(img.dimensions(), (12 * 32 * 3, 8 * 32 * 3));
        std::fs::remove_dir_all(&dir).ok();
    }
}
