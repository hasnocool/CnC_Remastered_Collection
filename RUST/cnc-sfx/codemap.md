# RUST/cnc-sfx/

## Responsibility

Deterministic procedural sound-effect synthesis for the C&C Rust port (per its Cargo.toml description). Replacement one-shots and loops for the original Westwood `.AUD`/`.VOC` audio, which cannot be redistributed. Every effect is a pure function of its seed — the same seed produces byte-identical output — so bakes can be cached and committed like any other asset (lib.rs docs). Output is mono 16-bit PCM WAV via `hound`.

## Modules

- `waves.rs` — pure primitives: `sine`/`square`/`saw`, white `noise` (drawn from `cnc_procgen::prng::Rng`), envelopes `decay_env`/`ramp`/`adsr`, and a `one_pole` low-pass. No wall clock, no shared mutable state.
- `effects.rs` — the effect set: `explosion`, `gunfire`, `tesla_zap`, `engine_loop`, `select_beep`, `construction_loop`, plus the WAV writers `to_wav_bytes`/`write_wav_16` and `DEFAULT_SAMPLE_RATE = 22_050`.

## Determinism & seamless loops

Durations are expressed in samples so the math is independent of the write rate. Loop generators are exactly periodic by construction: `engine_loop` uses only integer multiples of `1/duration` and sines only (a saw wave would click at the seam); `construction_loop` generates one 0.5 s period and tiles it exactly. The seed only varies the harmonic mix — it cannot break periodicity.

## Flow

`effect(seed, len)` → per-sample waveform from `waves` (envelope × oscillator × filtered noise) → clamp to `[-1, 1]` → `to_wav_bytes(samples, rate)` → `write_wav_16(path, ...)` (atomic temp-file + rename).

## Integration

- Deps: `cnc-procgen` (PRNG) + `hound 3.5`. Depended on by `cnc-music`, which reuses `waves::adsr`/`sine`; otherwise an offline bake tool with no in-workspace consumer wired to the audio pipeline.
- Tests: unit tests assert bounded/non-zero RMS, per-seed determinism + seed sensitivity, percussive decay, seam continuity, and RIFF/WAVE header shape.
