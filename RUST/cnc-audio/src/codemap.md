# RUST/cnc-audio/src/

## Responsibility

Three modules declared in `lib.rs`: `mixer` (channel allocation + playback), `music` (track queue + explore/combat layering), `sfx` (sound-effect handles), plus an in-crate `test_support` module emitting synthetic WAV bytes.

## Design

- `mixer.rs` (188 L) — `AudioMixer`, `AudioMixer::new()` (hardware) and `::headless()` (records every play request; same channel/priority/volume logic); `Channel`/`ChannelId` slots; `AudioError::ChannelBusy`; `FadeSource`/`SharedFade` for crossfade envelopes; `RecordedMusic`/`RecordedPlay` test-visible event log; free functions `attenuate`, `pan_gains`, `doppler_factor`.
- `music.rs` (27 L) — `MusicPlayer` with `MusicTrack` queue and `MusicLayer` explore/combat pair; switch via `Player::set_intensity` (intensity primitives live here, crossfades in mixer).
- `sfx.rs` (17 L) — `SoundEffect::from_bytes(...)` builder plus `play_on(&mixer, Channel, priority: u8, volume, pan)` and paused/stopped variants through `SfxHandle`.
- `lib.rs` — long crate-level contract doc (channel model, headless mode, music semantics, doctest example) + re-exports; `test_support::wav_bytes(samples, freq, rate)` synthesizes valid 16-bit mono PCM WAV bytes.

## Flow

Caller builds `SoundEffect` from already-decoded/encoded bytes → `play_on` → mixer: busy check (priority eviction rules) → rodio decode/source → volume = `master * channel * voice`, pan via `pan_gains` → output. Music intensity events drive `MusicPlayer` track switches with `FadeSource` envelopes on alternating sinks.

## Integration

- Owns rodio only; gains re-exports are the stable public API (`AudioMixer`, `Channel`, `AudioError`, `MusicPlayer`, `SoundEffect`…).
- No cross-module state beyond `MusicPlayer` owning two sinks provided by `mixer`; engines call it per event, never concurrently — no game-state feedback.
