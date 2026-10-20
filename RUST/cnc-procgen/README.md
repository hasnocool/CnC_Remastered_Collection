# cnc-procgen

This crate follows strict semantic versioning (version.workspace = true, edition = 2021, SemVer 2.0.0).

## Scope and limits

cnc-procgen is the built-in, free asset path: it deterministically generates
terrain heightmaps with biome band textures, noise/Tiberium textures, and
parametric placeholder unit sprites — no original game files or downloads
needed. The terrain and texture generators are the working core; the sprite
lane is a foundation-grade set of 32px placeholders rather than final art,
with deeper per-class sprite profiles left as future work.

## Documentation
- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## CI / Git
- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
