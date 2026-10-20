# RUST/cnc-asset/tests/

## Responsibility

Single integration test file `cli.rs` (420 lines): end-to-end tests of the real `cnc-asset-cli` binary, one test group per converter command (BUILD_STATUS: 8 tests).

## Design

- Test harness helpers build minimal valid Westwood-format fixtures in memory: `palette_bytes()`, `shp_bytes(&[(w,h,pal)])`, `voc_bytes()`, `aud_bytes()` / `aud_bytes_with_samples(n)` — no fixture files on disk.
- `cli()` constructs the clap `Command` used by the binary; `test_dir(name)` creates isolated temp dirs (clean, atomic verification).

## Flow

Each test synthesizes bytes → writes temp files → runs the actual CLI subcommand (mix extract/list, shp→PNG, voc/aud→WAV, pal conversion, batch pipeline, generate-atlas) → asserts output files/JSON manifests/exit status; cleanup via test dirs.

## Integration

Exercises the `Commands` surface defined in `cnc-asset/src/cli.rs` and `convert`/`archive`/`shape`/`palette`/`audio` implementations; complements the 26 unit tests living inside `src/` (per BUILD_STATUS "parsers + converters + Ogg").
