# Asset Strategy Decision Record

Scope: Phase 10, item 229 - original files + free alternatives.

Strategy (verified against current tree):
- Zero original files required at runtime. Default assets are procgen
  (cnc-procgen: fBm terrain, textures, placeholder sprites) plus in-repo
  generated tileset and regenerated atlas.
- Optional OpenRA download: SCRIPTS/download_openra_assets.py fetches
  CC-BY-SA art (verified URLs, sha256 manifest). Per ASSETS/OpenRA/README.md,
  derived original EA art is NOT covered.
- Full fidelity: requires original .mix/.shp/.voc/.aud/.pal files,
  converted ahead of time via cnc-asset CLI. Originals optional, never
  scanned at runtime.
