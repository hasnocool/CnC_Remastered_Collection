# Build Status

_Last verified: cargo 1.98.1 / rustc 1.98.1 on Linux._

## Verification

~~~
cargo check --workspace --all-targets   # clean (warnings only)
cargo test --workspace                 # 223 passed, 0 failed
~~~

Per-target tests:

| Target | Tests |
|---|---|
| cnc-core (unit + INI + Rules.ini) | 57 |
| cnc-asset (parsers + converters + Ogg) | 26 |
| cnc-asset-cli (end-to-end integration) | 8 |
| cnc-tiberian-dawn (game logic + rules + lockstep + netcode + sidebar/AI) | 127 |
| cnc-renderer (camera + sprite batch) | 3 |
| cnc-renderer (GPU smoke, ignored unless adapter present) | 2 |

## Phase 2: Asset Pipeline - COMPLETE

### Implemented Parsers
- **archive.rs**: MIX v1/v2 (.mix) - Zlib decompression, name tables, extract/extract_all
  - Payload offsets start after the name table; compressed-entry lengths are honoured.
- **shape.rs**: SHP (.shp) - Frame headers, RLE decoding, bounds, palette-to-RGBA
- **audio.rs**: VOC (.voc) - Creative Voice blocks 0x00-0x0A, AUD (.aud) - Westwood Audio
- **palette.rs**: PAL (.pal) - 768-byte RGB, gamma correction, grayscale, round-trip test

## Phase 3: Rendering - COMPLETE

- **sprite_batch.rs**: Instanced sprite batching (10K capacity), z-order + texture sorting
- **shaders/sprite_batch.wgsl**: Palette remapping (256 colors), alpha blending
- **camera.rs**: Orthographic camera, world<->screen transforms, zoom/pan
- **renderer.rs**: Renderer trait, WgpuRenderer (async), SoftwareRenderer fallback

## Phase 4: Asset Converter CLI - COMPLETE

convert.rs is the shared library behind the cnc-asset-cli binary; tests/cli.rs
exercises the binary itself.

| Command | Behaviour |
|---|---|
| mix -i X.MIX -o DIR [--list] | List or extract every MIX entry |
| shp -i X.SHP -o OUT [-p PAL] [--sprite-sheet] [--opaque] | Per-frame PNGs, or one sheet + JSON manifest |
| voc -i X.voc|X.aud -o OUT [--format wav|ogg] [--loop-start N --loop-end N] [-q Q] | WAV or Ogg Vorbis with loop points |
| pal -i X.PAL -o DIR [--format json|png|css] | Palette export |
| batch -i SRC -o OUT -g td|ra [-p PAL] [--sprite-sheet] [--audio-format wav|ogg] | Recursive conversion mirroring the tree |
| generate-atlas -o DIR [--style classic|neon] | Placeholder sprite atlas |

Details:
- SHP frames honour palette index 0 as transparent (disable with --opaque).
- Sprite sheets are packed horizontally and described by a JSON manifest.
- VOC loop blocks (0x09/0x0A) are preserved by default; CLI flags can override them.
- Ogg Vorbis (pure Rust binding to libvorbis) writes LOOPSTART/LOOPEND/LOOPLENGTH
  Vorbis comments; WAV output gets a RIFF smpl chunk with a forward loop.
- Batch conversion records per-file failures and exits non-zero if any asset fails.

## Phase 5: Game Logic - Shared Foundation (IN PROGRESS)

### 5.1 Command queue & lockstep - DONE for local/LAN transport
- Deterministic command ordering by (frame, player, hash), independent of UDP arrival.
- LockstepSession: validated scheduling, frame gating, ordered replay log, per-frame
  checksums, remote desync detection, replay reconstruction.
- Binary packet codec (magic CNCP, version 1) with defensive limits, trailing-byte
  checks and checksum validation.
- UdpTransport: explicit peer list, encoded multi-peer broadcast, validated receive,
  plus datagram dispatch that routes command and handshake frames on one socket.
- Replay persistence: versioned CNCR file (magic/version/player count/command records)
  with an FNV-1a trailer; `Replay::save/load` and `LockstepSession::save_replay/
  load_replay` reject corruption, truncation and unsupported versions.
- Peer join/leave handshake: CNCH `JoinRequest/JoinAccepted/JoinRejected/Leave`
  frames and a `PeerHandshake` state machine that validates slot numbers, room size,
  duplicate addresses and emits `PeerJoined/PeerLeft` events.
- Still TODO: lockstep mesh relay (broadcast works today, forwarding does not).

### 5.2 Rules/INI registry - DONE (first pass)
- cnc-core/src/rules.rs parses Rules.ini into typed data: GeneralRules (RULES.CPP
  defaults), DifficultyRule, WeaponRule, WarheadRule, TypeRule, HouseRule and the
  Infantry/Units/Aircraft/Buildings categories.
- Unlisted sections are classified by their keys (Damage/ROF/Range/Warhead ->
  weapon, Verses/Spread -> warhead, Cost/Strength/Prerequisite -> type).
- INI fixes: decimal + 0x-hex integers, numeric/boolean disambiguation,
  case-insensitive lookup, multi-level inheritance, cycle detection.
- Costs, strength and production times use rules overrides.
- Weapons: Damage, ROF and Range overrides replace CONST.CPP values; Warhead=
  selects a warhead and Verses/Spread replace the armor-modifier table for both
  projectiles and explosion damage.
- Tech tree: Prerequisite= tokens are enforced for unit and building production,
  including GDI/NOD faction tokens and PYLE/HAND, HPAD/AFLD aliases; missing
  prerequisites are exposed through missing_prerequisites() for UI use.
- TechLevel gating: per-type build levels from Rules.ini `TechLevel=` (fallback:
  TDATA/UDATA/BDATA values) are checked against the house build level before
  production; houses default to level 98 (skirmish/multiplayer) and scenarios may
  lower it.
- Sidebar visibility: `Game::sidebar_options` / `visible_sidebar_options` expose
  rules cost, TechLevel, affordability, producer and unmet prerequisites; the
  launcher renders only visible rows and uses rules-derived costs.
- AI production weighting: `AI::production_weights` scores units from the rules
  registry (faction, TechLevel, prerequisites, cost, health-per-credit, difficulty)
  and `choose_weighted` makes a deterministic, lockstep-safe selection; the game
  exposes `ai_production_choice` and `ai_queue_production`.
- Launcher accepts --rules Rules.ini and applies the overrides to the demo game.

## Core Types (Phase 1) - COMPLETE

- Fixed16_16, FloatBias, ClassId, CodePointers/DecodePointers, CCPtr/CCWeak/NoInit,
  ObjectPool, INI parser with inheritance and the typed Rules registry

## Workspace Structure
- 8 crates: cnc-core, cnc-renderer, cnc-audio, cnc-asset, cnc-tiberian-dawn,
  cnc-red-alert, cnc-map-editor, cnc-launcher
- All dependencies in workspace Cargo.toml

## Next Steps
1. Phase 6: expand Tiberian Dawn game logic (houses, missions, command execution).
2. Lockstep mesh relay and host migration.
3. Faction `Owner=` filtering for sidebar and AI production.
