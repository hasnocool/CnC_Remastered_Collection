//! Extract a Westwood MEG container (magic `a4 70 7d 3f`) to a directory.
//!
//! Layout (little-endian), reverse-engineered from `Data/TEXTURES_TD_SRGB.MEG`:
//!   0x00  u32 0xffffffff
//!   0x04  u32 magic 0x3f7d70a4
//!   0x08  u32 file size
//!   0x0c  u32 index end offset
//!   0x10  u32 index start offset
//!   0x14  u32 name-table byte size (names span 0x18 .. 0x18 + this)
//!   0x18  N x {u16 len, path bytes[]}  (NUL-free paths, packed back to back)
//!   pad to 20
//!   N x 20 bytes {u32 offset, u32 size, u32 uncompressed, u32 index, u32 tag}
//! The index is ordered by offset, not by name order: sorting the records by
//! offset before pairing them with the name table in order yields DDS payloads
//! that all begin with the `DDS ` magic.
//! Payload blobs are stored raw between offset and offset+size.
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn u16_at(d: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([d[o], d[o + 1]])
}
fn u32_at(d: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([d[o], d[o + 1], d[o + 2], d[o + 3]])
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .context("usage: meg_extract <file.meg> <outdir> [filter]")?;
    let outdir = PathBuf::from(args.next().context("missing outdir")?);
    let filter = args.next();
    let d = fs::read(&path)?;
    anyhow::ensure!(d.len() >= 0x18, "MEG too small");
    anyhow::ensure!(u32_at(&d, 4) == 0x3f7d70a4, "not a Westwood MEG");
    let names_end = 0x18 + u32_at(&d, 0x14) as usize;

    let mut names: Vec<String> = Vec::new();
    let mut pos = 0x18usize;
    while pos + 2 <= names_end {
        let len = u16_at(&d, pos) as usize;
        pos += 2;
        if len == 0 || pos + len > names_end {
            break;
        }
        let nm = &d[pos..pos + len];
        if !nm.iter().all(|&c| c.is_ascii_graphic()) {
            break;
        }
        names.push(String::from_utf8_lossy(nm).into_owned());
        pos += len;
    }

    // The name table is followed by a 14-byte trailer before the payload
    // records begin (observed as a trailing partial index record).
    let ipos = names_end + 14;

    // Collect the payload slots keyed by their sequential index field (byte 12),
    // which is authoritative for pairing with the name table. Field 16 is the
    // member's true byte length including its mip chain (a 128x128 DXT5 with 8
    // mips is 22000 bytes, not the 9287-byte base level alone).
    let mut records: Vec<(usize, usize, usize)> = Vec::with_capacity(names.len());
    for i in 0..names.len() {
        let rec = ipos + i * 20;
        if rec + 20 > d.len() {
            break;
        }
        let off = u32_at(&d, rec) as usize;
        let size = u32_at(&d, rec + 16) as usize;
        let index = u32_at(&d, rec + 12) as usize;
        if size > 0 && off + size <= d.len() && index == i + 1 {
            records.push((index, off, size));
        }
    }
    records.sort_unstable_by_key(|&(index, _, _)| index);
    records.dedup_by_key(|&mut (index, _, _)| index);

    let mut extracted = 0usize;
    let mut valid = 0usize;
    for &(index, off, size) in records.iter() {
        valid += usize::from(&d[off..off + 4] == b"DDS ");
        // Records are 1-based; the name table is 0-based.
        let Some(name) = names.get(index - 1) else {
            continue;
        };
        if let Some(f) = &filter {
            if !name.to_uppercase().contains(&f.to_uppercase()) {
                continue;
            }
        }
        let dst = outdir.join(name.replace('\\', "/"));
        fs::create_dir_all(dst.parent().unwrap())?;
        let mut fh = fs::File::create(&dst)?;
        fh.write_all(&strip_dds_mips(&d[off..off + size]))?;
        extracted += 1;
    }
    println!(
        "extracted {} of {} records ({} DDS magic) to {}",
        extracted,
        records.len(),
        valid,
        outdir.display()
    );
    Ok(())
}

/// Rewrite a DDS to hold only its single largest fully-present mip level.
///
/// The shipped members declare an 8-level mip chain, but the container only
/// guarantees each slot's bytes; the tail is padded into the following slot, so
/// a member can be shorter than its own declared chain. Decoders reject that.
/// This walks the chain from the top and keeps the first level the bytes cover,
/// hoisting the mip pointers so the result decodes as a plain single-level
/// DXT5 texture.
fn strip_dds_mips(data: &[u8]) -> Vec<u8> {
    if data.len() < 124 || &data[..4] != b"DDS " {
        return data.to_vec();
    }
    let start = std::cmp::max(
        124usize,
        u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize,
    );
    let height = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
    let width = u32::from_le_bytes(data[16..20].try_into().unwrap()) as usize;
    let mips = u32::from_le_bytes(data[28..32].try_into().unwrap()) as usize;
    // Pixel data begins 4 bytes after the declared header: the block-compressed
    // DDS write path emits a 124-byte header plus a trailing member marker.
    let start = start + 4;
    let avail = data.len().saturating_sub(start);
    let base = std::cmp::min(height, width);
    // Take the largest declared level whose bytes are covered, preferring 64px or
    // larger: a 4x4 mip upscaled into the 16x16 engine grid shows as hard
    // stripes, so it is only used when nothing larger fits.
    let mut side = 4;
    for level in 0..mips + 1 {
        let s = std::cmp::max(4usize, base >> level);
        if (s / 4).pow(2) * 16 > avail {
            continue; // level exceeds the payload; try the next-smaller level
        }
        side = s;
        break; // first fitting level wins; keep it even if small
    }
    let keep = start + (side / 4).pow(2) * 16;
    let mut out = data[..std::cmp::min(keep, data.len())].to_vec();
    if out.len() >= 128 {
        let side = side as u32;
        out[4..8].copy_from_slice(&124u32.to_le_bytes());
        out[12..16].copy_from_slice(&side.to_le_bytes()); // height
        out[16..20].copy_from_slice(&side.to_le_bytes()); // width
        out[28..32].copy_from_slice(&1u32.to_le_bytes()); // mipMapCount
        out[108..112].copy_from_slice(&0x0000_1000u32.to_le_bytes()); // DDSCAPS_TEXTURE
    }
    out
}
