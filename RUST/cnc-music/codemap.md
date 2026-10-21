# RUST/cnc-music/

## Responsibility
Deterministic MIDI-to-OGG music recreation for the C&C Rust port. Original Westwood soundtracks cannot be redistributed; this crate parses Standard MIDI Files, renders deterministic sample buffers, and encodes OGG Vorbis for runtime playback. Pure offline baking pipeline - no game state interaction.

## Design Patterns
- Four-Stage Pipeline: Parse MIDI to Pair Notes to Render Samples to Encode OGG
- Deterministic Rendering: Identical MIDI input to bit-identical PCM output (container bytes may differ due to Vorbis serial randomization)
- Tempo Map Honoring: Full MIDI tempo meta events mapped to sample-accurate timing
- Additive Synthesis: Harmonic rendering with ADSR-ish envelope per note, soft-clipped to [-1, 1]
- Offline Baking: No runtime dependency - produces asset files consumed by cnc-audio

## Data & Control Flow
1. Parse: midi::parse(bytes) to MidiFile (tracks, events, tempo map)
2. Pair Notes: synth::notes_from(midi_file) to Vec<Note> (onset/offset in seconds, pitch, velocity)
3. Render: synth::render_score(notes, sample_rate) to Vec<f32> mono PCM buffer
4. Encode: encode::encode_ogg(samples, sample_rate) to Vec<u8> OGG Vorbis
5. Write: encode::write_ogg(path, samples, sample_rate) - convenience file write

## Public API
- midi: parse(), MidiFile, MidiTrack, MidiEvent, EventKind, DEFAULT_TEMPO_US, MidiError
- synth: notes_from(), render_score(), midi_freq(note), tick_to_secs(), Note struct
- encode: encode_ogg(), write_ogg()

## Integration Points
- Dependencies: cnc-sfx (shared synthesis primitives), cnc-procgen (procedural content), vorbis_rs, anyhow
- Consumed by: Asset pipeline (bakes music assets), cnc-audio (plays encoded OGG via AudioSource)
- No runtime link - build-time only, outputs committed assets

## Key Files
- src/lib.rs - Pipeline overview, module re-exports
- src/midi.rs - SMF parser (delta-VLQ, running status, tempo meta)
- src/synth.rs - Note pairing, additive synthesis, ADSR envelope, frequency conversion
- src/encode.rs - OGG Vorbis encoding via vorbis_rs

