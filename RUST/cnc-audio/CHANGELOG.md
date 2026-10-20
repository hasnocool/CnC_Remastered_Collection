## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### Unreleased
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- `music_volume` and `effects_volume` control buses on `AudioMixer` (set/get, clamped 0..=1, default 1.0 preserves existing playback)
- `RecordedMusic::effective_volume` plus effective-volume capture on `crossfade_music`/`stop_music` for headless tests
