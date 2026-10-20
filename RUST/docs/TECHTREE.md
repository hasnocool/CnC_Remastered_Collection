# Tech Tree & Branching Meta — Design Suggestions

**Status:** proposal (not implemented) · **Companion doc:** [`PROGRESSION.md`](PROGRESSION.md) (profile, XP, stars, streaks, ranked split — this doc builds on its `cnc-meta` crate, `RuleTweak` table, and integrity rules).
**References:** StarCraft II (co-op mastery/prestige, in-match research) and Path of Exile (large branching passive web, keystones, respec).

## Goal

A **branching doctrine tree** players invest earned points into — PoE-style
web between matches, SC2-style research and prestige choices inside and around
matches — that creates build identity ("I'm a hover-swarm commander", "I play
economy-stall") and long-term goals, without breaking deterministic lockstep
or ranked integrity.

## Design Pillars

1. **Choices are tradeoffs, not upgrades.** PoE's genius is keystones
   (+X / −Y). Flat "+" nodes are boring and compress balance; branch-defining
   nodes with costs create identity and discussion.
2. **Same determinism contract as buffs.** Every tree effect is a
   code-reviewed mutation of `RulesDatabase` (`cnc-core/src/rules.rs` —
   `GeneralRules`, `TypeRule`, `HouseRule`, `WeaponRule`, `WarheadRule`),
   applied inside `GameSnapshot.rules` (`cnc-tiberian-dawn/src/savegame.rs:64`)
   before frame 0, or delivered as a `Command` in the lockstep stream
   (`cnc-tiberian-dawn/src/network.rs`). Never client-side.
3. **Ranked stays clean.** The MMR ladder (`cnc-net-directory/src/matchmaking.rs`,
   100–8000 per-title) only sees standard rulesets. Trees apply to campaign,
   unranked, co-op, and vs-AI.
4. **Respec must be cheap enough to experiment.** Fear of wasted points kills
   exploration; PoE's mistake (respec scarcity) becomes our lesson.

---

## Layer 1 — Doctrine Web (meta, between matches; PoE-style)

### Structure

One web per faction (GDI / NOD / Allies / Soviet), sharing a layout
convention so UI code is common (`cnc-renderer` draws nodes/edges;
`SpriteBatch` + a zoomable/pannable camera like the map editor's).

- **Center: Command Core** — starting node, no effect; the identity anchor.
- **Six branch axes** radiating outward (PoE-style arcs, not linear columns):
  `Armor`, `Infantry`, `Air`, `Economy`, `Support/Intel`, plus one
  **faction-unique branch** (e.g. NOD: Stealth/Terror; Allies: Chrono/Spy).
- **Cross-links** between adjacent branches at mid-radius (hybrid builds —
  this is what makes the web feel like a web).
- **Node types:**
  | Type | Count per branch | Effect shape |
  |---|---|---|
  | Minor passive | 8–10 | small ±, the connective tissue |
  | Notable | 2–3 | named, build-defining (e.g. "Combined Arms: +12% damage vs. infantry on vehicles, −8% vs. armor") |
  | **Keystone** | 1 (branch exit) | big tradeoff, mutually exclusive with the sibling keystone |
  | Capstone | 1 per faction | prestige-like identity rewrite (Layer 3) |

### Keystone examples (tradeoff-first)

- **Steel Tide** (Armor): vehicles +20% HP, −15% speed, infantry production
  disabled.
- **Swarm Doctrine** (Infantry): infantry cost −30%, max infantry cap +50%,
  vehicle production −25% slower.
- **Scorched Ledger** (Economy): +25% harvester yield, refineries explode on
  death (yours and theirs).
- **Silent Service** (NOD stealth): all stealth units +1 visibility-reduction
  tier, base defenses −20% damage (you traded safety for offense).

### Points economy (feeds from PROGRESSION.md)

- **Doctrine points** earned per account level (System 5), per campaign star
  (System 2), and weekly from streaks (System 3) — three loops feeding one
  sink.
- Budget scaled so a full branch is a *season* goal, not a weekend: e.g.
  ~1.5 points/active day, branch ≈ 25 points.
- **Respec:** full refund tokens drop monthly + free respec for 48h after
  each balance patch (players should never feel punished for patch changes).

### Variety rotation

- **Weekly "Focus Branch":** one branch grants +10% point gain; rotating
  spotlight nudges players into unfamiliar builds (retention through variety,
  zero new code paths — it's a multiplier on the existing grant).

## Layer 2 — In-Match Research (SC2 cyber-core style; live decisions)

**This is the lockstep-safe version of mid-match upgrades — the repo already
proves the pattern.** `Command::Sabotage` is a mid-match state mutation with a
documented determinism contract (`network.rs:190-209`: "carries no random or
time-derived data… two peers applying the same command stream reach identical
state, which is what the lockstep frame checksums compare").

### Mechanism

- New `Command::Research { building: u32, tech: u16 }` following the same
  contract: validated against the issuing player's owned building, applies a
  **predefined `RuleTweak` scoped to that player's `HouseRule`/production
  set** at apply-time. Both peers apply it at the same frame; checksums hold.
- **Choose-1-of-3** offered when a tech building (e.g. Tech Center) completes:
  concrete, visible, mutually exclusive per building (SC2's most-praised
  tension: same building, different identity per match).
- Cost: credits + production pause on that building (the tradeoff is
  opportunity cost, not hidden math).
- **Feedback moment:** research completes → EVA voice line + distinct
  `AudioMixer` sting + renderer flash on the building (dopamine beat inside
  the match, complementing match-end rewards).

### Scope guard

In-match research pool is **small (6–9 techs per faction) and flat-strong**
(+10% unit-line speed, unlock a special ability, −15% cost of one category).
Meta-tree points give *breadth*; in-match research gives *this-match* adaptivity.

## Layer 3 — Capstones / Prestige Loadouts (SC2 prestige style)

- Each faction has 3 **capstones** at web edges: a playstyle rewrite (e.g.
  Allies "Temporal Corps": Chronosphere-capable units, but no air production).
- **Only one active per loadout.** Loadouts are saved, named, and shown in
  the pre-match screen (identity display — social dopamine).
- Unlock: faction level 10+ + campaign completion of the faction's arc
  (prestige must be earned through *play*, not points alone).

## Campaign Integration

- **Intel caches:** missions hide 1–2 collectibles granting doctrine points +
  lore fragments (leverages existing `Trigger`/`Condition`/`Action` in
  `cnc-tiberian-dawn/src/mission.rs` — a `GrantDoctrine` action type).
- **Carry-over:** tree applies to campaign replays and skirmish vs-AI; stars
  (System 2) are per-mission and unaffected by tree strength, so leaderboards
  stay comparable.
- **AI ally synergy:** `cnc-ai`'s `AgentScript` commanders can be tagged with
  the player's active branch doctrine (`AgentLadderCard` gains a doctrine
  field) — your AI co-op partner fights in *your* style.

## Data Format & Sharing

- Trees are **static JSON definitions** (nodes, edges, effects referencing
  `RuleTweakId`s, costs, localized names) loaded like other INI/JSON assets
  (`cnc-core/src/rules.rs::RulesDatabase::from_ini` precedent; atlas/JSON
  contract in `cnc-asset`).
- **Versioned:** each tree JSON carries a `ruleset_fingerprint`; handshake
  (`NetworkConfig`) exchanges it — mismatch aborts with a clear error, never a
  desync.
- **Future: community trees.** `cnc-net-directory` already ships torrent
  distribution (`torrent.rs`, `TorrentSource`, node lists) — user-authored
  doctrine trees for custom maps (via `cnc-map-editor`) can ride the same
  content pipeline. Label clearly as modded (never rated).

## Ranked Integrity & Anti-Abuse

- Ranked queue (`cnc-net-directory` matchmaking) pins the **stock ruleset +
  empty tree**; identical policy to the buff-token exclusion in
  `PROGRESSION.md` §4.
- Every node effect must exist in the **same code-reviewed `RuleTweak` table**
  as buffs, with the same per-node determinism test (identical
  `frame_checksum` sequences across two simulated runs —
  `LockstepSession::frame_checksum`).
- Tree JSON is hashed and exchanged pre-match; tampering on one client
  surfaces as a handshake mismatch, not a silent advantage.

## UI & Presentation

- **Doctrine web screen:** new `Screen` variant (`cnc-launcher/src/menu.rs`),
  full-bleed web with zoom/pan, allocated-node glow, unallocated dimmed;
  hovering a node shows exact delta text (numbers, not vibes).
- **Mastery tab:** per-faction level bar + capstone cards (SC2 co-op style).
- **Pre-match screen:** your loadout name + keystone icon visible to all
  players in unranked lobbies (build identity as social signal).
- **Audio:** shared progression leitmotif (PROGRESSION.md §7), pitched variant
  for keystone respec and capstone unlock.

## Suggested Build Order

| Phase | Deliverable | Depends on |
|---|---|---|
| 1 | Tree JSON schema + loader + `RuleTweak` scoped-application | `cnc-meta` (PROGRESSION.md P1) |
| 2 | Doctrine web UI screen + point granting | launcher menu, renderer |
| 3 | Campaign intel caches (`GrantDoctrine` trigger action) | mission.rs actions |
| 4 | `Command::Research` + tech buildings + choose-1-of-3 UI | network.rs Command contract |
| 5 | Capstones/loadouts + pre-match display | tree data, lobby UI |
| 6 | Ranked pinning + fingerprint handshake | matchmaking, `NetworkConfig` |
| 7 | Community tree sharing via directory torrents | future |

## Non-Goals

- No pay-for-points; no tree effects in ranked; no random node rolls (fixed
  web, deterministic); no mid-match tree editing (research commands only);
  no per-player RNG anywhere in the simulation.
