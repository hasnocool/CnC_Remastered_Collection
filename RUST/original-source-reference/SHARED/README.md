# Shared Graphics and Asset Formats

Cross-cutting format documentation shared by Tiberian Dawn and Red Alert.

## Format Guides

| Format | Files | Status | Rust Mapping |
|--------|-------|--------|--------------|
| **ICONSET** | `original/TIBERIANDAWN/WIN32LIB/ICONSET.CPP`, `original/REDALERT/WIN32LIB/ICONSET.CPP` | Reviewed | cnc-asset/src/iconset.rs |
| **TILE.H** | `original/TIBERIANDAWN/WIN32LIB/TILE.H`, `original/REDALERT/WIN32LIB/TILE.H` | Reviewed | cnc-asset/src/iconset.rs (header only) |
| **MIX Archive** | `original/TIBERIANDAWN/MIXFILE.CPP`, `original/REDALERT/MIXFILE.CPP` | Reviewed | cnc-asset/src/classic_mix.rs |
| **IFF Container** | `original/TIBERIANDAWN/WIN32LIB/IFF.CPP`, `original/REDALERT/WIN32LIB/IFF.CPP` | Partially reviewed | cnc-asset/src/iff.rs |
| **Palette** | `original/TIBERIANDAWN/WIN32LIB/PALETTE.CPP`, `original/REDALERT/WIN32LIB/PALETTE.CPP` | Partially reviewed | cnc-asset/src/palette.rs |
| **Template Types** | `original/TIBERIANDAWN/CDATA.CPP`, `original/REDALERT/CDATA.CPP`, `original/TIBERIANDAWN/TEMPLATE.CPP`, `original/TIBERIANDAWN/TEMPLATE.H` | Reviewed | cnc-asset/src/original_templates.rs |
| **Script Utility** | `original/SCRIPTS/tgautil.py` | Inventory only | N/A |
| **Font** | `original/TIBERIANDAWN/WIN32LIB/FONT.CPP`, `original/REDALERT/WIN32LIB/FONT.CPP` | Inventory only | cnc-asset/src/font.rs |
| **Shape** | `original/TIBERIANDAWN/WIN32LIB/SHAPE.H`, `original/TIBERIANDAWN/WIN32LIB/SHAPE.INC` | Inventory only | N/A |
| **Audio (VOC/AUD)** | `original/TIBERIANDAWN/WIN32LIB/AUDIO.H`, `original/TIBERIANDAWN/WIN32LIB/LOAD.CPP` | Partially reviewed | cnc-audio, cnc-sfx |

## TD vs RA Differences

| Format | Tiberian Dawn | Red Alert |
|--------|---------------|-----------|
| **ICONSET** | Standard IFF container | Adds map width/height metadata |
| **TILE.H** | C&C `IControl_Type` under `#if (1)` | RA `IControl_Type` under `#if (0)`; adds map width/height fields |
| **Template Dimensions** | From definitions | From icon-set metadata |

## Quick Links

- [System Map](../01-system-map.md)
- [Cross-Reference](../03-rust-port-cross-reference.md)
- [Inventory](../02-original-file-inventory.md)
