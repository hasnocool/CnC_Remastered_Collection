//! Westwood CSF (string table) reader and writer.
//!
//! CSF is the text catalogue both original games load next to their
//! executable: a header followed by a flat list of `LBL` / `RTS` blocks.
//! The layout implemented here is the documented original format:
//!
//! ```text
//! header (0x18 bytes)
//!   0x00  char[4]  " FSC"      (also tolerated: " FCS", the order used in
//!                               older tool documentation)
//!   0x04  u32      version     2 or 3
//!   0x08  u32      label count
//!   0x0C  u32      value count (string pairs across all labels)
//!   0x10  u32      unused
//!   0x14  u32      language    (0 = US, see [`CsfLanguage`])
//!
//! label block
//!   0x00  char[4]  " LBL"
//!   0x04  u32      values in this label
//!   0x08  u32      name length in bytes
//!   0x0C  char[n]  name, NOT zero terminated, surrounding blanks are ignored
//!
//! value block
//!   0x00  char[4]  " RTS"  (no extra value) or "WRTS" (extra value follows)
//!   0x04  u32      character count of the value
//!   0x08  u16[n]   characters, UTF-16LE with every word bitwise inverted
//!   ...   u32      extra value length (WRTS only)
//!   ...   char[m]  extra value (WRTS only)
//! ```
//!
//! Only the first value of a label is used by the games, but the format
//! allows several and the loader keeps them all. Label lookup is
//! case-insensitive and a duplicated label resolves to its last definition,
//! which is what the original loader does.
//!
//! ## String references
//!
//! Campaign text occasionally wants one string to reuse another. The
//! reference form adopted by this port (and used by the synthetic fixture in
//! `tests/fixtures/synthetic.csf`) is `%LABEL%`: [`CsfFile::resolve`]
//! expands it recursively, leaves unknown labels untouched and stops on
//! cycles instead of overflowing the stack.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

/// `" FSC"` as stored: the magic of every CSF file.
pub const CSF_MAGIC: [u8; 4] = *b" FSC";
/// Legacy alias of [`CSF_MAGIC`] found in older tool documentation.
pub const CSF_MAGIC_LEGACY: [u8; 4] = *b" FCS";
/// Label block tag.
pub const CSF_LABEL_TAG: [u8; 4] = *b" LBL";
/// Value block tag, no extra value.
pub const CSF_VALUE_TAG: [u8; 4] = *b" RTS";
/// Value block tag followed by the extra value.
pub const CSF_EXTRA_VALUE_TAG: [u8; 4] = *b"WRTS";
/// Header size in bytes.
pub const CSF_HEADER_LEN: usize = 0x18;
/// Highest depth `%LABEL%` references are expanded to.
pub const CSF_MAX_REFERENCE_DEPTH: usize = 8;

/// Language slot of a CSF file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CsfLanguage {
    /// English (US), the slot the original tables use.
    Us,
    /// English (UK).
    Uk,
    /// German.
    German,
    /// French.
    French,
    /// Spanish.
    Spanish,
    /// Italian.
    Italian,
    /// Japanese.
    Japanese,
    /// Jabberwockie.
    Jabberwocky,
    /// Korean.
    Korean,
    /// Chinese.
    Chinese,
    /// Any slot the format does not define.
    Unknown(u32),
}

impl CsfLanguage {
    /// Decode the header language value.
    pub fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Self::Us,
            1 => Self::Uk,
            2 => Self::German,
            3 => Self::French,
            4 => Self::Spanish,
            5 => Self::Italian,
            6 => Self::Japanese,
            7 => Self::Jabberwocky,
            8 => Self::Korean,
            9 => Self::Chinese,
            other => Self::Unknown(other),
        }
    }

    /// The header value for this language.
    pub fn to_raw(self) -> u32 {
        match self {
            Self::Us => 0,
            Self::Uk => 1,
            Self::German => 2,
            Self::French => 3,
            Self::Spanish => 4,
            Self::Italian => 5,
            Self::Japanese => 6,
            Self::Jabberwocky => 7,
            Self::Korean => 8,
            Self::Chinese => 9,
            Self::Unknown(raw) => raw,
        }
    }
}

/// One value of a label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsfValue {
    /// The decoded (UTF-8) value text.
    pub text: String,
    /// The optional `WRTS` extra value.
    pub extra: Option<String>,
}

impl CsfValue {
    /// A plain `RTS` value without extra data.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            extra: None,
        }
    }

    /// A value with its `WRTS` extra data.
    pub fn with_extra(text: impl Into<String>, extra: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            extra: Some(extra.into()),
        }
    }
}

/// One label plus every value attached to it, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsfLabel {
    /// Label name as stored (blanks trimmed, case preserved).
    pub name: String,
    /// Values of this label; index 0 is the one the games use.
    pub values: Vec<CsfValue>,
}

/// A parsed CSF string table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsfFile {
    /// Format version from the header (2 or 3).
    pub version: u32,
    /// Language slot from the header.
    pub language: CsfLanguage,
    /// Labels keyed by their upper-cased name for case-insensitive lookup.
    labels: BTreeMap<String, CsfLabel>,
}

/// Everything that can make [`CsfFile::parse`] fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsfError {
    /// The first four bytes are not a CSF magic.
    BadMagic([u8; 4]),
    /// The header version is not 2 or 3.
    UnsupportedVersion(u32),
    /// The buffer ended before a structure it advertised.
    Truncated {
        /// Byte offset where the read started.
        offset: usize,
        /// Bytes the structure needed.
        needed: usize,
        /// Bytes actually left.
        remaining: usize,
    },
    /// A label block did not start with `LBL`.
    BadLabelTag {
        /// Byte offset of the tag.
        offset: usize,
        /// The four bytes found there.
        found: [u8; 4],
    },
    /// A value block did not start with `RTS`/`WRTS`.
    BadValueTag {
        /// Byte offset of the tag.
        offset: usize,
        /// The four bytes found there.
        found: [u8; 4],
    },
    /// A label name was empty after trimming blanks.
    EmptyLabelName {
        /// Byte offset of the label block.
        offset: usize,
    },
    /// A label name was not ASCII.
    NonAsciiLabelName {
        /// Byte offset of the label block.
        offset: usize,
    },
    /// A UTF-16 value contained an unpaired surrogate.
    InvalidValueEncoding {
        /// Byte offset of the value block.
        offset: usize,
    },
    /// The value payload was not twice its declared character count.
    ValueLengthMismatch {
        /// Byte offset of the value block.
        offset: usize,
        /// Character count from the block header.
        declared: u32,
        /// Byte length available for the payload.
        available: usize,
    },
    /// The header value count disagreed with the parsed values.
    ValueCountMismatch {
        /// Count from the header.
        declared: u32,
        /// Pairs actually parsed.
        parsed: usize,
    },
    /// Bytes were left over after the last label (non-zero padding only).
    TrailingBytes {
        /// Byte offset of the first trailing byte.
        offset: usize,
        /// Number of trailing bytes.
        len: usize,
    },
    /// The file could not be read from disk.
    Io(String),
}

impl fmt::Display for CsfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadMagic(found) => {
                write!(f, "not a CSF file: magic {found:?}, expected \" FSC\"")
            }
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported CSF version {version} (expected 2 or 3)")
            }
            Self::Truncated {
                offset,
                needed,
                remaining,
            } => write!(
                f,
                "truncated CSF at offset {offset}: need {needed} bytes, {remaining} left"
            ),
            Self::BadLabelTag { offset, found } => {
                write!(f, "bad label tag {found:?} at offset {offset}")
            }
            Self::BadValueTag { offset, found } => {
                write!(f, "bad value tag {found:?} at offset {offset}")
            }
            Self::EmptyLabelName { offset } => {
                write!(f, "empty label name at offset {offset}")
            }
            Self::NonAsciiLabelName { offset } => {
                write!(f, "non-ASCII label name at offset {offset}")
            }
            Self::InvalidValueEncoding { offset } => {
                write!(f, "invalid UTF-16 in value at offset {offset}")
            }
            Self::ValueLengthMismatch {
                offset,
                declared,
                available,
            } => write!(
                f,
                "value at offset {offset} declares {declared} chars but {available} bytes remain"
            ),
            Self::ValueCountMismatch { declared, parsed } => {
                write!(f, "header declares {declared} values, parsed {parsed}")
            }
            Self::TrailingBytes { offset, len } => {
                write!(
                    f,
                    "{len} trailing bytes after the last label at offset {offset}"
                )
            }
            Self::Io(err) => write!(f, "reading CSF file: {err}"),
        }
    }
}

impl std::error::Error for CsfError {}

impl CsfFile {
    /// Parse a CSF image.
    pub fn parse(data: &[u8]) -> Result<Self, CsfError> {
        if data.len() < CSF_HEADER_LEN {
            return Err(CsfError::Truncated {
                offset: 0,
                needed: CSF_HEADER_LEN,
                remaining: data.len(),
            });
        }
        let magic: [u8; 4] = data[0..4].try_into().expect("4-byte slice");
        if magic != CSF_MAGIC && magic != CSF_MAGIC_LEGACY {
            return Err(CsfError::BadMagic(magic));
        }
        let version = read_u32(data, 4)?;
        if version != 2 && version != 3 {
            return Err(CsfError::UnsupportedVersion(version));
        }
        let declared_labels = read_u32(data, 8)?;
        let declared_values = read_u32(data, 12)?;
        let language = CsfLanguage::from_raw(read_u32(data, 20)?);

        let mut labels = BTreeMap::new();
        let mut offset = CSF_HEADER_LEN;
        let mut parsed_values: usize = 0;
        for _ in 0..declared_labels {
            let label = Self::parse_label(data, &mut offset, &mut parsed_values)?;
            // A duplicated label keeps the last definition, like the game.
            labels.insert(label.name.to_uppercase(), label);
        }
        if declared_values as usize != parsed_values {
            return Err(CsfError::ValueCountMismatch {
                declared: declared_values,
                parsed: parsed_values,
            });
        }
        if offset != data.len() {
            let trailing = &data[offset..];
            if trailing.iter().any(|byte| *byte != 0) {
                return Err(CsfError::TrailingBytes {
                    offset,
                    len: trailing.len(),
                });
            }
        }
        Ok(Self {
            version,
            language,
            labels,
        })
    }

    fn parse_label(
        data: &[u8],
        offset: &mut usize,
        parsed_values: &mut usize,
    ) -> Result<CsfLabel, CsfError> {
        let block_offset = *offset;
        let tag = read_tag(data, offset)?;
        if tag != CSF_LABEL_TAG && tag != *b"LBL " {
            return Err(CsfError::BadLabelTag {
                offset: block_offset,
                found: tag,
            });
        }
        let value_count = read_u32(data, *offset)?;
        *offset += 4;
        let name_len = read_u32(data, *offset)? as usize;
        *offset += 4;
        let name_bytes = take(data, offset, name_len)?;
        let name = std::str::from_utf8(name_bytes)
            .map_err(|_| CsfError::NonAsciiLabelName {
                offset: block_offset,
            })?
            .trim()
            .to_string();
        if name.is_empty() {
            return Err(CsfError::EmptyLabelName {
                offset: block_offset,
            });
        }
        let mut values = Vec::with_capacity(value_count as usize);
        for _ in 0..value_count {
            values.push(Self::parse_value(data, offset)?);
        }
        *parsed_values += values.len();
        Ok(CsfLabel { name, values })
    }

    fn parse_value(data: &[u8], offset: &mut usize) -> Result<CsfValue, CsfError> {
        let block_offset = *offset;
        let tag = read_tag(data, offset)?;
        let has_extra = match tag {
            t if t == CSF_VALUE_TAG || t == *b"STR " => false,
            t if t == CSF_EXTRA_VALUE_TAG || t == *b"STRW" => true,
            found => {
                return Err(CsfError::BadValueTag {
                    offset: block_offset,
                    found,
                })
            }
        };
        let chars = read_u32(data, *offset)? as usize;
        *offset += 4;
        let payload_len = chars * 2;
        let payload =
            take(data, offset, payload_len).map_err(|_| CsfError::ValueLengthMismatch {
                offset: block_offset,
                declared: chars as u32,
                available: data.len().saturating_sub(*offset),
            })?;
        let text = decode_inverted_utf16(payload, block_offset)?;
        let extra = if has_extra {
            let extra_len = read_u32(data, *offset)? as usize;
            *offset += 4;
            let extra_bytes = take(data, offset, extra_len)?;
            let extra = std::str::from_utf8(extra_bytes)
                .map_err(|_| CsfError::NonAsciiLabelName {
                    offset: block_offset,
                })?
                .to_string();
            Some(extra)
        } else {
            None
        };
        Ok(CsfValue { text, extra })
    }

    /// Load and parse a CSF file from disk.
    pub fn from_file(path: &Path) -> Result<Self, CsfError> {
        let data = std::fs::read(path).map_err(|err| CsfError::Io(err.to_string()))?;
        Self::parse(&data)
    }

    /// Look a label up case-insensitively.
    pub fn label(&self, name: &str) -> Option<&CsfLabel> {
        self.labels.get(&name.to_uppercase())
    }

    /// The text of a label's first value.
    pub fn text(&self, name: &str) -> Option<&str> {
        self.label(name)
            .and_then(|label| label.values.first())
            .map(|value| value.text.as_str())
    }

    /// The text of a label's first value, with `%LABEL%` references expanded.
    pub fn resolve(&self, name: &str) -> Option<String> {
        let text = self.text(name)?;
        Some(self.resolve_text(text))
    }

    /// Expand `%LABEL%` references inside arbitrary text.
    ///
    /// Unknown labels are left verbatim; recursion stops at
    /// [`CSF_MAX_REFERENCE_DEPTH`] and on reference cycles, so malformed
    /// tables cannot panic or recurse forever.
    pub fn resolve_text(&self, text: &str) -> String {
        self.resolve_inner(text, 0, &mut Vec::new())
    }

    fn resolve_inner(&self, text: &str, depth: usize, seen: &mut Vec<String>) -> String {
        if depth >= CSF_MAX_REFERENCE_DEPTH {
            return text.to_string();
        }
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(start) = rest.find('%') {
            out.push_str(&rest[..start]);
            let after = &rest[start + 1..];
            let Some(end) = after.find('%') else {
                out.push_str(&rest[start..]);
                return out;
            };
            let label = &after[..end];
            let key = label.to_uppercase();
            let replacement = if seen.iter().any(|seen_key| seen_key == &key) {
                Some(format!("%{label}%"))
            } else {
                self.label(label).map(|_| {
                    seen.push(key);
                    let resolved = self.text(label).map_or_else(
                        || format!("%{label}%"),
                        |inner| self.resolve_inner(inner, depth + 1, seen),
                    );
                    seen.pop();
                    resolved
                })
            };
            match replacement {
                Some(resolved) => out.push_str(&resolved),
                None => out.push_str(&format!("%{label}%")),
            }
            rest = &after[end + 1..];
        }
        out.push_str(rest);
        out
    }

    /// Every label, in name order.
    pub fn labels(&self) -> impl Iterator<Item = &CsfLabel> {
        self.labels.values()
    }

    /// Number of distinct labels.
    pub fn label_count(&self) -> usize {
        self.labels.len()
    }

    /// Encode this table back to CSF bytes.
    ///
    /// Labels are written in name order, which makes the output canonical and
    /// round-trippable: parsing it yields an equal [`CsfFile`].
    pub fn encode(&self) -> Vec<u8> {
        let mut body = Vec::new();
        let mut value_count = 0u32;
        for label in self.labels.values() {
            body.extend_from_slice(&CSF_LABEL_TAG);
            body.extend_from_slice(&(label.values.len() as u32).to_le_bytes());
            body.extend_from_slice(&(label.name.len() as u32).to_le_bytes());
            body.extend_from_slice(label.name.as_bytes());
            for value in &label.values {
                match &value.extra {
                    Some(extra) => {
                        body.extend_from_slice(&CSF_EXTRA_VALUE_TAG);
                        body.extend_from_slice(&(value.text.chars().count() as u32).to_le_bytes());
                        body.extend_from_slice(&encode_inverted_utf16(&value.text));
                        body.extend_from_slice(&(extra.len() as u32).to_le_bytes());
                        body.extend_from_slice(extra.as_bytes());
                    }
                    None => {
                        body.extend_from_slice(&CSF_VALUE_TAG);
                        body.extend_from_slice(&(value.text.chars().count() as u32).to_le_bytes());
                        body.extend_from_slice(&encode_inverted_utf16(&value.text));
                    }
                }
                value_count += 1;
            }
        }
        let mut out = Vec::with_capacity(CSF_HEADER_LEN + body.len());
        out.extend_from_slice(&CSF_MAGIC);
        out.extend_from_slice(&self.version.to_le_bytes());
        out.extend_from_slice(&(self.labels.len() as u32).to_le_bytes());
        out.extend_from_slice(&value_count.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&self.language.to_raw().to_le_bytes());
        out.extend_from_slice(&body);
        out
    }
}

fn read_tag(data: &[u8], offset: &mut usize) -> Result<[u8; 4], CsfError> {
    let bytes = take(data, offset, 4)?;
    Ok(bytes.try_into().expect("4-byte slice"))
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, CsfError> {
    if data.len() < offset + 4 {
        return Err(CsfError::Truncated {
            offset,
            needed: 4,
            remaining: data.len().saturating_sub(offset),
        });
    }
    Ok(u32::from_le_bytes(
        data[offset..offset + 4].try_into().expect("4-byte slice"),
    ))
}

fn take<'a>(data: &'a [u8], offset: &mut usize, len: usize) -> Result<&'a [u8], CsfError> {
    let end = offset.saturating_add(len);
    if end > data.len() {
        return Err(CsfError::Truncated {
            offset: *offset,
            needed: len,
            remaining: data.len().saturating_sub(*offset),
        });
    }
    let slice = &data[*offset..end];
    *offset = end;
    Ok(slice)
}

fn decode_inverted_utf16(payload: &[u8], offset: usize) -> Result<String, CsfError> {
    let mut units = Vec::with_capacity(payload.len() / 2);
    for chunk in payload.chunks_exact(2) {
        let raw = u16::from_le_bytes([chunk[0], chunk[1]]);
        units.push(!raw);
    }
    String::from_utf16(&units).map_err(|_| CsfError::InvalidValueEncoding { offset })
}

fn encode_inverted_utf16(text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len() * 2);
    for unit in text.encode_utf16() {
        out.extend_from_slice(&(!unit).to_le_bytes());
    }
    out
}
