//! In-game audio director: routes engine observation events to the mixer.
//!
//! The engines publish observation-only events through the shared
//! [`cnc_events::EventBus`] (see `cnc_tiberian_dawn::events` and
//! `cnc_red_alert::events`). The launcher's EventSubscriber drains that bus
//! into HUD state; the director is the second consumer of the same drained
//! batch and turns gameplay events into procedural SFX through the
//! [`AudioMixer`].
//!
//! Sound design follows Westwood channel semantics (see `original/` AUDIO.CPP
//! references in `cnc-sfx::effects`): combat noises ride the SFX channels,
//! UI confirmations ride channel 0, and EVA speech rides the Voice channel
//! through the existing `EvaPlayer` wiring.
//!
//! The director is presentation-only: it never writes back into simulation
//! state and never gates on audio success. A mixer error (busy channel,
//! missing device) is silently dropped — plan line 421 "audio device absent →
//! silent fallback, sim unaffected".

use std::collections::HashSet;

use cnc_audio::{AudioMixer, Channel, MusicLayer, MusicPlayer, MusicTrack, SoundEffect};
use cnc_procgen::prng::Seed;
use cnc_sfx::effects;

use crate::events_subscriber::ReceivedEvent;

/// Procedural effect render length in samples (~1.4 s at the default rate).
const EFFECT_LEN_SAMPLES: usize = 30_000;

/// One canonical sound the director knows how to play.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSound {
    /// Weapon fire / projectile launch (gunfire crack).
    Fire,
    /// Projectile impact / warhead detonation.
    Explosion,
    /// Unit or structure selected (UI beep).
    Select,
    /// Move order acknowledged (UI blip).
    Move,
    /// Production completed: unit delivered or structure ready.
    BuildComplete,
}

impl GameSound {
    /// All sounds, in a stable order for tests.
    pub const ALL: [GameSound; 5] = [
        GameSound::Fire,
        GameSound::Explosion,
        GameSound::Select,
        GameSound::Move,
        GameSound::BuildComplete,
    ];

    /// Render the deterministic procedural samples for this sound.
    fn samples(self, seed: Seed) -> Vec<f32> {
        match self {
            GameSound::Fire => effects::gunfire(seed, EFFECT_LEN_SAMPLES / 4),
            GameSound::Explosion => effects::explosion(seed, EFFECT_LEN_SAMPLES),
            GameSound::Select => effects::select_beep(seed, EFFECT_LEN_SAMPLES / 8),
            GameSound::Move => effects::ui_blip(seed, EFFECT_LEN_SAMPLES / 12),
            GameSound::BuildComplete => effects::ui_affirm(seed, EFFECT_LEN_SAMPLES / 10),
        }
    }

    /// Mixer channel this sound plays on.
    fn channel(self) -> Channel {
        match self {
            // Combat noises get dedicated slots; channel 0 stays free for
            // UI pips so they can never evict a cannon report.
            GameSound::Fire => Channel::Sfx(1),
            GameSound::Explosion => Channel::Sfx(2),
            GameSound::Select | GameSound::Move | GameSound::BuildComplete => Channel::Sfx(0),
        }
    }

    /// Westwood priority 0-255: combat noises outrank UI pips so a busy
    /// UI channel cannot evict an in-flight cannon report.
    fn priority(self) -> u8 {
        match self {
            GameSound::Explosion => 200,
            GameSound::Fire => 180,
            GameSound::BuildComplete => 140,
            GameSound::Select => 128,
            GameSound::Move => 110,
        }
    }

    /// Per-voice gain.
    fn volume(self) -> f32 {
        match self {
            GameSound::Explosion => 0.9,
            GameSound::Fire => 0.7,
            _ => 0.6,
        }
    }
}

/// Maps engine events to sounds and drives the mixer + music layers.
pub struct AudioDirector {
    /// Pre-rendered effect bytes, keyed per sound. Rendered once at
    /// construction (deterministic seeds) so play requests are cheap.
    bank: Vec<(GameSound, SoundEffect)>,
    /// Distinct sounds heard since the last reset (acceptance logging).
    heard: HashSet<GameSound>,
    /// Total accepted play requests.
    plays: u64,
}

impl AudioDirector {
    /// Build a director with the full procedural bank rendered up front.
    pub fn new() -> Self {
        let bank = GameSound::ALL
            .iter()
            .map(|&sound| {
                let seed = Seed::new(0x600D_5EED + sound as u64);
                let bytes =
                    effects::to_wav_bytes(&sound.samples(seed), effects::DEFAULT_SAMPLE_RATE);
                (sound, SoundEffect::from_bytes(bytes))
            })
            .collect();
        Self {
            bank,
            heard: HashSet::new(),
            plays: 0,
        }
    }

    /// Distinct sounds heard since construction (acceptance criterion:
    /// "scripted combat emits ≥4 distinct sfx through mixer").
    pub fn distinct_sounds(&self) -> usize {
        self.heard.len()
    }

    /// Total play requests accepted by the mixer.
    pub fn plays(&self) -> u64 {
        self.plays
    }

    /// True when every known sound has been heard at least once.
    pub fn all_sounds_heard(&self) -> bool {
        self.heard.len() == GameSound::ALL.len()
    }

    /// Snapshot of the distinct sounds heard (debug/test reporting).
    pub fn heard_snapshot(&self) -> Vec<GameSound> {
        let mut heard: Vec<GameSound> = self.heard.iter().copied().collect();
        heard.sort_by_key(|sound| GameSound::ALL.iter().position(|s| s == sound).unwrap_or(0));
        heard
    }

    /// Play one sound immediately. Errors are swallowed (busy channel =
    /// Westwood drop semantics; missing device = silent fallback).
    pub fn play(&mut self, mixer: &AudioMixer, sound: GameSound) {
        let Some((_, effect)) = self.bank.iter().find(|(known, _)| *known == sound) else {
            return;
        };
        let result = effect.play_on(
            mixer,
            sound.channel(),
            sound.priority(),
            sound.volume(),
            0.0,
        );
        if result.is_ok() {
            self.heard.insert(sound);
            self.plays += 1;
        }
    }

    /// Route one drained engine event to a sound, if it maps to one.
    ///
    /// Event names come from both engines' `names` modules:
    /// * production completion (`*_unit_produced`, `*_structure_produced`,
    ///   `td_structure_ready`) → BuildComplete
    /// * destruction (`*_unit_lost`, `*_structure_lost`) → Explosion
    /// * superweapon fire (`*_superweapon_fired`) → Explosion (the bigger
    ///   boom family in the original; the procedural bank has one blast)
    pub fn route(&mut self, mixer: &AudioMixer, event: &ReceivedEvent) {
        let sound = match event.name.as_str() {
            "td_unit_produced"
            | "ra_unit_produced"
            | "td_structure_ready"
            | "ra_structure_produced" => GameSound::BuildComplete,
            "td_weapon_fired" => GameSound::Fire,
            "td_projectile_impact" => GameSound::Explosion,
            "td_units_selected" => GameSound::Select,
            "td_move_ordered" => GameSound::Move,
            "td_unit_lost" | "ra_unit_lost" | "td_structure_lost" | "ra_structure_lost" => {
                GameSound::Explosion
            }
            "td_superweapon_fired" | "ra_superweapon_fired" => GameSound::Explosion,
            _ => return,
        };
        self.play(mixer, sound);
    }

    /// Route a batch of drained events in publish order.
    pub fn route_batch(&mut self, mixer: &AudioMixer, events: &[ReceivedEvent]) {
        for event in events {
            self.route(mixer, event);
        }
    }
}

impl Default for AudioDirector {
    fn default() -> Self {
        Self::new()
    }
}

/// Theme queue: scan/play/next tied to house state, on the MusicPlayer's
/// explore/combat layers (THEME.CPP port; plan line 417).
///
/// The original `ThemeClass` scans theme files, plays a score by index and
/// advances to the next available one. Here themes are procedural tracks
/// rendered at startup (no redistribution-encumbered files), split into an
/// explore layer and a combat layer; `Scan` becomes "queue what exists",
/// `Play_Song` becomes "start the next track of the current layer" and
/// `Next` becomes "advance within the layer". `On_Base_Attack` escalation
/// from THEME.CPP (`AI`-driven combat theme switching) becomes the
/// intensity bump that retargets `play_next` at the combat layer.
pub struct ThemeQueue {
    player: MusicPlayer,
    /// Theme names in queue order, for HUD/debug display.
    explore_names: Vec<&'static str>,
    combat_names: Vec<&'static str>,
    /// Theme currently playing (by name).
    current: Option<&'static str>,
}

/// Procedural theme track names, mirroring the original theme roster's
/// explore/combat split (THEME.CPP `_themes` table).
const EXPLORE_THEMES: &[&str] = &["warfare", "creeping", "valkyrie", "hell"];
const COMBAT_THEMES: &[&str] = &["march", "routine", "bog", "drill"];

impl ThemeQueue {
    /// Scan + queue: render every theme track once and split it across the
    /// two layers. Mirrors `ThemeClass::Scan` marking available themes.
    pub fn new() -> Self {
        let mut player = MusicPlayer::new();
        let mut explore_names = Vec::new();
        let mut combat_names = Vec::new();
        for (layer, names) in [(0u64, EXPLORE_THEMES), (1u64, COMBAT_THEMES)] {
            for (index, name) in names.iter().enumerate() {
                let seed = Seed::new(0x7BEA + layer * 1000 + index as u64);
                // A theme is a long engine-loop-derived phrase, ~4 s.
                let samples = effects::engine_loop(seed, EFFECT_LEN_SAMPLES * 3);
                let bytes = effects::to_wav_bytes(&samples, effects::DEFAULT_SAMPLE_RATE);
                let track = MusicTrack::from_bytes(bytes);
                if layer == 0 {
                    player.queue_explore(track);
                    explore_names.push(*name);
                } else {
                    player.queue_combat(track);
                    combat_names.push(*name);
                }
            }
        }
        Self {
            player,
            explore_names,
            combat_names,
            current: None,
        }
    }

    /// The underlying player (intensity/layer introspection for tests).
    pub fn player(&self) -> &MusicPlayer {
        &self.player
    }

    /// Mutable underlying player (intensity changes from house state).
    pub fn player_mut(&mut self) -> &mut MusicPlayer {
        &mut self.player
    }

    /// `ThemeClass::Play_Song`: start the next track of the current layer.
    /// Returns the theme name that started, if any.
    pub fn play_song(&mut self, mixer: &AudioMixer) -> Option<&'static str> {
        if self.player.play_next(mixer).ok().unwrap_or(false) {
            let layer = self.player.layer();
            // Each play consumes one track from the front of the active
            // layer's queue; names advance in the same order.
            let names = match layer {
                MusicLayer::Explore => &self.explore_names,
                MusicLayer::Combat => &self.combat_names,
            };
            let consumed = (names.len() as i64
                - match layer {
                    MusicLayer::Explore => self.player.queue_lengths().1,
                    MusicLayer::Combat => self.player.queue_lengths().2,
                } as i64)
                .clamp(1, names.len() as i64) as usize;
            self.current = names.get(consumed - 1).copied();
            self.current
        } else {
            None
        }
    }

    /// `ThemeClass::Next`: advance to the next theme of the active layer.
    /// Same behaviour as `play_song` here because layers are queues.
    pub fn next(&mut self, mixer: &AudioMixer) -> Option<&'static str> {
        self.play_song(mixer)
    }

    /// Base-attack switch: raise intensity so the next track pulls from the
    /// combat layer. Mirrors the original's combat-state theme escalation.
    /// Returns `true` when the layer changed.
    pub fn on_base_attack(&mut self) -> bool {
        let before = self.player.layer();
        self.player.set_intensity(1.0);
        self.player.layer() != before
    }

    /// De-escalate back to the explore layer (combat resolved).
    /// Returns `true` when the layer changed.
    pub fn on_combat_resolved(&mut self) -> bool {
        let before = self.player.layer();
        self.player.set_intensity(0.0);
        self.player.layer() != before
    }

    /// Names queued per layer (explore, combat).
    pub fn theme_names(&self) -> (&[&'static str], &[&'static str]) {
        (&self.explore_names, &self.combat_names)
    }

    /// The theme most recently started.
    pub fn current(&self) -> Option<&'static str> {
        self.current
    }
}

impl Default for ThemeQueue {
    fn default() -> Self {
        Self::new()
    }
}
