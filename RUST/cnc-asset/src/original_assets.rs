//! Native reading of the C&C Remastered Steam assets.
//!
//! The Remaster ships the original terrain art inside `Data/TEXTURES_TD_SRGB.MEG`
//! (and the RA equivalent), a Westwood Megafile whose members are NVTT-written
//! DXT5 DDS textures. This module provides the native path:
//!
//! - locate the Steam install and the per-theater texture archive,
//! - map an original template icon (e.g. `SH1` icon 0 of the Temperate theater)
//!   to its member name and decode it to RGBA,
//! - normalize multi-mip members down to their single largest fully-present
//!   level, because the shipped chains are padded into the next payload slot and
//!   the `image` crate mis-reads multi-mip DXT5 data offsets.
//!
//! Container layout (reverse-engineered, see `.omo/evidence/`):
//!
//! ```text
//! 0x00  u32 0xFFFFFFFF        marker
//! 0x04  u32 0x3F7D70A4        magic
//! 0x08  u32 first payload offset (data start)
//! 0x0C  u32 member count (= name count)
//! 0x10  u32 member count (repeat)
//! 0x14  u32 name table byte size (names span 0x18 .. 0x18 + this)
//! 0x18  N x { u16 len, bytes }  NUL-free paths, packed back to back
//! names_end + 14
//!       N x { u32 offset, u32 a, u32 b, u32 index(1-based), u32 c }
//!       (a/b/c are per-member metadata — compressed/uncompressed sizes, CRC —
//!        not needed to extract: members are raw between this record's offset
//!        and the next record's offset; the final member runs to EOF)
//! ```
//!
//! Names pair with records **positionally**: `names[i]` ↔ `record[i]`. Member
//! names look like
//! `DATA\ART\TEXTURES\SRGB\TIBERIAN_DAWN\TERRAIN\TEMPERATE\SH1\SH1.TEM-0000-00.DDS`
//! — `TEMPLATE.THEATER-MIP-ICON.DDS`, one file per (mip level, template icon).

use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Highest mip level index a member can carry.
///
/// Single-icon families ship one DDS per level: TD stops at 7, RA goes to 19
/// (`CLEAR1.TEM-0019.DDS`). Walking to 20 covers both.
pub const MAX_MIP_LEVELS: u32 = 20;

/// Terrain texture archives of the Remaster, per game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Game {
    TiberianDawn,
    RedAlert,
}

impl Game {
    /// Name of the texture archive inside the Steam `Data` directory.
    pub fn texture_meg(self) -> &'static str {
        match self {
            Self::TiberianDawn => "TEXTURES_TD_SRGB.MEG",
            Self::RedAlert => "TEXTURES_RA_SRGB.MEG",
        }
    }

    /// Path fragment members are stored under.
    pub fn member_root(self) -> &'static str {
        match self {
            Self::TiberianDawn => "TIBERIAN_DAWN",
            Self::RedAlert => "RED_ALERT",
        }
    }
}

/// Theater selector, named after the archive's directory and member suffix.
///
/// TD ships `TEMPERATE`/`WINTER`/`DESERT` (`TEM`/`WIN`/`DES`); RA ships
/// `TEMPERATE`/`SNOW`/`INTERIOR` (`TEM`/`SNO`/`INT`). The games share the
/// `TEMPERATE` spelling but spell their cold art differently, so the variant
/// names carry both the directory and the file extension to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Theater {
    /// `TEMPERATE` / `TEM` — both games.
    Temperate,
    /// TD `WINTER` / `WIN`.
    Winter,
    /// RA `SNOW` / `SNO`.
    Snow,
    /// TD `DESERT` / `DES`.
    Desert,
    /// RA `INTERIOR` / `INT`.
    Interior,
    /// TD `JUNGLE` / `JUN` (declared but never shipped).
    Jungle,
}

impl Theater {
    /// Member path component (`TEMPERATE`) and file extension (`TEM`).
    pub fn components(self) -> (&'static str, &'static str) {
        match self {
            Self::Temperate => ("TEMPERATE", "TEM"),
            Self::Winter => ("WINTER", "WIN"),
            Self::Snow => ("SNOW", "SNO"),
            Self::Desert => ("DESERT", "DES"),
            Self::Interior => ("INTERIOR", "INT"),
            Self::Jungle => ("JUNGLE", "JUN"),
        }
    }

    /// Theaters a game actually ships, in preference order.
    pub fn for_game(game: Game) -> &'static [Theater] {
        match game {
            Game::TiberianDawn => &[Theater::Temperate, Theater::Winter, Theater::Desert],
            Game::RedAlert => &[Theater::Temperate, Theater::Snow, Theater::Interior],
        }
    }

    /// Parse a theater from a scenario/INI spelling.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "TEMPERATE" => Some(Self::Temperate),
            "WINTER" => Some(Self::Winter),
            "SNOW" => Some(Self::Snow),
            "DESERT" => Some(Self::Desert),
            "INTERIOR" => Some(Self::Interior),
            "JUNGLE" => Some(Self::Jungle),
            _ => None,
        }
    }
}

/// Handle to the shipped texture archive of one game.
pub struct OriginalAssets {
    /// Member name → (payload offset, payload end).
    members: HashMap<String, (usize, usize)>,
    data: Vec<u8>,
    game: Game,
}

impl OriginalAssets {
    /// Open `Data/<TEXTURES_*.MEG>` from a Remaster install root.
    pub fn open(install_root: &Path, game: Game) -> Result<Self> {
        let path = install_root.join("Data").join(game.texture_meg());
        let data = std::fs::read(&path)
            .with_context(|| format!("reading Remaster texture archive {}", path.display()))?;
        let members =
            index_shipped_archive(&data).with_context(|| format!("indexing {}", path.display()))?;
        Ok(Self {
            members,
            data,
            game,
        })
    }

    /// Open the Steam install if it is present on this machine.
    pub fn open_default(game: Game) -> Result<Self> {
        let root = steam_install_root().ok_or_else(|| {
            anyhow::anyhow!("C&C Remastered Steam install not found on this machine")
        })?;
        Self::open(&root, game)
    }

    /// Total member count (sanity check / diagnostics).
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// All member names (diagnostics / discovery).
    pub fn member_names(&self) -> Vec<&str> {
        self.members.keys().map(|s| s.as_str()).collect()
    }

    /// Which game's textures this archive holds.
    pub fn game(&self) -> Game {
        self.game
    }

    /// Raw member bytes for one template icon.
    ///
    /// `template` is the original INI name (`"SH1"`), `icon` the flat icon index
    /// (`row * template_width + col`), `mip` the mip level (0 = largest).
    pub fn icon_bytes(
        &self,
        template: &str,
        icon: u32,
        theater: Theater,
        mip: u32,
    ) -> Result<Vec<u8>> {
        let (dir_ext, file_ext) = theater.components();
        // `SH1` -> `SH01` (RA zero-pads two-digit shoreline families).
        let padded = |name: &str| -> String {
            if name.len() > 2 {
                let (prefix, digits) = name.split_at(name.len() - 2);
                if digits.chars().all(|c| c.is_ascii_digit()) && !digits.starts_with('0') {
                    return format!("{prefix}0{digits}");
                }
            }
            name.to_string()
        };
        // The family directory is spelled `<TEMPLATE>` in TD and
        // `<TEMPLATE>.<EXT>` in RA (`SH1` vs `SH01.TEM`); the file itself always
        // uses `<TEMPLATE>.<EXT>-<mip>[-<icon>].DDS`. RA also zero-pads the
        // template name, so probe every combination rather than guessing.
        let prefix = format!(
            r"DATA\ART\TEXTURES\SRGB\{}\TERRAIN\{}",
            self.game.member_root(),
            dir_ext,
        );
        let roots: Vec<String> = [template.to_string(), padded(template)]
            .iter()
            .flat_map(|tpl| {
                // The family directory is either `<TPL>` (TD) or `<TPL>.<EXT>`
                // (RA); the file basename is always `<TPL>.<EXT>`.
                [
                    format!("{prefix}\\{tpl}\\{tpl}.{file_ext}"),
                    format!("{prefix}\\{tpl}.{file_ext}\\{tpl}.{file_ext}"),
                ]
            })
            .collect();
        // Level 0 is the sharpest image. Single-icon families ship a flat file
        // per level (`CLEAR1.TEM-0000.DDS` … `-0019.DDS` — RA carries 20 levels,
        // TD 8); icon families ship one per (level, icon). Walk outward from the
        // requested level so callers asking for level 0 get the sharpest art.
        for level in mip..MAX_MIP_LEVELS {
            for base in &roots {
                // Icon indices appear with two or four digits depending on
                // family (SH1.TEM-0004-00.DDS vs FALLS1.TEM-0000-0000.DDS).
                for width in [2usize, 4] {
                    let icon_name = format!("{base}-{level:04}-{icon:0width$}.DDS");
                    if let Some((start, end)) = self.members.get(&icon_name) {
                        return Ok(self.data[*start..*end].to_vec());
                    }
                }
                let flat_name = format!("{base}-{level:04}.DDS");
                if let Some((start, end)) = self.members.get(&flat_name) {
                    return Ok(self.data[*start..*end].to_vec());
                }
            }
        }
        bail!(
            "texture member {}.[EXT]-[0-{MAX_MIP_LEVELS}]-[icon].DDS not in archive",
            template
        )
    }

    /// Decode one template icon to RGBA at its largest available level.
    ///
    /// Multi-mip members are reduced to level 0 before decoding: the shipped
    /// chains are padded into the following payload slot, and decoding a
    /// multi-mip DXT5 through the image crate reads the wrong data offset.
    pub fn icon_rgba(
        &self,
        template: &str,
        icon: u32,
        theater: Theater,
    ) -> Result<(Vec<u8>, u32, u32)> {
        let bytes = self.icon_bytes(template, icon, theater, 0)?;
        let normalized = normalize_dds(&bytes);
        let img = image::load_from_memory(&normalized)
            .with_context(|| format!("decoding {} icon {icon}", template))?
            .to_rgba8();
        let (w, h) = (img.width(), img.height());
        Ok((img.into_raw(), w, h))
    }
}

/// Index a shipped texture archive: member name → (start, end) payload offsets.
///
/// Layout (see module docs): header, u16-prefixed name table, then one 20-byte
/// record per member. Names and records pair positionally; a member's bytes run
/// from its record's `offset` to the next record's `offset` (the last member
/// runs to EOF). Records carry a 1-based index field which is validated.
pub fn index_shipped_archive(data: &[u8]) -> Result<HashMap<String, (usize, usize)>> {
    if data.len() < 0x18 {
        bail!("texture archive too small");
    }
    if u32_at(data, 0) != 0xFFFF_FFFF || u32_at(data, 4) != 0x3F7D_70A4 {
        bail!("not a shipped texture archive (bad magic)");
    }
    let count = u32_at(data, 0x0C) as usize;
    let names_size = u32_at(data, 0x14) as usize;
    let names_end = 0x18 + names_size;
    if names_end > data.len() {
        bail!("name table exceeds archive bounds");
    }

    // Name table: u16-prefixed paths packed back to back.
    let mut names: Vec<String> = Vec::with_capacity(count);
    let mut pos = 0x18;
    while pos + 2 <= names_end {
        let len = u16_at(data, pos) as usize;
        pos += 2;
        if len == 0 || pos + len > names_end {
            bail!("corrupt name table at offset {pos:#x}");
        }
        let nm = &data[pos..pos + len];
        if !nm.iter().all(|&c| c.is_ascii_graphic()) {
            bail!("non-ASCII member name in table at offset {pos:#x}");
        }
        names.push(String::from_utf8_lossy(nm).into_owned());
        pos += len;
    }
    if names.len() != count {
        bail!(
            "name table holds {} names, header declares {count}",
            names.len()
        );
    }

    // The header count covers the names; the record table holds one record per
    // real member. The shipped archives have `count - 1` records: the final
    // name is a terminator whose payload simply runs to EOF from the previous
    // member's end. Records start 14 bytes after the name table.
    let records = names_end + 14;
    if records + count * 20 > data.len() + 20 {
        bail!("record table exceeds archive bounds");
    }

    // The shipped archives pair every name with a record except the final one,
    // whose slot is payload data: the 1-based index field stops validating at
    // `count - 1`. Accept either `count` or `count - 1` valid records.
    let mut members = HashMap::with_capacity(count);
    let mut offsets: Vec<usize> = Vec::with_capacity(count + 1);
    for i in 0..count {
        let rec = records + i * 20;
        if rec + 20 > raw_len_guard(data) {
            break;
        }
        let index = u32_at(data, rec + 12) as usize;
        if index != i + 1 {
            break;
        }
        offsets.push(u32_at(data, rec) as usize);
    }
    // The last payload runs to EOF when the final name carries no record.
    offsets.push(data.len());

    // `names.len() - offsets.len() + 1` names carry payload: the remaining names
    // are terminators with no bytes of their own.
    for (name, window) in names.into_iter().zip(offsets.windows(2)) {
        let (start, end) = (window[0], window[1]);
        if start >= end || end > data.len() {
            bail!("member {name} has empty or out-of-bounds payload");
        }
        members.insert(name, (start, end));
    }
    Ok(members)
}

/// Bound for record reads: leave room for a full 20-byte record.
fn raw_len_guard(data: &[u8]) -> usize {
    data.len()
}

fn u16_at(d: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([d[o], d[o + 1]])
}

fn u32_at(d: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([d[o], d[o + 1], d[o + 2], d[o + 3]])
}

/// Encode a shipped-style texture archive from named members.
///
/// This is the inverse of [`index_shipped_archive`]: header, u16-prefixed name
/// table, 14-byte gap, then one 20-byte record per member followed by the raw
/// payloads in name order. Members are written uncompressed, so each record's
/// metadata fields (stored/unpacked/full size) all carry the member length.
///
/// Returns the archive bytes.
pub fn write_shipped_archive(members: &[(String, Vec<u8>)]) -> Result<Vec<u8>> {
    if members.is_empty() {
        bail!("cannot encode an empty texture archive");
    }
    let count = members.len();
    if count < 1 {
        bail!("shipped archive needs at least one member");
    }
    let records = count;
    let names_size: usize = members.iter().map(|(n, _)| 2 + n.len()).sum();
    let data_start = 0x18 + names_size + 14 + records * 20;

    let payload: usize = members.iter().map(|(_, b)| b.len()).sum();
    let mut out = Vec::with_capacity(data_start + payload);
    out.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes()); // marker
    out.extend_from_slice(&0x3F7D_70A4u32.to_le_bytes()); // magic
    out.extend_from_slice(&0u32.to_le_bytes()); // data start, patched below
    out.extend_from_slice(&(count as u32).to_le_bytes());
    out.extend_from_slice(&(count as u32).to_le_bytes());
    out.extend_from_slice(&(names_size as u32).to_le_bytes());

    for (name, _) in members {
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(name.as_bytes());
    }
    // 14-byte gap between the name table and the record table.
    out.extend_from_slice(&[0u8; 14]);

    // Records cover every member except the terminator: the last payload runs
    // to EOF with no record of its own, mirroring the shipped layout.
    let mut offset = data_start;
    let mut offsets = Vec::with_capacity(count);
    for (_name, body) in members.iter() {
        offsets.push(offset);
        offset += body.len();
    }
    for (i, (_name, body)) in members.iter().enumerate() {
        let len = body.len() as u32;
        out.extend_from_slice(&(offsets[i] as u32).to_le_bytes()); // offset
        out.extend_from_slice(&len.to_le_bytes()); // metadata: stored size
        out.extend_from_slice(&len.to_le_bytes()); // metadata: unpacked size
        out.extend_from_slice(&((i + 1) as u32).to_le_bytes()); // 1-based index
        out.extend_from_slice(&len.to_le_bytes()); // metadata: full size
    }
    for (_, body) in members {
        out.extend_from_slice(body);
    }
    // Patch the data-start field now that the header size is fixed.
    out[8..12].copy_from_slice(&(data_start as u32).to_le_bytes());
    Ok(out)
}

/// Encode an RGBA8 buffer as a single-level DXT5 DDS in the shipped (NVTT)
/// layout: 124-byte header, 4-byte marker, then BC3 blocks. Produces files the
/// shipped pipeline and standard decoders read back.
///
/// Block compression is the fixed min/max-endpoint scheme — lossy, matching the
/// shipped members' format.
pub fn encode_dds_dxt5(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
    if width == 0 || height == 0 || width % 4 != 0 || height % 4 != 0 {
        bail!("DXT5 dimensions must be positive multiples of 4");
    }
    if rgba.len() != (width * height * 4) as usize {
        bail!(
            "RGBA payload is {} bytes, expected {} for {width}x{height}",
            rgba.len(),
            width * height * 4
        );
    }
    let mut out = Vec::with_capacity(128 + (width / 4 * height / 4 * 16) as usize);
    out.extend_from_slice(b"DDS ");
    out.extend_from_slice(&124u32.to_le_bytes()); // header size
                                                  // Flags: CAPS | HEIGHT | WIDTH | PIXELFORMAT | LINEARSIZE
    out.extend_from_slice(&0x000A_1007u32.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&(width * height / 2).to_le_bytes()); // linear size
    out.extend_from_slice(&0u32.to_le_bytes()); // depth
    out.extend_from_slice(&1u32.to_le_bytes()); // mip levels
    out.extend_from_slice(&[0u8; 44]); // reserved
    out.extend_from_slice(&32u32.to_le_bytes()); // pixel format size
    out.extend_from_slice(&0x4u32.to_le_bytes()); // DDPF_FOURCC
    out.extend_from_slice(b"DXT5");
    out.extend_from_slice(&[0u8; 20]); // bitcount + RGBA masks (unused for FOURCC)
    out.extend_from_slice(&0x1000u32.to_le_bytes()); // caps: DDSCAPS_TEXTURE
    out.extend_from_slice(&0u32.to_le_bytes()); // caps2
    out.extend_from_slice(&[0u8; 12]); // caps3, caps4, reserved2
    debug_assert_eq!(out.len(), 128);

    let px = |x: u32, y: u32| -> [u8; 4] {
        let o = ((y * width + x) * 4) as usize;
        [rgba[o], rgba[o + 1], rgba[o + 2], rgba[o + 3]]
    };
    for by in 0..height / 4 {
        for bx in 0..width / 4 {
            let mut block = [0u8; 16];
            encode_bc3_block(&mut block, |corner| {
                px(bx * 4 + corner.0, by * 4 + corner.1)
            });
            out.extend_from_slice(&block);
        }
    }
    Ok(out)
}

/// Encode one 4x4 block into BC3 layout: 8-byte alpha ramp (two endpoints +
/// 16 3-bit indices, packed into bytes 0..8) then 8-byte color (RGB565
/// endpoints + 16 2-bit indices, bytes 8..16).
fn encode_bc3_block(block: &mut [u8; 16], sample: impl Fn((u32, u32)) -> [u8; 4]) {
    let mut colors = [[0u8; 3]; 16];
    let mut lum = [0u32; 16];
    for i in 0..16usize {
        let p = sample(((i % 4) as u32, (i / 4) as u32));
        colors[i] = [p[0], p[1], p[2]];
        lum[i] = p[0] as u32 * 299 + p[1] as u32 * 587 + p[2] as u32 * 114;
    }
    // Color endpoints: the two samples furthest apart on the luminance axis.
    let (mut lo, mut hi) = (0usize, 0usize);
    let mut best = 0u32;
    for a in 0..16 {
        for b in (a + 1)..16 {
            let d = lum[a].abs_diff(lum[b]);
            if d > best {
                best = d;
                lo = a;
                hi = b;
            }
        }
    }
    let pack565 = |c: [u8; 3]| {
        ((u16::from(c[0]) >> 3) << 11) | ((u16::from(c[1]) >> 2) << 5) | (u16::from(c[2]) >> 3)
    };
    // Endpoint c0 must be the brighter sample for the four-color mode.
    let (c0, c1) = if lum[lo] > lum[hi] {
        (colors[lo], colors[hi])
    } else {
        (colors[hi], colors[lo])
    };
    block[8..10].copy_from_slice(&pack565(c0).to_le_bytes());
    block[10..12].copy_from_slice(&pack565(c1).to_le_bytes());
    let mut palette = [[0u8; 3]; 4];
    palette[0] = c0;
    palette[1] = c1;
    for i in 0..3 {
        palette[2][i] = ((2 * u16::from(c0[i]) + u16::from(c1[i])) / 3) as u8;
        palette[3][i] = ((u16::from(c0[i]) + 2 * u16::from(c1[i])) / 3) as u8;
    }
    let mut indices = 0u32;
    for (i, c) in colors.iter().enumerate() {
        let k = (0..4usize)
            .min_by_key(|n| {
                let p = palette[*n];
                i32::from(p[0]).abs_diff(i32::from(c[0]))
                    + i32::from(p[1]).abs_diff(i32::from(c[1]))
                    + i32::from(p[2]).abs_diff(i32::from(c[2]))
            })
            .unwrap();
        indices |= (k as u32) << (2 * i);
    }
    block[12..16].copy_from_slice(&indices.to_le_bytes());

    // Alpha endpoints: min/max of the 16 samples; indices into the 8-step ramp.
    let alphas: [u8; 16] = std::array::from_fn(|i| sample(((i % 4) as u32, (i / 4) as u32))[3]);
    let amin = alphas.iter().copied().min().unwrap_or(u8::MAX);
    let amax = alphas.iter().copied().max().unwrap_or(0);
    let mut alut = [0u8; 8];
    alut[0] = amax;
    alut[1] = amin;
    if amax > amin {
        for i in 0..6u32 {
            alut[2 + i as usize] = ((u32::from(amax) * (5 - i) + u32::from(amin) * i) / 5) as u8;
        }
    }
    let mut bits = 0u64;
    for (i, a) in alphas.iter().enumerate() {
        let k = (0..8u32)
            .min_by_key(|n| alut[*n as usize].abs_diff(*a))
            .unwrap();
        bits |= (k as u64) << (3 * i);
    }
    block[2..8].copy_from_slice(&bits.to_le_bytes()[..6]);
    block[0] = amax;
    block[1] = amin;
}

/// Locate the C&C Remastered install under the local Steam library.
/// Locate the C&C Remastered install under the local Steam library.
pub fn steam_install_root() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let mut candidates = vec![
        PathBuf::from(&home).join(".steam/steam/steamapps/common/CnCRemastered"),
        PathBuf::from(&home).join(".local/share/Steam/steamapps/common/CnCRemastered"),
    ];
    // Also scan libraryfolders.vdf for extra libraries.
    let vdf = PathBuf::from(&home).join(".steam/steam/steamapps/libraryfolders.vdf");
    if let Ok(text) = std::fs::read_to_string(&vdf) {
        for line in text.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("\"path\"") {
                if let Some(start) = rest.find('"') {
                    if let Some(end) = rest[start + 1..].find('"') {
                        let dir = &rest[start + 1..start + 1 + end];
                        candidates.push(
                            PathBuf::from(dir.replace("\\\\", "/"))
                                .join("steamapps/common/CnCRemastered"),
                        );
                    }
                }
            }
        }
    }
    candidates.into_iter().find(|c| c.join("Data").is_dir())
}

/// Reduce a (possibly multi-mip) DXT5 DDS member to its single largest
/// fully-present level, rewritten as a standard single-level DXT5 file whose
/// pixel data begins at `header_size + 4` where decoders expect it.
///
/// The shipped members carry a 124-byte header plus a 4-byte NVTT marker, so
/// their pixel data starts at 128. The mip chain after level 0 can be padded
/// into the next archive slot, so only levels whose bytes are present are
/// considered; the largest such level wins.
pub fn normalize_dds(data: &[u8]) -> Vec<u8> {
    if data.len() < 132 || &data[..4] != b"DDS " {
        return data.to_vec();
    }
    let header_size = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    let height = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
    let width = u32::from_le_bytes(data[16..20].try_into().unwrap()) as usize;
    let mips = u32::from_le_bytes(data[28..32].try_into().unwrap()) as usize;
    let data_start = usize::max(124, header_size) + 4;
    let avail = data.len().saturating_sub(data_start);

    // Largest declared level whose bytes are fully present. A single-level
    // member (mips == 1) declares its size directly; a multi-mip chain steps
    // down from the declared base.
    let base = usize::min(height, width);
    let mut side = 4usize;
    if mips <= 1 {
        if (base / 4).pow(2) * 16 <= avail {
            side = usize::max(4, base);
        }
    } else {
        for level in 0..mips {
            let s = usize::max(4, base >> level);
            if (s / 4).pow(2) * 16 > avail {
                continue;
            }
            side = s;
            break;
        }
    }

    let keep = data_start + (side / 4).pow(2) * 16;
    let mut out = data[..usize::min(keep, data.len())].to_vec();
    // Standard single-level header: data begins at 124 + 4.
    out[4..8].copy_from_slice(&124u32.to_le_bytes());
    out[12..16].copy_from_slice(&(side as u32).to_le_bytes());
    out[16..20].copy_from_slice(&(side as u32).to_le_bytes());
    out[28..32].copy_from_slice(&1u32.to_le_bytes());
    out[108..112].copy_from_slice(&0x0000_1000u32.to_le_bytes()); // DDSCAPS_TEXTURE
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped_archive() -> Option<OriginalAssets> {
        OriginalAssets::open_default(Game::TiberianDawn).ok()
    }

    fn shipped_archive_of(game: Game) -> Option<OriginalAssets> {
        OriginalAssets::open_default(game).ok()
    }

    #[test]
    fn normalize_keeps_single_level_files() {
        // A well-formed single-level DDS passes through unchanged apart from
        // the caps field. Payload must cover the declared level: 64x64 DXT5.
        let mut data = vec![0u8; 128 + 4096];
        data[..4].copy_from_slice(b"DDS ");
        data[4..8].copy_from_slice(&124u32.to_le_bytes());
        data[12..16].copy_from_slice(&64u32.to_le_bytes());
        data[16..20].copy_from_slice(&64u32.to_le_bytes());
        data[28..32].copy_from_slice(&1u32.to_le_bytes());
        let out = normalize_dds(&data);
        assert_eq!(out.len(), data.len());
        assert_eq!(u32::from_le_bytes(out[28..32].try_into().unwrap()), 1);
        assert_eq!(u32::from_le_bytes(out[12..16].try_into().unwrap()), 64);
    }

    #[test]
    fn normalize_drops_unavailable_mips() {
        // A 128x128 member declaring 8 mips but carrying only level 0.
        let mut data = vec![0u8; 128 + 16384];
        data[..4].copy_from_slice(b"DDS ");
        data[4..8].copy_from_slice(&124u32.to_le_bytes());
        data[12..16].copy_from_slice(&128u32.to_le_bytes());
        data[16..20].copy_from_slice(&128u32.to_le_bytes());
        data[28..32].copy_from_slice(&8u32.to_le_bytes());
        data[108..112].copy_from_slice(&0x401008u32.to_le_bytes());
        let out = normalize_dds(&data);
        assert_eq!(out.len(), data.len());
        assert_eq!(u32::from_le_bytes(out[28..32].try_into().unwrap()), 1);
        assert_eq!(u32::from_le_bytes(out[12..16].try_into().unwrap()), 128);
        assert_eq!(
            u32::from_le_bytes(out[108..112].try_into().unwrap()),
            0x1000
        );
    }

    #[test]
    fn member_name_layout() {
        // The member path must match the shipped archive layout exactly.
        let (dir, ext) = Theater::Temperate.components();
        assert_eq!(dir, "TEMPERATE");
        assert_eq!(ext, "TEM");
        let (dir, ext) = Theater::Winter.components();
        assert_eq!(dir, "WINTER");
        assert_eq!(ext, "WIN");
    }

    #[test]
    fn shipped_archive_roundtrip() {
        // Encode members, then read them back through the real indexer.
        let members = vec![
            (
                r"DATA\ART\TEXTURES\SRGB\TIBERIAN_DAWN\TERRAIN\TEMPERATE\B1\B1.TEM-0000.DDS"
                    .to_string(),
                b"\x01\x02\x03 member-one payload".to_vec(),
            ),
            (
                r"DATA\ART\TEXTURES\SRGB\TIBERIAN_DAWN\TERRAIN\WINTER\W1\W1.WIN-0000.DDS"
                    .to_string(),
                vec![7u8; 5000],
            ),
        ];
        let bytes = write_shipped_archive(&members).unwrap();
        let index = index_shipped_archive(&bytes).unwrap();
        assert_eq!(index.len(), members.len());
        for (name, body) in &members {
            let (start, end) = index[name];
            assert_eq!(&bytes[start..end], body.as_slice());
        }
    }

    #[test]
    fn shipped_archive_rejects_bad_magic() {
        let members = vec![
            ("A".to_string(), vec![1u8, 2, 3]),
            ("B".to_string(), vec![4u8, 5]),
        ];
        let mut bytes = write_shipped_archive(&members).unwrap();
        bytes[4] = 0;
        assert!(index_shipped_archive(&bytes).is_err());
    }

    #[test]
    fn shipped_archive_rejects_empty() {
        assert!(write_shipped_archive(&[]).is_err());
    }

    #[test]
    fn dds_dxt5_roundtrip() {
        // 8x8 gradient with alpha; encode then decode and compare channel
        // ordering (lossy BC3, so compare statistically, not exactly).
        let mut rgba = vec![0u8; 8 * 8 * 4];
        for y in 0..8u32 {
            for x in 0..8u32 {
                let o = ((y * 8 + x) * 4) as usize;
                rgba[o] = (x * 28) as u8; // red ramps with x
                rgba[o + 1] = 0;
                rgba[o + 2] = (y * 28) as u8; // blue ramps with y
                rgba[o + 3] = 255;
            }
        }
        let dds = encode_dds_dxt5(&rgba, 8, 8).unwrap();
        let img = image::load_from_memory(&dds)
            .expect("decode own DDS")
            .to_rgba8();
        assert_eq!((img.width(), img.height()), (8, 8));
        let px = |x: u32, y: u32| {
            let p = img.get_pixel(x, y);
            (p[0], p[1], p[2], p[3])
        };
        // Red must dominate on the right, blue on the bottom, alpha opaque.
        let right = px(6, 1);
        assert!(
            right.0 > right.2,
            "red channel must lead on the right: {right:?}"
        );
        let bottom = px(1, 6);
        assert!(
            bottom.2 > bottom.0,
            "blue channel must lead on the bottom: {bottom:?}"
        );
        assert_eq!(px(0, 0).3, 255);
    }

    #[test]
    fn dds_dxt5_rejects_bad_dimensions() {
        assert!(encode_dds_dxt5(&[0u8; 6 * 6 * 4], 6, 6).is_err());
        assert!(encode_dds_dxt5(&[0u8; 4], 4, 4).is_err());
    }

    #[test]
    fn shipped_member_decodes_after_roundtrip() {
        // Encode a shipped member's own bytes back into an archive and read it.
        let Some(assets) = shipped_archive() else {
            return;
        };
        let name =
            r"DATA\ART\TEXTURES\SRGB\TIBERIAN_DAWN\TERRAIN\TEMPERATE\SH1\SH1.TEM-0000-00.DDS";
        let original = assets
            .members
            .get(name)
            .map(|(s, e)| assets.data[*s..*e].to_vec())
            .unwrap();
        let bytes = write_shipped_archive(&[(name.to_string(), original)]).unwrap();
        let index = index_shipped_archive(&bytes).unwrap();
        let (start, end) = index[name];
        assert_eq!(
            &bytes[start..end],
            &assets.data[assets.members[name].0..assets.members[name].1]
        );
    }

    #[test]
    fn red_alert_templates_resolve_when_archive_present() {
        let Some(assets) = shipped_archive_of(Game::RedAlert) else {
            return; // Red Alert archive not installed
        };
        // RA spells its ocean shores zero-padded and puts the theater code in
        // the family directory (`SH01.TEM\SH01.TEM-...`).
        for tpl in ["CLEAR1", "P01", "W1", "SH01", "SH02", "S01", "D01", "RV01"] {
            let (rgba, w, h) = assets
                .icon_rgba(tpl, 0, Theater::Temperate)
                .unwrap_or_else(|e| panic!("{tpl}: {e:#}"));
            assert_eq!((w, h), (128, 128));
            assert_eq!(rgba.len(), (w * h * 4) as usize);
        }
    }

    #[test]
    fn archive_theater_names_differ_by_game() {
        // TD ships WINTER/DESERT; RA ships SNOW/INTERIOR.
        let td = Theater::for_game(Game::TiberianDawn);
        assert!(td.contains(&Theater::Winter));
        assert!(td.contains(&Theater::Desert));
        assert!(!td.contains(&Theater::Snow));
        let ra = Theater::for_game(Game::RedAlert);
        assert!(ra.contains(&Theater::Snow));
        assert!(ra.contains(&Theater::Interior));
        assert!(!ra.contains(&Theater::Winter));
        assert_eq!(Theater::Snow.components(), ("SNOW", "SNO"));
        assert_eq!(Theater::Interior.components(), ("INTERIOR", "INT"));
    }

    #[test]
    fn pack_slots_resolve_for_both_games() {
        // The semantic rows each game's composite must produce.
        for game in [Game::TiberianDawn, Game::RedAlert] {
            let Some(pack) =
                crate::tileset_pack::TilesetPack::load_original(game, Theater::Temperate)
            else {
                continue; // archive not installed
            };
            for fam in ["grass", "dirt", "sand", "rock", "deep_water", "shore_north"] {
                let row = if fam == "deep_water" || fam.starts_with("shore") {
                    pack.family_row("water_edges", fam)
                } else {
                    pack.family_row("terrain", fam)
                };
                assert!(row.is_some(), "{game:?} pack lacks family {fam}");
            }
        }
    }

    #[test]
    fn decodes_shore_icon_when_archive_present() {
        let Some(assets) = shipped_archive() else {
            return; // shipped assets not installed on this machine
        };
        assert!(assets.member_count() > 10_000);
        let (rgba, w, h) = assets.icon_rgba("SH1", 0, Theater::Temperate).unwrap();
        assert_eq!((w, h), (128, 128));
        assert_eq!(rgba.len(), (w * h * 4) as usize);
        // SH1 icon 0: sand at the top-left, water at the bottom-right. Sand is
        // warmer (R > B) at the top-left and water is cooler (B > R) at the
        // bottom-right — this pins the channel order, not just the decode.
        let px = |x: u32, y: u32| {
            let o = ((y * w + x) * 4) as usize;
            (rgba[o], rgba[o + 1], rgba[o + 2])
        };
        let top_left = px(8, 8);
        let bottom_right = px(119, 119);
        assert!(
            top_left.0 > top_left.2,
            "sand must be warmer than blue at top-left: {top_left:?}"
        );
        assert!(
            bottom_right.2 > bottom_right.0,
            "water must be cooler (more blue) at bottom-right: {bottom_right:?}"
        );
    }

    #[test]
    fn missing_member_reports_name() {
        let Some(assets) = shipped_archive() else {
            return;
        };
        let err = assets
            .icon_bytes("ZZZZ", 0, Theater::Temperate, 0)
            .unwrap_err();
        assert!(err.to_string().contains("ZZZZ"));
    }

    #[test]
    fn rejects_bad_signature() {
        assert_eq!(normalize_dds(b"NOPE"), b"NOPE".to_vec());
    }
}
