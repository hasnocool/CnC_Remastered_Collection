//! Audio stub
use anyhow::Result;

pub struct AudioMixer;

impl AudioMixer {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

pub struct MixerChannel(pub u32);
pub struct SoundEffect;
pub struct SfxHandle(pub u32);
pub struct MusicPlayer;
pub struct MusicTrack;
