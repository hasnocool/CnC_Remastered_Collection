//! SHP Shape Parser (Westwood .shp format)
use anyhow::{Result, Context, bail};
use cnc_core::Rect;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ShpFrame {
    pub width: u16,
    pub height: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub data: Vec<u8>, // Palette indices
    pub bounds: Rect,
}

pub struct ShpFile {
    pub frames: Vec<ShpFrame>,
    pub version: u16,
    pub flags: u16,
}

impl ShpFile {
    pub fn load(data: &[u8]) -> Result<Self> {
        if data.len() < 8 { bail!("SHP too small"); }
        let mut cursor = Cursor::new(data);
        let version = cursor.read_u16::<LittleEndian>()?;
        let flags = cursor.read_u16::<LittleEndian>()?;
        let frame_count = cursor.read_u16::<LittleEndian>()? as usize;
        let _ = cursor.read_u16::<LittleEndian>()?; // unknown

        let mut frames = Vec::with_capacity(frame_count);
        let mut frame_offsets = Vec::with_capacity(frame_count);

        // Read frame headers (8 bytes each: width, height, offset_x, offset_y)
        for _ in 0..frame_count {
            let width = cursor.read_u16::<LittleEndian>()?;
            let height = cursor.read_u16::<LittleEndian>()?;
            let offset_x = cursor.read_i16::<LittleEndian>()?;
            let offset_y = cursor.read_i16::<LittleEndian>()?;
            frame_offsets.push((width, height, offset_x, offset_y));
        }

        // Read frame data offsets (4 bytes each)
        let mut data_offsets = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            data_offsets.push(cursor.read_u32::<LittleEndian>()? as usize);
        }

        // Read frame data
        for i in 0..frame_count {
            let (width, height, offset_x, offset_y) = frame_offsets[i];
            let start = data_offsets[i];
            let end = if i + 1 < frame_count { data_offsets[i + 1] } else { data.len() };
            
            if start >= data.len() || end > data.len() || start > end {
                bail!("Invalid frame data offset");
            }

            let frame_data = data[start..end].to_vec();
            let expected_size = width as usize * height as usize;
            
            // Some frames might be RLE compressed
            let decompressed = if frame_data.len() < expected_size {
                Self::decode_rle(&frame_data, expected_size)?
            } else {
                frame_data
            };

            frames.push(ShpFrame {
                width,
                height,
                offset_x,
                offset_y,
                data: decompressed,
                bounds: Rect::new(offset_x as i32, offset_y as i32, width as i32, height as i32),
            });
        }

        Ok(Self { frames, version, flags })
    }

    fn decode_rle(compressed: &[u8], expected_size: usize) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(expected_size);
        let mut i = 0;
        while i < compressed.len() && output.len() < expected_size {
            let control = compressed[i];
            i += 1;
            if control < 128 {
                // Copy next (control + 1) bytes literally
                let count = (control + 1) as usize;
                if i + count > compressed.len() { bail!("RLE overrun"); }
                output.extend_from_slice(&compressed[i..i+count]);
                i += count;
            } else {
                // Repeat next byte (257 - control) times
                let count = (257u16 - control as u16) as usize;
                if i >= compressed.len() { bail!("RLE overrun"); }
                let byte = compressed[i];
                i += 1;
                output.extend(std::iter::repeat(byte).take(count));
            }
        }
        if output.len() != expected_size {
            bail!("RLE decode size mismatch: expected {}, got {}", expected_size, output.len());
        }
        Ok(output)
    }

    pub fn frame(&self, index: usize) -> Option<&ShpFrame> {
        self.frames.get(index)
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Convert frame to RGBA pixels using palette
    pub fn frame_to_rgba(&self, index: usize, palette: &[cnc_core::RGB888; 256]) -> Option<Vec<u8>> {
        let frame = self.frames.get(index)?;
        let mut rgba = Vec::with_capacity(frame.width as usize * frame.height as usize * 4);
        for &idx in &frame.data {
            let color = palette[idx as usize];
            rgba.push(color.r);
            rgba.push(color.g);
            rgba.push(color.b);
            rgba.push(255);
        }
        Some(rgba)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_rle_decode() {
        let compressed = vec![0x02, 0x01, 0x02, 0x03, 0xFD, 0x04]; // 3 literal, then 4 repeats of 0x04
        let decoded = ShpFile::decode_rle(&compressed, 7).unwrap();
        assert_eq!(decoded, vec![1, 2, 3, 4, 4, 4, 4]);
    }
}