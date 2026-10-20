# Progression & Reward System — Design Suggestions

**Status:** proposal (not implemented) · **Scope:** RUST workspace · **Grounding:** every suggestion cites existing code so engineering cost is visible.

## Goal

Give players a reason to come back every day and a visible sense of growth, in
both campaign and online play, without compromising the two load-bearing
properties of this codebase: **deterministic lockstep multiplayer** and the
**split between ranked integrity and casual fun**.

## Design Pillars

1. **Determinism first.** Any gameplay-affecting bonus must enter the
   simulation through the serialized `RulesDatabase` inside `GameSnapshot`
   (`cnc-tiberian-dawn/src/savegame.rs:44`) or as a frame-0 `NetCommand`
   (`cnc-tiberian-dawn/src/network.rs`, `LockstepSession::schedule`). Never as
   a client-side mutation — `frame_checksum`/`desynced` would flag it, and
   rightly so.
2. **Ranked stays clean.** The MMR ladder in `cnc-net-directory/src/matchmaking.rs`
   (`DEFAULT_RATING = 2000`, range 100–8000, per-title, JSON-persisted) is the
   competitive spine. Buffs and progression rewards apply only to unranked /
   campaign / co-op play, so the ladder keeps meaning.
3. **Dopamine from variable, visible, earned rewards.** Fixed rewards fade;
   variable-ratio rewards, streaks, near-miss feedback, and unlock trailers
   keep the loop alive. Everything must be *shown* at the moment it is earned
   (`AudioMixer` jingle + renderer overlay), not buried in a menu.
4. **No purchasable power.** Monetization, if ever added, is cosmetic only.
   This preserves trust and matches the GPL/community nature of the project.

---

## System 1 — Player Profile (foundation; build first)

**Current state:** no cross-session player persistence exists outside saves
(`ra_save_path_for(slot)` in `cnc-launcher/src/main.rs`) and the
net-directory rating JSON.

**Suggestion:** add a small `cnc-meta` crate (workspace member, leaf crate
depending only on `serde`/`thiserror`):

- `PlayerProfile { name, created, xp, level, streak { count, last_login, freezes }, buff_tokens, unlocked: BTreeSet<UnlockId>, stats: TitleStats }`
- Persisted as JSON at the platform config dir (same lifecycle pattern the
  net-directory ladder uses; `PLAYER_NAME_MAX = 32` from `matchmaking.rs`
  is the name-length contract).
- **Match telemetry source:** `GameSnapshot` already records
  `harvested_credits`, `units_produced`, `projectiles_fired`,
  `aircraft_shots`, `aircraft_rearmed`, `secondary_shots`, `elapsed`
  (`savegame.rs:44-64`). Extract these at match end into profile stats —
  zero new in-game instrumentation needed.

## System 2 — Campaign Progression (stars, unlocks, replay)

**Current state:** `Mission` (`cnc-tiberian-dawn/src/mission.rs:317`) with
`Trigger`/`Condition`/`Action`, `add_objective`/`complete_objective`/
`objectives_done()`, `MissionOutcome`, `is_over()`/`finish()`.

**Suggestions:**

- **1–3 star ratings per mission** derived from already-tracked values:
  - ★ objective completion (all objectives vs. partial via `objectives_done`)
  - ★ efficiency (`harvested_credits` vs. threshold; losses vs. threshold)
  - ★ speed (`elapsed` vs. par time; par times stored per-scenario INI)
- **Star-gated unlocks:** campaign stars → cosmetic unlocks + buff tokens
  (System 5). Mission select screen shows stars (menu already has per-campaign
  scenario rows — `MainMenu::discover`, `Screen` enum in
  `cnc-launcher/src/menu.rs`).
- **Replay incentive:** "par-beater" badges; per-mission best-time storage in
  the profile.
- **Achievements from the existing trigger system:** the INI trigger
  Condition/Action pipeline is already extensible — add a `ProfileAction`
  that writes an achievement event at `Mission::finish()` time. Examples:
  "First Blood" (first kill), "Economy Baron" (10k credits harvested in one
  mission), "Untouchable" (win with zero buildings lost).

## System 3 — Daily Rewards & Streaks

**Suggestions:**

- **Login streak:** day 1 → small token grant, escalating to day 7, then
  resets with a "streak freeze" item earned weekly (protects the feeling of
  investment; classic retention mechanic).
- **Variable-ratio grant:** the *day-7* reward is rolled from a weighted pool
  (guaranteed floor, small chance of a jackpot cosmetic) — variable rewards
  sustain engagement far longer than fixed ones.
- **Deterministic date handling:** streak logic uses local calendar date only;
  never enters the simulation, so no lockstep risk.
- **"Come back" hooks:** daily challenge seed of the day (a fixed random seed
  → same skirmish map/rules for everyone that day). One shared challenge
  creates community talk and is trivially fair: same seed, same rules.

## System 4 — In-Match Buffs (the core dopamine loop)

Two buff classes, both determinism-safe:

### 4a. Rule-Tweak Buffs (unranked only)

**Mechanism:** buffs are *pre-approved mutations of `RulesDatabase`* applied
at snapshot creation, before frame 0. Because `rules: RulesDatabase` is
serialized inside `GameSnapshot` (`savegame.rs:64`), both peers compute
identical results and checksums match. Implementation surface:

- A `BuffId` enum with a fixed, code-reviewed table: id → closure over
  `&mut RulesDatabase` (e.g. `+15% harvester speed` via `HouseRule`/`TypeRule`
  speed fields, `-10% build time` via `GeneralRules`, `+1 warhead verses tier`
  via `WarheadRule::verses_percent`). Keep the pool small (10–15) and unit-test
  each one for determinism (same input snapshot → same frame checksums).
- Both players' active buff tokens are exchanged in the pre-game handshake
  (`NetworkConfig` in the launcher) and injected identically.
- **Casual co-op modifier idea:** the mission INI's existing
  `DifficultyLevel` (`cnc-core/src/rules.rs:214`, `parse`/`label`) is a
  natural per-player scaling axis for co-op buffs.

### 4b. Meta Buffs (always safe, never touch simulation)

- Pre-match loadout cards (choose 1 of 3 at mission start — visible choice
  creates anticipation): e.g. "Double first 2 minutes of harvested credits"
  (bookkeeping-only, in launcher/stat layer), cosmetic unit skins, alternate
  EVA voice packs (`cnc-sfx`/`cnc-music`), banner/emblem for the lobby.
- **Earn-by-play:** buff tokens drop from streaks, stars, and match XP
  (System 5) — every loop feeds the next.

**Ranked exclusion:** matchmaking (`cnc-net-directory`) marks queue type;
ranked matches force empty buff slots and clean rules. The rating ladder
(100–8000) stays trustworthy.

## System 5 — XP & Levels (the visible growth curve)

**Suggestions:**

- **XP sources** (all from existing telemetry, credited at match end):
  victory, objectives completed, stars earned, streak multiplier, first daily
  win. Losses grant partial XP (keeps quitting from feeling total).
- **Levels unlock, not empower:** levels gate *cosmetics, buff token
  capacity, daily-challenge access, and AI commander slots* — never raw
  power in ranked. Power progression lives in skill (MMR) and collection
  (tokens), which avoids the "grind wall" that punishes new players.
- **Level-up moment:** full-screen level card + `AudioMixer` fanfare
  (`play`/`RecordedPlay` already supports programmatic jingles) at return-to-
  lobby, not mid-combat.

## System 6 — Social & AI Hooks (community dopamine)

**Current state:** `cnc-ai` ships `AgentScript` (deterministic), the
`AgentRuntime`, and `AgentLadderCard`/`AiLadderRegistration`
(`cnc-ai/src/ladder.rs`) designed for "the ranked ladder" per
`docs/AI_AGENTS.md`; `cnc-net-directory` has LFG/matchmaking and a mesh
relay with `HostMigration`.

**Suggestions:**

- **Community AI commanders:** players publish their `AgentScript` bots to
  the directory; others fight them. Beating a higher-ladder bot grants bonus
  XP + a "Bot Slayer" badge. This turns `AiLadderRegistration` into a
  retention feature and gives the map-editor AI authoring loop an audience.
- **Co-op campaign with AI ally:** a progression path for solo players —
  complete missions with an AI commander ally for a "witnessed" XP bonus.
  Deterministic because agents are deterministic by contract.
- **Weekly ladder seasons:** rating bands named (e.g. 2000+ = "Gold General"
  band from `DEFAULT_RATING`), season-end cosmetic rewards, soft MMR decay.
  Seasons give lapsed players a reason to return and a fresh goal.

## System 7 — Feedback & Presentation (where dopamine is actually delivered)

- **Match-end screen:** stars fly in one at a time with distinct sfx pitches
  (rising pitch = earned progress), XP bar animates, next-unlock progress
  preview ("2 more wins → [skin name]") — the *preview* of the next reward is
  the strongest retention hook.
- **Streak rituals:** day-counter badge on the main menu that visibly grows;
  losing a streak shows the freeze being consumed, never silent loss.
- **Audio signature:** one short leitmotif for all progression moments,
  pitched/stretched by reward size (`cnc-audio` fades/channels already
  support this; `cnc-music` can supply the motif).

---

## Integrity & Abuse Considerations

- **Profile tampering:** local JSON profiles are inherently editable. Accept
  local-trust for cosmetics/streaks (single-player harm only), but attach an
  HMAC keyed at first-run for the rating-adjacent fields, and always let the
  net-directory's server-side settled results be authoritative for MMR
  (it already owns rating updates).
- **Buff determinism tests:** every `BuffId` mutation must ship with a test
  asserting identical `frame_checksum` sequences across two simulated runs.
- **Save format:** never alter `GameSnapshot` field order; add progression
  data alongside, using the existing `version` field for gating.

## Suggested Build Order

| Phase | Deliverable | Touches |
|---|---|---|
| 1 | `cnc-meta` crate + profile JSON + match-end telemetry extraction | new crate, launcher |
| 2 | Campaign stars + mission select display + XP | launcher, mission.rs readouts |
| 3 | Daily streak + daily challenge seed | `cnc-meta`, menu |
| 4 | Rule-Tweak buff table + determinism test harness | `cnc-core` rules, TD/RA init, net handshake |
| 5 | Level/unlock cosmetics + feedback moments (sfx/overlay) | launcher, `cnc-sfx`, `cnc-audio` |
| 6 | Seasons + community AI ladder integration | `cnc-net-directory`, `cnc-ai` |

## Non-Goals

- No purchasable gameplay power; no buffs in ranked; no mid-match buff
  injection (breaks lockstep trust); no per-player random events inside the
  simulation (only the shared daily seed, which is identical for all peers).
