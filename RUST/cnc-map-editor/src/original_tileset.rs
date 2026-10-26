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
use cnc_core::Rect;

/// Combined-atlas geometry (matches `TilesetPack`: 16px tiles, 4x2 sheets).
const TILE: u32 = 16;
const SHEET: u32 = 256;
const COLUMNS: u32 = 4;
const ROWS: u32 = 2;

/// (category, family, original template) triples in slot order — the same
/// semantic rows `TilesetPack::load_original` composites in `cnc-asset`.
const FAMILIES: &[(&str, &str, &str)] = &[
    ("terrain", "grass", "P01"),
    ("terrain", "dirt", "P03"),
    ("terrain", "sand", "P04"),
    ("terrain", "rock", "P07"),
    ("terrain", "rough", "P08"),
    ("terrain", "mud", "P13"),
    ("terrain", "concrete", "P14"),
    ("water_edges", "deep_water", "W1"),
    ("water_edges", "shallow_water", "W2"),
    ("water_edges", "shore_north", "SH1"),
    ("water_edges", "shore_east", "SH2"),
    ("water_edges", "shore_south", "SH3"),
    ("water_edges", "shore_west", "SH4"),
    ("water_edges", "shore_nw", "SH5"),
    ("water_edges", "shore_ne", "SH6"),
    ("water_edges", "shore_sw", "SH7"),
    ("water_edges", "shore_se", "SH8"),
    ("transitions", "grass_rock", "S01"),
    ("transitions", "grass_sand", "S02"),
    ("transitions", "dirt_rock", "S03"),
    ("transitions", "sand_rock", "S04"),
    ("roads_bridges", "paved_ns", "D01"),
    ("roads_bridges", "paved_ew", "D02"),
    ("cliffs", "cliff_interior", "RV01"),
];

/// Combined original atlas plus the family row map.
pub struct OriginalTileset {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    /// category → (family, row)
    rows: std::collections::HashMap<(String, String), u8>,
    /// category → sheet origin (x, y)
    origins: std::collections::HashMap<String, (u32, u32)>,
}

impl OriginalTileset {
    /// Composite one theater of the original terrain art. Returns `None` when
    /// the Steam install is absent or every member fails to decode.
    pub fn load(theater: Theater) -> Option<Self> {
        Self::load_game(Game::TiberianDawn, theater)
    }

    /// Load one game's original art for a theater.
    pub fn load_game(game: Game, theater: Theater) -> Option<Self> {
        let assets = OriginalAssets::open_default(game).ok()?;
        Self::from_assets(&assets, theater)
    }

    /// Composite from an already-open archive.
    pub fn from_assets(assets: &OriginalAssets, theater: Theater) -> Option<Self> {
        // Deterministic slot order: categories in first-appearance order.
        let mut cats: Vec<&str> = Vec::new();
        for (cat, _, _) in FAMILIES {
            if !cats.contains(cat) {
                cats.push(cat);
            }
        }

        let width = COLUMNS * SHEET;
        let height = ROWS * SHEET;
        let mut rgba = vec![0u8; (width * height * 4) as usize];
        let mut rows = std::collections::HashMap::new();
        let mut origins = std::collections::HashMap::new();

        for (slot, cat) in cats.iter().enumerate() {
            let ox = ((slot as u32) % COLUMNS) * SHEET;
            let oy = (slot as u32 / COLUMNS) * SHEET;
            origins.insert((*cat).to_string(), (ox, oy));
            for (row, (_, family, template)) in
                FAMILIES.iter().filter(|(c, _, _)| c == cat).enumerate()
            {
                let (pixels, _w, _h) = assets.icon_rgba(template, 0, theater).ok()?;
                blit_icon(&mut rgba, width, &pixels, _w, ox, oy + row as u32 * TILE);
                rows.insert(((*cat).to_string(), (*family).to_string()), row as u8);
            }
        }

        Some(Self {
            rgba,
            width,
            height,
            rows,
            origins,
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

    /// Atlas rect for a semantic tile id written as `"<category>/<family>"`.
    pub fn sprite_rect(&self, sprite_id: &str) -> Option<Rect> {
        let (cat, fam) = sprite_id.split_once('/')?;
        let row = *self.rows.get(&(cat.to_string(), fam.to_string()))?;
        let (ox, oy) = self.origins.get(cat)?;
        Some(Rect::new(
            (*ox) as i32,
            (oy + u32::from(row) * TILE) as i32,
            TILE as i32,
            TILE as i32,
        ))
    }

    /// Per-cell tile pick for the plain-terrain chain. Unlike the generated
    /// pack there are no shore/cliff mask families encoded per cell here; the
    /// editor's mask logic still runs against the pack's `cell_draw`, so this
    /// adapter serves base terrain and lets shores degrade to water rows.
    pub fn cell_draw(&self, terrain_id: u8) -> Option<CellTile> {
        use super::generator::TerrainType;
        let (cat, fam) = match TerrainType::from_id(terrain_id)? {
            TerrainType::Grass | TerrainType::ForestFloor => ("terrain", "grass"),
            TerrainType::Water => ("water_edges", "deep_water"),
            TerrainType::Sand => ("terrain", "sand"),
            TerrainType::Dirt | TerrainType::Mud | TerrainType::Swamp => ("terrain", "dirt"),
            TerrainType::Rock | TerrainType::Crater => ("terrain", "rock"),
            TerrainType::Snow => ("water_edges", "shallow_water"),
            TerrainType::Rough | TerrainType::Scorched => ("terrain", "rough"),
            TerrainType::Ice => ("water_edges", "shore_west"),
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

/// Downsample a shipped icon (64 or 128 px) into one 16x16 cell.
fn blit_icon(
    atlas: &mut [u8],
    atlas_width: u32,
    icon: &[u8],
    icon_side: u32,
    dst_x: u32,
    dst_y: u32,
) {
    let step = icon_side / TILE;
    if step == 0 {
        return;
    }
    for ty in 0..TILE {
        for tx in 0..TILE {
            let sx = tx * step;
            let sy = ty * step;
            let o = ((sy * icon_side + sx) * 4) as usize;
            let d = ((dst_y + ty) * atlas_width + dst_x + tx) as usize * 4;
            if o + 4 <= icon.len() && d + 4 <= atlas.len() {
                atlas[d..d + 4].copy_from_slice(&icon[o..o + 4]);
            }
        }
    }
}
