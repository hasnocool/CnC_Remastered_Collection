# plan-original-assets — Use the real CnC Remastered art instead of generated art

Status: active
Owner: hyperion

## Why

Our `cnc32` pack is hand-generated. Terrain transitions in particular have been through
five failed attempts (organic blobs — chunky bands — wall strips — outline trace — diced
seam), all rejected visually, because there is no ground truth to compare against.

The Steam install ships the original art inside `Data/TEXTURES_*_SRGB.MEG`, a Westwood
"MegaFile" container. Reverse engineering that container (already done, evidence
`.omo/evidence/original-asset-format.md`) yields 11487 real files: every terrain family
(TEMPERATE/DESERT/WINTER + SHROUD), UI textures, and VFX. That is the reference.

## Scope

In scope:
- Extract the original textures from the Steam MEG archives into a clean, in-repo
  asset directory (`RUST/assets/original/`) with a documented, reproducible path.
- Decode DXT5 DDS tiles to RGBA so they can be loaded.
- Wire the launcher and map editor to render the original terrain art.
- Visual QA of both surfaces against the original assets.

Out of scope (explicitly deferred, no silent shrinkage):
- Replacing UAV/building/unit art (needs `MODELS.MEG` + `TEXTURES_PATCHES` work).
- RA (Red Alert) art (`TEXTURES_RA_SRGB.MEG`, 750 MB) — same pipeline, run later.
- Anything from the encrypted `Data/CNCDATA/*.MIX` (unreadable without running the game).
- Audio/movies (separate crates already handle original formats).

## Known facts (evidence)

Container format (observed, `TEXTURES_TD_SRGB.MEG`, 612877161 bytes):

| offset | value | meaning |
|---|---|---|
| 0x00 | `ff ff ff ff` | marker |
| 0x04 | `a4 70 7d 3f` | magic |
| 0x08 | filesize | |
| 0x14 | names byte size | name table spans 0x18 .. 0x18 + this |
| 0x18 | N x {u16 len, path} | NUL-free paths, packed |
| names_end + 14 | N x 20 bytes | {u32 offset, u32 size, u32 uncompressed, u32 index, u32 tag} |

- Index has exactly one record per name; record 0's `index` field is 1 (1-based).
- Pairing name[i] with the record whose index field == i+1 yields valid payloads.
- Payload blobs are raw. DDS tiles: `height 128, width 16384`, DXT5, 8 mips, caps 0x401008.
- Non-DDS members exist: 180 `ZIP` (VFX), 13 `TGA` (UI).

Extraction tool: `RUST/cnc-asset/examples/meg_extract.rs` (in this change).

## Steps

1. [x] Extract with `meg_extract.rs`; validate DDS magic count (11293/11489 valid).
2. [x] Decode a DXT5 128x128 tile to RGBA in a throwaway script.
3. [ ] Copy a curated original asset tree into `RUST/assets/original/` (terrain, UI).
4. [ ] Add `cnc-asset` support: read a `.DDS` DXT5 header + decode to RGBA, returning a
   `SpriteFrame`/tile-sized texture. No new crate.
5. [ ] Add a `tileset_pack` source variant that loads original terrain DDS by name
   (`TEMPERATE/SC1`, `SH1`, `TC01`, ...) instead of generating blobs.
6. [ ] Point the map editor's terrain draw at the original pack; verify visually.
7. [ ] Point the launcher's map render at the same pack; verify visually.
8. [ ] Screenshot both surfaces; compare side by side with the generated art; record.
9. [ ] Update `PARITY.md` and the per-crate codemaps with the asset source + extraction
   recipe; commit.

## Risks

- Generated `cnc32` art remains in the repo; both packs must coexist (no destructive swap).
- Original art is 4K-upscaled remaster art, not the 1996 16x16 pixels. If the match to
  gameplay authenticity matters (16x16 grid), we need the CNCDATA mix, which needs the
  running game to decrypt. Not in scope now.
- `.DDS` sizes are large (87 MB for TD terrain). Keep the in-repo copy curated, not a
  mirror of everything.

## Acceptance

- `RUST/assets/original/` exists with a README describing provenance and extraction command.
- Map editor renders original terrain for all four TD theaters on an original-style map.
- Launcher renders original terrain in the same map view.
- Both captured as screenshots under `.omo/evidence/`.
- `cargo fmt --check && cargo clippy --workspace --all-targets && cargo test --workspace` green.
- No pre-existing test weakened or deleted.
