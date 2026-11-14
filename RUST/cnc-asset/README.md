# cnc-asset

This crate follows strict semantic versioning (version.workspace = true, edition = 2021, SemVer 2.0.0).

## Documentation
- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## CI / Git
- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.

## Original Asset Pipeline

### `original-fetch`

Fetches original C&C Remastered assets from a local Steam install into a native asset store.

```bash
# Fetch both games' original texture archives into the default store
cargo run -p cnc-asset --bin cnc-asset-cli -- original-fetch

# Fetch only Tiberian Dawn
cargo run -p cnc-asset --bin cnc-asset-cli -- original-fetch --game td

# Fetch into a custom store directory
cargo run -p cnc-asset --bin cnc-asset-cli -- original-fetch --output /path/to/store

# Override Steam install path
cargo run -p cnc-asset --bin cnc-asset-cli -- original-fetch --steam-root /path/to/Steam
```

### `original-catalog`

Catalogs original assets: lists templates, renders previews and contact sheets.

```bash
# Render all previews and contact sheets from default store
cargo run -p cnc-asset --bin cnc-asset-cli -- original-catalog

# List templates only
cargo run -p cnc-asset --bin cnc-asset-cli -- original-catalog --list-only

# Catalog from a specific store
cargo run -p cnc-asset --bin cnc-asset-cli -- original-catalog --store /path/to/store

# Catalog only Red Alert
cargo run -p cnc-asset --bin cnc-asset-cli -- original-catalog --game ra
```

### Original Asset Store

The original asset store is located at `RUST/assets/original` by default, or
pointed to by the `CNC_ORIGINAL_ASSETS_DIR` environment variable. The store
must contain a `manifest.json` file.

### New Modules

- `iconset.rs` - TD DOS iconset MAP reader: parses classic TD theater `.TEM`
  iconsets from MIX archives (`TEMPERAT.MIX`, `WINTER.MIX`, `DESERT.MIX`).
- `classic_mix.rs` - Classic Westwood MIX archive parser for TD/RA DOS.
- `preview_rendering.rs` - Template previews and contact sheets.
- `original_assets.rs` - Native reading of C&C Remastered Steam assets.
