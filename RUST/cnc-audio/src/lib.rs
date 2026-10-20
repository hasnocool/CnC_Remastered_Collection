//! # cnc-audio — audio engine for the C&C Rust port
//!
//! The audio engine is **presentation-only**: the game simulation is a
//! deterministic lockstep simulation, so nothing in this crate ever writes
//! back into game state. Everything here is centered on the same flat
//! "channel" model the original DOS games used — a fixed set of single-voice
//! output channels where each one plays at most one sound at a time, and a
//! higher-priority request on a busy channel evicts what is playing.
//!
//! ## Channel model
//!
//! A [`Channel`] is either a numbered SFX channel ([`Channel::Sfx`] 0..8),
//! the EVA/voice-over channel ([`Channel::Voice`]) or the music channel
//! ([`Channel::Music`]). Priorities are Westwood-style 0-255 ([`u8`]); a play
//! request on a busy channel succeeds only with a strictly higher priority,
//! otherwise it is dropped (see [`mixer::AudioError::ChannelBusy`]).
//!
//! Audibility of a voice is `master * channel * voice` volume. Positional
//! SFX are expressed without an explicit listener model:
//! [`mixer::attenuate`] maps a distance to a linear volume factor and
//! [`mixer::pan_gains`] turns a pan value into constant-power stereo gains.
//! [`mixer::doppler_factor`] is an optional approximation utility the
//! original games did not have — callers may use it to derive a pitch
//! multiplier, but playback never applies it automatically.
//!
//! ## Volume buses
//!
//! [`AudioMixer`] layers two named buses on top of the per-channel volumes:
//! [`AudioMixer::music_volume`] (music bus, applied at
//! [`AudioMixer::crossfade_music`]) and [`AudioMixer::effects_volume`]
//! (SFX + voice bus, applied at [`AudioMixer::play`]). All three knobs
//! default to 1.0, so the default audibility product
//! `master * bus * channel * voice` is bit-identical to the plain
//! `master * channel * voice` formula above — raising or lowering a bus
//! scales every later play request without touching per-channel state.
//!
//! ## Headless mode
//!
//! CI hosts have no audio device, so [`mixer::AudioMixer::headless`] builds a
//! mixer that exercises the exact same channel-allocation, priority,
//! volume and registry logic while recording every play request instead of
//! touching hardware. All tests in this crate run headless.
//!
//! ## Music
//!
//! [`music::MusicPlayer`] owns a queue of [`music::MusicTrack`]s plus an
//! explore/combat layer pair selected by [`music::MusicPlayer::set_intensity`].
//! Track switches crossfade via [`mixer::FadeSource`] envelopes on alternating
//! music sinks.
//!
//! ```no_run
//! use cnc_audio::{AudioMixer, Channel, SoundEffect};
//!
//! # fn main() -> Result<(), cnc_audio::mixer::AudioError> {
//! // Real hardware mixer:
//! let mixer = AudioMixer::new()?;
//! // Or, without a device (CI, tests, ded servers):
//! let mixer = AudioMixer::headless();
//!
//! let boom = SoundEffect::from_bytes(vec![]);
//! let _ = boom.play_on(&mixer, Channel::Sfx(3), 200, 0.8, -0.25);
//! # Ok(())
//! # }
//! ```

pub mod csf;
pub mod eva;
pub mod eva_ra;
pub mod mixer;
pub mod music;
pub mod reader;
pub mod sfx;
pub mod speech;

pub use csf::{CsfError, CsfFile, CsfLabel, CsfLanguage, CsfValue};
pub use eva::{EvaLine, EvaPlayer, EvaQueue, TdEvent, TdFaction};
pub use eva_ra::{RaDirection, RaEvaLine, RaEvaQueue, RaEvent, RaUpgrade};
pub use mixer::{
    attenuate, doppler_factor, pan_gains, AudioError, AudioMixer, Channel, ChannelId, FadeSource,
    RecordedMusic, RecordedPlay, SfxHandle, SharedFade, SFX_CHANNEL_COUNT,
};
pub use music::{MusicLayer, MusicPlayer, MusicTrack};
pub use sfx::SoundEffect;
pub use speech::{
    EvaVocabulary, Playback, RaSpeechDirector, SpeakOutcome, SpeechCategory, SpeechDirector,
    SpeechQueue, TdSpeechDirector,
};

#[cfg(test)]
pub(crate) mod test_support {
    /// Build the bytes of a minimal valid 16-bit mono PCM WAV file so tests
    /// can exercise real decoders without any fixture files on disk.
    pub(crate) fn wav_bytes(samples: usize, freq: f32, rate: u32) -> Vec<u8> {
        use std::io::{Cursor, Write};

        let data_len = (samples * 2) as u32;
        let mut out = Cursor::new(Vec::with_capacity(data_len as usize + 44));
        out.write_all(b"RIFF").expect("write RIFF");
        out.write_all(&(36 + data_len).to_le_bytes())
            .expect("write RIFF size");
        out.write_all(b"WAVE").expect("write WAVE");
        out.write_all(b"fmt ").expect("write fmt id");
        out.write_all(&16u32.to_le_bytes()).expect("write fmt size");
        out.write_all(&1u16.to_le_bytes()).expect("write PCM tag");
        out.write_all(&1u16.to_le_bytes()).expect("write mono");
        out.write_all(&rate.to_le_bytes())
            .expect("write sample rate");
        out.write_all(&(rate * 2).to_le_bytes())
            .expect("write byte rate");
        out.write_all(&2u16.to_le_bytes())
            .expect("write block align");
        out.write_all(&16u16.to_le_bytes())
            .expect("write bits per sample");
        out.write_all(b"data").expect("write data id");
        out.write_all(&data_len.to_le_bytes())
            .expect("write data size");
        for i in 0..samples {
            let t = i as f32 / rate as f32;
            let amp = (2.0 * std::f32::consts::PI * freq * t).sin();
            let sample = (amp * 0.5 * i16::MAX as f32)
                .round()
                .clamp(-32768.0, 32767.0) as i16;
            out.write_all(&sample.to_le_bytes()).expect("write sample");
        }
        out.into_inner()
    }
}
