//! Audio Parsers (VOC, AUD formats)
use anyhow::{bail, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use hound;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct VocFile {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub data: Vec<i16>,
    pub loop_start: Option<u32>,
    pub loop_end: Option<u32>,
}

impl VocFile {
    pub fn load(data: &[u8]) -> Result<Self> {
        if data.len() < 26 {
            bail!("VOC too small");
        }
        if !data.starts_with(b"Creative Voice File\x1a") {
            bail!("Invalid VOC signature");
        }

        let header_size = u16::from_le_bytes([data[20], data[21]]) as usize;
        if header_size < 26 {
            bail!("Invalid VOC header size");
        }

        let _version = u16::from_le_bytes([data[22], data[23]]);
        let _checksum = u16::from_le_bytes([data[24], data[25]]);

        let mut cursor = Cursor::new(&data[header_size..]);
        let mut audio_data = Vec::new();
        let mut sample_rate = 22050u32;
        let mut channels = 1u16;
        let mut bits = 8u16;
        let mut loop_start = None;
        let mut loop_end = None;

        while cursor.position() < cursor.get_ref().len() as u64 {
            if cursor.position() + 4 > cursor.get_ref().len() as u64 {
                break;
            }
            let block_type = cursor.read_u8()?;
            let block_size = (cursor.read_u32::<LittleEndian>()? & 0xFFFFFF) as usize;

            match block_type {
                0x00 => {
                    /* Terminator */
                    break;
                }
                0x01 => {
                    /* Voice data */
                    if block_size < 4 {
                        bail!("Invalid voice block");
                    }
                    let tc = cursor.read_u8()?; // Time constant
                    let pack = cursor.read_u8()?; // Pack type
                    sample_rate = (1000000 / (256 - tc as u32)).max(1);
                    channels = if pack == 0 { 1 } else { 2 };
                    bits = 8;

                    let audio_bytes = {
                        let pos = cursor.position() as usize;
                        let end = (pos + block_size - 2).min(cursor.get_ref().len());
                        let data = cursor.get_ref()[pos..end].to_vec();
                        cursor.set_position(end as u64);
                        data
                    };
                    for &b in audio_bytes.iter() {
                        audio_data.push((b as i16 - 128) << 8); // 8-bit to 16-bit
                    }
                }
                0x02 => {
                    /* Voice data continuation */
                    let audio_bytes = {
                        let pos = cursor.position() as usize;
                        let end = (pos + block_size).min(cursor.get_ref().len());
                        let data = cursor.get_ref()[pos..end].to_vec();
                        cursor.set_position(end as u64);
                        data
                    };
                    for &b in audio_bytes.iter() {
                        audio_data.push((b as i16 - 128) << 8);
                    }
                }
                0x03 => {
                    /* Silence */
                    if block_size >= 3 {
                        let duration = cursor.read_u16::<LittleEndian>()?;
                        let _tc = cursor.read_u8()?;
                        let silence_samples = (duration as u32 * sample_rate / 1000) as usize;
                        audio_data.extend(std::iter::repeat_n(0, silence_samples));
                    }
                }
                0x08 => {
                    /* Extended block */
                    if block_size >= 4 {
                        let ext_type = cursor.read_u16::<LittleEndian>()?;
                        let _ext_size = cursor.read_u16::<LittleEndian>()?;
                        if ext_type == 0x0001 && block_size >= 12 {
                            sample_rate = cursor.read_u32::<LittleEndian>()?;
                            bits = cursor.read_u8()? as u16;
                            channels = cursor.read_u8()? as u16;
                        }
                    }
                }
                0x09 => {
                    /* Loop start */
                    if block_size >= 4 {
                        loop_start = Some(cursor.read_u32::<LittleEndian>()?);
                    }
                }
                0x0A => {
                    /* Loop end */
                    if block_size >= 4 {
                        loop_end = Some(cursor.read_u32::<LittleEndian>()?);
                    }
                }
                _ => {
                    /* Skip unknown */
                    {
                        let pos = cursor.position() as usize;
                        let end = (pos + block_size).min(cursor.get_ref().len());
                        let data = cursor.get_ref()[pos..end].to_vec();
                        cursor.set_position(end as u64);
                        data
                    };
                }
            }
        }

        Ok(Self {
            sample_rate,
            channels,
            bits_per_sample: bits,
            data: audio_data,
            loop_start,
            loop_end,
        })
    }

    /// Convert to WAV bytes
    pub fn to_wav(&self) -> Result<Vec<u8>> {
        let spec = hound::WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut buffer = Vec::new();
        {
            let mut writer = hound::WavWriter::new(std::io::Cursor::new(&mut buffer), spec)?;
            for &sample in &self.data {
                writer.write_sample(sample)?;
            }
            writer.finalize()?;
        }
        Ok(buffer)
    }
}

/// Westwood Audio (.aud) format
#[derive(Debug, Clone)]
pub struct AudFile {
    pub sample_rate: u32,
    pub channels: u16,
    pub data: Vec<i16>,
}

impl AudFile {
    pub fn load(data: &[u8]) -> Result<Self> {
        // AUD is typically raw PCM with a simple header
        if data.len() < 8 {
            bail!("AUD too small");
        }
        let mut cursor = Cursor::new(data);
        let sample_rate = cursor.read_u32::<LittleEndian>()?;
        let channels = cursor.read_u16::<LittleEndian>()?;
        let _unknown = cursor.read_u16::<LittleEndian>()?;

        let mut audio_data = Vec::new();
        while cursor.position() < cursor.get_ref().len() as u64 {
            audio_data.push(cursor.read_i16::<LittleEndian>()?);
        }

        Ok(Self {
            sample_rate,
            channels,
            data: audio_data,
        })
    }

    pub fn to_wav(&self) -> Result<Vec<u8>> {
        let spec = hound::WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut buffer = Vec::new();
        {
            let mut writer = hound::WavWriter::new(std::io::Cursor::new(&mut buffer), spec)?;
            for &sample in &self.data {
                writer.write_sample(sample)?;
            }
            writer.finalize()?;
        }
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_voc_basic() {
        // Minimal valid VOC: 20-byte signature + 6-byte header = 26 bytes
        let mut data = Vec::new();
        data.extend_from_slice(b"Creative Voice File\x1a");
        data.extend_from_slice(&26u16.to_le_bytes()); // header size = 26
        data.extend_from_slice(&0x0100u16.to_le_bytes()); // version
        data.extend_from_slice(&0u16.to_le_bytes()); // checksum
                                                     // Block: type 0 (terminator) + 4-byte block size
        data.push(0x00);
        data.extend_from_slice(&[0, 0, 0, 0]);

        let voc = VocFile::load(&data).unwrap();
        assert_eq!(voc.sample_rate, 22050);
    }
}
