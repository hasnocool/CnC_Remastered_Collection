//! Movie playback for VQA files (intro, briefing, ending).
//!
//! Wraps the cnc-asset VQA demuxer/decoder and routes decoded frames to a wgpu
//! texture slot while feeding audio samples to a rodio output stream. The API
//! mirrors the original Westwood MCI movie driver: a single active movie at a
//! time, deterministic frame pacing at the container's frame rate, and audio
//! sync within one frame.
//!
//! Falls back to cnc-procgen briefing art when no movie file is available.

use std::path::Path;
use std::time::{Duration, Instant};

use cnc_asset::video::{decode_frames, demux, VqaChunk, VqaError};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use thiserror::Error;

/// Playback state for a single movie.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovieState {
    /// Movie is actively playing (frames advancing, audio streaming).
    Playing,
    /// Movie has finished naturally (last frame presented, audio drained).
    Finished,
    /// Movie was stopped early (user skip, error, campaign transition).
    Stopped,
    /// No movie loaded or playback hasn't started.
    Idle,
}

/// Error during movie playback.
#[derive(Debug, Error)]
pub enum MovieError {
    #[error("VQA demux/decode error: {0}")]
    Vqa(#[from] VqaError),
    #[error("audio stream error: {0}")]
    AudioStream(#[from] rodio::StreamError),
    #[error("audio sink error: {0}")]
    AudioSink(#[from] rodio::PlayError),
    #[error("audio decoder error: {0}")]
    AudioDecoder(#[from] rodio::decoder::DecoderError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("no video frames in container")]
    NoFrames,
    #[error("no audio track in container")]
    NoAudio,
}

/// A queued movie in the campaign flow.
#[derive(Debug, Clone)]
pub struct MovieQueueItem {
    /// Movie file path or asset key.
    pub path: String,
    /// Phase this movie belongs to.
    pub phase: MoviePhase,
    /// Whether this movie is optional (fallback to procgen if missing).
    pub optional: bool,
}

/// Campaign movie playback phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoviePhase {
    /// Intro movie before campaign starts.
    Intro,
    /// Per-mission briefing movie.
    Briefing,
    /// Ending movie after campaign completes.
    Ending,
}

/// Decoded VQA movie ready for playback.
#[derive(Debug, Clone)]
pub struct Movie {
    /// Decoded RGBA frames (one per container frame).
    frames: Vec<Vec<u8>>,
    /// Frame width in pixels.
    frame_width: u32,
    /// Frame height in pixels.
    frame_height: u32,
    /// Frame rate from VQHD header (frames per second).
    frame_rate: u16,
    /// Audio samples (interleaved i16 PCM, mono or stereo).
    audio_samples: Vec<i16>,
    /// Audio sample rate.
    audio_sample_rate: u16,
    /// Number of audio channels (1 or 2).
    audio_channels: u16,
}

impl Movie {
    /// Load and decode a VQA movie from raw container bytes.
    ///
    /// Returns `Err(MovieError::NoFrames)` if the container has no decodable
    /// frames, `Err(MovieError::NoAudio)` if no audio track is present.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MovieError> {
        let vqa = demux(bytes)?;
        let frames = decode_frames(&vqa)?;

        if frames.is_empty() {
            return Err(MovieError::NoFrames);
        }

        // Extract first frame dimensions (all frames same size in v1).
        let header = vqa.header;
        let frame_width = u32::from(header.width_blocks) * 2;
        let frame_height = u32::from(header.height_blocks) * 2;
        let frame_rate = header.frame_rate.max(1); // clamp to at least 1 fps

        // Extract audio track (prefer ADPCM if present, fall back to PCM).
        let (audio_samples, audio_sample_rate, audio_channels) = vqa
            .chunks
            .iter()
            .find_map(|chunk| match chunk {
                VqaChunk::AudioPcm { samples } => {
                    Some((samples.clone(), 22050, 1)) // legacy fallback rate
                }
                VqaChunk::AudioAdpcm {
                    sample_rate,
                    channels,
                } => {
                    // Interleave stereo channels: L0, R0, L1, R1, ...
                    let interleaved = if channels.len() == 2 {
                        let left = &channels[0];
                        let right = &channels[1];
                        let len = left.len().min(right.len());
                        let mut out = Vec::with_capacity(len * 2);
                        for i in 0..len {
                            out.push(left[i]);
                            out.push(right[i]);
                        }
                        out
                    } else {
                        channels[0].clone()
                    };
                    Some((interleaved, *sample_rate, channels.len() as u16))
                }
                _ => None,
            })
            .ok_or(MovieError::NoAudio)?;

        Ok(Self {
            frames,
            frame_width,
            frame_height,
            frame_rate,
            audio_samples,
            audio_sample_rate,
            audio_channels,
        })
    }

    /// Load a movie from a file path.
    pub fn from_file(path: &Path) -> Result<Self, MovieError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// Number of frames in the movie.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Frame dimensions.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.frame_width, self.frame_height)
    }

    /// Frame duration at the container's frame rate.
    pub fn frame_duration(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.frame_rate as f64)
    }

    /// Total movie duration.
    pub fn total_duration(&self) -> Duration {
        self.frame_duration() * self.frame_count() as u32
    }

    /// Get a specific frame's RGBA data.
    pub fn frame(&self, index: usize) -> Option<&[u8]> {
        self.frames.get(index).map(|v| v.as_slice())
    }
}

/// Active movie playback handle.
///
/// Owns the movie data, the current frame index, timing, and the audio sink.
/// Designed for single-threaded use from the main render loop.
pub struct MoviePlayer {
    movie: Option<Movie>,
    state: MovieState,
    current_frame: usize,
    frame_timer: Instant,
    audio_sink: Option<Sink>,
    audio_stream: Option<OutputStream>,
    audio_handle: Option<OutputStreamHandle>,
    /// Whether the movie was explicitly skipped (vs finished naturally).
    skipped: bool,
}

impl MoviePlayer {
    /// Create a new idle movie player.
    pub fn new() -> Self {
        Self {
            movie: None,
            state: MovieState::Idle,
            current_frame: 0,
            frame_timer: Instant::now(),
            audio_sink: None,
            audio_stream: None,
            audio_handle: None,
            skipped: false,
        }
    }

    /// Load and start playing a movie from bytes.
    ///
    /// Takes ownership of the audio output stream and sink. Returns the
    /// first frame's RGBA data immediately for the first render.
    pub fn play(&mut self, bytes: &[u8]) -> Result<Option<&[u8]>, MovieError> {
        self.stop();

        let movie = Movie::from_bytes(bytes)?;
        let _frame_duration = movie.frame_duration();

        // Set up audio output stream and sink.
        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        // Convert audio samples to a rodio Source.
        let audio_source = AudioSource::new(
            movie.audio_samples.clone(),
            movie.audio_sample_rate,
            movie.audio_channels,
        );
        sink.append(audio_source);
        sink.set_volume(1.0);

        // Advance to first frame.
        self.current_frame = 0;
        self.frame_timer = Instant::now();
        self.movie = Some(movie);
        self.audio_sink = Some(sink);
        self.audio_stream = Some(stream);
        self.audio_handle = Some(handle);
        self.state = MovieState::Playing;
        self.skipped = false;

        Ok(self.current_frame_rgba())
    }

    /// Load and start playing a movie from a file.
    pub fn play_file(&mut self, path: &Path) -> Result<Option<&[u8]>, MovieError> {
        let bytes = std::fs::read(path)?;
        self.play(&bytes)
    }

    /// Get the current frame's RGBA data (if playing).
    pub fn current_frame_rgba(&self) -> Option<&[u8]> {
        self.movie
            .as_ref()
            .and_then(|m| m.frame(self.current_frame))
    }

    /// Current playback state.
    pub fn state(&self) -> MovieState {
        self.state
    }

    /// Current frame index (0-based).
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    /// Total frame count.
    pub fn total_frames(&self) -> usize {
        self.movie.as_ref().map(|m| m.frame_count()).unwrap_or(0)
    }

    /// Frame dimensions.
    pub fn dimensions(&self) -> Option<(u32, u32)> {
        self.movie.as_ref().map(|m| m.dimensions())
    }

    /// Advance playback by one frame if it's time.
    ///
    /// Returns `true` if a new frame was presented, `false` if waiting for
    /// the next frame interval or if the movie has finished.
    pub fn tick(&mut self) -> bool {
        if self.state != MovieState::Playing {
            return false;
        }

        let Some(movie) = &self.movie else {
            return false;
        };

        let frame_duration = movie.frame_duration();
        let elapsed = self.frame_timer.elapsed();

        if elapsed < frame_duration {
            return false;
        }

        // Advance frame.
        self.current_frame += 1;
        self.frame_timer = Instant::now();

        if self.current_frame >= movie.frame_count() {
            // Movie finished.
            self.state = MovieState::Finished;
            return false;
        }

        true
    }

    /// Skip the current movie (stop playback immediately).
    pub fn skip(&mut self) {
        self.skipped = true;
        self.stop();
    }

    /// Stop playback and clean up.
    pub fn stop(&mut self) {
        if let Some(sink) = self.audio_sink.take() {
            sink.stop();
        }
        self.audio_stream = None;
        self.audio_handle = None;
        self.movie = None;
        self.state = MovieState::Stopped;
        self.current_frame = 0;
    }

    /// Whether the movie was skipped (vs finished naturally).
    pub fn was_skipped(&self) -> bool {
        self.skipped
    }

    /// Check if audio is still playing.
    pub fn is_audio_playing(&self) -> bool {
        self.audio_sink
            .as_ref()
            .map(|s| !s.empty())
            .unwrap_or(false)
    }
}

impl Default for MoviePlayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Rodio Source for VQA audio samples.
struct AudioSource {
    samples: Vec<i16>,
    sample_rate: u16,
    channels: u16,
    position: usize,
}

impl AudioSource {
    fn new(samples: Vec<i16>, sample_rate: u16, channels: u16) -> Self {
        Self {
            samples,
            sample_rate,
            channels,
            position: 0,
        }
    }
}

impl Iterator for AudioSource {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.samples.len() {
            let sample = self.samples[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Source for AudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some((self.samples.len() - self.position) / self.channels as usize)
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.samples.len() as f64 / (self.sample_rate as f64 * self.channels as f64),
        ))
    }
}

/// Procgen fallback briefing art generator.
///
/// Uses cnc-procgen to generate a deterministic faction emblem and commander
/// portrait when no VQA movie is available for a briefing.
pub mod fallback {
    use cnc_core::color::PlayerRemap;
    use cnc_core::palette::Palette;
    use cnc_procgen::briefing::{
        generate_briefing_emblem, generate_briefing_portrait, BriefingFaction,
    };
    use cnc_procgen::prng::Seed;
    use image::RgbaImage;

    /// Generate fallback briefing art for a mission.
    ///
    /// Returns (emblem_rgba, portrait_rgba) as 48x48 RGBA images.
    pub fn generate_briefing_fallback(
        faction: BriefingFaction,
        mission_index: u32,
        palette: &Palette,
        remap: &PlayerRemap,
    ) -> (RgbaImage, RgbaImage) {
        let seed = Seed::new(mission_index as u64);
        let emblem = generate_briefing_emblem(faction, mission_index, seed, palette, remap);
        let portrait = generate_briefing_portrait(faction, seed, palette, remap);
        (emblem, portrait)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movie_player_idle_state() {
        let player = MoviePlayer::new();
        assert_eq!(player.state(), MovieState::Idle);
        assert_eq!(player.current_frame(), 0);
        assert_eq!(player.total_frames(), 0);
        assert!(player.current_frame_rgba().is_none());
    }

    #[test]
    fn movie_player_skip_idle() {
        let mut player = MoviePlayer::new();
        player.skip();
        assert_eq!(player.state(), MovieState::Stopped);
    }
}
