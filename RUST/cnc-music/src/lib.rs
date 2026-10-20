//! # cnc-music — deterministic MIDI-to-OGG music recreation for the C&C
//! Rust port
//!
//! The original Westwood soundtracks cannot be redistributed. This crate
//! recreates replacement music from Standard MIDI Files: parse → pair notes
//! → render a deterministic sample buffer → encode OGG Vorbis. The pipeline
//! is a pure function of the MIDI bytes, so bakes are reproducible and can
//! be committed like any other asset.
//!
//! ## Pipeline
//! 1. [`midi::parse`] — Standard MIDI File parsing (delta-VLQ timing,
//!    running status, tempo meta events).
//! 2. [`synth::notes_from`] — pair note on/off into notes in seconds,
//!    honoring the full tempo map.
//! 3. [`synth::render_score`] — additive harmonic render with an ADSR-ish
//!    envelope per note, soft-clipped to `[-1, 1]`.
//! 4. [`encode::encode_ogg`] / [`encode::write_ogg`] — OGG Vorbis out.
//!
//! Determinism note (see [`encode`]): rendered samples are bit-identical
//! for identical input; the OGG stream serial is randomized by `vorbis_rs`
//! defaults, so container bytes are not asserted byte-for-byte.
//!
//! Nothing here touches game state; all work is offline baking.

pub mod encode;
pub mod midi;
pub mod synth;

pub use encode::{encode_ogg, write_ogg};
pub use midi::{parse, EventKind, MidiError, MidiEvent, MidiFile, MidiTrack, DEFAULT_TEMPO_US};
pub use synth::{midi_freq, notes_from, render_score, tick_to_secs, Note};
