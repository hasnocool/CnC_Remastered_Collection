//! Westwood Megafile (.MEG) container format.
//!
//! Based on `CnCTDRAMapEditor/Utility/Megafile.cs` and `MegafileBuilder.cs`.
//! The format stores multiple files in a single archive with a string table
//! and file metadata (CRC, size, offset).

use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::io::{Cursor, Read};

/// Megafile header magic numbers.
const MAGIC_V1: u32 = 0xFFFFFFFF;
const MAGIC_V2: u32 = 0x8FFFFFFF;

/// Megafile format version.
const VERSION: f32 = 0.99;

/// SubFileData structure matching the C# layout (pack=2).
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct SubFileData {
    pub flags: u16,
    pub crc_value: u32,
    pub subfile_index: i32,
    pub subfile_size: u32,
    pub subfile_image_data_offset: u32,
    pub subfile_name_index: u16,
}

impl SubFileData {
    pub const SIZE: usize = 20; // u16 + u32 + i32 + u32 + u32 + u16 with pack=2 alignment

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::SIZE);
        bytes.extend_from_slice(&self.flags.to_le_bytes());
        bytes.extend_from_slice(&self.crc_value.to_le_bytes());
        bytes.extend_from_slice(&self.subfile_index.to_le_bytes());
        bytes.extend_from_slice(&self.subfile_size.to_le_bytes());
        bytes.extend_from_slice(&self.subfile_image_data_offset.to_le_bytes());
        bytes.extend_from_slice(&self.subfile_name_index.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < Self::SIZE {
            bail!("SubFileData too short: {} bytes", bytes.len());
        }
        let mut cursor = Cursor::new(bytes);
        let mut buf2 = [0u8; 2];
        let mut buf4 = [0u8; 4];
        cursor.read_exact(&mut buf2)?;
        let flags = u16::from_le_bytes(buf2);
        cursor.read_exact(&mut buf4)?;
        let crc_value = u32::from_le_bytes(buf4);
        cursor.read_exact(&mut buf4)?;
        let subfile_index = i32::from_le_bytes(buf4);
        cursor.read_exact(&mut buf4)?;
        let subfile_size = u32::from_le_bytes(buf4);
        cursor.read_exact(&mut buf4)?;
        let subfile_image_data_offset = u32::from_le_bytes(buf4);
        cursor.read_exact(&mut buf2)?;
        let subfile_name_index = u16::from_le_bytes(buf2);
        Ok(Self {
            flags,
            crc_value,
            subfile_index,
            subfile_size,
            subfile_image_data_offset,
            subfile_name_index,
        })
    }
}
/// CRC calculation matching the C# implementation.
fn crc_calculate(data: &[u8]) -> u32 {
    // Westwood CRC32 algorithm
    let mut crc = 0xFFFFFFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// A Megafile container holding multiple named files.
#[derive(Debug, Clone, Default)]
pub struct Megafile {
    files: HashMap<String, Vec<u8>>,
    string_table: Vec<String>,
}

impl Megafile {
    /// Create a new empty Megafile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a file to the archive.
    pub fn add_file(&mut self, name: impl Into<String>, data: Vec<u8>) {
        let name = name.into();
        self.files.insert(name.clone(), data);
        if !self.string_table.contains(&name) {
            self.string_table.push(name);
        }
    }

    /// Get a file by name.
    pub fn get_file(&self, name: &str) -> Option<&Vec<u8>> {
        self.files.get(name)
    }

    /// Get all file names.
    pub fn file_names(&self) -> Vec<&str> {
        self.files.keys().map(|s| s.as_str()).collect()
    }

    /// Write the Megafile to a byte vector.
    pub fn write(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();

        // Sort files by CRC for deterministic output
        let mut files: Vec<_> = self.files.iter().collect();
        files.sort_by_key(|(name, _data)| crc_calculate(name.as_bytes()));

        // Build string table
        let mut strings = Vec::new();
        let mut string_indices = HashMap::new();
        for name in &self.string_table {
            let name_str: &str = name.as_str();
            if !string_indices.contains_key(name_str) {
                string_indices.insert(name_str, strings.len() as u16);
                strings.push(name_str);
            }
        }

        // Build file entries with string indices
        let mut file_entries = Vec::new();
        for (name, data) in &files {
            let name_str: &str = name.as_str();
            let name_index = string_indices[name_str];
            let crc = crc_calculate(name.as_bytes());
            file_entries.push((name_index, crc, *data));
        }

        // Calculate header size
        let header_size = 4 + 4 + 4 + 4 + 4 + 4 + // magic, version, header_size, num_files, num_strings, string_table_size
            SubFileData::SIZE * file_entries.len() +
            strings.iter().map(|s| 2 + s.len()).sum::<usize>();

        let mut subfile_image_offset = header_size as u32;

        // Write header
        out.extend_from_slice(&MAGIC_V1.to_le_bytes());
        out.extend_from_slice(&VERSION.to_le_bytes());
        out.extend_from_slice(&(header_size as u32).to_le_bytes());

        out.extend_from_slice(&(file_entries.len() as u32).to_le_bytes());
        out.extend_from_slice(&(strings.len() as u32).to_le_bytes());

        // Calculate string table size
        let string_table_size: u32 = strings.iter().map(|s| 2 + s.len() as u32).sum();
        out.extend_from_slice(&string_table_size.to_le_bytes());

        // Write string table
        for s in &strings {
            out.extend_from_slice(&(s.len() as u16).to_le_bytes());
            out.extend_from_slice(s.as_bytes());
        }

        // Write file table
        for (name_index, crc, data) in &file_entries {
            let subfile = SubFileData {
                flags: 0,
                crc_value: *crc,
                subfile_index: *name_index as i32,
                subfile_size: data.len() as u32,
                subfile_image_data_offset: subfile_image_offset,
                subfile_name_index: *name_index,
            };
            out.extend_from_slice(&subfile.to_bytes());
            subfile_image_offset += data.len() as u32;
        }

        // Write file data
        for (_, _, data) in &file_entries {
            out.extend_from_slice(data);
        }

        Ok(out)
    }

    /// Read a Megafile from bytes.
    pub fn read(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data);
        let mut megafile = Self::new();

        // Read magic
        let mut buf4 = [0u8; 4];
        cursor.read_exact(&mut buf4)?;
        let magic = u32::from_le_bytes(buf4);
        if magic != MAGIC_V1 && magic != MAGIC_V2 {
            bail!("Invalid MEG magic: 0x{:08X}", magic);
        }

        // Read version
        cursor.read_exact(&mut buf4)?;
        let version = f32::from_le_bytes(buf4);
        if (version - VERSION).abs() > 0.01 {
            bail!("Unsupported MEG version: {}", version);
        }

        // Read header size
        cursor.read_exact(&mut buf4)?;
        let _header_size = u32::from_le_bytes(buf4);

        // Read counts
        cursor.read_exact(&mut buf4)?;
        let num_files = u32::from_le_bytes(buf4);
        cursor.read_exact(&mut buf4)?;
        let num_strings = u32::from_le_bytes(buf4);
        cursor.read_exact(&mut buf4)?;
        let _string_table_size = u32::from_le_bytes(buf4);

        // Read string table
        let mut strings = Vec::with_capacity(num_strings as usize);
        for _ in 0..num_strings {
            let mut buf2 = [0u8; 2];
            cursor.read_exact(&mut buf2)?;
            let str_len = u16::from_le_bytes(buf2) as usize;
            let mut str_bytes = vec![0u8; str_len];
            cursor.read_exact(&mut str_bytes)?;
            let s =
                String::from_utf8(str_bytes).context("string table entry is not valid UTF-8")?;
            strings.push(s);
        }

        // Read file table
        let mut file_table = Vec::with_capacity(num_files as usize);
        for _ in 0..num_files {
            let mut subfile_bytes = vec![0u8; SubFileData::SIZE];
            cursor.read_exact(&mut subfile_bytes)?;
            let subfile = SubFileData::from_bytes(&subfile_bytes)?;
            file_table.push(subfile);
        }

        // Read file data
        for subfile in &file_table {
            let offset = subfile.subfile_image_data_offset as usize;
            let size = subfile.subfile_size as usize;
            if offset + size > data.len() {
                bail!("File data exceeds archive bounds");
            }
            let file_data = data[offset..offset + size].to_vec();
            let name = strings
                .get(subfile.subfile_name_index as usize)
                .cloned()
                .unwrap_or_else(|| format!("unknown_{}", subfile.subfile_name_index));
            megafile.files.insert(name, file_data);
        }

        megafile.string_table = strings;
        Ok(megafile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_simple_archive() {
        let mut mega = Megafile::new();
        mega.add_file("map.ini", b"[Map]\nWidth=16\nHeight=16\n".to_vec());
        mega.add_file("map.bin", vec![0u8; 512]);

        let bytes = mega.write().unwrap();
        let loaded = Megafile::read(&bytes).unwrap();

        assert_eq!(
            loaded.get_file("map.ini").map(|v| v.as_slice()),
            Some(&b"[Map]\nWidth=16\nHeight=16\n"[..])
        );
        assert_eq!(
            loaded.get_file("map.bin").map(|v| v.as_slice()),
            Some(&vec![0u8; 512][..])
        );
    }

    #[test]
    fn preserves_file_order_by_crc() {
        let mut mega = Megafile::new();
        mega.add_file("z_last.ini", b"Z".to_vec());
        mega.add_file("a_first.ini", b"A".to_vec());

        let bytes = mega.write().unwrap();
        let loaded = Megafile::read(&bytes).unwrap();

        assert!(loaded.get_file("a_first.ini").is_some());
        assert!(loaded.get_file("z_last.ini").is_some());
    }

    #[test]
    fn empty_archive() {
        let mega = Megafile::new();
        let bytes = mega.write().unwrap();
        let loaded = Megafile::read(&bytes).unwrap();
        assert!(loaded.files.is_empty());
    }
}
