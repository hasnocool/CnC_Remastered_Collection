# RUST/cnc-audio/

## Responsibility

Audio presentation engine for the Rust port: a fixed single-voice channel model (8 numbered SFX channels + Voice + Music) with Westwood-style priority eviction, positional attenuation/pan math, PCM/SoundEffect playback via rodio, and an intensity-driven music player with explore/combat crossfading. Presentation-only — never writes back into game state (lib.rs docs).

## Design

- Channel model copied from the DOS originals: `Channel::Sfx(u8)` (0..8, `SFX_CHANNEL_COUNT`), `Channel::Voice`, `Channel::Music`; priority is 0-255 u8; `mixer::AudioMixer` rejects a lower/equal-priority request on a busy channel (`ChannelBusy`) and evicts on strict-higher.
- `mixer::AudioError` (thiserror) as the error type; `FadeSource`/`SharedFade` envelopes for crossfades; `RecordedMusic`/`RecordedPlay` let `headless()` mode record play events instead of using hardware — all tests run headless (CI has no device).
- Positional SFX without a listener model: `attenuate(distance)` → linear volume, `pan_gains(pan)` → constant-power stereo pair, `doppler_factor()` as an optional caller-applied pitch utility.
- `music::MusicPlayer` queues `MusicTrack`s and owns a `MusicLayer` explore/combat pair, switched by `set_intensity` with crossfades on alternating music sinks.

## Flow

Engine/launcher calls `SoundEffect::from_bytes(...).play_on(&mixer, Channel::Sfx(3), priority=200, volume=0.8, pan=-0.25)`; mixer decodes (rodio source), applies `master * channel * voice` volume and pan gains, plays to the default output device (or records in headless mode). Music: intensity changes trigger long crossfades over the two music sinks.

## Integration

- Depends on: `cnc-core` + rodio only; typed errors surface to callers via `AudioError`/`Result`.
- Consumed by `cnc-tiberian-dawn` and `cnc-red-alert` (playback of EVA/voice/SFX/music events in their game loops) and `cnc-launcher` (menu/game audio wiring).
- Tests: all unit tests run headless (record-only mixer) using `test_support::wav_bytes`; no external `tests/` directory exists.
