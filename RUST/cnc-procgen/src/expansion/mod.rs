//! CNC32 **v6.x additive expansion layers**, ported from the Python pack's
//! generators.
//!
//! [`crate::tileset::pack`] reproduces the v5.5.0 tileset pack (terrain,
//! roads, units, effects — 3909 sprites). The v6.2.1 pack ships four further
//! Python generators that layer 668 more sprites on top; each is ported here
//! as one module, in the same order the Python pack was assembled:
//!
//! 1. [`cutscene_v58`] — six story sequences, overlays, wipes, cards
//!    (`cutscene_generator_v5_8.py`)
//! 2. [`campaign_v59`] — twelve campaign scenes, props, portraits, key art
//!    (`campaign_expansion_v5_9.py`)
//! 3. [`campaign_v60`] — ten more scenes, 48 effect frames, prop atlases
//!    (`campaign_expansion_v6_0.py`)
//! 4. [`briefing_v61`] — eight briefing sequences, icons, personnel
//!    (`campaign_expansion_v6_1.py`)
//! 5. [`ui_v62`] — commander/NPC portraits, tactical icons, UI controls,
//!    screen mockups, wireframes (`asset_expansion_v6_2.py`)
//!
//! ## Fidelity contract
//!
//! Same standard as the tileset port: identical structure, categories, ids,
//! file layout, metadata and seed derivation. Pixels differ because this
//! crate uses SplitMix64 instead of Python's MT19937 and its own rasterizer
//! instead of Pillow, so `sha256_file` / `pixel_sha256` are computed over
//! this crate's own output. Every layer keeps its Python script's default
//! seed (`DEFAULT_SEED`); the pack's `--seed` only drives the v5 base.
//!
//! Draw order and draw counts are part of the contract: the Python scripts
//! were ported call-for-call so that the same seeds select the same motifs,
//! palettes and geometry, even though the numeric stream itself differs.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::tileset::canvas::Canvas;
use crate::tileset::pack::PackError;

pub mod briefing_v61;
pub mod campaign_v59;
pub mod campaign_v60;
pub mod cutscene_v58;
pub mod ui_v62;

/// Everything one expansion layer contributes to the finished pack.
///
/// The Python scripts mutate `manifest.json` and
/// `sprites/cinematics/timeline.json` in place and write their artefacts
/// straight to disk; this crate generates in one deterministic pass, so a
/// layer returns its contributions and [`crate::tileset::pack::build`]
/// merges them in layer order.
#[derive(Default)]
pub struct LayerOutput {
    /// Manifest `sprites[]` records in the Python registration order.
    pub sprites: Vec<Value>,
    /// Entries for `manifest["atlases"]` (only some layers extend it).
    pub atlases: Vec<Value>,
    /// Top-level manifest keys this layer writes, e.g. `cinematics`.
    pub manifest_extra: Map<String, Value>,
    /// Standalone report files: `(relative path, JSON body)`.
    pub reports: Vec<(String, Value)>,
    /// Contribution to `sprites/cinematics/timeline.json`, when the layer
    /// has one. The builder merges these in layer order.
    pub timeline: Option<TimelineContribution>,
    /// Pack-relative paths of support artefacts (contact sheets, overlays,
    /// timelines) that the Python scripts list in `supporting_assets`.
    pub supporting_assets: Vec<String>,
}

/// One layer's contribution to the shared cinematics timeline.
#[derive(Default)]
pub struct TimelineContribution {
    /// Sequence records, appended/replaced by `id`.
    pub sequences: Vec<Value>,
    /// Campaign records, appended/replaced by `id`.
    pub campaigns: Vec<Value>,
    /// Free-form notes, de-duplicated in insertion order by the builder.
    pub notes: Vec<String>,
    /// Timeline `schema_version` this layer asserts.
    pub schema_version: &'static str,
    /// Timeline `generator_version` this layer asserts (empty = keep the
    /// value already in the file).
    pub generator_version: &'static str,
    /// Timeline `seed` this layer asserts (`None` = keep the file's value).
    pub seed: Option<u64>,
    /// Additional top-level timeline keys this layer asserts, e.g. Python
    /// v6.1's `briefings` index.
    pub extra: Map<String, Value>,
}

/// Sprite metadata for [`PackWriter::write_sprite`].
///
/// Mirrors the dicts the Python `register()` helpers build: the five common
/// keys plus whichever provenance keys that script uses (`generated_by` +
/// `seed` for v5.8–v6.1, `version` for v6.2) and any category-specific
/// extras such as `art_role`.
#[derive(Default)]
pub struct SpriteSpec<'a> {
    pub category: &'a str,
    pub generated_by: Option<&'a str>,
    pub seed: Option<u64>,
    pub version: Option<&'a str>,
    pub extra: Vec<(&'a str, Value)>,
}

/// Filesystem sink for a layer's artefacts, rooted at the pack directory.
///
/// The builder hands layers a writer pointed at its staging directory, so a
/// layer never sees absolute paths or the caller's output location.
pub struct PackWriter {
    root: PathBuf,
}

impl PackWriter {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Pack directory every relative path is resolved against.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Absolute path for a pack-relative path.
    pub fn path(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    /// Write raw bytes, creating parent directories.
    pub fn write_bytes(&self, rel: &str, bytes: &[u8]) -> Result<(), PackError> {
        let path = self.path(rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(path, bytes)?;
        Ok(())
    }

    /// Write UTF-8 text (SVG, Markdown, timelines), creating parents.
    pub fn write_text(&self, rel: &str, text: &str) -> Result<(), PackError> {
        self.write_bytes(rel, text.as_bytes())
    }

    /// Write pretty-printed JSON with a trailing newline, like the Python
    /// scripts' `json.dump(..., indent=2)`.
    pub fn write_json(&self, rel: &str, value: &Value) -> Result<(), PackError> {
        let mut body = serde_json::to_string_pretty(value)?;
        body.push('\n');
        self.write_text(rel, &body)
    }

    /// Write a canvas as PNG. Channel count follows the canvas: fully opaque
    /// images become RGB, anything with transparency stays RGBA — matching
    /// the Python scripts, which save `Image.new("RGB")` canvases as RGB.
    pub fn write_png(&self, rel: &str, canvas: &Canvas) -> Result<(), PackError> {
        let path = self.path(rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(path, canvas.encode_png()?)?;
        Ok(())
    }

    /// SHA-256 of an already-written file, lowercase hex.
    pub fn sha256_file(&self, rel: &str) -> Result<String, PackError> {
        let bytes = std::fs::read(self.path(rel))?;
        Ok(sha256_hex(&bytes))
    }

    /// Read and parse a pack-relative JSON file; `None` when it is absent.
    pub fn read_json(&self, rel: &str) -> Result<Option<Value>, PackError> {
        let path = self.path(rel);
        if !path.is_file() {
            return Ok(None);
        }
        Ok(Some(serde_json::from_slice(&std::fs::read(path)?)?))
    }

    /// Write a PNG **and** return its manifest sprite record, exactly like
    /// the Python `register(png)` helpers: the id is the pack-relative path
    /// with `.png` stripped.
    pub fn write_sprite(
        &self,
        rel: &str,
        canvas: &Canvas,
        spec: SpriteSpec<'_>,
    ) -> Result<Value, PackError> {
        let path = self.path(rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let bytes = canvas.encode_png()?;
        std::fs::write(&path, &bytes)?;
        Ok(sprite_record(rel, canvas, &bytes, spec))
    }
}

/// Build a manifest sprite record without touching the filesystem (for
/// atlases that are registered after being composed in memory).
pub fn sprite_record(rel: &str, canvas: &Canvas, png_bytes: &[u8], spec: SpriteSpec<'_>) -> Value {
    let mut rec = Map::new();
    let id = rel.strip_suffix(".png").unwrap_or(rel);
    rec.insert("id".to_string(), Value::String(id.to_string()));
    rec.insert(
        "category".to_string(),
        Value::String(spec.category.to_string()),
    );
    rec.insert("file".to_string(), Value::String(rel.to_string()));
    rec.insert(
        "size".to_string(),
        Value::Array(vec![
            Value::from(canvas.width()),
            Value::from(canvas.height()),
        ]),
    );
    rec.insert(
        "sha256_file".to_string(),
        Value::String(sha256_hex(png_bytes)),
    );
    rec.insert(
        "pixel_sha256".to_string(),
        Value::String(sha256_hex(canvas.as_rgba().as_raw())),
    );
    if let Some(generated_by) = spec.generated_by {
        rec.insert(
            "generated_by".to_string(),
            Value::String(generated_by.to_string()),
        );
    }
    if let Some(seed) = spec.seed {
        rec.insert("seed".to_string(), Value::from(seed));
    }
    if let Some(version) = spec.version {
        rec.insert("version".to_string(), Value::String(version.to_string()));
    }
    for (key, value) in spec.extra {
        rec.insert(key.to_string(), value);
    }
    Value::Object(rec)
}

/// Tile `cells` into a grid of `columns` columns whose cells are `cell`
/// pixels square, composites in list order (PIL `alpha_composite`), and
/// nearest-resizes each source to the cell size when `resize` is set (the
/// effect/prop/icon atlas builders all upscale from smaller source art).
pub fn compose_atlas(cells: &[&Canvas], columns: usize, cell: u32, resize: bool) -> Canvas {
    let columns = columns.max(1);
    let rows = cells.len().div_ceil(columns).max(1);
    let mut atlas = Canvas::transparent(columns as u32 * cell, rows as u32 * cell);
    for (index, src) in cells.iter().enumerate() {
        let (col, row) = (index % columns, index / columns);
        let cell_src = if resize && (src.width() != cell || src.height() != cell) {
            src.resize_nearest(cell, cell)
        } else {
            (*src).clone()
        };
        atlas.alpha_composite(
            &cell_src,
            ((col * cell as usize) as i32, (row * cell as usize) as i32),
        );
    }
    atlas
}

/// Lowercase-hex SHA-256 helper shared by the layer modules.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
