# RUST/cnc-audio/tests/

## Responsibility
Integration tests for cnc-audio CSF (sound effect) parsing and speech synthesis pipeline. Validates audio format handling and TTS integration.

## Design
- **csf.rs** (6,774 lines): Tests for Westwood CSF (Command & Conquer Sound Format) file parsing — header validation, chunk reading, sample rate/format verification, sample data integrity.
- **speech_pipeline.rs** (13,647 lines): Tests for EVA/voice-over speech synthesis pipeline — text-to-speech generation, phoneme alignment, audio encoding, channel assignment (Voice channel), priority handling.
- **fixtures/**: Test audio files (CSF, VOC, WAV) for parser validation.

## Flow
1. Tests load fixture audio files or synthesize test speech
2. Parse via cnc-audio source/decoder modules
3. Validate output samples, format metadata, channel assignment
4. Assert deterministic output for identical inputs

## Integration
- Exercises cnc-audio/src/source.rs (CSF/VOC/WAV loaders)
- Exercises cnc-audio/src/mixer.rs (Voice channel handling)
- Exercises procedural speech via cnc-sfx integration
- Validates audio determinism for lockstep compatibility

## Key Files
- csf.rs - CSF format parsing tests
- speech_pipeline.rs - Speech synthesis pipeline tests
- fixtures/ - Test audio fixtures

