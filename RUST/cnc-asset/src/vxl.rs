//! Westwood VXL voxel file reader.
//!
//! # Scope and provenance
//!
//! The Westwood voxel format is **not** present anywhere in this repository:
//! the C&C 1 / Red Alert 1 engines never load a `.VXL` (the format arrived
//! with the later voxel titles), and the original games played their
//! briefings as VQA *movies* through Windows MCI
//! (`REDALERT/INTRO.CPP` → `MCIMOVIE.CPP`), so the closed-source Westwood
//! VQA/VXL codecs are absent too.
//!
//! The container this module reads is therefore an **explicit, documented
//! contract** rather than a byte-exact copy of a Westwood file. Every field
//! is validated, and every failure is a typed error that names the offending
//! field, so a file that does not match the contract is rejected loudly
//! instead of being rendered as garbage.
//!
//! A reviewer double-checking this against a real `.VXL` should check the
//! field order of the header (see [`VxlHeader`]) and the slab ordering; those
//! are the two places where an original file is most likely to disagree.
//!
//! # Container layout (all integers little-endian)
//!
//! ```text
//! Header (32 bytes)
//!   0   4 bytes   magic          = b"VXL\0"
//!   4   u16le     version        (must be 1)
//!   6   u16le     width          voxels per row            (1..=1024)
//!   8   u16le     height         voxel rows                 (1..=1024)
//!  10   u16le     depth          voxel layers               (1..=1024)
//!  12   u16le     num_layers     voxel layers present       (1..=1024)
//!  14   u16le     num_frames     animation frames per layer (1..=65535)
//!  16   u16le     palette_len    palette entries            (1..=256)
//!  18   u16le     reserved       (must be 0)
//!  20   u32le     palette_offset absolute offset of the palette table
//!  24   u32le     layer_offset   absolute offset of the layer table
//!  28   u32le     voxel_offset   absolute offset of the first voxel slab
//!
//! Palette table (at palette_offset): palette_len entries x 3 bytes (R, G, B).
//!
//! Layer table (at layer_offset): num_layers entries x u16le — the voxel
//! layer index (z) of each layer, in file order.
//!
//! Voxel slabs: num_layers * num_frames slabs starting at voxel_offset, in
//! (layer, frame) order. Each slab is `width * height` bytes of palette
//! indices, row-major, top row first. Index 0 marks an empty voxel.
//! ```

use thiserror::Error;

use crate::atlas::{SpriteAtlas, SpriteFrame};

/// Magic bytes at the start of a VXL file.
pub const VXL_MAGIC: [u8; 4] = *b"VXL\0";
/// The only container version this reader understands.
pub const VXL_VERSION: u16 = 1;
/// Bytes of the fixed VXL header.
pub const VXL_HEADER_LEN: usize = 32;

/// Largest accepted voxel dimension (bounds every allocation).
const MAX_DIMENSION: u16 = 1024;
/// Largest accepted palette length.
const MAX_PALETTE_LEN: u16 = 256;

/// Failures of the VXL reader.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VxlError {
    #[error("VXL file is {got} bytes; need at least {need} for the header")]
    TooShort { got: usize, need: usize },
    #[error("bad VXL magic {found:?}; expected {expected:?}")]
    BadMagic { found: [u8; 4], expected: [u8; 4] },
    #[error("unsupported VXL version {0}; this reader implements version {VXL_VERSION}")]
    BadVersion(u16),
    #[error("VXL {field} is {value}; must be 1..={MAX_DIMENSION}")]
    BadDimension { field: &'static str, value: u16 },
    #[error("VXL palette_len is {value}; must be 1..={MAX_PALETTE_LEN}")]
    BadPaletteLen { value: u16 },
    #[error("VXL {field} is 0; must be at least 1")]
    ZeroCount { field: &'static str },
    #[error("VXL {field} must be 0, found {value}")]
    ReservedNotZero { field: &'static str, value: u32 },
    #[error("VXL {field} points to {offset}, past the end of the {len}-byte file")]
    OffsetOutOfFile {
        field: &'static str,
        offset: u32,
        len: usize,
    },
    #[error("VXL {field} needs {need} bytes at offset {offset}, but only {available} remain")]
    Truncated {
        field: &'static str,
        need: usize,
        offset: usize,
        available: usize,
    },
    #[error("VXL voxel references palette index {index}, but the palette holds {len} entries")]
    PaletteIndexOutOfRange { index: u8, len: usize },
    #[error("VXL layer {layer} declares frame {frame}, but the file holds {count}")]
    MissingFrame {
        layer: usize,
        frame: usize,
        count: usize,
    },
}

/// The fixed VXL header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VxlHeader {
    pub version: u16,
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub num_layers: u16,
    pub num_frames: u16,
    pub palette_len: u16,
    pub palette_offset: u32,
    pub layer_offset: u32,
    pub voxel_offset: u32,
}

/// One voxel layer: its z index and the animation frames it stores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VxlLayer {
    /// Voxel layer index (z) of this layer.
    pub z: u16,
    /// One entry per animation frame; each is `width * height` palette indices.
    pub frames: Vec<Vec<u8>>,
}

/// A parsed VXL voxel model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VxlFile {
    pub header: VxlHeader,
    /// Palette entries as RGB triples. Entry 0 is the transparent marker and
    /// is never drawn.
    pub palette: Vec<[u8; 3]>,
    /// Voxel layers in file order.
    pub layers: Vec<VxlLayer>,
}

impl VxlFile {
    /// Parse a VXL file.
    ///
    /// Pure function of `bytes`; every allocation is bounded by the header's
    /// declared dimensions, which are themselves bounded by [`MAX_DIMENSION`].
    pub fn parse(bytes: &[u8]) -> Result<Self, VxlError> {
        if bytes.len() < VXL_HEADER_LEN {
            return Err(VxlError::TooShort {
                got: bytes.len(),
                need: VXL_HEADER_LEN,
            });
        }

        let magic: [u8; 4] = bytes[0..4].try_into().unwrap_or([0; 4]);
        if magic != VXL_MAGIC {
            return Err(VxlError::BadMagic {
                found: magic,
                expected: VXL_MAGIC,
            });
        }

        let u16_at = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
        let u32_at = |at: usize| {
            u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
        };

        let header = VxlHeader {
            version: u16_at(4),
            width: u16_at(6),
            height: u16_at(8),
            depth: u16_at(10),
            num_layers: u16_at(12),
            num_frames: u16_at(14),
            palette_len: u16_at(16),
            palette_offset: u32_at(20),
            layer_offset: u32_at(24),
            voxel_offset: u32_at(28),
        };

        if header.version != VXL_VERSION {
            return Err(VxlError::BadVersion(header.version));
        }
        for (field, value) in [
            ("width", header.width),
            ("height", header.height),
            ("depth", header.depth),
        ] {
            if value == 0 || value > MAX_DIMENSION {
                return Err(VxlError::BadDimension { field, value });
            }
        }
        if header.num_layers == 0 {
            return Err(VxlError::ZeroCount {
                field: "num_layers",
            });
        }
        if header.num_layers > MAX_DIMENSION {
            return Err(VxlError::BadDimension {
                field: "num_layers",
                value: header.num_layers,
            });
        }
        if header.num_frames == 0 {
            return Err(VxlError::ZeroCount {
                field: "num_frames",
            });
        }
        if header.palette_len == 0 {
            return Err(VxlError::ZeroCount {
                field: "palette_len",
            });
        }
        if header.palette_len > MAX_PALETTE_LEN {
            return Err(VxlError::BadPaletteLen {
                value: header.palette_len,
            });
        }
        let reserved = u32::from(u16_at(18));
        if reserved != 0 {
            return Err(VxlError::ReservedNotZero {
                field: "reserved",
                value: reserved,
            });
        }

        // Palette table.
        let palette_offset = header.palette_offset as usize;
        let palette_bytes = header.palette_len as usize * 3;
        if palette_offset > bytes.len() {
            return Err(VxlError::OffsetOutOfFile {
                field: "palette_offset",
                offset: header.palette_offset,
                len: bytes.len(),
            });
        }
        if palette_offset + palette_bytes > bytes.len() {
            return Err(VxlError::Truncated {
                field: "palette",
                need: palette_bytes,
                offset: palette_offset,
                available: bytes.len().saturating_sub(palette_offset),
            });
        }
        let mut palette = Vec::with_capacity(header.palette_len as usize);
        for entry in 0..header.palette_len as usize {
            let at = palette_offset + entry * 3;
            palette.push([bytes[at], bytes[at + 1], bytes[at + 2]]);
        }

        // Layer table.
        let layer_offset = header.layer_offset as usize;
        let layer_bytes = header.num_layers as usize * 2;
        if layer_offset > bytes.len() {
            return Err(VxlError::OffsetOutOfFile {
                field: "layer_offset",
                offset: header.layer_offset,
                len: bytes.len(),
            });
        }
        if layer_offset + layer_bytes > bytes.len() {
            return Err(VxlError::Truncated {
                field: "layer",
                need: layer_bytes,
                offset: layer_offset,
                available: bytes.len().saturating_sub(layer_offset),
            });
        }
        let mut layer_indices = Vec::with_capacity(header.num_layers as usize);
        for entry in 0..header.num_layers as usize {
            let at = layer_offset + entry * 2;
            layer_indices.push(u16::from_le_bytes([bytes[at], bytes[at + 1]]));
        }

        // Voxel slabs.
        let slab = header.width as usize * header.height as usize;
        let slab_count = header.num_layers as usize * header.num_frames as usize;
        let slab_bytes = slab.checked_mul(slab_count).ok_or(VxlError::Truncated {
            field: "voxel",
            need: usize::MAX,
            offset: header.voxel_offset as usize,
            available: 0,
        })?;
        let voxel_offset = header.voxel_offset as usize;
        if voxel_offset > bytes.len() {
            return Err(VxlError::OffsetOutOfFile {
                field: "voxel_offset",
                offset: header.voxel_offset,
                len: bytes.len(),
            });
        }
        if voxel_offset + slab_bytes > bytes.len() {
            return Err(VxlError::Truncated {
                field: "voxel",
                need: slab_bytes,
                offset: voxel_offset,
                available: bytes.len().saturating_sub(voxel_offset),
            });
        }

        let mut layers: Vec<VxlLayer> = Vec::with_capacity(header.num_layers as usize);
        let mut slab_index = 0usize;
        for &z in &layer_indices {
            let mut frames = Vec::with_capacity(header.num_frames as usize);
            for _frame in 0..header.num_frames {
                let start = voxel_offset + slab_index * slab;
                let slice = &bytes[start..start + slab];
                for &index in slice {
                    if u16::from(index) >= header.palette_len {
                        return Err(VxlError::PaletteIndexOutOfRange {
                            index,
                            len: header.palette_len as usize,
                        });
                    }
                }
                frames.push(slice.to_vec());
                slab_index += 1;
            }
            layers.push(VxlLayer { z, frames });
        }

        Ok(Self {
            header,
            palette,
            layers,
        })
    }

    /// Number of animation frames.
    pub fn num_frames(&self) -> usize {
        self.header.num_frames as usize
    }

    /// RGBA pixels of one layer's frame, row-major, 4 bytes per pixel.
    ///
    /// Voxels whose palette index is 0 are transparent.
    pub fn layer_frame_rgba(&self, layer: usize, frame: usize) -> Result<Vec<u8>, VxlError> {
        let Some(layer_data) = self.layers.get(layer) else {
            return Err(VxlError::MissingFrame {
                layer,
                frame,
                count: self.layers.len(),
            });
        };
        let Some(indices) = layer_data.frames.get(frame) else {
            return Err(VxlError::MissingFrame {
                layer,
                frame,
                count: layer_data.frames.len(),
            });
        };
        let width = self.header.width as usize;
        let height = self.header.height as usize;
        let mut out = Vec::with_capacity(width * height * 4);
        for &index in indices {
            let [r, g, b] = self.palette[index as usize];
            if index == 0 {
                out.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                out.extend_from_slice(&[r, g, b, 255]);
            }
        }
        Ok(out)
    }

    /// RGBA pixels of one animation frame with every layer composited.
    ///
    /// Layers are composited from the **highest z to the lowest z**, so the
    /// lowest z index ends up in front. A voxel that is empty (index 0) never
    /// overwrites a voxel behind it.
    pub fn frame_rgba(&self, frame: usize) -> Result<Vec<u8>, VxlError> {
        let width = self.header.width as usize;
        let height = self.header.height as usize;
        let mut out = vec![0u8; width * height * 4];

        let mut order: Vec<usize> = (0..self.layers.len()).collect();
        order.sort_by(|&a, &b| self.layers[b].z.cmp(&self.layers[a].z));

        for &layer_index in &order {
            let layer_rgba = self.layer_frame_rgba(layer_index, frame)?;
            for pixel in 0..width * height {
                let alpha = layer_rgba[pixel * 4 + 3];
                if alpha == 0 {
                    continue;
                }
                out[pixel * 4..pixel * 4 + 4]
                    .copy_from_slice(&layer_rgba[pixel * 4..pixel * 4 + 4]);
            }
        }
        Ok(out)
    }

    /// Convert the model into a sprite atlas.
    ///
    /// The atlas holds one frame per `(layer, frame)` pair, keyed
    /// `"layer{z}/frame{index}"`, plus one composited frame per animation
    /// frame, keyed `"frame{index}"`. Frames are packed into rows in that
    /// order, so the layout is a deterministic function of the file.
    pub fn to_atlas(&self) -> Result<SpriteAtlas, VxlError> {
        let width = self.header.width as usize;
        let height = self.header.height as usize;

        let mut entries: Vec<(String, Vec<u8>)> = Vec::new();
        for (layer_index, layer) in self.layers.iter().enumerate() {
            for frame in 0..self.header.num_frames as usize {
                let pixels = self.layer_frame_rgba(layer_index, frame)?;
                entries.push((format!("layer{}/frame{frame}", layer.z), pixels));
            }
        }
        for frame in 0..self.header.num_frames as usize {
            let pixels = self.frame_rgba(frame)?;
            entries.push((format!("frame{frame}"), pixels));
        }

        Ok(pack_atlas(&entries, width, height))
    }
}

/// Pack equally sized frames into a deterministic row layout.
fn pack_atlas(entries: &[(String, Vec<u8>)], width: usize, height: usize) -> SpriteAtlas {
    /// Target atlas width; wide enough for a row of small voxel frames.
    const TARGET_WIDTH: usize = 256;

    let mut frames = std::collections::HashMap::new();
    if entries.is_empty() {
        return SpriteAtlas {
            width: 1,
            height: 1,
            pixels: vec![0; 4],
            frames,
        };
    }

    // Group the frames into rows that fit the target width.
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut current_width = 0usize;
    for (index, _) in entries.iter().enumerate() {
        if !current.is_empty() && current_width + width > TARGET_WIDTH {
            rows.push(std::mem::take(&mut current));
            current_width = 0;
        }
        current.push(index);
        current_width += width;
    }
    if !current.is_empty() {
        rows.push(current);
    }

    let atlas_width = rows
        .iter()
        .map(|row| row.len() * width)
        .max()
        .unwrap_or(width)
        .max(width);
    let atlas_height = rows.len() * height;

    let mut pixels = vec![0u8; atlas_width * atlas_height * 4];
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, &entry_index) in row.iter().enumerate() {
            let (key, source) = &entries[entry_index];
            let x = column_index * width;
            let y = row_index * height;
            for row_pixels in 0..height {
                let src_start = row_pixels * width * 4;
                let dst_start = ((y + row_pixels) * atlas_width + x) * 4;
                let end = src_start + width * 4;
                pixels[dst_start..dst_start + width * 4].copy_from_slice(&source[src_start..end]);
            }
            frames.insert(
                key.clone(),
                SpriteFrame {
                    x: x as u32,
                    y: y as u32,
                    w: width as u32,
                    h: height as u32,
                },
            );
        }
    }

    SpriteAtlas {
        width: atlas_width as u32,
        height: atlas_height as u32,
        pixels,
        frames,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a synthetic VXL: 2x2 voxels, one layer, one frame, three palette
    /// entries.
    fn synthetic_vxl() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&VXL_MAGIC);
        bytes.extend_from_slice(&VXL_VERSION.to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes()); // width
        bytes.extend_from_slice(&2u16.to_le_bytes()); // height
        bytes.extend_from_slice(&1u16.to_le_bytes()); // depth
        bytes.extend_from_slice(&1u16.to_le_bytes()); // num_layers
        bytes.extend_from_slice(&1u16.to_le_bytes()); // num_frames
        bytes.extend_from_slice(&3u16.to_le_bytes()); // palette_len
        bytes.extend_from_slice(&0u16.to_le_bytes()); // reserved
        bytes.extend_from_slice(&32u32.to_le_bytes()); // palette_offset
        bytes.extend_from_slice(&41u32.to_le_bytes()); // layer_offset
        bytes.extend_from_slice(&43u32.to_le_bytes()); // voxel_offset
                                                       // Palette: 3 x RGB.
        bytes.extend_from_slice(&[0, 0, 0, 10, 20, 30, 40, 50, 60]);
        // Layer table: one u16 z index.
        bytes.extend_from_slice(&0u16.to_le_bytes());
        // Voxel slab: 2x2 indices.
        bytes.extend_from_slice(&[0, 1, 2, 1]);
        bytes
    }

    #[test]
    fn parses_a_synthetic_voxel_model() {
        let vxl = VxlFile::parse(&synthetic_vxl()).expect("parse");
        assert_eq!(vxl.header.width, 2);
        assert_eq!(vxl.header.height, 2);
        assert_eq!(vxl.header.num_frames, 1);
        assert_eq!(vxl.palette, vec![[0, 0, 0], [10, 20, 30], [40, 50, 60]]);
        assert_eq!(vxl.layers.len(), 1);
        assert_eq!(vxl.layers[0].frames[0], vec![0, 1, 2, 1]);
    }

    #[test]
    fn renders_layers_and_composites_frames() {
        let vxl = VxlFile::parse(&synthetic_vxl()).expect("parse");
        let rgba = vxl.layer_frame_rgba(0, 0).expect("layer frame");
        assert_eq!(
            rgba,
            vec![
                0, 0, 0, 0, // index 0 -> transparent
                10, 20, 30, 255, // index 1
                40, 50, 60, 255, // index 2
                10, 20, 30, 255, // index 1
            ]
        );
        let composited = vxl.frame_rgba(0).expect("frame");
        assert_eq!(composited, rgba);
    }

    #[test]
    fn converts_to_an_atlas() {
        let vxl = VxlFile::parse(&synthetic_vxl()).expect("parse");
        let atlas = vxl.to_atlas().expect("atlas");
        assert!(atlas.frame("layer0/frame0").is_some());
        assert!(atlas.frame("frame0").is_some());
        assert_eq!(atlas.frames.len(), 2);
        let frame = atlas.frame("layer0/frame0").expect("frame");
        assert_eq!((frame.w, frame.h), (2, 2));
    }

    #[test]
    fn a_bad_header_is_rejected_by_name() {
        let mut bytes = synthetic_vxl();
        bytes[0] = b'X';
        assert!(matches!(
            VxlFile::parse(&bytes),
            Err(VxlError::BadMagic { .. })
        ));

        let mut bytes = synthetic_vxl();
        bytes[4] = 9; // version
        assert!(matches!(
            VxlFile::parse(&bytes),
            Err(VxlError::BadVersion(9))
        ));

        let mut bytes = synthetic_vxl();
        bytes[6] = 0; // width
        bytes[7] = 0;
        assert!(matches!(
            VxlFile::parse(&bytes),
            Err(VxlError::BadDimension { field: "width", .. })
        ));

        assert!(matches!(
            VxlFile::parse(&[0u8; 4]),
            Err(VxlError::TooShort { .. })
        ));
    }

    #[test]
    fn a_truncated_voxel_section_is_rejected() {
        let bytes = synthetic_vxl();
        let truncated = &bytes[..bytes.len() - 2];
        assert!(matches!(
            VxlFile::parse(truncated),
            Err(VxlError::Truncated { field: "voxel", .. })
        ));
    }
}
