//! Channel-based audio mixer.
//!
//! The mixer owns one output channel slot per [`Channel`] (see the crate
//! docs). Channel slots are single-voice: a play request on a busy channel
//! succeeds only with a strictly higher priority — equal or lower requests
//! are dropped (Westwood priority semantics, priorities 0-255).
//!
//! Two construction modes exist:
//!
//! * [`AudioMixer::new`] opens a real output stream, one [`rodio::Sink`] per
//!   SFX/voice channel and two alternating sinks for music crossfading.
//! * [`AudioMixer::headless`] skips all audio hardware and records every
//!   play request into an in-memory log that tests can inspect via
//!   [`AudioMixer::played`] and [`AudioMixer::music_log`]. Allocation,
//!   priority and volume logic is identical in both modes.

use std::collections::HashMap;
use std::io::{Read, Seek};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use rodio::source::ChannelVolume;
use rodio::{Decoder, OutputStream, Sample, Sink, Source};
use thiserror::Error;

/// Number of dedicated SFX channels (0..8) alongside `Voice` and `Music`.
pub const SFX_CHANNEL_COUNT: u8 = 8;

/// Sentinel "no fade-out in progress" marker inside [`SharedFade`].
const FADE_NONE: u64 = u64::MAX;

/// Upper bound for a fade ramp in samples (~6,000 years at 44.1 kHz stereo)
/// so completion math can never wrap around `u64`.
const RAMP_SAMPLES_CAP: u64 = 1_000_000_000_000_000;

/// Typed error for every fallible mixer operation.
#[derive(Debug, Error)]
pub enum AudioError {
    /// No default output device or the device could not be configured.
    #[error("audio output stream unavailable: {0}")]
    OutputStream(#[from] rodio::StreamError),
    /// A sink could not be attached to the output stream handle.
    #[error("audio sink unavailable: {0}")]
    Sink(#[from] rodio::PlayError),
    /// Compressed sample data could not be decoded.
    #[error("audio decoder error: {0}")]
    Decoder(#[from] rodio::decoder::DecoderError),
    /// The channel cannot service this request: either the SFX id is out of
    /// range, or it is the music channel, which is driven by
    /// [`AudioMixer::crossfade_music`] instead.
    #[error("invalid audio channel {0:?}")]
    InvalidChannel(Channel),
    /// The channel is occupied by a sound of equal or higher priority; the
    /// request was dropped and nothing was played.
    #[error("channel {0:?} is busy with an equal or higher priority sound")]
    ChannelBusy(Channel),
    /// Reading a sample file from disk failed.
    #[error("audio file I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// A mixer output channel. Each channel plays at most one sound at a time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    /// Numbered SFX channel, valid ids 0..[`SFX_CHANNEL_COUNT`].
    Sfx(u8),
    /// EVA / unit voice-over channel.
    Voice,
    /// Background music channel (backed by two alternating sinks).
    Music,
}

/// Alias for [`Channel`] for call sites that think in "channel ids".
pub type ChannelId = Channel;

impl Channel {
    /// All valid SFX channels in ascending id order.
    pub fn sfx_all() -> impl ExactSizeIterator<Item = Channel> {
        (0..SFX_CHANNEL_COUNT).map(Channel::Sfx)
    }

    /// Internal slot index of this channel; `None` for out-of-range SFX ids.
    pub fn slot(self) -> Option<usize> {
        match self {
            Channel::Sfx(index) => (index < SFX_CHANNEL_COUNT).then_some(index as usize),
            Channel::Voice => Some(SFX_CHANNEL_COUNT as usize),
            Channel::Music => Some((SFX_CHANNEL_COUNT + 1) as usize),
        }
    }

    /// Total number of channel slots (8 SFX + voice + music).
    pub fn slot_count() -> usize {
        SFX_CHANNEL_COUNT as usize + 2
    }

    /// Short readable name, useful for logs.
    pub fn name(self) -> &'static str {
        match self {
            Channel::Sfx(_) => "sfx",
            Channel::Voice => "voice",
            Channel::Music => "music",
        }
    }
}

/// Handle to a sound voice that the mixer accepted. Can stop the sound
/// later; invalidated the moment the voice is evicted, replaced, or ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SfxHandle(pub u32);

/// Linear distance-attenuation factor for positional SFX. Returns 1.0 at
/// `distance == 0`, ramps linearly down to 0.0 at `max_distance` and stays
/// clamped to [0, 1] beyond it. A non-positive `max_distance` means the
/// sound is audible only at distance zero. NaN inputs collapse to the
/// nearest sane value (0 distance for the measure, 0 volume for the range).
pub fn attenuate(distance: f32, max_distance: f32) -> f32 {
    let distance = if distance.is_nan() {
        0.0
    } else {
        distance.max(0.0)
    };
    if !distance.is_finite() {
        return 0.0; // infinitely far away — inaudible at any ceiling
    }
    if max_distance.is_nan() || max_distance <= 0.0 {
        // Zero/negative/no ceiling: audible only when already at the source.
        return if distance <= 0.0 { 1.0 } else { 0.0 };
    }
    // An infinite ceiling degrades to full volume for finite distances.
    let value = 1.0 - distance / max_distance;
    value.clamp(0.0, 1.0)
}

/// Approximate one-axis Doppler ratio for `speed_of_sound = 343.0` m/s.
/// `listener_vel` and `source_vel` are signed velocities along the line
/// source -> listener; **positive means approaching** the other party. The
/// result is `(c + listener) / (c - source)` — the classic moving source
/// toward a listener / moving listener toward a source composition folded
/// into one scalar multiplier for `Source::speed`.
///
/// Values are clamped to `[1/8, 8]`: a source racing head-on at the speed
/// of sound returns the clamp maximum rather than the divergent infinity
/// the real physics would produce at that threshold.
///
/// The original C&C had no Doppler at all; this is an optional utility and
/// is never applied by the mixer automatically.
#[allow(clippy::manual_clamp)] // clamping band is [1/8, 8], not [min, max] literals
pub fn doppler_factor(listener_vel: f32, source_vel: f32, speed_of_sound: f32) -> f32 {
    const MAX: f32 = 8.0;
    const MIN: f32 = 1.0 / 8.0;
    if !speed_of_sound.is_finite() || speed_of_sound <= 0.0 {
        return 1.0;
    }
    let listener = if listener_vel.is_finite() {
        listener_vel
    } else {
        0.0
    };
    let source = if source_vel.is_finite() {
        source_vel
    } else {
        0.0
    };
    if source >= speed_of_sound {
        return MAX; // shock-front saturation
    }
    let ratio = (speed_of_sound + listener) / (speed_of_sound - source);
    if ratio.is_nan() {
        return 1.0;
    }
    if ratio < MIN {
        MIN
    } else if ratio > MAX {
        MAX
    } else {
        ratio
    }
}

/// Constant-power stereo gains for a pan position in `-1..1`
/// (hard left .. hard right). At 0 both channels sit at `sqrt(0.5)` so the
/// total signal power is preserved (standard linear crossfeed pan law).
/// NaN pans fall back to centre and out-of-range pans clamp.
pub fn pan_gains(pan: f32) -> (f32, f32) {
    let pan = if pan.is_nan() {
        0.0
    } else {
        pan.clamp(-1.0, 1.0)
    };
    let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
    (angle.cos(), angle.sin())
}

/// Clamp an `f32` to `[min, max]`, mapping NaN/infinite input to `fallback`.
fn clamp_unit(value: f32, min: f32, max: f32, fallback: f32) -> f32 {
    if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    }
}

/// Shared control object for a music fade envelope (see [`FadeSource`] for
/// the sample-producing wrapper). One `SharedFade` governs one playing
/// track: it ramps 0 -> 1 over `duration` while the track fades in, holds
/// 1.0, and — once `begin_fade_out` is called from any thread — ramps back
/// down to 0.0 over the same duration, after which the wrapper ends the
/// source so its sink empties itself.
#[derive(Debug)]
pub struct SharedFade {
    /// Length of one linear ramp in samples (covering every channel).
    ramp_samples: u64,
    /// Samples emitted so far; advanced by the audio pull thread.
    elapsed: AtomicU64,
    /// Elapsed value at which the fade-out began, [`FADE_NONE`] until then.
    fade_out_start: AtomicU64,
    /// Never fade in; report full gain until the fade-out starts. Used by
    /// the stand-alone [`FadeSource::fade_out`] constructor.
    instant: bool,
}

impl SharedFade {
    /// Fade-in envelope: 0.0 at the first sample, 1.0 after `duration`.
    pub fn new(duration: Duration, sample_rate: u32, channels: u16) -> Self {
        let ramp = ((duration.as_secs_f64() * f64::from(sample_rate) * f64::from(channels))
            .round()
            .min(RAMP_SAMPLES_CAP as f64)) as u64;
        Self {
            ramp_samples: ramp,
            elapsed: AtomicU64::new(0),
            fade_out_start: AtomicU64::new(FADE_NONE),
            instant: false,
        }
    }

    /// Stand-alone fade-out envelope: 1.0 at the first sample, 0.0 after
    /// `duration`, then the source ends.
    pub fn fade_out(duration: Duration, sample_rate: u32, channels: u16) -> Self {
        let mut this = Self::new(duration, sample_rate, channels);
        this.instant = true;
        this.fade_out_start = AtomicU64::new(0);
        this
    }

    /// Length of one linear ramp in samples.
    pub fn ramp_samples(&self) -> u64 {
        self.ramp_samples
    }

    /// Trigger the fade-out phase "now": fading down from the currently
    /// held gain to 0.0 over the ramp length. Calls after the first are
    /// ignored so the fade length stays constant.
    pub fn begin_fade_out(&self) {
        if self.fade_out_start.load(Ordering::Relaxed) == FADE_NONE {
            let now = self.elapsed.load(Ordering::Relaxed);
            // Only claim the slot if nobody raced us to it.
            let _ = self.fade_out_start.compare_exchange(
                FADE_NONE,
                now,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
        }
    }

    /// Envelope gain at a given elapsed sample count. Pure and
    /// deterministic so tests can assert exact ramp positions without an
    /// audio thread.
    pub fn envelope_at(&self, elapsed: u64) -> f32 {
        let in_gain = if self.instant || self.ramp_samples == 0 {
            1.0
        } else {
            elapsed.min(self.ramp_samples) as f32 / self.ramp_samples as f32
        };
        let start = self.fade_out_start.load(Ordering::Relaxed);
        if start == FADE_NONE {
            return in_gain;
        }
        let out_gain = if self.ramp_samples == 0 {
            0.0
        } else {
            1.0 - elapsed.saturating_sub(start) as f32 / self.ramp_samples as f32
        };
        in_gain * out_gain.clamp(0.0, 1.0)
    }
}

/// A source wrapper that multiplies every sample with a linear fade
/// envelope (see [`SharedFade`]). Wrap a source at the moment it starts
/// playing with `fade_in`; for crossfades, keep the wrapper's
/// [`SharedFade`] handle around and call `SharedFade::begin_fade_out` when
/// the track should retreat under a new one.
///
/// The envelope counts *samples*: a `duration` of `d` seconds at rate `r`
/// with `c` channels ramps over `d * r * c` output samples, so the envelope
/// keeps its wall-clock length no matter the channel count.
pub struct FadeSource<S> {
    inner: S,
    shared: Arc<SharedFade>,
}

impl<S> FadeSource<S>
where
    S: Source,
    S::Item: Sample,
{
    /// Wrap `inner` to ramp its gain from 0 -> 1 over `duration`.
    pub fn fade_in(inner: S, duration: Duration) -> Self {
        let shared = SharedFade::new(duration, inner.sample_rate(), inner.channels());
        Self::new(inner, Arc::new(shared))
    }

    /// Wrap `inner` to ramp its gain from 1 -> 0 over `duration` and then
    /// end the source (so an attached sink empties itself).
    pub fn fade_out(inner: S, duration: Duration) -> Self {
        let shared = SharedFade::fade_out(duration, inner.sample_rate(), inner.channels());
        Self::new(inner, Arc::new(shared))
    }

    /// Wrap `inner` with a pre-built shared envelope (crossfade path).
    pub fn new(inner: S, shared: Arc<SharedFade>) -> Self {
        Self { inner, shared }
    }

    /// Envelope gain the next yielded sample receives.
    pub fn envelope_at(&self, elapsed: u64) -> f32 {
        self.shared.envelope_at(elapsed)
    }

    /// The shared envelope, for calling `SharedFade::begin_fade_out` later.
    pub fn shared(&self) -> &Arc<SharedFade> {
        &self.shared
    }
}

impl<S> Iterator for FadeSource<S>
where
    S: Source,
    S::Item: Sample,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<S::Item> {
        let item = self.inner.next()?;
        let elapsed = self.shared.elapsed.fetch_add(1, Ordering::Relaxed);
        // Once the fade-out ramp fully elapsed (including the final
        // zero-gain sample) the source is done.
        let start = self.shared.fade_out_start.load(Ordering::Relaxed);
        if start != FADE_NONE && elapsed > start + self.shared.ramp_samples {
            return None;
        }
        Some(item.amplify(self.shared.envelope_at(elapsed)))
    }
}

impl<S> Source for FadeSource<S>
where
    S: Source,
    S::Item: Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.inner.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
}

/// Per-channel bookkeeping: volume, pitch, pause state and current voice.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ChannelState {
    volume: f32,
    pitch: f32,
    paused: bool,
    voice: Option<ActiveVoice>,
}

impl ChannelState {
    const fn new() -> Self {
        Self {
            volume: 1.0,
            pitch: 1.0,
            paused: false,
            voice: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ActiveVoice {
    handle: SfxHandle,
    priority: u8,
}

/// One play request as recorded by the headless backend (CI/test mode).
#[derive(Clone, Debug, PartialEq)]
pub struct RecordedPlay {
    /// Channel the request was made on.
    pub channel: Channel,
    /// Request priority (0-255).
    pub priority: u8,
    /// Clamped per-voice volume (0..1).
    pub volume: f32,
    /// Clamped pan value (-1..1).
    pub pan: f32,
    /// Channel pitch multiplier at the time of the request.
    pub pitch: f32,
    /// Full audibility product `master * channel * voice` volume.
    pub effective_volume: f32,
    /// Handle assigned to the new voice.
    pub handle: SfxHandle,
    /// Handle of the voice that was evicted by this request, if any.
    pub evicted: Option<SfxHandle>,
}

/// One music transition as recorded by the headless backend.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordedMusic {
    /// Crossfade duration requested for the transition, in seconds.
    pub crossfade: f32,
    /// Whether a previously playing track began its fade-out.
    pub faded_out_previous: bool,
    /// Whether this entry is a `stop_music` tear-down.
    pub stop_requested: bool,
    /// Audibility product `master * music bus * music channel` the track
    /// would play at (see [`AudioMixer::set_music_volume`]).
    pub effective_volume: f32,
}

#[derive(Default)]
struct HeadlessMusicLog {
    entries: Vec<RecordedMusic>,
    active: bool,
}

struct RealMusicBackend {
    /// Which of the two music sinks currently holds the leading track.
    active: usize,
    /// Shared fade envelopes of both music sink slots.
    states: [Option<Arc<SharedFade>>; 2],
}

enum Backend {
    /// No audio device; all playback requests are recorded instead.
    Headless {
        plays: Mutex<Vec<RecordedPlay>>,
        music: Mutex<HeadlessMusicLog>,
    },
    /// Real output stream with dedicated sinks per channel slot.
    Real {
        sinks: Vec<Sink>,
        music_sinks: [Sink; 2],
        music: Mutex<RealMusicBackend>,
        // Declared last so the cpal stream is torn down after its sinks.
        _stream: OutputStream,
    },
}

/// The C&C-style channel mixer: one output device, one voice per channel.
pub struct AudioMixer {
    /// Master volume applied on top of per-channel volumes.
    master_volume: Mutex<f32>,
    /// Music bus gain (0..=1) multiplied at the music call sites
    /// ([`AudioMixer::crossfade_music`]); 1.0 keeps the default behavior.
    music_volume: Mutex<f32>,
    /// Effects bus gain (0..=1) shared by every SFX and voice channel,
    /// multiplied at the [`AudioMixer::play`] call sites; 1.0 keeps the
    /// default behavior.
    effects_volume: Mutex<f32>,
    /// Per-slot channel state.
    channels: Mutex<Vec<ChannelState>>,
    /// Active-voice registry: handle -> channel the voice plays on.
    voices: Mutex<HashMap<SfxHandle, Channel>>,
    /// Monotonic handle generator (starts at 1; 0 is never assigned).
    next_handle: AtomicU32,
    backend: Backend,
}

impl AudioMixer {
    /// Open the default output device and attach one sink per channel.
    /// Returns an [`AudioError`] when no device is present — CI hosts and
    /// tests should use [`AudioMixer::headless`] instead.
    pub fn new() -> Result<Self, AudioError> {
        let (stream, handle) = OutputStream::try_default()?;
        let mut sinks = Vec::with_capacity(Channel::slot_count() - 1);
        for _ in 0..Channel::slot_count() - 1 {
            sinks.push(Sink::try_new(&handle)?);
        }
        let music_sinks = [Sink::try_new(&handle)?, Sink::try_new(&handle)?];
        Ok(Self {
            master_volume: Mutex::new(1.0),
            music_volume: Mutex::new(1.0),
            effects_volume: Mutex::new(1.0),
            channels: Mutex::new(vec![ChannelState::new(); Channel::slot_count()]),
            voices: Mutex::new(HashMap::new()),
            next_handle: AtomicU32::new(1),
            backend: Backend::Real {
                sinks,
                music_sinks,
                music: Mutex::new(RealMusicBackend {
                    active: 0,
                    states: [None, None],
                }),
                _stream: stream,
            },
        })
    }

    /// Build a mixer with no audio device: play requests are recorded into
    /// in-memory logs ([`AudioMixer::played`], [`AudioMixer::music_log`])
    /// while exercising the same allocation, priority and volume logic.
    pub fn headless() -> Self {
        Self {
            master_volume: Mutex::new(1.0),
            music_volume: Mutex::new(1.0),
            effects_volume: Mutex::new(1.0),
            channels: Mutex::new(vec![ChannelState::new(); Channel::slot_count()]),
            voices: Mutex::new(HashMap::new()),
            next_handle: AtomicU32::new(1),
            backend: Backend::Headless {
                plays: Mutex::new(Vec::new()),
                music: Mutex::new(HeadlessMusicLog::default()),
            },
        }
    }

    /// `true` when this mixer never touches audio hardware.
    pub fn is_headless(&self) -> bool {
        matches!(self.backend, Backend::Headless { .. })
    }

    /// Snapshot of recorded SFX play requests (headless only, else empty).
    pub fn played(&self) -> Vec<RecordedPlay> {
        match &self.backend {
            Backend::Headless { plays, .. } => plays.lock().clone(),
            Backend::Real { .. } => Vec::new(),
        }
    }

    /// Snapshot of recorded music transitions (headless only, else empty).
    pub fn music_log(&self) -> Vec<RecordedMusic> {
        match &self.backend {
            Backend::Headless { music, .. } => music.lock().entries.clone(),
            Backend::Real { .. } => Vec::new(),
        }
    }

    /// Assign the next [`SfxHandle`].
    fn next_handle(&self) -> SfxHandle {
        SfxHandle(self.next_handle.fetch_add(1, Ordering::Relaxed))
    }

    /// Play a decoded sound on `channel`.
    ///
    /// `priority` follows Westwood 0-255 semantics: while the channel is
    /// occupied, a strictly higher priority evicts the current voice;
    /// equal or lower priorities are rejected with
    /// [`AudioError::ChannelBusy`] and nothing is played.
    ///
    /// `volume` is the per-voice gain (0..1); audibility is
    /// `master * channel * voice`. `pan` is -1..1, hard left..hard right.
    /// Distance attenuation and Doppler are caller-side utilities
    /// ([`attenuate`], [`doppler_factor`]); the mixer never moves sources.
    ///
    /// The music channel is driven by [`AudioMixer::crossfade_music`].
    pub fn play<R>(
        &self,
        channel: Channel,
        source: Decoder<R>,
        priority: u8,
        volume: f32,
        pan: f32,
    ) -> Result<SfxHandle, AudioError>
    where
        R: Read + Seek + Send + 'static,
    {
        if channel == Channel::Music {
            return Err(AudioError::InvalidChannel(channel));
        }
        let Some(slot) = channel.slot() else {
            return Err(AudioError::InvalidChannel(channel));
        };
        let volume = clamp_unit(volume, 0.0, 1.0, 0.0);
        let handle = self.next_handle();

        // Channel bookkeeping under one lock: the eviction decision plus a
        // consistent read of pitch and volume. The master lock is taken
        // nested and always in the order master-after-channels.
        let (evicted, pitch, effective_volume, base_volume) = {
            let mut channels = self.channels.lock();
            let state = &mut channels[slot];
            let evicted = match state.voice {
                Some(current) if priority > current.priority => Some(current.handle),
                Some(_) => return Err(AudioError::ChannelBusy(channel)),
                None => None,
            };
            state.voice = Some(ActiveVoice { handle, priority });
            let master = *self.master_volume.lock();
            let effects = *self.effects_volume.lock();
            (
                evicted,
                state.pitch,
                master * effects * state.volume * volume,
                master * effects * state.volume,
            )
        };
        if let Some(old) = evicted {
            self.voices.lock().remove(&old);
        }
        self.voices.lock().insert(handle, channel);

        match &self.backend {
            Backend::Real { sinks, .. } => {
                let sink = &sinks[slot];
                // Evict anything still queued on this single-voice sink and
                // start the new voice right away (append un-stops the sink).
                if evicted.is_some() {
                    sink.stop();
                }
                let (left, right) = pan_gains(pan);
                let panned = ChannelVolume::new(source, vec![left, right]);
                sink.append(panned.amplify(volume));
                sink.set_speed(pitch);
                sink.set_volume(base_volume);
            }
            Backend::Headless { plays, .. } => {
                plays.lock().push(RecordedPlay {
                    channel,
                    priority,
                    volume,
                    pan,
                    pitch,
                    effective_volume,
                    handle,
                    evicted,
                });
            }
        }
        Ok(handle)
    }

    /// Stop the voice behind `handle`. Returns `true` when the handle still
    /// owned the channel's current voice and playback was cut; `false` when
    /// the voice had already ended or been replaced.
    pub fn stop(&self, handle: SfxHandle) -> bool {
        let Some(channel) = self.voices.lock().remove(&handle) else {
            return false;
        };
        let Some(slot) = channel.slot() else {
            return false;
        };
        let stopped = {
            let mut channels = self.channels.lock();
            let matches = channels[slot]
                .voice
                .is_some_and(|voice| voice.handle == handle);
            if matches {
                channels[slot].voice = None;
            }
            matches
        };
        if stopped {
            if let Backend::Real { sinks, .. } = &self.backend {
                sinks[slot].stop();
            }
        }
        stopped
    }

    /// Stop whatever is playing on `channel`. Returns `true` when a voice
    /// was cut. (Music state is managed separately; prefer
    /// [`AudioMixer::stop_music`].)
    pub fn stop_channel(&self, channel: Channel) -> bool {
        let Some(slot) = channel.slot() else {
            return false;
        };
        let current = self.channels.lock()[slot].voice;
        let Some(voice) = current else {
            return false;
        };
        self.stop(voice.handle)
    }

    /// Stop every SFX and voice channel. Music keeps playing;
    /// [`AudioMixer::stop_music`] handles it.
    pub fn stop_all(&self) {
        {
            let mut voices = self.voices.lock();
            let mut channels = self.channels.lock();
            let voice_slots = Channel::slot_count() - 1;
            for slot in 0..voice_slots {
                if let Some(voice) = channels[slot].voice.take() {
                    voices.remove(&voice.handle);
                }
            }
        }
        if let Backend::Real { sinks, .. } = &self.backend {
            for sink in sinks.iter() {
                sink.stop();
            }
        }
    }

    /// Pause a channel (hardware sinks and future bookkeeping).
    pub fn pause(&self, channel: Channel) -> Result<(), AudioError> {
        let Some(slot) = channel.slot() else {
            return Err(AudioError::InvalidChannel(channel));
        };
        self.channels.lock()[slot].paused = true;
        if let Backend::Real {
            sinks, music_sinks, ..
        } = &self.backend
        {
            if channel == Channel::Music {
                music_sinks.iter().for_each(|sink| sink.pause());
            } else {
                sinks[slot].pause();
            }
        }
        Ok(())
    }

    /// Resume a paused channel.
    pub fn resume(&self, channel: Channel) -> Result<(), AudioError> {
        let Some(slot) = channel.slot() else {
            return Err(AudioError::InvalidChannel(channel));
        };
        self.channels.lock()[slot].paused = false;
        if let Backend::Real {
            sinks, music_sinks, ..
        } = &self.backend
        {
            if channel == Channel::Music {
                music_sinks.iter().for_each(|sink| sink.play());
            } else {
                sinks[slot].play();
            }
        }
        Ok(())
    }

    /// `true` when the channel was last put into the paused state.
    pub fn is_paused(&self, channel: Channel) -> bool {
        channel
            .slot()
            .is_some_and(|slot| self.channels.lock()[slot].paused)
    }

    /// Set the master volume (0..1, clamped). Applies immediately in real
    /// mode by adjusting every sink's volume.
    pub fn set_master_volume(&self, volume: f32) {
        let volume = clamp_unit(volume, 0.0, 1.0, 0.0);
        *self.master_volume.lock() = volume;
        if let Backend::Real {
            sinks, music_sinks, ..
        } = &self.backend
        {
            let channels = self.channels.lock();
            let effects = *self.effects_volume.lock();
            let music = *self.music_volume.lock();
            for (slot, sink) in sinks.iter().enumerate() {
                sink.set_volume(volume * effects * channels[slot].volume);
            }
            let music_volume = volumes_of(&channels);
            music_sinks
                .iter()
                .for_each(|sink| sink.set_volume(volume * music * music_volume));
        }
    }

    /// Current master volume (0..1).
    pub fn master_volume(&self) -> f32 {
        *self.master_volume.lock()
    }

    /// Set the music bus volume (0..1, clamped). Applies immediately in
    /// real mode by adjusting the music sinks; tracks started later play
    /// at `master * music bus * music channel` audibility (plan line 206,
    /// launcher options data layer).
    pub fn set_music_volume(&self, volume: f32) {
        let volume = clamp_unit(volume, 0.0, 1.0, 0.0);
        *self.music_volume.lock() = volume;
        if let Backend::Real { music_sinks, .. } = &self.backend {
            let channels = self.channels.lock();
            let master = *self.master_volume.lock();
            let music_volume = volumes_of(&channels);
            music_sinks
                .iter()
                .for_each(|sink| sink.set_volume(master * volume * music_volume));
        }
    }

    /// Current music bus volume (0..1).
    pub fn music_volume(&self) -> f32 {
        *self.music_volume.lock()
    }

    /// Set the effects bus volume (0..1, clamped) shared by every SFX and
    /// voice channel. Applies immediately in real mode by adjusting those
    /// sinks; later plays are `master * effects bus * channel * voice`
    /// (plan line 206, launcher options data layer).
    pub fn set_effects_volume(&self, volume: f32) {
        let volume = clamp_unit(volume, 0.0, 1.0, 0.0);
        *self.effects_volume.lock() = volume;
        if let Backend::Real { sinks, .. } = &self.backend {
            let channels = self.channels.lock();
            let master = *self.master_volume.lock();
            for (slot, sink) in sinks.iter().enumerate() {
                sink.set_volume(master * volume * channels[slot].volume);
            }
        }
    }

    /// Current effects bus volume (0..1).
    pub fn effects_volume(&self) -> f32 {
        *self.effects_volume.lock()
    }

    /// Audibility product `master * music bus * music channel` a track
    /// started now plays at. Shared by the headless recorder and the real
    /// music sinks so both modes report identical gains.
    fn music_effective_volume(&self) -> f32 {
        let channels = self.channels.lock();
        let master = *self.master_volume.lock();
        let music = *self.music_volume.lock();
        master * music * channels[Channel::Music.slot().expect("music slot always valid")].volume
    }

    /// Set a channel's volume (0..1, clamped). Applies to currently playing
    /// audio in real mode.
    pub fn set_channel_volume(&self, channel: Channel, volume: f32) -> Result<(), AudioError> {
        let Some(slot) = channel.slot() else {
            return Err(AudioError::InvalidChannel(channel));
        };
        let volume = clamp_unit(volume, 0.0, 1.0, 0.0);
        self.channels.lock()[slot].volume = volume;
        if let Backend::Real {
            sinks, music_sinks, ..
        } = &self.backend
        {
            let master = *self.master_volume.lock();
            let music = *self.music_volume.lock();
            let effects = *self.effects_volume.lock();
            if channel == Channel::Music {
                music_sinks
                    .iter()
                    .for_each(|sink| sink.set_volume(master * music * volume));
            } else {
                sinks[slot].set_volume(master * effects * volume);
            }
        }
        Ok(())
    }

    /// Current per-channel volume (0..1).
    pub fn channel_volume(&self, channel: Channel) -> Result<f32, AudioError> {
        let Some(slot) = channel.slot() else {
            return Err(AudioError::InvalidChannel(channel));
        };
        Ok(self.channels.lock()[slot].volume)
    }

    /// Set a channel's playback-rate multiplier (pitch). Applies
    /// immediately in real mode via `Sink::set_speed`; values are clamped
    /// to 0.01..16 and NaN maps back to 1.0.
    pub fn set_channel_pitch(&self, channel: Channel, pitch: f32) -> Result<(), AudioError> {
        let Some(slot) = channel.slot() else {
            return Err(AudioError::InvalidChannel(channel));
        };
        let pitch = clamp_unit(pitch, 0.01, 16.0, 1.0);
        self.channels.lock()[slot].pitch = pitch;
        if let Backend::Real {
            sinks, music_sinks, ..
        } = &self.backend
        {
            if channel == Channel::Music {
                music_sinks.iter().for_each(|sink| sink.set_speed(pitch));
            } else {
                sinks[slot].set_speed(pitch);
            }
        }
        Ok(())
    }

    /// Current per-channel pitch multiplier.
    pub fn channel_pitch(&self, channel: Channel) -> Result<f32, AudioError> {
        let Some(slot) = channel.slot() else {
            return Err(AudioError::InvalidChannel(channel));
        };
        Ok(self.channels.lock()[slot].pitch)
    }

    /// Start `source` as the new music track and fade the previous one out
    /// over `crossfade` while the new one fades in. Also safe to call when
    /// music never started (then there is nothing to fade out).
    pub fn crossfade_music<R>(
        &self,
        source: Decoder<R>,
        crossfade: Duration,
    ) -> Result<(), AudioError>
    where
        R: Read + Seek + Send + 'static,
    {
        match &self.backend {
            Backend::Headless { music, .. } => {
                let effective_volume = self.music_effective_volume();
                let mut log = music.lock();
                let faded_out_previous = log.active;
                log.entries.push(RecordedMusic {
                    crossfade: sanitize_crossfade(crossfade),
                    faded_out_previous,
                    stop_requested: false,
                    effective_volume,
                });
                log.active = true;
            }
            Backend::Real {
                music_sinks, music, ..
            } => {
                let volume = self.music_effective_volume();
                let mut state = music.lock();
                let slot = 1 - state.active;
                if let Some(previous) = state.states[state.active].as_ref() {
                    previous.begin_fade_out();
                }
                let shared = Arc::new(SharedFade::new(
                    crossfade,
                    source.sample_rate(),
                    source.channels(),
                ));
                let faded = FadeSource::new(source, shared.clone());
                music_sinks[slot].append(faded);
                music_sinks[slot].set_volume(volume);
                state.states[slot] = Some(shared);
                state.active = slot;
            }
        }
        Ok(())
    }

    /// Stop music immediately and drop all crossfade bookkeeping.
    pub fn stop_music(&self) {
        match &self.backend {
            Backend::Headless { music, .. } => {
                let effective_volume = self.music_effective_volume();
                let mut log = music.lock();
                if log.active {
                    log.entries.push(RecordedMusic {
                        crossfade: 0.0,
                        faded_out_previous: true,
                        stop_requested: true,
                        effective_volume,
                    });
                    log.active = false;
                }
            }
            Backend::Real {
                music_sinks, music, ..
            } => {
                let mut state = music.lock();
                for sink in music_sinks {
                    sink.stop();
                }
                state.states = [None, None];
                state.active = 0;
            }
        }
    }
}

/// Music-channel volume readout from a channel slice.
fn volumes_of(channels: &[ChannelState]) -> f32 {
    channels[Channel::slot_count() - 1].volume
}

/// Clamp a crossfade duration to a bounded number of seconds for recording.
fn sanitize_crossfade(crossfade: Duration) -> f32 {
    let seconds = crossfade.as_secs_f32();
    if seconds.is_finite() && seconds > 0.0 {
        seconds.min(3600.0)
    } else {
        0.0
    }
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::SharedBytes;
    use crate::test_support::wav_bytes;

    /// The launcher stores `AudioMixer::new()?` inside an
    /// `anyhow::Result`, which requires `AudioError` to be `Error +
    /// Send + Sync + 'static`. Assert the bounds at compile time so a
    /// future error variant cannot silently break the `?` conversion.
    #[test]
    fn audio_error_is_send_sync_std_error() {
        fn assert_send_sync_std_error<T: Send + Sync + std::error::Error>() {}
        assert_send_sync_std_error::<AudioError>();
    }

    /// Near-equality for magnitude < 2 floats.
    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() < 1.0e-4
    }

    fn decoder_for(freq: f32) -> Decoder<SharedBytes> {
        let data = wav_bytes(400, freq, 8000);
        Decoder::new(SharedBytes::new(Arc::new(data))).expect("test WAV must decode")
    }

    #[test]
    fn attenuate_endpoints_and_linear_middle() {
        assert_eq!(attenuate(0.0, 100.0), 1.0);
        assert_eq!(attenuate(100.0, 100.0), 0.0);
        assert!(near(attenuate(25.0, 100.0), 0.75));
        assert!(near(attenuate(75.0, 100.0), 0.25));
        assert_eq!(attenuate(150.0, 100.0), 0.0);
        assert_eq!(attenuate(-10.0, 100.0), 1.0);
        assert_eq!(attenuate(0.0, 0.0), 1.0);
        assert_eq!(attenuate(5.0, 0.0), 0.0);
        assert_eq!(attenuate(5.0, -1.0), 0.0);
        assert_eq!(attenuate(f32::INFINITY, 100.0), 0.0);
        assert_eq!(attenuate(10.0, f32::INFINITY), 1.0);
    }

    #[test]
    fn doppler_factor_is_sane() {
        // Stationary: exactly neutral.
        assert_eq!(doppler_factor(0.0, 0.0, 343.0), 1.0);
        // Approaching source: playback ratio above 1.
        let approaching = doppler_factor(0.0, 50.0, 343.0);
        assert!(approaching > 1.0 && approaching < 8.0, "{approaching}");
        // Receding source: ratio below 1.
        let receding = doppler_factor(0.0, -50.0, 343.0);
        assert!(receding > 1.0 / 8.0 && receding < 1.0, "{receding}");
        // At or beyond the speed of sound: saturates at the clamp.
        assert_eq!(doppler_factor(0.0, 343.0, 343.0), 8.0);
        assert_eq!(doppler_factor(0.0, 500.0, 343.0), 8.0);
        // Degenerate inputs collapse to neutral.
        assert_eq!(doppler_factor(0.0, 0.0, 0.0), 1.0);
        assert_eq!(doppler_factor(0.0, 0.0, -343.0), 1.0);
        assert_eq!(doppler_factor(f32::NAN, 0.0, 343.0), 1.0);
    }

    #[test]
    fn pan_gains_keep_constant_power() {
        let (left, right) = pan_gains(-1.0);
        assert!(near(left, 1.0) && near(right, 0.0), "{left} {right}");
        let (left, right) = pan_gains(1.0);
        assert!(near(left, 0.0) && near(right, 1.0), "{left} {right}");
        let (left, right) = pan_gains(0.5);
        assert!(near(left, 0.382_683_4) && near(right, 0.923_879_5));
        let (left, right) = pan_gains(0.0);
        assert!(near(left * left + right * right, 1.0));
        // NaN falls back to the centre.
        let (left, right) = pan_gains(f32::NAN);
        assert!(
            near(left, std::f32::consts::FRAC_1_SQRT_2)
                && near(right, std::f32::consts::FRAC_1_SQRT_2)
        );
        // Out-of-range pans clamp.
        let (left, _right) = pan_gains(-42.0);
        assert!(near(left, 1.0));
    }

    #[test]
    fn headless_play_records_request() {
        let mixer = AudioMixer::headless();
        assert!(mixer.is_headless());

        let handle = mixer
            .play(Channel::Sfx(0), decoder_for(220.0), 100, 0.8, 0.25)
            .expect("empty channel accepts");
        assert_eq!(handle, SfxHandle(1));
        let plays = mixer.played();
        assert_eq!(plays.len(), 1);
        let entry = &plays[0];
        assert_eq!(entry.channel, Channel::Sfx(0));
        assert_eq!(entry.priority, 100);
        assert!(near(entry.volume, 0.8));
        assert!(near(entry.pan, 0.25));
        assert_eq!(entry.handle, handle);
        assert_eq!(entry.evicted, None);
        assert!(near(entry.effective_volume, 0.8));
        assert!(near(entry.pitch, 1.0));
    }

    #[test]
    fn channel_busy_equal_or_lower_priority_is_dropped() {
        let mixer = AudioMixer::headless();
        let first = mixer
            .play(Channel::Sfx(1), decoder_for(440.0), 100, 1.0, 0.0)
            .expect("first accepts");
        // Equal priority: dropped.
        assert!(matches!(
            mixer.play(Channel::Sfx(1), decoder_for(440.0), 100, 1.0, 0.0),
            Err(AudioError::ChannelBusy(Channel::Sfx(1)))
        ));
        // Lower priority: dropped too.
        assert!(mixer
            .play(Channel::Sfx(1), decoder_for(440.0), 99, 1.0, 0.0)
            .is_err());
        // Nothing was recorded for the dropped requests.
        assert_eq!(mixer.played().len(), 1);
        // Strictly higher priority evicts.
        let second = mixer
            .play(Channel::Sfx(1), decoder_for(440.0), 101, 1.0, 0.0)
            .expect("higher priority evicts");
        let plays = mixer.played();
        assert_eq!(plays.len(), 2);
        assert_eq!(plays[1].evicted, Some(first));
        assert_eq!(plays[1].handle, second);
        // The evicted handle must stop as a no-op now.
        assert!(!mixer.stop(first));
    }

    #[test]
    fn master_channel_voice_volume_product() {
        let mixer = AudioMixer::headless();
        mixer.set_master_volume(0.5);
        assert!(near(mixer.master_volume(), 0.5));
        mixer
            .set_channel_volume(Channel::Sfx(3), 0.8)
            .expect("valid");
        assert!(near(
            mixer.channel_volume(Channel::Sfx(3)).expect("valid"),
            0.8
        ));
        mixer
            .play(Channel::Sfx(3), decoder_for(330.0), 10, 0.7, 0.0)
            .expect("plays");
        let entry = &mixer.played()[0];
        assert!(near(entry.effective_volume, 0.5 * 0.8 * 0.7));
    }

    // --- Music / effects volume buses (plan line 206, launcher options) ---

    #[test]
    fn music_and_effects_volume_knobs_default_full_and_clamp() {
        let mixer = AudioMixer::headless();
        // Defaults preserve the current behavior bit-for-bit.
        assert_eq!(mixer.music_volume(), 1.0);
        assert_eq!(mixer.effects_volume(), 1.0);
        mixer.set_music_volume(0.5);
        mixer.set_effects_volume(0.25);
        assert!(near(mixer.music_volume(), 0.5));
        assert!(near(mixer.effects_volume(), 0.25));
        // Out-of-range values clamp into 0..=1; NaN/infinite collapse to
        // silence exactly like every other mixer gain (clamp_unit).
        mixer.set_music_volume(2.0);
        assert_eq!(mixer.music_volume(), 1.0);
        mixer.set_effects_volume(-1.0);
        assert_eq!(mixer.effects_volume(), 0.0);
        mixer.set_music_volume(f32::NAN);
        assert_eq!(mixer.music_volume(), 0.0);
        mixer.set_effects_volume(f32::INFINITY);
        assert_eq!(mixer.effects_volume(), 0.0);
    }

    #[test]
    fn default_volume_knobs_keep_headless_recordings_identical() {
        // With both knobs at their 1.0 default the recorded audibility must
        // stay exactly `master * channel * voice` (bit-identical behavior).
        let mixer = AudioMixer::headless();
        mixer.set_master_volume(0.5);
        mixer
            .set_channel_volume(Channel::Sfx(3), 0.8)
            .expect("valid channel");
        mixer
            .play(Channel::Sfx(3), decoder_for(330.0), 10, 0.7, 0.0)
            .expect("plays");
        let entry = &mixer.played()[0];
        assert_eq!(entry.effective_volume, 0.5 * 0.8 * 0.7);
        // The music bus records the same product.
        mixer
            .set_channel_volume(Channel::Music, 0.8)
            .expect("valid channel");
        mixer
            .crossfade_music(decoder_for(440.0), Duration::from_secs_f32(1.0))
            .expect("headless ok");
        assert_eq!(mixer.music_log()[0].effective_volume, 0.5 * 0.8);
    }

    #[test]
    fn effects_and_music_volumes_scale_headless_recordings() {
        let mixer = AudioMixer::headless();
        mixer.set_effects_volume(0.5);
        mixer.set_music_volume(0.25);
        mixer
            .play(Channel::Sfx(0), decoder_for(220.0), 10, 0.8, 0.0)
            .expect("plays");
        let entry = &mixer.played()[0];
        // master(1) * effects(0.5) * channel(1) * voice(0.8)
        assert!(near(entry.effective_volume, 0.5 * 0.8));
        mixer
            .crossfade_music(decoder_for(440.0), Duration::from_secs_f32(1.0))
            .expect("headless ok");
        // master(1) * music(0.25) * channel(1)
        assert!(near(mixer.music_log()[0].effective_volume, 0.25));
    }

    #[test]
    fn volume_buses_compose_with_master_and_channel() {
        let mixer = AudioMixer::headless();
        mixer.set_master_volume(0.5);
        mixer.set_effects_volume(0.5);
        mixer
            .set_channel_volume(Channel::Sfx(1), 0.5)
            .expect("valid channel");
        mixer
            .play(Channel::Sfx(1), decoder_for(440.0), 10, 0.5, 0.0)
            .expect("plays");
        // 0.5^4 is exact in binary floating point.
        assert_eq!(mixer.played()[0].effective_volume, 0.0625);
    }

    #[test]
    fn stop_by_handle_frees_channel() {
        let mixer = AudioMixer::headless();
        let handle = mixer
            .play(Channel::Voice, decoder_for(110.0), 50, 1.0, 0.0)
            .expect("plays");
        assert!(mixer.stop(handle));
        // Stopping twice is a no-op.
        assert!(!mixer.stop(handle));
        // The channel is free again for an equal priority.
        assert!(mixer
            .play(Channel::Voice, decoder_for(110.0), 50, 1.0, 0.0)
            .is_ok());
    }

    #[test]
    fn stop_channel_cuts_current_voice() {
        let mixer = AudioMixer::headless();
        mixer
            .play(Channel::Sfx(2), decoder_for(110.0), 10, 1.0, 0.0)
            .expect("plays");
        assert!(mixer.stop_channel(Channel::Sfx(2)));
        assert!(mixer
            .play(Channel::Sfx(2), decoder_for(110.0), 10, 1.0, 0.0)
            .is_ok());
    }

    #[test]
    fn invalid_sfx_channel_is_rejected() {
        let mixer = AudioMixer::headless();
        let bad = Channel::Sfx(SFX_CHANNEL_COUNT);
        assert!(matches!(
            mixer.play(bad, decoder_for(110.0), 1, 1.0, 0.0),
            Err(AudioError::InvalidChannel(_))
        ));
        assert_eq!(bad.slot(), None, "slot_count {}", Channel::slot_count());
        assert_eq!(Channel::Voice.slot(), Some(SFX_CHANNEL_COUNT as usize));
        // The music channel is not a priority SFX channel either.
        assert!(matches!(
            mixer.play(Channel::Music, decoder_for(110.0), 1, 1.0, 0.0),
            Err(AudioError::InvalidChannel(Channel::Music))
        ));
    }

    #[test]
    fn channel_volume_and_pitch_setters_clamp() {
        let mixer = AudioMixer::headless();
        mixer.set_channel_volume(Channel::Sfx(0), 2.0).unwrap();
        assert!(near(mixer.channel_volume(Channel::Sfx(0)).unwrap(), 1.0));
        mixer.set_channel_pitch(Channel::Sfx(0), 0.0).unwrap();
        assert!(near(mixer.channel_pitch(Channel::Sfx(0)).unwrap(), 0.01));
        mixer.set_channel_pitch(Channel::Sfx(0), f32::NAN).unwrap();
        assert!(near(mixer.channel_pitch(Channel::Sfx(0)).unwrap(), 1.0));
    }

    #[test]
    fn pause_and_resume_track_channel_state() {
        let mixer = AudioMixer::headless();
        assert!(!mixer.is_paused(Channel::Music));
        mixer.pause(Channel::Music).expect("valid");
        assert!(mixer.is_paused(Channel::Music));
        mixer.resume(Channel::Music).expect("valid");
        assert!(!mixer.is_paused(Channel::Music));
        assert!(mixer.pause(Channel::Sfx(9)).is_err());
    }

    #[test]
    fn stop_all_clears_every_voice() {
        let mixer = AudioMixer::headless();
        for channel in Channel::sfx_all() {
            mixer
                .play(channel, decoder_for(440.0), 200, 1.0, 0.0)
                .expect("own channel");
        }
        mixer
            .play(Channel::Voice, decoder_for(220.0), 200, 1.0, 0.0)
            .expect("own channel");
        mixer.stop_all();
        // All channels must accept equal-priority sounds again.
        assert!(mixer
            .play(Channel::Sfx(0), decoder_for(110.0), 200, 1.0, 0.0)
            .is_ok());
    }

    #[test]
    fn shared_fade_ramp_sample_count_math() {
        let shared = SharedFade::new(Duration::from_secs_f32(1.0), 44_100, 2);
        assert_eq!(shared.ramp_samples(), 88_200);
        let shared = SharedFade::new(Duration::from_secs_f32(0.5), 1_000, 1);
        assert_eq!(shared.ramp_samples(), 500);
        // Zero-durations degenerate to instant full gain.
        let shared = SharedFade::new(Duration::from_secs_f32(0.0), 44_100, 2);
        assert_eq!(shared.ramp_samples(), 0);
        assert_eq!(shared.envelope_at(0), 1.0);
    }

    #[test]
    fn fade_in_envelope_starts_at_zero_and_ramps_to_one() {
        let shared = Arc::new(SharedFade::new(Duration::from_secs_f32(0.5), 1_000, 1));
        let mut source = FadeSource::new(TestSource::new(1_500, 1_000, 1), shared);
        let mut values = Vec::new();
        for _ in 0..600 {
            values.push(source.next().expect("source long enough"));
        }
        assert_eq!(values[0], 0.0, "starts silent");
        assert!(near(values[250], 0.5));
        assert!(near(values[500], 1.0), "reaches full gain");
        assert!(near(values[599], 1.0), "stays at full gain");
        // Fade-in never terminates the source by itself.
        assert!(source.next().is_some());
    }

    #[test]
    fn static_fade_out_envelope_starts_at_one_and_ends_silent() {
        let source = FadeSource::fade_out(
            TestSource::new(1_500, 1_000, 1),
            Duration::from_secs_f32(0.5),
        );
        let mut values = Vec::new();
        for value in source {
            values.push(value);
        }
        assert_eq!(values.len(), 501);
        assert!(near(values[0], 1.0), "starts at full gain");
        assert!(near(values[250], 0.5));
        assert_eq!(values[500], 0.0, "ends silent");
    }

    #[test]
    fn runtime_fade_out_from_any_elapsed() {
        let shared = Arc::new(SharedFade::new(Duration::from_secs_f32(0.5), 1_000, 1));
        let mut source = FadeSource::new(TestSource::new(4_000, 1_000, 1), shared.clone());
        // Consume the whole fade-in ramp first.
        for _ in 0..600 {
            assert!(source.next().is_some());
        }
        shared.begin_fade_out();
        // Pure envelope assertions on the shared state.
        let start = shared.fade_out_start.load(Ordering::Relaxed);
        assert_eq!(start, 600);
        assert!(near(shared.envelope_at(600), 1.0));
        assert!(near(shared.envelope_at(600 + 250), 0.5));
        assert_eq!(shared.envelope_at(600 + 500), 0.0);
        // Iterating to exhaustion yields exactly one extra silence sample.
        let mut consumed = 600;
        while source.next().is_some() {
            consumed += 1;
        }
        assert_eq!(consumed, 600 + 500 + 1);
        // begin_fade_out is idempotent: a second call keeps the ramp start.
        shared.begin_fade_out();
        assert_eq!(shared.fade_out_start.load(Ordering::Relaxed), 600);
    }

    #[test]
    fn crossfade_music_records_in_headless() {
        let mixer = AudioMixer::headless();
        mixer
            .crossfade_music(decoder_for(440.0), Duration::from_secs_f32(2.0))
            .expect("headless ok");
        let log = mixer.music_log();
        assert_eq!(log.len(), 1);
        assert!(near(log[0].crossfade, 2.0));
        assert!(!log[0].faded_out_previous);
        // A second switch fades the previous track out.
        mixer
            .crossfade_music(decoder_for(220.0), Duration::from_secs_f32(1.0))
            .expect("headless ok");
        let log = mixer.music_log();
        assert_eq!(log.len(), 2);
        assert!(log[1].faded_out_previous);
        // stop_music records a tear-down entry.
        mixer.stop_music();
        let log = mixer.music_log();
        assert!(log[2].stop_requested);
        // And the next switch starts fresh again.
        mixer
            .crossfade_music(decoder_for(110.0), Duration::from_secs_f32(1.0))
            .expect("headless ok");
        assert!(!mixer.music_log()[3].faded_out_previous);
    }

    #[test]
    fn headless_zero_crossfade_is_recorded_as_zero() {
        let mixer = AudioMixer::headless();
        mixer
            .crossfade_music(decoder_for(110.0), Duration::from_secs_f32(0.0))
            .expect("headless ok");
        assert_eq!(mixer.music_log()[0].crossfade, 0.0);
        // Oversized values are capped for the record.
        mixer
            .crossfade_music(decoder_for(110.0), Duration::from_secs(24 * 3600))
            .expect("headless ok");
        assert!(near(mixer.music_log()[1].crossfade, 3600.0));
    }

    /// Minimal deterministic f32 source for envelope tests.
    struct TestSource {
        remaining: usize,
        frame_len: usize,
        channels: u16,
    }

    impl TestSource {
        fn new(remaining: usize, frame_len: usize, channels: u16) -> Self {
            Self {
                remaining,
                frame_len,
                channels,
            }
        }
    }

    impl Iterator for TestSource {
        type Item = f32;

        fn next(&mut self) -> Option<f32> {
            if self.remaining == 0 {
                return None;
            }
            self.remaining -= 1;
            Some(1.0)
        }
    }

    impl Source for TestSource {
        fn current_frame_len(&self) -> Option<usize> {
            Some(self.frame_len.min(self.remaining))
        }

        fn channels(&self) -> u16 {
            self.channels
        }

        fn sample_rate(&self) -> u32 {
            1_000
        }

        fn total_duration(&self) -> Option<Duration> {
            None
        }
    }
}
