# RUST/cnc-sfx/

## Responsibility
Deterministic procedural sound-effect synthesis for the C&C Rust port. Replaces non-redistributable Westwood .AUD/.VOC files with pure-Rust generated WAV. Every effect is a pure function of its seed - identical seeds produce byte-identical output for cacheable, committable assets.

## Design Patterns
- Pure Functional Synthesis: All effects are fn(seed: u64) -> Vec<i16> - no global state, no RNG (deterministic hash-derived noise)
- Waveform Primitives: waves module - sine, square, saw, noise, ADSR envelope, decay envelope, ramp
- Effect Library: effects module - explosion, gunfire, tesla_zap, engine_loop, construction_loop, select_beep
- WAV Output: write_wav_16(), to_wav_bytes() produce mono 16-bit PCM WAV (44.1kHz default)
- Seed Domain Separation: Effect name hashed into seed for independent variation

## Data & Control Flow
1. Seed Input: Caller provides u64 seed (e.g., from asset manifest hash)
2. Waveform Gen: waves::sine(), square(), saw(), noise() produce sample iterators
3. Envelope: waves::adsr(), decay_env() shape amplitude over time
4. Effect Composition: effects::explosion() combines noise bursts, filter sweeps, decay
5. WAV Encode: effects::to_wav_bytes(samples) / write_wav_16(path, samples)
6. Consume: cnc-audio::AudioSource::from_procedural(bytes) for runtime playback

## Public API
- waves: sine(freq, duration, sample_rate), square(), saw(), noise(), adsr(), decay_env(), ramp()
- effects: explosion(seed), gunfire(seed), tesla_zap(seed), engine_loop(seed), construction_loop(seed), select_beep(seed), to_wav_bytes(samples), write_wav_16(path, samples), DEFAULT_SAMPLE_RATE (44100)

## Integration Points
- Dependencies: cnc-procgen (shared synthesis), hound (WAV writing)
- Consumed by: cnc-audio (procedural AudioSource), cnc-music (shared waves), asset pipeline (baked WAV assets)
- No runtime dependency - build-time bake, runtime plays pre-generated WAV

## Key Files
- src/lib.rs - Module declarations, re-exports
- src/waves.rs - Waveform primitives, envelopes, noise
- src/effects.rs - Effect compositions, WAV writing

