// Sound Effects
use rodio::{Decoder, Source};
use std::io::Cursor;
use anyhow::Result;
use std::sync::Arc;

pub struct SoundEffect { data: Arc<Vec<u8>> }

impl SoundEffect {
    pub fn from_bytes(data: Vec<u8>) -> Self { Self { data: Arc::new(data) } }
    pub fn from_file(path: &std::path::Path) -> Result<Self> { Ok(Self { data: Arc::new(std::fs::read(path)?) }) }
    pub fn decoder(&self) -> Result<Decoder<Cursor<&[u8]>>> { Ok(Decoder::new(Cursor::new(&self.data[..]))?) }
    pub fn play(&self, mixer: &crate::mixer::AudioMixer) -> Result<()> { let source = self.decoder()?; mixer.play_sfx(source); Ok(()) }
}

#[derive(Clone, Copy, Debug)]
pub struct SfxHandle(pub u32);
