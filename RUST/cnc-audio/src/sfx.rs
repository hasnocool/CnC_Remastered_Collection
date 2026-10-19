//! Sound effects: decoded sample data, played through mixer channels.

use std::path::Path;
use std::sync::Arc;

use rodio::Decoder;

use crate::mixer::{attenuate, AudioError, AudioMixer, Channel, SfxHandle};
use crate::reader::SharedBytes;

/// Sample data for one sound effect (in-memory, immutable, cheaply shared
/// via [`Arc`]). Use [`SoundEffect::from_bytes`] for in-memory data or
/// [`SoundEffect::from_file`] to load a compressed sample from disk.
#[derive(Clone)]
pub struct SoundEffect {
    data: Arc<Vec<u8>>,
}

impl SoundEffect {
    /// Wrap in-memory sample bytes (WAV, OGG, FLAC or MP3 depending on the
    /// rodio decoder set compiled in).
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    /// Load sample bytes from a file.
    ///
    /// # Errors
    /// [`AudioError::Io`] when the file cannot be read.
    pub fn from_file(path: &Path) -> Result<Self, AudioError> {
        Ok(Self {
            data: Arc::new(std::fs::read(path)?),
        })
    }

    /// Build a fresh decoder over the sample data; decoders own a shallow
    /// clone of the shared bytes so one effect can play on many channels.
    ///
    /// # Errors
    /// [`AudioError::Decoder`] when the data is not a supported format.
    pub fn decoder(&self) -> Result<Decoder<SharedBytes>, AudioError> {
        Ok(Decoder::new(SharedBytes::new(Arc::clone(&self.data)))?)
    }

    /// Play on the lowest-priority SFX path used by one-shot ambient calls:
    /// SFX channel 0 at priority 128, full volume, centred. Prefer
    /// [`SoundEffect::play_on`] for channel-aware gameplay sounds.
    pub fn play(&self, mixer: &AudioMixer) -> Result<SfxHandle, AudioError> {
        self.play_on(mixer, Channel::Sfx(0), 128, 1.0, 0.0)
    }

    /// Play on `channel` with Westwood-style priority eviction semantics
    /// (`priority` 0-255): a strictly higher priority evicts the current
    /// voice; equal or lower is dropped with [`AudioError::ChannelBusy`].
    pub fn play_on(
        &self,
        mixer: &AudioMixer,
        channel: Channel,
        priority: u8,
        volume: f32,
        pan: f32,
    ) -> Result<SfxHandle, AudioError> {
        mixer.play(channel, self.decoder()?, priority, volume, pan)
    }

    /// Positional variant: scales the voice volume with
    /// [`attenuate`] between the listener and the source and leaves the pan
    /// centred (callers with a real listener model can pan themselves by
    /// projecting the source direction onto the screen axis).
    /// A distance at or beyond `max_distance` plays at zero volume but
    /// still occupies the channel; call with an in-range distance for
    /// audible results.
    pub fn play_positional(
        &self,
        mixer: &AudioMixer,
        channel: Channel,
        priority: u8,
        distance: f32,
        max_distance: f32,
    ) -> Result<SfxHandle, AudioError> {
        let volume = attenuate(distance, max_distance);
        mixer.play(channel, self.decoder()?, priority, volume, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::wav_bytes;
    use std::path::Path;

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() < 1.0e-3
    }

    fn effect() -> SoundEffect {
        SoundEffect::from_bytes(wav_bytes(200, 1_000.0, 8_000))
    }

    #[test]
    fn play_on_records_request_with_defaults() {
        let mixer = AudioMixer::headless();
        let handle = effect()
            .play_on(&mixer, Channel::Sfx(4), 180, 0.9, -0.4)
            .expect("plays on idle channel");
        let entry = &mixer.played()[0];
        assert_eq!(entry.channel, Channel::Sfx(4));
        assert_eq!(entry.priority, 180);
        assert!(near(entry.volume, 0.9));
        assert!(near(entry.pan, -0.4));
        assert_eq!(entry.handle, handle);
    }

    #[test]
    fn play_uses_documented_ambient_defaults() {
        let mixer = AudioMixer::headless();
        effect().play(&mixer).expect("plays");
        let entry = &mixer.played()[0];
        assert_eq!(entry.channel, Channel::Sfx(0));
        assert_eq!(entry.priority, 128);
        assert!(near(entry.volume, 1.0));
        assert!(near(entry.pan, 0.0));
    }

    #[test]
    fn play_positional_uses_attenuation() {
        let mixer = AudioMixer::headless();
        let effect = effect();
        // Full volume when at the listener.
        effect
            .play_positional(&mixer, Channel::Sfx(1), 10, 0.0, 20.0)
            .expect("plays");
        assert!(near(mixer.played()[0].volume, 1.0));
        // Half volume at half range.
        effect
            .play_positional(&mixer, Channel::Sfx(2), 10, 10.0, 20.0)
            .expect("plays");
        assert!(near(mixer.played()[1].volume, 0.5));
        // Zero volume at the cut-off, but the channel is still occupied.
        effect
            .play_positional(&mixer, Channel::Sfx(3), 10, 20.0, 20.0)
            .expect("plays silently");
        assert!(near(mixer.played()[2].volume, 0.0));
        // Priority eviction still applies for positional sounds.
        assert!(effect
            .play_positional(&mixer, Channel::Sfx(3), 5, 0.0, 20.0)
            .is_err());
    }

    #[test]
    fn invalid_bytes_fail_with_decoder_error() {
        let mixer = AudioMixer::headless();
        let garbage = SoundEffect::from_bytes(vec![0u8; 64]);
        let result = garbage.play_on(&mixer, Channel::Sfx(0), 10, 1.0, 0.0);
        assert!(matches!(result, Err(AudioError::Decoder(_))));
        assert!(mixer.played().is_empty());
    }

    #[test]
    fn from_file_missing_reports_io_error() {
        let path = Path::new("/definitely/not/a/sample.wav");
        assert!(matches!(
            SoundEffect::from_file(path),
            Err(AudioError::Io(_))
        ));
    }

    #[test]
    fn same_effect_can_play_on_two_channels() {
        let mixer = AudioMixer::headless();
        let effect = effect();
        let _ = effect
            .play_on(&mixer, Channel::Sfx(0), 10, 1.0, 0.0)
            .unwrap();
        let _ = effect
            .play_on(&mixer, Channel::Sfx(1), 10, 1.0, 0.0)
            .unwrap();
        assert_eq!(mixer.played().len(), 2);
        // But two voices never share one channel.
        assert!(effect
            .play_on(&mixer, Channel::Sfx(0), 10, 1.0, 0.0)
            .is_err());
    }
}
