//! # cnc-sfx — deterministic procedural sound effects for the C&C Rust port
//!
//! Replacement effects for the original Westwood `.AUD`/`.VOC` audio, which
//! cannot be redistributed. Every effect is a pure function of its seed:
//! identical seeds produce byte-identical output, so bakes can be cached and
//! committed like any other asset. Output is mono 16-bit PCM suitable for the
//! existing audio pipeline (`hound` WAV; the asset CLI converts further).
//!
//! ## Modules
//! - [`waves`] — waveforms, envelopes and noise shaping.
//! - [`effects`] — the effect set (explosions, gunfire, Tesla, engine loops,
//!   beeps, construction) plus WAV writing.

pub mod effects;
pub mod waves;

pub use effects::{
    construction_loop, engine_loop, explosion, gunfire, select_beep, tesla_zap, to_wav_bytes,
    write_wav_16, DEFAULT_SAMPLE_RATE,
};
pub use waves::{adsr, decay_env, noise, ramp, saw, sine, square};
