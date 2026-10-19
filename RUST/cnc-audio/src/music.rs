//! Background music: queued tracks, explore/combat layer switching and
//! crossfades driven through the mixer's music channel.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use rodio::Decoder;

use crate::mixer::{AudioError, AudioMixer, Channel};
use crate::reader::SharedBytes;

/// Longest crossfade the player will ever honour, in seconds.
const MAX_CROSSFADE: f32 = 3600.0;

/// Intensity threshold separating the explore layer (below it) from the
/// combat layer (at or above it).
const COMBAT_INTENSITY_THRESHOLD: f32 = 0.5;

/// Which dynamic music layer [`MusicPlayer::set_intensity`] would pull the
/// next track from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MusicLayer {
    /// Quiet/explore layer (intensity < 0.5).
    Explore,
    /// Action/combat layer (intensity >= 0.5).
    Combat,
}

/// One music track: owned compressed bytes plus optional loop points.
/// `loop_start`/`loop_end` are *seconds* into the track, mirroring the
/// in-place loop markers the remastered games use; they are bookkeeping
/// metadata for callers (the rodio decoder cannot seek-loop a live stream),
/// and playback here treats a track as one long fade-in-able source.
#[derive(Clone)]
pub struct MusicTrack {
    data: Arc<Vec<u8>>,
    loop_start: Option<f32>,
    loop_end: Option<f32>,
}

impl MusicTrack {
    /// Wrap in-memory compressed audio (WAV, OGG, FLAC or MP3).
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(data),
            loop_start: None,
            loop_end: None,
        }
    }

    /// Builder-style loop marker setter, in seconds from track start.
    pub fn with_loop_points(mut self, start: f32, end: f32) -> Self {
        self.loop_start = Some(start.max(0.0).min(end.max(0.0)));
        self.loop_end = Some(end.max(0.0));
        self
    }

    /// The configured `(loop_start, loop_end)` markers, if any.
    pub fn loop_points(&self) -> (Option<f32>, Option<f32>) {
        (self.loop_start, self.loop_end)
    }

    /// Build a fresh decoder over the track's bytes.
    ///
    /// # Errors
    /// [`AudioError::Decoder`] with unsupported data.
    pub fn decoder(&self) -> Result<Decoder<SharedBytes>, AudioError> {
        Ok(Decoder::new(SharedBytes::new(Arc::clone(&self.data)))?)
    }
}

/// Queued background music with two dynamic layers.
///
/// Layer selection is deliberately simple (documented behaviour): every
/// `play_next` pulls from the explore queue while [`MusicLayer::Explore`]
/// is active and from the combat queue while [`MusicLayer::Combat`] is
/// active, where the intensity threshold is 0.5. When the active layer's
/// queue is drained, `play_next` falls back to any tracks queued through
/// [`MusicPlayer::queue_track`] so a plain single queue workflow keeps
/// working. Layer switches do not interrupt the playing track; they only
/// decide where the *next* track comes from.
pub struct MusicPlayer {
    /// Generic single track queue (layer-independent fallback).
    queue: VecDeque<MusicTrack>,
    /// Explore layer queue.
    explore: VecDeque<MusicTrack>,
    /// Combat layer queue.
    combat: VecDeque<MusicTrack>,
    /// Currently playing (or most recent) track.
    current: Option<MusicTrack>,
    /// Dynamic-layer intensity in 0..1.
    intensity: f32,
    /// Crossfade duration in seconds.
    crossfade_time: f32,
    /// Whether the player considers playback paused.
    paused: bool,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicPlayer {
    /// Player with an empty queue, neutral intensity, 2-second crossfades.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            explore: VecDeque::new(),
            combat: VecDeque::new(),
            current: None,
            intensity: 0.0,
            crossfade_time: 2.0,
            paused: false,
        }
    }

    /// Queue a track on the generic fallback queue.
    pub fn queue_track(&mut self, track: MusicTrack) {
        self.queue.push_back(track);
    }

    /// Queue a track on the explore layer.
    pub fn queue_explore(&mut self, track: MusicTrack) {
        self.explore.push_back(track);
    }

    /// Queue a track on the combat layer.
    pub fn queue_combat(&mut self, track: MusicTrack) {
        self.combat.push_back(track);
    }

    /// Number of queued tracks per (generic, explore, combat) queue.
    pub fn queue_lengths(&self) -> (usize, usize, usize) {
        (self.queue.len(), self.explore.len(), self.combat.len())
    }

    /// Set the dynamic-layer intensity in 0..1 (clamped). At 0.5 and above
    /// the next `play_next` pulls from the combat layer.
    pub fn set_intensity(&mut self, intensity: f32) -> &mut Self {
        self.intensity = if intensity.is_finite() {
            intensity.clamp(0.0, 1.0)
        } else {
            0.0
        };
        self
    }

    /// Current intensity (0..1).
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Layer the *next* `play_next` would pull from.
    pub fn layer(&self) -> MusicLayer {
        if self.intensity >= COMBAT_INTENSITY_THRESHOLD {
            MusicLayer::Combat
        } else {
            MusicLayer::Explore
        }
    }

    /// Crossfade duration in seconds (0..=3600).
    pub fn crossfade_time(&self) -> f32 {
        self.crossfade_time
    }

    /// Set the crossfade duration in seconds (clamped to 0..3600, NaN
    /// falls back to 0).
    pub fn set_crossfade(&mut self, seconds: f32) -> &mut Self {
        self.crossfade_time = sanitize_seconds(seconds);
        self
    }

    /// The currently playing (or most recently played) track.
    pub fn current_track(&self) -> Option<&MusicTrack> {
        self.current.as_ref()
    }

    /// Whether the player is in the paused state.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Advance to the next track with a crossfade through the mixer.
    /// Returns `Ok(false)` when no track was queued.
    ///
    /// # Errors
    /// Propagates mixer errors ([`AudioError::Decoder`], hardware issues).
    pub fn play_next(&mut self, mixer: &AudioMixer) -> Result<bool, AudioError> {
        let next = match self.layer() {
            MusicLayer::Combat => self.combat.pop_front().or_else(|| self.queue.pop_front()),
            MusicLayer::Explore => self.explore.pop_front().or_else(|| self.queue.pop_front()),
        };
        let Some(track) = next else {
            return Ok(false);
        };
        let source = track.decoder()?;
        let crossfade = Duration::from_secs_f32(self.crossfade_time);
        mixer.crossfade_music(source, crossfade)?;
        self.current = Some(track);
        self.paused = false;
        Ok(true)
    }

    /// Pause through the mixer's music channel and mark the player paused.
    pub fn pause(&mut self, mixer: &AudioMixer) -> Result<(), AudioError> {
        mixer.pause(Channel::Music)?;
        self.paused = true;
        Ok(())
    }

    /// Resume playback through the mixer's music channel.
    pub fn resume(&mut self, mixer: &AudioMixer) -> Result<(), AudioError> {
        mixer.resume(Channel::Music)?;
        self.paused = false;
        Ok(())
    }

    /// Stop music and forget the current track (queued tracks stay).
    pub fn stop(&mut self, mixer: &AudioMixer) {
        mixer.stop_music();
        self.current = None;
        self.paused = false;
    }
}

/// Bound a user-provided duration in seconds into a safe range.
fn sanitize_seconds(seconds: f32) -> f32 {
    if seconds.is_finite() && seconds > 0.0 {
        seconds.min(MAX_CROSSFADE)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::wav_bytes;

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() < 1.0e-6
    }

    fn track(freq: f32) -> MusicTrack {
        // 150 ms, 8 kHz mono: fast to decode, no fixtures on disk.
        MusicTrack::from_bytes(wav_bytes(1_200, freq, 8_000))
    }

    #[test]
    fn default_player_is_neutral() {
        let player = MusicPlayer::new();
        assert_eq!(player.layer(), MusicLayer::Explore);
        assert!(near(player.crossfade_time(), 2.0));
        assert!(near(player.intensity(), 0.0));
        assert!(player.current_track().is_none());
        assert!(!player.is_paused());
        assert_eq!(player.queue_lengths(), (0, 0, 0));
    }

    #[test]
    fn play_next_walks_queue_in_order() {
        let mixer = AudioMixer::headless();
        let mut player = MusicPlayer::new();
        player.queue_explore(track(220.0));
        player.queue_track(track(330.0));
        // An empty player reports "nothing to play".
        let mut empty = MusicPlayer::new();
        assert!(!empty.play_next(&mixer).expect("no error"));
        assert!(empty.current_track().is_none());

        assert!(player.play_next(&mixer).expect("first"));
        assert!(player.current_track().is_some());
        // One explore track was consumed; the generic queue still has one.
        queue_lengths_are(&player, (1, 0, 0), "explore consumed, generic intact");

        // Explore queue drained -> falls back to the generic queue.
        assert!(player.play_next(&mixer).expect("second"));
        queue_lengths_are(&player, (0, 0, 0), "generic consumed too");

        // Third play finds nothing and keeps the last track mounted.
        assert!(!player.play_next(&mixer).expect("empty"));
        assert!(player.current_track().is_some());
        // Two crossfade transitions were recorded.
        assert_eq!(mixer.music_log().len(), 2);
    }

    #[test]
    fn set_intensity_switches_layers() {
        let mixer = AudioMixer::headless();
        let mut player = MusicPlayer::new();
        player.queue_explore(track(110.0));
        player.queue_combat(track(440.0));
        player.set_intensity(0.2);
        assert!(player.play_next(&mixer).expect("explore track"));
        assert_eq!(player.layer(), MusicLayer::Explore);

        // Crossing the threshold switches the active layer without
        // interrupting the current track.
        player.set_intensity(0.8);
        assert_eq!(player.layer(), MusicLayer::Combat);
        assert!(player.current_track().is_some());
        assert!(player.queue_lengths().1 == 0 && player.queue_lengths().2 == 1);
        assert!(player.play_next(&mixer).expect("combat track"));

        // Intensity clamps into 0..1.
        player.set_intensity(12.0);
        assert!(near(player.intensity(), 1.0));
        assert_eq!(player.layer(), MusicLayer::Combat);
        player.set_intensity(-7.0);
        assert!(near(player.intensity(), 0.0));
        player.set_intensity(f32::NAN);
        assert!(near(player.intensity(), 0.0));
    }

    #[test]
    fn crossfade_time_is_recorded_and_sanitized() {
        let mixer = AudioMixer::headless();
        let mut player = MusicPlayer::new();
        player.queue_track(track(110.0));
        player.set_crossfade(3.5);
        player.play_next(&mixer).expect("plays");
        assert!(near(player.crossfade_time(), 3.5));
        assert!(near(mixer.music_log()[0].crossfade, 3.5));

        player.set_crossfade(f32::NAN);
        assert_eq!(player.crossfade_time(), 0.0);
        player.set_crossfade(1e9);
        assert!(near(player.crossfade_time(), 3600.0));
    }

    #[test]
    fn pause_resume_and_stop_track_the_mixer() {
        let mixer = AudioMixer::headless();
        let mut player = MusicPlayer::new();
        player.queue_track(track(110.0));
        player.play_next(&mixer).expect("plays");

        player.pause(&mixer).expect("pauses");
        assert!(player.is_paused());
        assert!(mixer.is_paused(Channel::Music));

        player.resume(&mixer).expect("resumes");
        assert!(!player.is_paused());
        assert!(!mixer.is_paused(Channel::Music));

        player.stop(&mixer);
        assert!(player.current_track().is_none());
        assert!(!player.is_paused());
        // A stop entry was recorded; queued tracks survive a stop.
        let log = mixer.music_log();
        let last = log.last().expect("stop entry recorded");
        assert!(last.stop_requested);
    }

    #[test]
    fn looping_metadata_roundtrips() {
        let track = track(110.0).with_loop_points(1.0, 4.0);
        assert_eq!(track.loop_points(), (Some(1.0), Some(4.0)));
        // Negative starts clamp to zero, and the builder keeps the data.
        let track = MusicTrack::from_bytes(Vec::new()).with_loop_points(-3.0, 2.0);
        assert_eq!(track.loop_points(), (Some(0.0), Some(2.0)));
    }

    /// Assert queue lengths with a readable panic label.
    fn queue_lengths_are(player: &MusicPlayer, expected: (usize, usize, usize), label: &str) {
        assert_eq!(player.queue_lengths(), expected, "{label}");
    }
}
