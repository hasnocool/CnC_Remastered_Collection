//! Byte readers with interior-shared sample data.
//!
//! rodio decoders need owned `Read + Seek` sources, but sample bytes are
//! stored once per `SoundEffect`/`MusicTrack` (cheaply cloneable and
//! playable on several channels at once). This module provides that glue:
//! a tiny cursor over `Arc<Vec<u8>>` that implements `Read + Seek`.

use std::io::{self, Read, Seek, SeekFrom};
use std::sync::Arc;

/// A restartable read view over shared sample bytes.
#[derive(Clone)]
pub struct SharedBytes {
    data: Arc<Vec<u8>>,
    pos: u64,
}

impl SharedBytes {
    /// New read cursor positioned at the start of `data`.
    pub fn new(data: Arc<Vec<u8>>) -> Self {
        Self { data, pos: 0 }
    }
}

impl Read for SharedBytes {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = usize::try_from(self.pos).unwrap_or(usize::MAX);
        if pos >= self.data.len() || buf.is_empty() {
            return Ok(0);
        }
        let take = (self.data.len() - pos).min(buf.len());
        buf[..take].copy_from_slice(&self.data[pos..pos + take]);
        self.pos += take as u64;
        Ok(take)
    }
}

impl Seek for SharedBytes {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let len = self.data.len() as u64;
        let absolute = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(delta) => moved_by(self.pos, delta)?,
            SeekFrom::End(delta) => moved_by(len, delta)?,
        };
        self.pos = absolute;
        Ok(absolute)
    }
}

/// Apply a signed offset to `base`, rejecting underflow/overflow.
fn moved_by(base: u64, delta: i64) -> io::Result<u64> {
    if delta >= 0 {
        base.checked_add(delta as u64)
    } else {
        base.checked_sub(delta.unsigned_abs())
    }
    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "seek outside sample data"))
}
