## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Tile edge-matching fix round (2026-10-19)

#### Fixed
- Sprite UV sampling is now clamped to each sprite's own UV rect. Terrain tiles draw at cell size + 0.5 to hide seams between neighbouring quads; without the clamp the outermost pixel sampled the first texel column of the NEXT atlas tile, leaving a 1px sliver of foreign art on tile edges (software backend: rect clamp in `SoftwareRenderer::draw_sprite`; wgpu backend: half-texel UV inset in the sprite vertex shader)
- Removed the now-unused `SoftwareAtlas::height` field (sampling clamps per sprite rect, atlas height is no longer read)
