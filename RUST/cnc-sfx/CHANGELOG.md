## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- 13 procedural effect voices in `effects.rs`: weapons (`cannon_double_shot`, `autocannon_stutter`, `rocket_whoosh`, `artillery_boom`), explosions (`ore_refinery_boom`, `building_collapse`), UI bleeps (`ui_bleep_high`, `ui_bleep_low`, `ui_blip`, `ui_affirm`, `ui_deny_soft`) and superweapons (`chronosphere_shift`, `iron_curtain_hum`), each traced to `REDALERT/AUDIO.CPP:139-203`
- 6 new tests covering the weapon/explosion/UI-bleep/superweapon variants (distinctness, boundedness, per-seed determinism, decay tails, seamless loops), bringing the crate to 17 tests; the EVA voice bus remains out of scope
