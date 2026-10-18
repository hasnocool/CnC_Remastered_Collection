//! MIX Archive Parser
use anyhow::{Result, Context, bail};
use std::collections::HashMap;
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MixEntry {
    pub name: String,
    pub offset: u32,
    pub size: u32,
    pub compressed_size: Option<u32>,
    pub is_compressed: bool,
}

pub struct MixArchive {
    entries: HashMap<String, MixEntry>,
    data: Vec<u8>,
    is_compressed: bool,
}

impl MixArchive {
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read(path).with_context(|| format!("Failed to read MIX: {:?}", path))?;
        Self::from_bytes(&data)
    }
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 8 { bail!("MIX too small"); }
        let mut cursor = Cursor::new(data);
        let mut sig = [0u8; 4]; cursor.read_exact(&mut sig)?;
        let is_compressed = match &sig { b"MIX\x00" => false, b"MIX\x01" => true, _ => bail!("Invalid MIX sig") };
        let file_count = cursor.read_u32::<LittleEndian>()?;
        let mut entries = HashMap::with_capacity(file_count as usize);
        let mut name_offsets = vec![0u32; file_count as usize];
        let mut sizes = vec![0u32; file_count as usize];
        let mut comp_sizes = vec![0u32; file_count as usize];
        for i in 0..file_count as usize { name_offsets[i] = cursor.read_u32::<LittleEndian>()?; }
        for i in 0..file_count as usize { sizes[i] = cursor.read_u32::<LittleEndian>()?; }
        if is_compressed {
            for i in 0..file_count as usize { comp_sizes[i] = cursor.read_u32::<LittleEndian>()?; }
        }

        // The payload is stored back to back at the end of the archive, so its start
        // is derivable from the total stored size. Everything between the numeric
        // tables and the payload is the NUL-separated name table.
        let total_stored: u64 = sizes.iter().enumerate().map(|(i, size)| {
            if is_compressed { u64::from(comp_sizes[i]) } else { u64::from(*size) }
        }).sum();
        let data_start = data.len() as u64 - total_stored;
        if data_start < cursor.position() {
            bail!("MIX tables overlap payload");
        }
        let name_table = &data[cursor.position() as usize..data_start as usize];
        let mut cur = data_start;
        for i in 0..file_count as usize {
            let name = Self::read_name(name_table, name_offsets[i] as usize)?;
            let stored = if is_compressed { comp_sizes[i] } else { sizes[i] };
            entries.insert(name.clone(), MixEntry {
                name,
                offset: cur as u32,
                size: sizes[i],
                compressed_size: if is_compressed { Some(stored) } else { None },
                is_compressed: is_compressed && stored != sizes[i],
            });
            cur += u64::from(stored);
        }
        Ok(Self { entries, data: data.to_vec(), is_compressed })
    }
    fn read_name(data: &[u8], offset: usize) -> Result<String> {
        if offset >= data.len() { bail!("Name offset OOB"); }
        let end = data[offset..].iter().position(|&b| b == 0).unwrap_or(data.len() - offset);
        Ok(String::from_utf8_lossy(&data[offset..offset+end]).to_string())
    }
    pub fn get(&self, name: &str) -> Option<&MixEntry> { self.entries.get(name) }
    /// True for MIX v1 archives whose entries may be zlib-compressed.
    pub fn is_compressed(&self) -> bool { self.is_compressed }
    pub fn extract(&self, name: &str) -> Result<Vec<u8>> {
        let entry = self.entries.get(name).ok_or_else(|| anyhow::anyhow!("Not found: {}", name))?;
        let stored_len = entry.compressed_size.unwrap_or(entry.size);
        let start = entry.offset as usize; let end = start + stored_len as usize;
        if end > self.data.len() { bail!("Entry beyond data"); }
        let file_data = &self.data[start..end];
        if entry.is_compressed { let mut d = ZlibDecoder::new(file_data); let mut o = Vec::new(); d.read_to_end(&mut o)?; Ok(o) } else { Ok(file_data.to_vec()) }
    }
    pub fn list(&self) -> Vec<&str> { self.entries.keys().map(|s| s.as_str()).collect() }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn extract_all(&self, dir: &Path) -> Result<()> { std::fs::create_dir_all(dir)?; for n in self.list() { std::fs::write(dir.join(n), self.extract(n)?)?; } Ok(()) }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn uncompressed_mix(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"MIX\x00");
        out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
        let mut names = Vec::new();
        let mut offsets = Vec::new();
        for (name, _) in entries {
            offsets.push(names.len() as u32);
            names.extend_from_slice(name.as_bytes());
            names.push(0);
        }
        for offset in offsets { out.extend_from_slice(&offset.to_le_bytes()); }
        for (_, data) in entries { out.extend_from_slice(&(data.len() as u32).to_le_bytes()); }
        out.extend_from_slice(&names);
        for (_, data) in entries { out.extend_from_slice(data); }
        out
    }

    #[test]
    fn extracts_uncompressed_entries() {
        let archive = MixArchive::from_bytes(&uncompressed_mix(&[("README.TXT", b"hello"), ("DATA.BIN", b"1234")])).unwrap();
        assert_eq!(archive.len(), 2);
        assert_eq!(archive.extract("README.TXT").unwrap(), b"hello");
        assert_eq!(archive.extract("DATA.BIN").unwrap(), b"1234");
    }

    #[test]
    fn extracts_compressed_entries() {
        use flate2::write::ZlibEncoder;
        use flate2::Compression;
        use std::io::Write;

        let plain = b"compress me, this is repeated data repeated data repeated data";
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(plain).unwrap();
        let compressed = encoder.finish().unwrap();

        let mut out = Vec::new();
        out.extend_from_slice(b"MIX\x01");
        out.extend_from_slice(&1u32.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes()); // name offset
        out.extend_from_slice(&(plain.len() as u32).to_le_bytes());
        out.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        out.extend_from_slice(b"BIG.BIN\0");
        out.extend_from_slice(&compressed);

        let archive = MixArchive::from_bytes(&out).unwrap();
        assert!(archive.is_compressed());
        assert!(archive.get("BIG.BIN").unwrap().is_compressed);
        assert_eq!(archive.extract("BIG.BIN").unwrap(), plain);
    }

    #[test]
    fn rejects_bad_signature() {
        assert!(MixArchive::from_bytes(b"NOPE1234").is_err());
        assert!(MixArchive::from_bytes(b"MIX").is_err());
    }
}
