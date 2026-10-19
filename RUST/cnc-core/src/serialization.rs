//! Binary serialization framework (little-endian, versioned)

use crate::{Fixed16_16, FloatBias};
use anyhow::{Context, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

/// Trait for types that can be serialized to/from binary
pub trait Serializable: Sized {
    fn read(reader: &mut BinaryReader) -> Result<Self>;
    fn write(&self, writer: &mut BinaryWriter) -> Result<()>;
}

/// Binary reader with versioning support
pub struct BinaryReader<'a> {
    cursor: Cursor<&'a [u8]>,
    version: u32,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8], version: u32) -> Self {
        Self {
            cursor: Cursor::new(data),
            version,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn set_position(&mut self, pos: u64) {
        self.cursor.set_position(pos);
    }

    pub fn remaining(&self) -> usize {
        (self.cursor.get_ref().len() as u64 - self.cursor.position()) as usize
    }

    pub fn is_eof(&self) -> bool {
        self.cursor.position() >= self.cursor.get_ref().len() as u64
    }

    // Primitive reads
    pub fn read_u8(&mut self) -> Result<u8> {
        self.cursor.read_u8().context("Failed to read u8")
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.cursor.read_i8().context("Failed to read i8")
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.cursor
            .read_u16::<LittleEndian>()
            .context("Failed to read u16")
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        self.cursor
            .read_i16::<LittleEndian>()
            .context("Failed to read i16")
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.cursor
            .read_u32::<LittleEndian>()
            .context("Failed to read u32")
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        self.cursor
            .read_i32::<LittleEndian>()
            .context("Failed to read i32")
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        self.cursor
            .read_u64::<LittleEndian>()
            .context("Failed to read u64")
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        self.cursor
            .read_i64::<LittleEndian>()
            .context("Failed to read i64")
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        self.cursor
            .read_f32::<LittleEndian>()
            .context("Failed to read f32")
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        self.cursor
            .read_f64::<LittleEndian>()
            .context("Failed to read f64")
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    // Fixed16_16
    pub fn read_fixed(&mut self) -> Result<Fixed16_16> {
        Ok(Fixed16_16::from_bits(self.read_i32()?))
    }

    // FloatBias
    pub fn read_float_bias(&mut self) -> Result<FloatBias> {
        Ok(FloatBias::from_f32(self.read_f32()?))
    }

    // String (length-prefixed)
    pub fn read_string(&mut self) -> Result<String> {
        let len = self.read_u32()? as usize;
        let mut buf = vec![0u8; len];
        self.cursor
            .read_exact(&mut buf)
            .context("Failed to read string bytes")?;
        String::from_utf8(buf).context("Invalid UTF-8 in string")
    }

    // Variable-length string (null-terminated)
    pub fn read_cstring(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        loop {
            let b = self.read_u8()?;
            if b == 0 {
                break;
            }
            buf.push(b);
        }
        String::from_utf8(buf).context("Invalid UTF-8 in C string")
    }

    // Fixed-length string
    pub fn read_fixed_string(&mut self, len: usize) -> Result<String> {
        let mut buf = vec![0u8; len];
        self.cursor
            .read_exact(&mut buf)
            .context("Failed to read fixed string")?;
        // Trim at null terminator if present
        if let Some(null_pos) = buf.iter().position(|&b| b == 0) {
            buf.truncate(null_pos);
        }
        String::from_utf8(buf).context("Invalid UTF-8 in fixed string")
    }

    // Bytes
    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.cursor
            .read_exact(&mut buf)
            .context("Failed to read bytes")?;
        Ok(buf)
    }

    // Read array of serializable items
    pub fn read_array<T: Serializable>(&mut self, count: usize) -> Result<Vec<T>> {
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            vec.push(T::read(self)?);
        }
        Ok(vec)
    }

    // Read length-prefixed array
    pub fn read_vec<T: Serializable>(&mut self) -> Result<Vec<T>> {
        let count = self.read_u32()? as usize;
        self.read_array(count)
    }

    // Skip bytes
    pub fn skip(&mut self, n: usize) -> Result<()> {
        let new_pos = self.cursor.position() + n as u64;
        if new_pos > self.cursor.get_ref().len() as u64 {
            return Err(anyhow::anyhow!("Skip past end of buffer"));
        }
        self.cursor.set_position(new_pos);
        Ok(())
    }

    // Align to boundary
    pub fn align(&mut self, alignment: u64) -> Result<()> {
        let pos = self.cursor.position();
        let remainder = pos % alignment;
        if remainder != 0 {
            self.skip((alignment - remainder) as usize)?;
        }
        Ok(())
    }
}

/// Binary writer with versioning support
pub struct BinaryWriter {
    cursor: Cursor<Vec<u8>>,
    version: u32,
}

impl BinaryWriter {
    pub fn new(version: u32) -> Self {
        Self {
            cursor: Cursor::new(Vec::new()),
            version,
        }
    }

    pub fn with_capacity(version: u32, capacity: usize) -> Self {
        Self {
            cursor: Cursor::new(Vec::with_capacity(capacity)),
            version,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn position(&self) -> usize {
        self.cursor.position() as usize
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.cursor.into_inner()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.cursor.get_ref()
    }

    // Primitive writes
    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        self.cursor.write_u8(value).context("Failed to write u8")
    }

    pub fn write_i8(&mut self, value: i8) -> Result<()> {
        self.cursor.write_i8(value).context("Failed to write i8")
    }

    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        self.cursor
            .write_u16::<LittleEndian>(value)
            .context("Failed to write u16")
    }

    pub fn write_i16(&mut self, value: i16) -> Result<()> {
        self.cursor
            .write_i16::<LittleEndian>(value)
            .context("Failed to write i16")
    }

    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        self.cursor
            .write_u32::<LittleEndian>(value)
            .context("Failed to write u32")
    }

    pub fn write_i32(&mut self, value: i32) -> Result<()> {
        self.cursor
            .write_i32::<LittleEndian>(value)
            .context("Failed to write i32")
    }

    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        self.cursor
            .write_u64::<LittleEndian>(value)
            .context("Failed to write u64")
    }

    pub fn write_i64(&mut self, value: i64) -> Result<()> {
        self.cursor
            .write_i64::<LittleEndian>(value)
            .context("Failed to write i64")
    }

    pub fn write_f32(&mut self, value: f32) -> Result<()> {
        self.cursor
            .write_f32::<LittleEndian>(value)
            .context("Failed to write f32")
    }

    pub fn write_f64(&mut self, value: f64) -> Result<()> {
        self.cursor
            .write_f64::<LittleEndian>(value)
            .context("Failed to write f64")
    }

    pub fn write_bool(&mut self, value: bool) -> Result<()> {
        self.write_u8(if value { 1 } else { 0 })
    }

    // Fixed16_16
    pub fn write_fixed(&mut self, value: Fixed16_16) -> Result<()> {
        self.write_i32(value.to_bits())
    }

    // FloatBias
    pub fn write_float_bias(&mut self, value: FloatBias) -> Result<()> {
        self.write_f32(value.to_f32())
    }

    // String (length-prefixed)
    pub fn write_string(&mut self, value: &str) -> Result<()> {
        let bytes = value.as_bytes();
        self.write_u32(bytes.len() as u32)?;
        self.cursor
            .write_all(bytes)
            .context("Failed to write string bytes")
    }

    // Null-terminated string
    pub fn write_cstring(&mut self, value: &str) -> Result<()> {
        self.cursor
            .write_all(value.as_bytes())
            .context("Failed to write C string")?;
        self.write_u8(0)
    }

    // Fixed-length string (padded with nulls)
    pub fn write_fixed_string(&mut self, value: &str, len: usize) -> Result<()> {
        let bytes = value.as_bytes();
        let write_len = bytes.len().min(len);
        self.cursor
            .write_all(&bytes[..write_len])
            .context("Failed to write fixed string")?;
        for _ in write_len..len {
            self.write_u8(0)?;
        }
        Ok(())
    }

    // Bytes
    pub fn write_bytes(&mut self, value: &[u8]) -> Result<()> {
        self.cursor
            .write_all(value)
            .context("Failed to write bytes")
    }

    // Write array of serializable items
    pub fn write_array<T: Serializable>(&mut self, items: &[T]) -> Result<()> {
        for item in items {
            item.write(self)?;
        }
        Ok(())
    }

    // Write length-prefixed array
    pub fn write_vec<T: Serializable>(&mut self, items: &[T]) -> Result<()> {
        self.write_u32(items.len() as u32)?;
        self.write_array(items)
    }

    // Align to boundary
    pub fn align(&mut self, alignment: usize) -> Result<()> {
        let pos = self.cursor.get_ref().len();
        let remainder = pos % alignment;
        if remainder != 0 {
            for _ in 0..(alignment - remainder) {
                self.write_u8(0)?;
            }
        }
        Ok(())
    }
}

// Serializable implementations for primitives
impl Serializable for u8 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_u8()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_u8(*self)
    }
}

impl Serializable for i8 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_i8()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_i8(*self)
    }
}

impl Serializable for u16 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_u16()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_u16(*self)
    }
}

impl Serializable for i16 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_i16()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_i16(*self)
    }
}

impl Serializable for u32 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_u32()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_u32(*self)
    }
}

impl Serializable for i32 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_i32()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_i32(*self)
    }
}

impl Serializable for u64 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_u64()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_u64(*self)
    }
}

impl Serializable for i64 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_i64()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_i64(*self)
    }
}

impl Serializable for f32 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_f32()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_f32(*self)
    }
}

impl Serializable for f64 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_f64()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_f64(*self)
    }
}

impl Serializable for bool {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_bool()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_bool(*self)
    }
}

impl Serializable for Fixed16_16 {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_fixed()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_fixed(*self)
    }
}

impl Serializable for FloatBias {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_float_bias()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_float_bias(*self)
    }
}

impl Serializable for String {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_string()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_string(self)
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        reader.read_vec()
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_vec(self)
    }
}

impl<T: Serializable, const N: usize> Serializable for [T; N] {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        let mut arr: Vec<T> = Vec::with_capacity(N);
        for _ in 0..N {
            arr.push(T::read(reader)?);
        }
        Ok(arr.try_into().unwrap_or_else(|_| unreachable!()))
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_array(self)
    }
}

// Option<T> - write as u8 (0 = None, 1 = Some) + value
impl<T: Serializable> Serializable for Option<T> {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        let has_value = reader.read_u8()? != 0;
        if has_value {
            Ok(Some(T::read(reader)?))
        } else {
            Ok(None)
        }
    }
    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        match self {
            Some(v) => {
                writer.write_u8(1)?;
                v.write(writer)
            }
            None => writer.write_u8(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_serialization() {
        let mut writer = BinaryWriter::new(1);
        writer.write_u32(0x12345678).unwrap();
        writer.write_i16(-42).unwrap();
        writer.write_string("hello").unwrap();
        writer.write_fixed(Fixed16_16::from_int(5)).unwrap();

        let data = writer.into_bytes();
        let mut reader = BinaryReader::new(&data, 1);

        assert_eq!(reader.read_u32().unwrap(), 0x12345678);
        assert_eq!(reader.read_i16().unwrap(), -42);
        assert_eq!(reader.read_string().unwrap(), "hello");
        assert_eq!(reader.read_fixed().unwrap(), Fixed16_16::from_int(5));
    }

    #[test]
    fn test_vec_serialization() {
        let mut writer = BinaryWriter::new(1);
        let vec = vec![1u32, 2, 3, 4, 5];
        writer.write_vec(&vec).unwrap();

        let data = writer.into_bytes();
        let mut reader = BinaryReader::new(&data, 1);
        let read_vec: Vec<u32> = reader.read_vec().unwrap();
        assert_eq!(read_vec, vec);
    }

    #[test]
    fn test_option_serialization() {
        let mut writer = BinaryWriter::new(1);
        writer.write_vec(&[Some(42u32), None, Some(100)]).unwrap();

        let data = writer.into_bytes();
        let mut reader = BinaryReader::new(&data, 1);
        let read_vec: Vec<Option<u32>> = reader.read_vec().unwrap();
        assert_eq!(read_vec, vec![Some(42), None, Some(100)]);
    }

    /// Property-based round-trip tests (proptest).
    #[cfg(test)]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(64))]

            #[test]
            fn write_then_read_round_trips_mixed_values(
                u in any::<u32>(),
                i in any::<i16>(),
                f in any::<f32>(),
                bits in any::<i32>(),
                opt in proptest::option::of(any::<u32>()),
                bytes in proptest::collection::vec(any::<u8>(), 0..=64),
            ) {
                let mut writer = BinaryWriter::new(7);
                writer.write_u32(u).unwrap();
                writer.write_i16(i).unwrap();
                writer.write_f32(f).unwrap();
                writer.write_fixed(Fixed16_16::from_bits(bits)).unwrap();
                let opt_val: Option<u32> = opt;
                let byte_vec: Vec<u8> = bytes;
                Serializable::write(&opt_val, &mut writer).unwrap();
                Serializable::write(&byte_vec, &mut writer).unwrap();

                let data = writer.into_bytes();
                let mut reader = BinaryReader::new(&data, 7);
                prop_assert_eq!(reader.version(), 7);
                prop_assert_eq!(reader.read_u32().unwrap(), u);
                prop_assert_eq!(reader.read_i16().unwrap(), i);
                // f32 must survive bit-identical, NaN payloads included.
                prop_assert_eq!(reader.read_f32().unwrap().to_bits(), f.to_bits());
                prop_assert_eq!(reader.read_fixed().unwrap(), Fixed16_16::from_bits(bits));
                let read_opt: Option<u32> = Serializable::read(&mut reader).unwrap();
                prop_assert_eq!(read_opt, opt_val);
                let read_vec: Vec<u8> = Serializable::read(&mut reader).unwrap();
                prop_assert_eq!(read_vec, byte_vec);
                prop_assert!(reader.is_eof(), "all written bytes must be consumed");
            }
        }
    }
}
