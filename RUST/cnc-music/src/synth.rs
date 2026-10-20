//! Deterministic synthesis: MIDI note events → mono sample buffer.
//!
//! The render is a pure function of the parsed file: note pairing is FIFO
//! per (channel, note), tick→seconds walks the tempo map exactly, and the
//! waveform is a fixed harmonic stack with an attack-decay envelope. Cost
//! is O(samples × active notes) — fine for short replacement tracks; large
//! files should chunk by note.

use crate::midi::{EventKind, MidiFile, DEFAULT_TEMPO_US};
use cnc_sfx::waves::{adsr, sine};

/// One resolved note in seconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub start_sec: f32,
    pub dur_sec: f32,
    pub note: u8,
    /// 0.0..=1.0, from MIDI velocity / 127.
    pub velocity: f32,
}

/// Equal temperament, A4 (note 69) = 440 Hz.
pub fn midi_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((f32::from(note) - 69.0) / 12.0)
}

/// Tempo changes as sorted (tick, microseconds_per_quarter) pairs.
fn tempo_map(file: &MidiFile) -> Vec<(u32, u32)> {
    let mut map: Vec<(u32, u32)> = Vec::new();
    for track in &file.tracks {
        for event in &track.events {
            if let EventKind::Tempo {
                microseconds_per_quarter,
            } = event.kind
            {
                map.push((event.tick, microseconds_per_quarter));
            }
        }
    }
    map.sort_by_key(|(tick, _)| *tick);
    map
}

/// Seconds since the start at `tick`, honoring every tempo change.
pub fn tick_to_secs(file: &MidiFile, tick: u32) -> f32 {
    let ppq = f64::from(file.division.max(1));
    let map = tempo_map(file);
    let mut secs = 0.0_f64;
    let mut cur_tick: u32 = 0;
    let mut cur_tempo: u32 = map.first().map_or(DEFAULT_TEMPO_US, |(_, t)| *t);
    for &(change_tick, tempo) in &map {
        if change_tick >= tick {
            break;
        }
        let span = f64::from(change_tick - cur_tick);
        secs += span * f64::from(cur_tempo) / 1e6 / ppq;
        cur_tick = change_tick;
        cur_tempo = tempo;
    }
    let span = f64::from(tick - cur_tick);
    (secs + span * f64::from(cur_tempo) / 1e6 / ppq) as f32
}

/// Pair note-on/note-off across all tracks into notes in seconds.
/// Unmatched note-ons stay open until the last tick seen.
pub fn notes_from(file: &MidiFile) -> Vec<Note> {
    let mut open: Vec<(u8, u8, u32, f32)> = Vec::new(); // (channel, note, tick, velocity)
    let mut notes: Vec<Note> = Vec::new();
    let mut last_tick = 0;
    let mut events: Vec<(u32, &EventKind)> = file
        .tracks
        .iter()
        .flat_map(|t| t.events.iter().map(|e| (e.tick, &e.kind)))
        .collect();
    // Stable ordering: by tick, note-offs before note-ons at equal ticks so
    // a re-struck note doesn't extend the previous one.
    events.sort_by_key(|(tick, _)| *tick);
    for (tick, kind) in events {
        last_tick = last_tick.max(tick);
        match *kind {
            EventKind::NoteOn {
                channel,
                note,
                velocity,
            } => {
                open.push((channel, note, tick, f32::from(velocity) / 127.0));
            }
            EventKind::NoteOff { channel, note } => {
                if let Some(index) = open
                    .iter()
                    .position(|(c, n, _, _)| *c == channel && *n == note)
                {
                    let (c, n, start, vel) = open.remove(index);
                    notes.push(Note {
                        start_sec: tick_to_secs(file, start),
                        dur_sec: (tick_to_secs(file, tick) - tick_to_secs(file, start)).max(0.01),
                        note: n,
                        velocity: vel,
                    });
                    let _ = c;
                }
            }
            _ => {}
        }
    }
    // Close still-open notes at the final tick.
    for (_, note, start, vel) in open {
        notes.push(Note {
            start_sec: tick_to_secs(file, start),
            dur_sec: (tick_to_secs(file, last_tick) - tick_to_secs(file, start)).max(0.01),
            note,
            velocity: vel,
        });
    }
    notes.sort_by(|a, b| a.start_sec.total_cmp(&b.start_sec));
    notes
}

/// Per-note waveform: fundamental + two soft harmonics.
fn note_sample(note: &Note, t_local: f32) -> f32 {
    let f = midi_freq(note.note);
    let body = sine(f, t_local) + 0.5 * sine(2.0 * f, t_local) + 0.25 * sine(3.0 * f, t_local);
    let env = adsr(t_local, 0.01, note.dur_sec * 0.6);
    body * 0.33 * env * note.velocity
}

/// Render notes to a mono buffer at `sample_rate`.
pub fn render_score(notes: &[Note], sample_rate: u32) -> Vec<f32> {
    let total = notes
        .iter()
        .map(|n| n.start_sec + n.dur_sec + 0.05)
        .fold(0.0_f32, f32::max);
    let len = (total * sample_rate as f32).ceil() as usize;
    let mut out = vec![0.0_f32; len];
    for note in notes {
        let start = (note.start_sec * sample_rate as f32) as usize;
        let span = (note.dur_sec * sample_rate as f32) as usize;
        for i in 0..span {
            let idx = start + i;
            if idx >= out.len() {
                break;
            }
            out[idx] += note_sample(note, i as f32 / sample_rate as f32);
        }
    }
    // Soft clip to keep the mix inside [-1, 1] deterministically.
    out.into_iter().map(|v| v.clamp(-1.0, 1.0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::midi::{EventKind, MidiEvent, MidiTrack};

    fn file_with_tempo(tick_per_quarter: u16, tempo: u32) -> MidiFile {
        MidiFile {
            format: 0,
            division: tick_per_quarter,
            tracks: vec![MidiTrack {
                events: vec![MidiEvent {
                    tick: 0,
                    kind: EventKind::Tempo {
                        microseconds_per_quarter: tempo,
                    },
                }],
            }],
        }
    }

    #[test]
    fn midi_freq_matches_equal_temperament() {
        assert!((midi_freq(69) - 440.0).abs() < 1e-3);
        assert!((midi_freq(81) - 880.0).abs() < 1e-3); // A5
        assert!((midi_freq(60) - 261.6256).abs() < 1e-3); // C4
    }

    #[test]
    fn tick_to_secs_uses_tempo() {
        // 120 BPM at 480 PPQ: one quarter = 0.5 s.
        let file = file_with_tempo(480, 500_000);
        assert!((tick_to_secs(&file, 480) - 0.5).abs() < 1e-6);
        // 240 BPM: one quarter = 0.25 s.
        let fast = file_with_tempo(480, 250_000);
        assert!((tick_to_secs(&fast, 480) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn tick_to_secs_honors_mid_file_tempo_change() {
        let file = MidiFile {
            format: 1,
            division: 480,
            tracks: vec![MidiTrack {
                events: vec![
                    MidiEvent {
                        tick: 0,
                        kind: EventKind::Tempo {
                            microseconds_per_quarter: 500_000,
                        },
                    },
                    MidiEvent {
                        tick: 960,
                        kind: EventKind::Tempo {
                            microseconds_per_quarter: 1_000_000,
                        },
                    },
                ],
            }],
        };
        // First 960 ticks = 2 quarters at 120 BPM = 1.0 s; the new tempo
        // 1_000_000us is 60 BPM (1 s per quarter), so +480 ticks = +1.0 s.
        assert!((tick_to_secs(&file, 960) - 1.0).abs() < 1e-6);
        assert!((tick_to_secs(&file, 1440) - 2.0).abs() < 1e-6);
    }

    #[test]
    fn notes_pair_on_off_and_render_is_deterministic() {
        let file = MidiFile {
            format: 0,
            division: 480,
            tracks: vec![MidiTrack {
                events: vec![
                    MidiEvent {
                        tick: 0,
                        kind: EventKind::Tempo {
                            microseconds_per_quarter: 500_000,
                        },
                    },
                    MidiEvent {
                        tick: 0,
                        kind: EventKind::NoteOn {
                            channel: 0,
                            note: 69,
                            velocity: 127,
                        },
                    },
                    MidiEvent {
                        tick: 480,
                        kind: EventKind::NoteOff {
                            channel: 0,
                            note: 69,
                        },
                    },
                ],
            }],
        };
        let notes = notes_from(&file);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].note, 69);
        assert!((notes[0].dur_sec - 0.5).abs() < 1e-3);
        assert!(notes[0].velocity > 0.99);
        let a = render_score(&notes, 22_050);
        let b = render_score(&notes, 22_050);
        assert_eq!(a, b);
        assert!(a.iter().any(|s| s.abs() > 0.05), "render is silent");
        assert!(a.iter().all(|s| s.abs() <= 1.0), "render out of range");
    }

    #[test]
    fn overlapping_notes_mix_without_clipping() {
        let notes = vec![
            Note {
                start_sec: 0.0,
                dur_sec: 0.5,
                note: 60,
                velocity: 1.0,
            },
            Note {
                start_sec: 0.0,
                dur_sec: 0.5,
                note: 64,
                velocity: 1.0,
            },
            Note {
                start_sec: 0.0,
                dur_sec: 0.5,
                note: 67,
                velocity: 1.0,
            },
        ];
        let rendered = render_score(&notes, 8_000);
        assert!(rendered.iter().all(|s| s.abs() <= 1.0));
    }
}
