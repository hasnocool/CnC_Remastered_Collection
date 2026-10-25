//! Typed, read-only loader for the generated C&C remaster-style 32px asset
//! pack `assets/packs/cnc32_assets_v6_2_1` (manifest schema 1.3.0,
//! generator 6.2.1).
//!
//! This module exists primarily to serve the pack's engine-test suite, which
//! lives in `<pack>/engine_tests/rust/` and is bridged into this crate's test
//! tree by `tests/cnc32_engine_tests.rs` (so `cargo test -p cnc-asset` runs it
//! directly against the checked-in pack). It deliberately models a minimal,
//! strict subset of the two JSON documents — `manifest.json` and
//! `ui/ui_asset_index_v6_2.json` — and ignores every by-design field it does
//! not need, so pack revisions with extra fields keep parsing. No sprite,
//! atlas, or manifest data is rewritten or copied; everything is resolved
//! relative to the pack root.
//!
//! Pack discovery follows the same convention as [`crate::tileset_pack`]:
//! an environment override first, then a workspace-relative path from this
//! crate, then a CWD-relative fallback.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Re-exported so pack-bridged engine tests (which only link against this
/// crate, not its private dependencies) can parse auxiliary JSON metadata.
pub use serde_json;

/// `manifest.json` header fields, schema 1.3.0.
#[derive(Debug, Clone, Deserialize)]
pub struct Cnc32Manifest {
    /// `"1.3.0"` for this pack.
    pub schema_version: String,
    /// Generator version, e.g. `"6.2.1"` (starts with `"6.2"`).
    pub generator_version: String,
    /// Human-readable baseline tag, e.g. `"cnc32-v5.6.0-validation-baseline"`.
    pub seed: String,
    /// Base tile edge length in pixels (`32`).
    pub tile_size: u32,
    /// Directional bit orders used to pack `mask_bits` into `mask`.
    pub mask_convention: MaskConvention,
    /// Every generated sprite entry.
    #[serde(default)]
    pub sprites: Vec<SpriteEntry>,
}

/// Bit-packing conventions for terrain/road/shore masks.
#[derive(Debug, Clone, Deserialize)]
pub struct MaskConvention {
    /// Cardinal order (`[N, E, S, W]`) used by masks with fewer than 8 bits.
    pub bit_order: Vec<String>,
    /// Blob order (`[N, NE, E, SE, S, SW, W, NW]`) used by 8-bit blob masks.
    pub blob_bit_order: Vec<String>,
}

/// One `sprites[]` entry. Category-specific extras (`terrain`, `variant`,
/// `pivot`, `frame`, `generated_by`, `semantics`, ...) are ignored.
#[derive(Debug, Clone, Deserialize)]
pub struct SpriteEntry {
    /// Stable id, e.g. `terrain/transitions/grass_to_dirt/blob_01`.
    pub id: String,
    /// Generator category, e.g. `terrain_transition` or `unit_animation`.
    pub category: String,
    /// Pack-relative PNG path (`sprites/...`, no `..` components).
    pub file: String,
    /// Declared pixel size `[w, h]`.
    pub size: (u32, u32),
    /// Lowercase 64-hex sha256 of the PNG file bytes.
    pub sha256_file: String,
    /// Lowercase 64-hex sha256 of the decoded pixel bytes.
    pub pixel_sha256: String,
    /// Packed directional mask integer, when the sprite carries terrain masks.
    #[serde(default)]
    pub mask: Option<u32>,
    /// Named directional bits; re-packing with [`Cnc32Pack::mask_bits_value`]
    /// must reproduce [`SpriteEntry::mask`].
    #[serde(default)]
    pub mask_bits: Option<BTreeMap<String, bool>>,
    /// Base terrain name (`terrain_transition` / `terrain_base` entries).
    #[serde(default)]
    pub terrain: Option<String>,
    /// Destination terrain name for transition tiles.
    #[serde(default)]
    pub alternate_terrain: Option<String>,
    /// Atlas filename under `atlases/`, e.g. `"terrain_transition.png"`.
    #[serde(default)]
    pub atlas: Option<String>,
    /// Placement inside the atlas PNG: `[x, y, w, h]`.
    #[serde(default)]
    pub atlas_rect: Option<(u32, u32, u32, u32)>,
    /// Faction for unit sprites (`gdi` / `nod`).
    #[serde(default)]
    pub faction: Option<String>,
    /// Unit kind for unit sprites (`tank`, `harvester`, ...).
    #[serde(default)]
    pub unit: Option<String>,
    /// Animation state for unit sprites (`idle`, `move`, `turn`, `attack`,
    /// `death`, ...).
    #[serde(default)]
    pub state: Option<String>,
    /// Facing index 0..8 for faceted sprites (units, cliffs, walls).
    #[serde(default)]
    pub facing: Option<u8>,
    /// Frame index within an animation.
    #[serde(default)]
    pub frame: Option<u8>,
    /// Frame duration in milliseconds (`unit_animation` entries).
    #[serde(default)]
    pub duration_ms: Option<u16>,
    /// Whether the animation loops.
    #[serde(default)]
    pub loop_: Option<bool>,
    /// Condition token for `unit_condition` entries
    /// (`intact` / `damaged` / `critical` / `destroyed`).
    #[serde(default)]
    pub condition: Option<String>,
}

/// Decoded RGBA snapshot of a PNG, buffer flattened row-major.
#[derive(Debug, Clone)]
pub struct DecodedRgba {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl DecodedRgba {
    /// Bytes per row.
    pub fn stride(&self) -> usize {
        self.width as usize * 4
    }
}

/// `ui/ui_asset_index_v6_2.json` (version 6.2.1).
#[derive(Debug, Clone, Deserialize)]
pub struct UiAssetIndex {
    /// `"6.2.1"` for this pack.
    pub version: String,
    /// Numeric generation seed (`62021`).
    pub seed: u64,
    /// Canonical art resolutions per role, e.g. `{"avatar": [128,128],
    /// "icon": [32,32], "screen": [1600,900]}`.
    pub resolution: BTreeMap<String, (u32, u32)>,
    /// Total generated personnel portraits (`commander_count + npc_count`).
    pub avatar_count: u32,
    pub commander_count: u32,
    pub npc_count: u32,
    pub icon_count: u32,
    /// Number of entries in [`UiAssetIndex::components`].
    pub control_component_count: u32,
    /// Number of entries in [`UiAssetIndex::mockups`].
    pub mockup_count: u32,
    /// Number of entries in [`UiAssetIndex::wireframes`].
    pub wireframe_count: u32,
    /// Screen-flow edges, matching `edges` in `ui/screen_flow_v6_2.json`.
    pub screen_flow_edges: u32,
    /// Pack-relative paths of reserved-reference-only UI mockup PNGs.
    pub mockups: Vec<String>,
    /// Pack-relative paths of reserved-reference-only wireframe PNGs.
    pub wireframes: Vec<String>,
    /// Control-panel components: `name -> {file, size}`.
    pub components: BTreeMap<String, UiComponent>,
    /// Character records keyed by portrait name (commanders + NPCs).
    pub characters: BTreeMap<String, UiCharacter>,
    /// Pack-relative paths of contact-sheet previews.
    pub contact_sheets: Vec<String>,
}

/// One `components` entry.
#[derive(Debug, Clone, Deserialize)]
pub struct UiComponent {
    /// Pack-relative PNG path, e.g. `sprites/ui/controls/panel.png`.
    pub file: String,
    /// Declared pixel size `[w, h]`.
    pub size: (u32, u32),
}

/// One `characters` entry (commander or NPC portrait record).
#[derive(Debug, Clone, Deserialize)]
pub struct UiCharacter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub faction: Option<String>,
    /// Pack-relative portrait PNG path.
    pub file: String,
    /// Sheet placement metadata (reserved for engines; not checked by the
    /// engine-test suite).
    pub x: u32,
    pub y: u32,
    /// Declared portrait pixel size, matching `resolution["avatar"]`.
    pub width: u32,
    pub height: u32,
}

/// Errors surfaced while loading the pack or decoding its PNGs.
#[derive(Debug, thiserror::Error)]
pub enum Cnc32PackError {
    #[error("cnc32 asset pack directory does not exist: {0}")]
    MissingDir(PathBuf),
    #[error("pack metadata read/IO error")]
    Io(#[from] std::io::Error),
    #[error("failed to parse {0}")]
    Parse(&'static str, #[source] serde_json::Error),
    #[error("failed to decode PNG {0}")]
    Decode(String, #[source] image::ImageError),
    #[error("rect x={x} y={y} {w}x{h} is out of bounds for {aw}x{ah} image")]
    OutOfBounds {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        aw: u32,
        ah: u32,
    },
    #[error("composed tileset atlas exceeds the {2}x{2} pixel cap: {0}x{1}")]
    AtlasTooLarge(u32, u32, u32),
    #[error("kit sprite {0} has a zero-pixel declared size")]
    ZeroSizeSprite(String),
    #[error("atlas file is missing from the pack: {0}")]
    MissingAtlas(String),
    #[error("sprite {0}: atlas_rect size {1}x{2} does not match declared size {3}x{4}")]
    RectSizeMismatch(String, u32, u32, u32, u32),
    #[error("unsupported manifest tile_size {0} (composed tilesets expect 32)")]
    UnsupportedTileSize(u32),
}

/// Loaded pack: root directory plus the parsed manifest and UI index.
#[derive(Debug, Clone)]
pub struct Cnc32Pack {
    /// Absolute pack root, e.g. `<workspace>/assets/packs/cnc32_assets_v6_2_1`.
    pub root: PathBuf,
    /// Parsed `manifest.json`.
    pub manifest: Cnc32Manifest,
    /// Parsed `ui/ui_asset_index_v6_2.json`.
    pub ui_index: UiAssetIndex,
    /// Sprite PNG paths resolved against `root`, index-aligned with
    /// `manifest.sprites`.
    pub sprite_paths: Vec<PathBuf>,
}

impl Cnc32Pack {
    /// Parse `manifest.json` and `ui/ui_asset_index_v6_2.json` under `dir`.
    pub fn load(dir: &Path) -> Result<Cnc32Pack, Cnc32PackError> {
        if !dir.is_dir() {
            return Err(Cnc32PackError::MissingDir(dir.to_path_buf()));
        }
        let manifest_bytes = std::fs::read(dir.join("manifest.json"))?;
        let manifest: Cnc32Manifest = serde_json::from_slice(&manifest_bytes)
            .map_err(|e| Cnc32PackError::Parse("manifest.json", e))?;
        let index_bytes = std::fs::read(dir.join("ui").join("ui_asset_index_v6_2.json"))?;
        let ui_index: UiAssetIndex = serde_json::from_slice(&index_bytes)
            .map_err(|e| Cnc32PackError::Parse("ui/ui_asset_index_v6_2.json", e))?;
        let sprite_paths = manifest
            .sprites
            .iter()
            .map(|entry| dir.join(&entry.file))
            .collect();
        Ok(Cnc32Pack {
            root: dir.to_path_buf(),
            manifest,
            ui_index,
            sprite_paths,
        })
    }

    /// Pack directory resolution:
    ///
    /// 1. `CNC_CNC32_PACK_DIR` environment override (when non-empty),
    /// 2. `../assets/packs/cnc32_assets_v6_2_1` relative to this crate (the
    ///    workspace layout for every member crate),
    /// 3. `assets/packs/cnc32_assets_v6_2_1` relative to the current directory.
    ///
    /// Returns `None` when no existing directory is found.
    pub fn fallback_dir() -> Option<PathBuf> {
        if let Ok(env_dir) = std::env::var("CNC_CNC32_PACK_DIR") {
            if !env_dir.trim().is_empty() {
                let dir = PathBuf::from(env_dir);
                return dir.is_dir().then_some(dir);
            }
        }
        let from_crate = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("assets")
            .join("packs")
            .join(CNC32_PACK_DIRNAME);
        if from_crate.is_dir() {
            return Some(from_crate);
        }
        let from_cwd = Path::new("assets").join("packs").join(CNC32_PACK_DIRNAME);
        if from_cwd.is_dir() {
            return Some(from_cwd);
        }
        None
    }

    /// Look up a sprite entry by id.
    pub fn sprite(&self, id: &str) -> Option<&SpriteEntry> {
        self.manifest.sprites.iter().find(|entry| entry.id == id)
    }

    /// Absolute path of a sprite's standalone PNG.
    pub fn sprite_path(&self, entry: &SpriteEntry) -> PathBuf {
        self.root.join(&entry.file)
    }

    /// Absolute path of a sprite's `atlases/<atlas>` PNG, when it has one.
    pub fn atlas_path(&self, entry: &SpriteEntry) -> Option<PathBuf> {
        entry
            .atlas
            .as_ref()
            .map(|name| self.root.join("atlases").join(name))
    }

    /// Resolve an arbitrary pack-relative path from the UI index or docs.
    pub fn pack_path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    /// Re-pack [`SpriteEntry::mask_bits`] into an integer using the
    /// manifest's own bit orders: `blob_bit_order` when 8 bits are present,
    /// otherwise `bit_order`. Unknown bit names are treated as unset. The
    /// engine-test suite asserts this equals the stored [`SpriteEntry::mask`].
    pub fn mask_bits_value(&self, entry: &SpriteEntry) -> Option<u32> {
        let bits = entry.mask_bits.as_ref()?;
        let order: &[String] = if bits.len() == 8 {
            &self.manifest.mask_convention.blob_bit_order
        } else {
            &self.manifest.mask_convention.bit_order
        };
        let mut value = 0u32;
        for (bit, name) in order.iter().enumerate() {
            if bits.get(name).copied().unwrap_or(false) {
                value |= 1 << bit;
            }
        }
        Some(value)
    }
}

/// Distinct ids are cheap enough to re-derive where needed.
pub fn unique_ids(entries: &[SpriteEntry]) -> HashSet<&str> {
    entries.iter().map(|entry| entry.id.as_str()).collect()
}

/// Directory name of the pack, shared by the discovery fallbacks.
const CNC32_PACK_DIRNAME: &str = "cnc32_assets_v6_2_1";

/// Decode a PNG as RGBA8 (flattened row-major). Engine tests use this instead
/// of depending on the image crate directly.
pub fn decode_png_rgba(path: &Path) -> Result<DecodedRgba, Cnc32PackError> {
    let image =
        image::open(path).map_err(|e| Cnc32PackError::Decode(path.display().to_string(), e))?;
    let rgba = image.into_rgba8();
    let (width, height) = rgba.dimensions();
    Ok(DecodedRgba {
        width,
        height,
        rgba: rgba.into_raw(),
    })
}

/// Crop an axis-aligned RGBA rectangle. The pixel layout follows
/// `image::imageops::crop`: rows of `w` pixels, alpha preserved bit-exactly.
pub fn crop_rgba(
    src: &DecodedRgba,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<DecodedRgba, Cnc32PackError> {
    if w == 0 || h == 0 || x.saturating_add(w) > src.width || y.saturating_add(h) > src.height {
        return Err(Cnc32PackError::OutOfBounds {
            x,
            y,
            w,
            h,
            aw: src.width,
            ah: src.height,
        });
    }
    let stride = src.stride();
    let mut rgba = Vec::with_capacity(w as usize * h as usize * 4);
    for row in y..y + h {
        let start = row as usize * stride + x as usize * 4;
        let end = start + w as usize * 4;
        rgba.extend_from_slice(&src.rgba[start..end]);
    }
    Ok(DecodedRgba {
        width: w,
        height: h,
        rgba,
    })
}

/// `true` when the heavy full-PNG verification pass is requested via
/// `CNC_CNC32_VERIFY=1`.
pub fn verify_env_enabled() -> bool {
    std::env::var("CNC_CNC32_VERIFY").is_ok_and(|v| v == "1")
}

// ---------------------------------------------------------------------------
// cnc32 composed tileset (editor-facing)
// ---------------------------------------------------------------------------
//
// This section composes the editor-kit atlases of the `cnc32_assets_v6_2_1`
// pack into a single deterministic RGBA atlas with stable per-sprite-id
// addressing, so a client (the map editor) can hard-switch from the legacy
// 16px `tileset_pack::TilesetPack` to one composed texture.
//
// IMPORTANT for consumers: [`PackTileset::rect`] and the `*_rect`
// convenience methods return coordinates in the COMPOSED atlas coordinate
// space (i.e. they already include the composition offsets chosen here) —
// never the `atlas_rect` coordinates of the original kit PNG from the
// manifest. The composed layout depends only on the manifest contents
// (sprite ids and declared sizes), so it is stable for a given pack
// revision and is asserted bit-identical across loads by the pack's
// `tileset_matcher` engine tests.

/// Terrain-transition blob masks are 8-bit packs over the manifest's
/// `blob_bit_order` (`N, NE, E, SE, S, SW, W, NW`): bit 0 = N,
/// 1 = NE, 2 = E, 3 = SE, 4 = S, 5 = SW, 6 = W, 7 = NW.
///
/// Diagonal bits (NE = 1, SE = 3, SW = 5, NW = 7) are only semantically
/// meaningful when both of the cardinal directions they sit between are
/// also set, i.e. bit `(d + 1) % 8` and bit `(d + 7) % 8`. This exactly
/// ports the pack generator's `valid_blob_masks()` rule
/// (`assets/packs/cnc32_assets_v6_2_1/source/cnc32_asset_generator_v5.py`,
/// lines 141–149).
pub const fn blob_mask_is_valid(mask: u8) -> bool {
    const DIAGONALS: [u8; 4] = [1, 3, 5, 7]; // NE, SE, SW, NW
    let mut i = 0;
    while i < DIAGONALS.len() {
        let d = DIAGONALS[i];
        if mask & (1 << d) != 0 {
            let cardinal_a = 1u8 << ((d + 1) % 8);
            let cardinal_b = 1u8 << ((d + 7) % 8);
            if (mask & cardinal_a) == 0 || (mask & cardinal_b) == 0 {
                return false;
            }
        }
        i += 1;
    }
    true
}

/// The 47 valid terrain-transition blob masks, sorted ascending. Computed
/// from [`blob_mask_is_valid`] at compile time (as sorted with the counting
/// scan below) rather than transcribed, so the rule and the list can never
/// drift apart; a rule that does not produce exactly 47 masks fails to
/// compile via the const-time assert.
const fn build_blob_masks() -> [u8; 47] {
    let mut masks = [0u8; 47];
    let mut count = 0usize;
    let mut m = 0usize;
    while m <= 255 {
        if blob_mask_is_valid(m as u8) {
            if count >= masks.len() {
                panic!("blob mask rule yielded more than 47 valid masks");
            }
            masks[count] = m as u8;
            count += 1;
        }
        m += 1;
    }
    assert!(count == masks.len(), "blob mask rule must yield 47 masks");
    masks
}

static BLOB_MASKS: [u8; 47] = build_blob_masks();

/// All 47 valid blob masks, ascending (`0x00..=0xff`).
pub fn valid_blob_masks() -> &'static [u8] {
    &BLOB_MASKS
}

/// Result of composing the editor-kit atlases of a [`Cnc32Pack`] into one
/// RGBA8 atlas with stable per-sprite-id addressing.
#[derive(Debug, Clone)]
pub struct PackTileset {
    /// Base tile edge in pixels; mirrored from the manifest (`32`).
    pub tile_size: u32,
    /// Width of the composed atlas in pixels (bounding box of placements).
    pub atlas_width: u32,
    /// Height of the composed atlas in pixels (bounding box of placements).
    pub atlas_height: u32,
    /// Composed RGBA8 pixels, `atlas_width * atlas_height * 4` bytes,
    /// flattened row-major.
    pub rgba: Vec<u8>,
    /// Sprite id -> rect in the COMPOSED atlas coordinate space
    /// `(x, y, w, h)`. Private: read through [`PackTileset::rect`].
    rects: BTreeMap<String, (u32, u32, u32, u32)>,
}

/// Editor-kit sprite categories included in the composed tileset. The
/// atlas filename for each sprite is taken from that sprite's manifest
/// `atlas` field — never hardcoded — so pack revisions that rename or drop
/// atlases keep working.
const PACK_TILESET_KIT_CATEGORIES: [&str; 9] = [
    "terrain_base",
    "terrain_transition",
    "road_connection",
    "shoreline",
    "cliff_connection",
    "wall_connection",
    "resource",
    "bridge",
    "decoration",
];

/// Road id suffixes by mask (`{mask:02x}`), as observed in the
/// `cnc32_assets_v6_2_1` manifest (descriptive names, not mechanically
/// derivable from the mask). Pinned to the manifest by the
/// `tileset_matcher` engine tests; a pack revision that renames road ids
/// must update this table.
const ROAD_ID_SUFFIXES: [&str; 16] = [
    "isolated",             // 0x00
    "north_end",            // 0x01
    "east_end",             // 0x02
    "north_east_corner",    // 0x03
    "south_end",            // 0x04
    "vertical",             // 0x05
    "east_south_corner",    // 0x06
    "t_north_missing_west", // 0x07
    "west_end",             // 0x08
    "west_north_corner",    // 0x09
    "horizontal",           // 0x0a
    "t_east_missing_south", // 0x0b
    "south_west_corner",    // 0x0c
    "t_south_missing_east", // 0x0d
    "t_west_missing_north", // 0x0e
    "cross",                // 0x0f
];

/// Maximum width (and height) of the composed tileset atlas in pixels.
pub const PACK_TILESET_MAX_DIM: u32 = 8192;

/// Terrain id prefix of a terrain-transition blob sprite id.
const BLOB_ID_PREFIX: &str = "terrain/transitions/";

impl PackTileset {
    /// All 47 valid blob masks, ascending (`0x00..=0xff`).
    pub fn valid_blob_masks() -> &'static [u8] {
        &BLOB_MASKS
    }

    /// Repair an arbitrary 8-bit directional mask into the nearest valid
    /// blob mask: every diagonal bit with at least one of its two adjacent
    /// cardinal bits clear is cleared; cardinal bits are preserved. One
    /// pass is provably sufficient (cardinals never change, so once a
    /// diagonal satisfies the rule it keeps satisfying it), but the loop is
    /// run to a fixpoint anyway for clarity; this function is always
    /// idempotent and its result is always a member of
    /// [`PackTileset::valid_blob_masks`].
    pub fn repair_blob_mask(mask: u8) -> u8 {
        let mut current = mask;
        loop {
            let mut next = current;
            for &d in &[1u8, 3, 5, 7] {
                let bit = 1u8 << d;
                let cardinal_a = 1u8 << ((d + 1) % 8);
                let cardinal_b = 1u8 << ((d + 7) % 8);
                if next & bit != 0 && ((next & cardinal_a) == 0 || (next & cardinal_b) == 0) {
                    next &= !bit;
                }
            }
            if next == current {
                break;
            }
            current = next;
        }
        current
    }

    /// Terrain-transition blob sprite id for `base` chip's edge against an
    /// `alt` neighbor terrain, e.g.
    /// `terrain/transitions/grass_to_dirt/blob_05` (`mask` as 2-digit
    /// lowercase hex). Names come from the manifest's `terrain` /
    /// `alternate_terrain` fields (e.g. `"grass"`, `"dirt"`).
    pub fn blob_id(base: &str, alt: &str, mask: u8) -> String {
        format!("{BLOB_ID_PREFIX}{base}_to_{alt}/blob_{mask:02x}")
    }

    /// Road sprite id for a connection mask, e.g.
    /// `roads/road_05_vertical`. Mask-validity rules differ from blob masks
    /// (all 16 N/E/S/W combinations exist as road tiles).
    pub fn road_id(mask: u8) -> String {
        format!(
            "roads/road_{:02x}_{}",
            mask,
            ROAD_ID_SUFFIXES[(mask & 0x0f) as usize]
        )
    }

    /// Water-land shoreline sprite id for a land-edge mask, e.g.
    /// `shoreline/water_landmask_0a`.
    pub fn shore_id(mask: u8) -> String {
        format!("shoreline/water_landmask_{mask:02x}")
    }

    /// Decode every editor-kit atlas referenced by the manifest once
    /// (kit categories in [`PACK_TILESET_KIT_CATEGORIES`], atlas filenames
    /// from the sprite entries themselves) and compose them into a single
    /// deterministic RGBA atlas:
    ///
    /// - entries are sorted by sprite id (`BTreeMap` order),
    /// - shelf/row packing, left-to-right, rows capped at
    ///   [`PACK_TILESET_MAX_DIM`] pixels wide,
    /// - each tile's pixels are copied from its manifest `atlas_rect`
    ///   inside the original atlas PNG (cross-checked against the sprite's
    ///   declared `size`).
    ///
    /// Because the layout depends only on the manifest, two compositions
    /// of the same pack produce byte-identical output (asserted by the
    /// pack's `tileset_matcher` engine test suite).
    pub fn load(pack: &Cnc32Pack) -> Result<PackTileset, Cnc32PackError> {
        if pack.manifest.tile_size != 32 {
            return Err(Cnc32PackError::UnsupportedTileSize(pack.manifest.tile_size));
        }

        // Kit sprites that actually carry atlas data, in deterministic
        // (sprite id) order.
        let mut ordered: Vec<&SpriteEntry> = pack
            .manifest
            .sprites
            .iter()
            .filter(|entry| {
                PACK_TILESET_KIT_CATEGORIES.contains(&entry.category.as_str())
                    && entry.atlas.is_some()
                    && entry.atlas_rect.is_some()
            })
            .collect();
        ordered.sort_by(|a, b| a.id.cmp(&b.id));

        // The set of atlas files actually referenced by those sprites,
        // decoded once each.
        let mut atlas_names: BTreeSet<String> = BTreeSet::new();
        for entry in &ordered {
            if let Some(name) = entry.atlas.as_deref() {
                atlas_names.insert(name.to_owned());
            }
        }
        let mut decoded: Vec<(String, DecodedRgba)> = Vec::with_capacity(atlas_names.len());
        let mut decoded_index: BTreeMap<String, usize> = BTreeMap::new();
        for name in &atlas_names {
            let path = pack.root.join("atlases").join(name);
            if !path.is_file() {
                return Err(Cnc32PackError::MissingAtlas(name.clone()));
            }
            let image = decode_png_rgba(&path)?;
            decoded_index.insert(name.clone(), decoded.len());
            decoded.push((name.clone(), image));
        }

        // Shelf packing: fixed order, fixed rules, no randomness.
        let mut rects: BTreeMap<String, (u32, u32, u32, u32)> = BTreeMap::new();
        let mut placements: Vec<(u32, u32)> = Vec::with_capacity(ordered.len());
        let (mut pen_x, mut pen_y, mut row_h, mut max_w) = (0u32, 0u32, 0u32, 0u32);
        for entry in &ordered {
            let (w, h) = entry.size;
            if w == 0 || h == 0 {
                return Err(Cnc32PackError::ZeroSizeSprite(entry.id.clone()));
            }
            if w > PACK_TILESET_MAX_DIM || h > PACK_TILESET_MAX_DIM {
                return Err(Cnc32PackError::AtlasTooLarge(w, h, PACK_TILESET_MAX_DIM));
            }
            if pen_x > 0 && pen_x + w > PACK_TILESET_MAX_DIM {
                pen_y += row_h;
                pen_x = 0;
                row_h = 0;
            }
            if pen_x + w > PACK_TILESET_MAX_DIM || pen_y + h > PACK_TILESET_MAX_DIM {
                return Err(Cnc32PackError::AtlasTooLarge(w, h, PACK_TILESET_MAX_DIM));
            }
            placements.push((pen_x, pen_y));
            rects.insert(entry.id.clone(), (pen_x, pen_y, w, h));
            pen_x += w;
            row_h = row_h.max(h);
            max_w = max_w.max(pen_x);
        }
        let atlas_height = pen_y + row_h;
        let atlas_width = max_w;

        // Compose: copy each tile from its source atlas into the layout slot.
        let mut rgba = vec![0u8; atlas_width as usize * atlas_height as usize * 4];
        for (entry, (x, y)) in ordered.iter().zip(&placements) {
            let name = entry.atlas.as_deref().unwrap_or_default();
            let source = &decoded[decoded_index[name]].1;
            let (sx, sy, sw, sh) = entry.atlas_rect.unwrap_or((0, 0, 0, 0));
            if (sw, sh) != entry.size {
                return Err(Cnc32PackError::RectSizeMismatch(
                    entry.id.clone(),
                    sw,
                    sh,
                    entry.size.0,
                    entry.size.1,
                ));
            }
            let tile = crop_rgba(source, sx, sy, sw, sh)?;
            for row in 0..sh {
                let src_start = row as usize * tile.stride();
                let src_end = src_start + sw as usize * 4;
                let dst_start = ((y + row) as usize * atlas_width as usize + *x as usize) * 4;
                rgba[dst_start..dst_start + src_end - src_start]
                    .copy_from_slice(&tile.rgba[src_start..src_end]);
            }
        }

        Ok(PackTileset {
            tile_size: pack.manifest.tile_size,
            atlas_width,
            atlas_height,
            rgba,
            rects,
        })
    }

    /// Pixel rect of a composer sprite id, in COMPOSED-atlas coordinates
    /// (`x, y, w, h`) — i.e. after the composition offsets this loader
    /// chose, NOT the manifest's original `atlas_rect`.
    pub fn rect(&self, sprite_id: &str) -> Option<(u32, u32, u32, u32)> {
        self.rects.get(sprite_id).copied()
    }

    /// Read-only view of the whole composed rect map (deterministic
    /// iteration order: sprite id ascending).
    pub fn rects(&self) -> &BTreeMap<String, (u32, u32, u32, u32)> {
        &self.rects
    }

    /// Composed rect for a terrain-transition blob. The resulting rect is
    /// in composed-atlas coordinates; the manifest lookup via
    /// [`Cnc32Pack::sprite`] is used to confirm the id exists in the pack
    /// (so callers with a stale terrain name get `None` rather than a
    /// rectangle for a phantom id).
    pub fn blob_rect(
        &self,
        pack: &Cnc32Pack,
        base: &str,
        alt: &str,
        mask: u8,
    ) -> Option<(u32, u32, u32, u32)> {
        let id = Self::blob_id(base, alt, mask);
        pack.sprite(&id)?;
        self.rect(&id)
    }

    /// Composed rect for a road sprite (see [`PackTileset::blob_rect`]
    /// for coordinate-space semantics).
    pub fn road_rect(&self, pack: &Cnc32Pack, mask: u8) -> Option<(u32, u32, u32, u32)> {
        let id = Self::road_id(mask);
        pack.sprite(&id)?;
        self.rect(&id)
    }

    /// Composed rect for a shoreline sprite (see
    /// [`PackTileset::blob_rect`] for coordinate-space semantics).
    pub fn shore_rect(&self, pack: &Cnc32Pack, mask: u8) -> Option<(u32, u32, u32, u32)> {
        let id = Self::shore_id(mask);
        pack.sprite(&id)?;
        self.rect(&id)
    }

    /// Terrain names appearing in the manifest's terrain-transition
    /// sprites (the tokenizer set the editor needs for base/alternate
    /// terrain combos). Sorted, deduped.
    pub fn list_terrains(pack: &Cnc32Pack) -> BTreeSet<String> {
        pack.manifest
            .sprites
            .iter()
            .filter(|entry| entry.category == "terrain_transition")
            .filter_map(|entry| entry.terrain.clone())
            .collect()
    }

    /// Stable, fast digest (FNV-1a 64) of the composed RGBA bytes plus the
    /// atlas dimensions. Used by the engine tests to assert composition
    /// determinism.
    pub fn fingerprint(&self) -> u64 {
        let mut hash =
            0xcbf2_9ce4_8422_2325u64 ^ (self.atlas_width as u64) ^ (self.atlas_height as u64) << 8;
        for &byte in &self.rgba {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }
}
