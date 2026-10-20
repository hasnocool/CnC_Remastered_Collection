//! Standard MIDI File (SMF) parsing: header, tracks, delta-VLQ timing.
//!
//! Supports the subset a deterministic music renderer needs: format 0/1
//! files, note on/off (with running status), program changes, and tempo
//! meta events. Sysex and unknown metas are skipped structurally.

use std::fmt;

/// Tempo assumed until a tempo meta event says otherwise (120 BPM).
pub const DEFAULT_TEMPO_US: u32 = 500_000;

/// Parsed file: header fields plus every track's absolute-tick events.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MidiFile {
    pub format: u16,
    pub division: u16,
    pub tracks: Vec<MidiTrack>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MidiTrack {
    pub events: Vec<MidiEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MidiEvent {
    pub tick: u32,
    pub kind: EventKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventKind {
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8 },
    ProgramChange { channel: u8, program: u8 },
    Tempo { microseconds_per_quarter: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MidiError {
    Truncated,
    BadChunk(&'static str),
    BadStatus(u8),
}

impl fmt::Display for MidiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MidiError::Truncated => write!(f, "midi data truncated"),
            MidiError::BadChunk(c) => write!(f, "bad or missing chunk: {c}"),
            MidiError::BadStatus(s) => write!(f, "invalid midi status byte 0x{s:02x}"),
        }
    }
}

impl std::error::Error for MidiError {}

type Result<T> = std::result::Result<T, MidiError>;

fn take<'a>(data: &'a [u8], pos: &mut usize, n: usize) -> Result<&'a [u8]> {
    let end = pos.checked_add(n).ok_or(MidiError::Truncated)?;
    let slice = data.get(*pos..end).ok_or(MidiError::Truncated)?;
    *pos = end;
    Ok(slice)
}

fn u16be(data: &[u8]) -> u16 {
    u16::from_be_bytes([data[0], data[1]])
}

fn u32be(data: &[u8]) -> u32 {
    u32::from_be_bytes([data[0], data[1], data[2], data[3]])
}

/// MIDI variable-length quantity: 7 bits per byte, high bit = continuation.
fn vlq(data: &[u8], pos: &mut usize) -> Result<u32> {
    let mut value: u32 = 0;
    for _ in 0..4 {
        let byte = *take(data, pos, 1)?.first().ok_or(MidiError::Truncated)?;
        value = (value << 7) | u32::from(byte & 0x7F);
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(MidiError::Truncated) // >4 continuation bytes is invalid
}

/// Parse a Standard MIDI File. Tolerates trailing bytes after the last
/// declared track; errors on structural corruption.
pub fn parse(data: &[u8]) -> Result<MidiFile> {
    let magic = take(data, &mut 0, 4)?;
    if magic != b"MThd" {
        return Err(MidiError::BadChunk("MThd"));
    }
    let mut pos = 4;
    let header_len = u32be(take(data, &mut pos, 4)?);
    if header_len < 6 {
        return Err(MidiError::BadChunk("MThd length"));
    }
    let header = take(data, &mut pos, 6)?;
    let format = u16be(&header[0..2]);
    let ntrks = u16be(&header[2..4]);
    let division = u16be(&header[4..6]);
    // Skip any extra header bytes (rarely > 6).
    pos += (header_len - 6) as usize;

    let mut tracks = Vec::new();
    for _ in 0..ntrks {
        let magic = take(data, &mut pos, 4)?;
        if magic != b"MTrk" {
            return Err(MidiError::BadChunk("MTrk"));
        }
        let track_len = u32be(take(data, &mut pos, 4)?) as usize;
        let track_end = pos.checked_add(track_len).ok_or(MidiError::Truncated)?;
        let track_data = data.get(pos..track_end).ok_or(MidiError::Truncated)?;
        pos = track_end;
        tracks.push(parse_track(track_data)?);
    }
    Ok(MidiFile {
        format,
        division,
        tracks,
    })
}

fn parse_track(data: &[u8]) -> Result<MidiTrack> {
    let mut pos = 0;
    let mut tick: u32 = 0;
    let mut running_status: Option<u8> = None;
    let mut events = Vec::new();
    while pos < data.len() {
        tick = tick
            .checked_add(vlq(data, &mut pos)?)
            .ok_or(MidiError::Truncated)?;
        let mut status = *take(data, &mut pos, 1)?
            .first()
            .ok_or(MidiError::Truncated)?;
        if status < 0x80 {
            // Running status: this byte is data, reuse the previous status.
            pos -= 1;
            match running_status {
                Some(s) => status = s,
                None => return Err(MidiError::BadStatus(status)),
            }
        } else {
            running_status = Some(status);
        }
        match status {
            0xFF => {
                let meta = take(data, &mut pos, 1)?[0];
                let len = vlq(data, &mut pos)? as usize;
                let payload = take(data, &mut pos, len)?;
                if meta == 0x51 && len == 3 {
                    events.push(MidiEvent {
                        tick,
                        kind: EventKind::Tempo {
                            microseconds_per_quarter: u32be24(payload),
                        },
                    });
                }
                // 0x2F end-of-track and other metas carry no renderable data.
            }
            0xF0 | 0xF7 => {
                let len = vlq(data, &mut pos)? as usize;
                take(data, &mut pos, len)?; // sysex: skip
            }
            0x80..=0xEF => {
                let high = status & 0xF0;
                let channel = status & 0x0F;
                match high {
                    0x80 => {
                        let d = take(data, &mut pos, 2)?;
                        events.push(MidiEvent {
                            tick,
                            kind: EventKind::NoteOff {
                                channel,
                                note: d[0],
                            },
                        });
                    }
                    0x90 => {
                        let d = take(data, &mut pos, 2)?;
                        // velocity 0 note-on is the common note-off spelling.
                        if d[1] == 0 {
                            events.push(MidiEvent {
                                tick,
                                kind: EventKind::NoteOff {
                                    channel,
                                    note: d[0],
                                },
                            });
                        } else {
                            events.push(MidiEvent {
                                tick,
                                kind: EventKind::NoteOn {
                                    channel,
                                    note: d[0],
                                    velocity: d[1],
                                },
                            });
                        }
                    }
                    0xC0 | 0xD0 => {
                        let d = take(data, &mut pos, 1)?;
                        if high == 0xC0 {
                            events.push(MidiEvent {
                                tick,
                                kind: EventKind::ProgramChange {
                                    channel,
                                    program: d[0],
                                },
                            });
                        }
                    }
                    0xA0 | 0xB0 | 0xE0 => {
                        take(data, &mut pos, 2)?;
                    }
                    _ => unreachable!("channel status covered above"),
                }
            }
            _ => return Err(MidiError::BadStatus(status)),
        }
    }
    Ok(MidiTrack { events })
}

/// 24-bit big-endian (tempo meta payloads use this).
fn u32be24(data: &[u8]) -> u32 {
    (u32::from(data[0]) << 16) | (u32::from(data[1]) << 8) | u32::from(data[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal format-0 file: 480 PPQ, tempo 500000, one note 60 → tick 480.
    fn fixture() -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(b"MThd");
        v.extend_from_slice(&6u32.to_be_bytes());
        v.extend_from_slice(&0u16.to_be_bytes()); // format 0
        v.extend_from_slice(&1u16.to_be_bytes()); // one track
        v.extend_from_slice(&480u16.to_be_bytes()); // division
                                                    // Track: tempo meta + note on C4 + note off
        let track: Vec<u8> = {
            let mut t = Vec::new();
            t.extend_from_slice(&[0x00, 0xFF, 0x51, 0x03, 0x07, 0xA1, 0x20]); // 500000 us
            t.extend_from_slice(&[0x00, 0x90, 60, 100]); // note on
            t.extend_from_slice(&[0x83, 0x60, 0x80, 60, 64]); // delta 480, note off
            t.extend_from_slice(&[0x00, 0xFF, 0x2F, 0x00]); // end of track
            t
        };
        v.extend_from_slice(b"MTrk");
        v.extend_from_slice(&(track.len() as u32).to_be_bytes());
        v.extend_from_slice(&track);
        v
    }

    #[test]
    fn parses_fixture() {
        let file = parse(&fixture()).expect("fixture");
        assert_eq!(file.format, 0);
        assert_eq!(file.division, 480);
        assert_eq!(file.tracks.len(), 1);
        let events = &file.tracks[0].events;
        assert_eq!(
            events[0].kind,
            EventKind::Tempo {
                microseconds_per_quarter: 500_000
            }
        );
        assert_eq!(
            events[1].kind,
            EventKind::NoteOn {
                channel: 0,
                note: 60,
                velocity: 100
            }
        );
        assert_eq!(events[1].tick, 0);
        assert_eq!(events[2].tick, 480);
        assert_eq!(
            events[2].kind,
            EventKind::NoteOff {
                channel: 0,
                note: 60
            }
        );
    }

    #[test]
    fn vlq_decodes_multi_byte() {
        // 0xFF 0x7F = (0x7F << 7) | 0x7F = 16383
        let data = [0xFF, 0x7F];
        assert_eq!(vlq(&data, &mut 0).unwrap(), 16383);
        assert_eq!(vlq(&[0x00], &mut 0).unwrap(), 0);
    }

    #[test]
    fn running_status_reuses_note_on() {
        let track = {
            let mut t = Vec::new();
            t.extend_from_slice(&[0x00, 0x90, 60, 100]);
            t.extend_from_slice(&[0x00, 62, 100]); // no status byte: note 62
            t
        };
        let mut file = Vec::new();
        file.extend_from_slice(b"MThd");
        file.extend_from_slice(&6u32.to_be_bytes());
        file.extend_from_slice(&0u16.to_be_bytes());
        file.extend_from_slice(&1u16.to_be_bytes());
        file.extend_from_slice(&96u16.to_be_bytes());
        file.extend_from_slice(b"MTrk");
        file.extend_from_slice(&(track.len() as u32).to_be_bytes());
        file.extend_from_slice(&track);
        let parsed = parse(&file).expect("running status file");
        assert_eq!(parsed.tracks[0].events.len(), 2);
        assert_eq!(
            parsed.tracks[0].events[1].kind,
            EventKind::NoteOn {
                channel: 0,
                note: 62,
                velocity: 100
            }
        );
    }

    #[test]
    fn note_on_velocity_zero_is_note_off() {
        let track = [0x00, 0x90, 64, 0x00];
        let mut file = Vec::new();
        file.extend_from_slice(b"MThd");
        file.extend_from_slice(&6u32.to_be_bytes());
        file.extend_from_slice(&0u16.to_be_bytes());
        file.extend_from_slice(&1u16.to_be_bytes());
        file.extend_from_slice(&96u16.to_be_bytes());
        file.extend_from_slice(b"MTrk");
        file.extend_from_slice(&(track.len() as u32).to_be_bytes());
        file.extend_from_slice(&track);
        let parsed = parse(&file).expect("velocity-0 file");
        assert_eq!(
            parsed.tracks[0].events[0].kind,
            EventKind::NoteOff {
                channel: 0,
                note: 64
            }
        );
    }

    #[test]
    fn errors_on_garbage() {
        assert!(parse(b"XXXX").is_err());
        let short = b"MThd";
        assert_eq!(parse(&short[..]), Err(MidiError::Truncated));
    }
}
