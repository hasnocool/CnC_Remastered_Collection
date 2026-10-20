# RUST

Rust workspace for the CnC Remastered Collection ports: one deterministic
lockstep simulation core shared by the Tiberian Dawn and Red Alert
front-ends, the launcher, the map editor, and the viewer.

## Asset sources

- The apps run with zero original game files; nothing at runtime scans for,
  requires, or detects them. That is the intended design.
- The default experience is fully in-repo and built/generated locally: the
  generated terrain pack `assets/packs/rts_16x16_tileset` (`ra_inspired` /
  `td_inspired`) backs the editor, launcher, and both games — `docs/TILESET.md`
  documents the layout and the editor's `--no-tileset` flag that forces flat
  colored quads; unit/building icons use the in-repo `assets/atlas.png` +
  `atlas.json`, which `cnc-asset`'s `load_or_generate` (cnc-asset/src/atlas.rs)
  regenerates in memory as a Neon-style atlas when absent; `cnc-procgen`
  deterministically synthesizes terrain heightmaps, biome/noise/Tiberium
  textures, and placeholder sprites; campaign maps are authored in-repo under
  `maps/`.
- Original game data is optional and enters only through the `cnc-asset`
  conversion CLI, which converts `.mix`/`.shp`/`.voc`/`.aud`/`.pal` ahead of
  time (`mix`, `shp`, `voc`, `pal`, `batch` subcommands).
- `ASSETS/OpenRA/` is a separate, optional download. Per its README:
  OpenRA-original artwork is CC-BY-SA (3.0/4.0) with share-alike and
  attribution requirements; art derived from the original EA Command &
  Conquer titles is not covered by OpenRA's licenses.

## Documentation
- Per-crate docs: every crate carries README.md, CHANGELOG.md, codemap.md.
- Rendering and tileset pack details: docs/TILESET.md.

## CI / Git
- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
