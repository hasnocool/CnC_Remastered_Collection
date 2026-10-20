# RUST/cnc-music/

## Responsibility

Deterministic MIDI-to-OGG music recreation for the C&C Rust port (per its Cargo.toml description). The original Westwood soundtracks cannot be redistributed, so this recreates replacement music from Standard MIDI Files. The pipeline is a pure function of the MIDI bytes, so bakes are reproducible and can be committed like any other asset (lib.rs docs). Offline only; touches no game state.

## Pipeline modules

- `midi::parse` — SMF parsing: header, tracks, delta-VLQ timing, running status, note on/off (velocity-0 note-on treated as note-off), program change, tempo meta; sysex and unknown metas are skipped structurally. Types: `MidiFile`/`MidiTrack`/`MidiEvent`/`EventKind`/`MidiError`, with `DEFAULT_TEMPO_US = 500_000`.
- `synth::notes_from` — pairs note on/off across all tracks into `Note { start_sec, dur_sec, note, velocity }` (FIFO per channel/note; note-offs ordered before note-ons at equal ticks). `tick_to_secs` walks the full tempo map; `render_score` is an additive harmonic stack (`midi_freq`, equal temperament, A4 = 440 Hz) with an ADSR-ish envelope per note, soft-clipped to `[-1, 1]`.
- `encode::encode_ogg` / `encode::write_ogg` — mono f32 → OGG Vorbis via `vorbis_rs`; `write_ogg` is atomic (temp file + rename).

## Determinism note

Rendered **samples** are bit-identical for identical input. The OGG **container** is not byte-stable: `vorbis_rs` randomizes the stream serial by default, so tests assert structure/size rather than exact bytes (see `encode` docs and its tests).

## Flow

`.mid` bytes → `midi::parse` → `synth::notes_from` → `synth::render_score` (mono f32 buffer) → `encode::encode_ogg` / `write_ogg` → `.ogg`.

## Integration

- Deps: `cnc-sfx` (reuses `waves::adsr`/`sine`), `cnc-procgen`, `vorbis_rs 0.5`, `anyhow 1.0`. No in-workspace dependents yet — this is the top of the audio bake stack.
- Cost is O(samples × active notes) — fine for short replacement tracks; the module doc notes large files should be chunked per note.
