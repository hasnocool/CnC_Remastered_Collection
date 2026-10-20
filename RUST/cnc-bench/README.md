# cnc-bench

This crate follows strict semantic versioning (version.workspace = true, edition = 2021, SemVer 2.0.0).

## Documentation
- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## CI / Git
- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
