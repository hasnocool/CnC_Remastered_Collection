# cnc-viewer - Semantic Versioning Enforced

This crate follows strict semantic versioning (SemVer 2.0.0). All releases must follow SemVer guidelines.

## Documentation
- CHANGELOG.md records version history.
- codemap.md for architecture docs.

## CI / Git
- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
