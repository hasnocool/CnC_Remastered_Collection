## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### Unreleased
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- `src/briefing.rs` static briefing art: `BriefingFaction` (Gdi/Nod/Allies/Soviet, built via `from_td_player`/`from_ra_player`), `generate_briefing_emblem` (faction shield + sigil + mission plaque) and `generate_briefing_portrait` (faction bust with headgear), both at `BRIEFING_ART_SIZE = 48`, palette-snapped through `Palette::find_closest` with `PlayerRemap` accents and no I/O
- 8 `briefing.rs` unit tests plus `tests/procgen.rs` briefing determinism/palette-boundedness coverage (42 crate tests); original VQA briefing movies stay out of reach (licensed decoders are stubs)
