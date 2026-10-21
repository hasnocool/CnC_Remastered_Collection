# RUST/cnc-audio/

## Responsibility
Presentation-only audio engine for the C&C Rust port. Implements Westwood-style channel model (fixed single-voice channels, priority preemption) with modern mixer features (volume buses, crossfade, positional attenuation, panning). Zero game-state mutation - pure output side.

## Design Patterns
- Channel Model: Fixed channels (SFX 0-8, Voice, Music) with u8 priority (0-255), Westwood-style preemption
- Volume Buses: Master x Music Bus x Effects Bus x Channel x Voice layered volume control
- Positional Audio: Distance attenuation (linear), constant-power stereo panning, optional Doppler factor utility
- Mixer as Singleton: AudioMixer manages all channels, buses, crossfade state
- Source Abstraction: AudioSource trait for decoded PCM (WAV, OGG, procedural from cnc-sfx)

## Data & Control Flow
1. Init: AudioMixer::new() creates channel array, buses at 1.0 default
2. Load: AudioSource::from_wav / from_ogg / from_procedural to AudioClip (owned PCM buffer)
3. Play: mixer.play(channel, clip, priority, volume, pan, distance) to preemption check to channel assignment
4. Update: mixer.update(dt) advances all playing clips, applies volume buses, mixes to output buffer
5. Music: mixer.crossfade_music(new_clip, duration) handles music transitions
6. Shutdown: mixer.shutdown() stops all, releases resources

## Public API
- Channel: Sfx(u8), Voice, Music enum variants
- AudioMixer: new(), play(), stop(), stop_channel(), update(), crossfade_music(), music_volume(), effects_volume(), master_volume()
- AudioSource: from_wav(), from_ogg(), from_procedural(), samples(), sample_rate(), channels()
- mixer module: attenuate(distance), pan_gains(pan), doppler_factor(vel)
- Error: AudioError::ChannelBusy, AudioError::NoSuchChannel

## Integration Points
- Dependencies: cnc-core, rodio, parking_lot, once_cell, anyhow, thiserror, serde
- Consumed by: cnc-tiberian-dawn, cnc-red-alert (game audio), cnc-classic (classic mode audio), cnc-console (event audio)
- Procedural SFX: Consumes cnc-sfx generated WAV bytes via AudioSource::from_procedural
- No simulation dependency - purely presentational, safe for deterministic lockstep

## Key Files
- src/lib.rs - Module declarations, channel model, mixer API
- src/mixer.rs - AudioMixer implementation, channel management, bus mixing
- src/source.rs - AudioSource trait, WAV/OGG/procedural loaders
- src/channel.rs - Channel enum, priority logic
- src/error.rs - AudioError variants

