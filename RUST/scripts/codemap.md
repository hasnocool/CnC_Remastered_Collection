# RUST/scripts/

## Responsibility

Asset-generation helpers that sit outside Cargo: shell wrappers and Python tools that produce the runtime art and AI-intent files consumed by `cnc-launcher`, `cnc-map-editor` and `cnc-asset`.

## Design

Three tools, one convention: outputs land in `assets/` and `assets/missions/` and are regenerable (git-ignored per `scripts/ai_sprites.py` docs).

- `generate_assets.sh` — wraps `cargo run -q -p cnc-asset --bin cnc-asset-cli -- generate-atlas --out <dir> --style <neon|...>`; the procedural atlas comes from `cnc_asset::atlas` (`AtlasStyle`).
- `ai_sprites.py` — two-stage AI sprite pipeline (`generate` = one SDXL concept render per sprite class via the local image generator at `/mnt/aux/projects/image-generator`, overridable with `CNC_IMAGE_GENERATOR`; `assemble` = chroma-key, crop, despill, quantize, pack into `assets/atlas.png` + `assets/atlas.json`).
- `ai_oracle.py` — sends the Agent-Studio prompts (`ai_oracle_request.json` / `ai_oracle_prompt.txt`, exported next to a scenario) to an OpenAI-compatible chat endpoint (Ollama default, env `CNC_LLM_*`), validates the JSON against requested slots, writes `ai_oracle_response.json` = the `cnc_ai::IntentCache`.
- `generate_ai_assets.sh` — thin dispatch over `ai_sprites.py generate|assemble|all`.
- `generate_rts_tileset.py` lives in `assets/packs/rts_16x16_tileset/` (kept beside its data), not here.

## Flow

Shell script → cargo/python → files under `assets/` and alongside maps → loaded by the launcher at startup (atlas slots) or by editor/launcher as `ai_oracle_response.json` IntentCache.

## Integration

 Consumers: `cnc-asset` (CLI binary invoked by shell), `cnc-ai::intent` (oracle response shape must match `parse_response`), launcher/editor asset loading. Environment secrets handled only via `CNC_LLM_API_KEY`; the only Python codemap in the repo lives here.
