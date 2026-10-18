use rodio::{OutputStream, Sink, Source};
use anyhow::Result;

/// Audio mixer for SFX and music
pub struct AudioMixer {
    output_stream: Option<OutputStream>,
    sfx_sink: Option<Sink>,
    music_sink: Option<Sink>,
}

impl AudioMixer {
    pub fn new() -> Result<Self> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sfx_sink = Sink::try_new(&handle)?;
        let music_sink = Sink::try_new(&handle)?;
        Ok(Self { output_stream: Some(_stream), sfx_sink: Some(sfx_sink), music_sink: Some(music_sink) })
    }

    pub fn play_sfx<S: Source + Send + 'static>(&self, source: S) {
        if let Some(sink) = &self.sfx_sink {
            sink.append(source);
        }
    }

    pub fn play_music<S: Source + Send + 'static>(&self, source: S) {
        if let Some(sink) = &self.music_sink {
            sink.append(source);
        }
    }

    pub fn set_sfx_volume(&self, volume: f32) {
        if let Some(sink) = &self.sfx_sink {
            sink.set_volume(volume);
        }
    }

    pub fn set_music_volume(&self, volume: f32) {
        if let Some(sink) = &self.music_sink {
            sink.set_volume(volume);
        }
    }
}

pub struct MixerChannel(pub u32);
