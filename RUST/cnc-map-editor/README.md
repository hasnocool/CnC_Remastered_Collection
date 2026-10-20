# cnc-map-editor

This crate follows strict semantic versioning (version.workspace = true, edition = 2021, SemVer 2.0.0).

## Documentation
- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## Steam Divergence
**This editor intentionally diverges from the original C# Mobius editor in the following ways:**
- **No Steam Workshop integration** — The original C# editor integrated with Steam Workshop (AppID 1213210) for map publishing. This Rust reimplementation uses plain INI files for map storage and does not include Steamworks.NET integration.
- **Map format** — Maps are saved as plain INI files, not the original binary format. This enables version control, cross-platform compatibility, and easier debugging.
- **No Steam authentication** — No Steam API keys or authentication required for map editing or testing.

## CI / Git
- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
