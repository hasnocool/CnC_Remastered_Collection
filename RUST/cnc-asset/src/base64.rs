//! Base64 codec as used by the original engines' `Put_UUBlock` / `Get_UUBlock`.
//!
//! RA stores bulk binary data (`MapPack`, `OverlayPack`, digests) in INI files as
//! numbered base64 lines. `REDALERT/BASE64.CPP` defines the alphabet
//! `A-Za-z0-9+/` with `=` padding, and `REDALERT/INI.CPP` writes each encoded
//! line as a numbered key (`1`, `2`, ...) holding at most 70 characters.

/// Padding character (`BASE64.CPP` `_pad`).
pub const PAD: char = '=';

/// Longest encoded line `Put_UUBlock` writes (`char buffer[71]`).
pub const UU_LINE_WIDTH: usize = 70;

/// Encoder table (`BASE64.CPP` `_encoder`).
const ENCODER: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Why a decode failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base64Error {
    /// A character outside the alphabet and not `=`.
    InvalidCharacter(u8),
    /// The encoded length was not a multiple of four.
    BadLength(usize),
}

impl core::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidCharacter(byte) => {
                write!(f, "invalid base64 character 0x{byte:02X}")
            }
            Self::BadLength(len) => write!(f, "base64 length {len} is not a multiple of 4"),
        }
    }
}

impl std::error::Error for Base64Error {}

/// Encode bytes to base64 text, unpadded input is padded with `=`.
pub fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(ENCODER[((triple >> 18) & 0x3F) as usize] as char);
        out.push(ENCODER[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(ENCODER[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push(PAD);
        }
        if chunk.len() > 2 {
            out.push(ENCODER[(triple & 0x3F) as usize] as char);
        } else {
            out.push(PAD);
        }
    }
    out
}

/// Decode base64 text.
///
/// Whitespace and newlines are skipped so multi-line blocks decode as one
/// stream, matching `Base64Pipe::DECODE`.
pub fn decode(text: &str) -> Result<Vec<u8>, Base64Error> {
    // Collect symbols, recording which entries were padding so the trailing
    // bytes can be trimmed per quad.
    let mut symbols = Vec::with_capacity(text.len());
    for raw in text.bytes() {
        match raw {
            b'A'..=b'Z' => symbols.push(Some(raw - b'A')),
            b'a'..=b'z' => symbols.push(Some(raw - b'a' + 26)),
            b'0'..=b'9' => symbols.push(Some(raw - b'0' + 52)),
            b'+' => symbols.push(Some(62)),
            b'/' => symbols.push(Some(63)),
            b'=' => symbols.push(None),
            // Whitespace is ignored by the original decoder.
            b' ' | b'\t' | b'\r' | b'\n' => {}
            other => return Err(Base64Error::InvalidCharacter(other)),
        }
    }
    if symbols.len() % 4 != 0 {
        return Err(Base64Error::BadLength(symbols.len()));
    }

    let mut out = Vec::with_capacity(symbols.len() / 4 * 3);
    for quad in symbols.chunks(4) {
        // Pad symbols contribute zero bits; they only mark absent bytes.
        let value = |slot: &Option<u8>| u32::from(slot.unwrap_or(0));
        let triple = (value(&quad[0]) << 18)
            | (value(&quad[1]) << 12)
            | (value(&quad[2]) << 6)
            | value(&quad[3]);
        out.push((triple >> 16) as u8);
        if quad[2].is_some() {
            out.push((triple >> 8) as u8);
        }
        if quad[3].is_some() {
            out.push(triple as u8);
        }
    }
    Ok(out)
}

/// Split encoded text into the numbered lines `Put_UUBlock` writes.
pub fn uu_lines(bytes: &[u8]) -> Vec<String> {
    encode(bytes)
        .as_bytes()
        .chunks(UU_LINE_WIDTH)
        .map(|line| String::from_utf8_lossy(line).into_owned())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_rfc4648_vectors() {
        // RFC 4648 section 10 test vectors.
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
        assert_eq!(encode(b"foo"), "Zm9v");
        assert_eq!(encode(b"foob"), "Zm9vYg==");
        assert_eq!(encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn decodes_rfc4648_vectors() {
        assert_eq!(decode("Zg==").unwrap(), b"f");
        assert_eq!(decode("Zm8=").unwrap(), b"fo");
        assert_eq!(decode("Zm9v").unwrap(), b"foo");
        assert_eq!(decode("Zm9vYg==").unwrap(), b"foob");
        assert_eq!(decode("Zm9vYmFy").unwrap(), b"foobar");
    }

    #[test]
    fn round_trips_binary() {
        let data: Vec<u8> = (0u8..=255).collect();
        assert_eq!(decode(&encode(&data)).unwrap(), data);
    }

    #[test]
    fn skips_whitespace_between_lines() {
        assert_eq!(decode("Zm9v\r\nYmFy").unwrap(), b"foobar");
        assert_eq!(decode("Zm9v YmFy").unwrap(), b"foobar");
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(decode("Zm9v!"), Err(Base64Error::InvalidCharacter(b'!')));
        assert_eq!(decode("Zm9"), Err(Base64Error::BadLength(3)));
    }

    #[test]
    fn uu_lines_respect_the_westernwood_width() {
        let data: Vec<u8> = (0u8..=99).collect();
        let lines = uu_lines(&data);
        assert!(lines.len() > 1, "100 bytes must wrap past one 70-char line");
        assert!(lines.iter().all(|line| line.len() <= UU_LINE_WIDTH));
        let joined: String = lines.concat();
        assert_eq!(decode(&joined).unwrap(), data);
    }
}
