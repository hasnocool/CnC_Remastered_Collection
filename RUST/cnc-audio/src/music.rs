// Music Player
use rodio::{Decoder, Source};
use std::io::Cursor;
use anyhow::Result;
use std::sync::Arc;
use std::collections::VecDeque;

pub struct MusicTrack { data: Arc<Vec<u8>>, loop_start: Option<f32>, loop_end: Option<f32> }

impl MusicTrack {
    pub fn from_bytes(data: Vec<u8>) -> Self { Self { data: Arc::new(data), loop_start: None, loop_end: None } }
    pub fn with_loop_points(mut self, start: f32, end: f32) -> Self { self.loop_start = Some(start); self.loop_end = Some(end); self }
    pub fn decoder(&self) -> Result<Decoder<Cursor<&[u8]>>> { Ok(Decoder::new(Cursor::new(&self.data[..]))?) }
}

pub struct MusicPlayer {
    queue: VecDeque<MusicTrack>,
    current: Option<MusicTrack>,
    crossfade_time: f32,
}

impl MusicPlayer {
    pub fn new() -> Self { Self { queue: VecDeque::new(), current: None, crossfade_time: 2.0 } }
    pub fn queue(&mut self, track: MusicTrack) { self.queue.push_back(track); }
    pub fn play_next(&mut self, mixer: &crate::mixer::AudioMixer) -> Result<()> { if let Some(track) = self.queue.pop_front() { mixer.play_music(track.decoder()?); self.current = Some(track); } Ok(()) }
    pub fn set_crossfade(&mut self, seconds: f32) { self.crossfade_time = seconds; }
}
