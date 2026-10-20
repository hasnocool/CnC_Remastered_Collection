# Crafting & Procedural Items — Design Suggestions

**Status:** proposal (not implemented) · **Companion docs:** [`PROGRESSION.md`](PROGRESSION.md) (profile, salvage economy, ranked split) · [`TECHTREE.md`](TECHTREE.md) (RuleTweak contract, fingerprint handshake, doctrine sets).
**Reference feel:** Diablo II — white/blue/yellow/gold rarities, affix tiers, two-part rare names, uniques, runewords, gambling, the Horadric Cube.

## Goal

A **procedurally rolled item system** for vehicles, weapons, and support
modules: every crafted unit is a base chassis plus randomly rolled affixes
("Reinforced Cannon of Haste, +18% HP / +9% speed / −12% build time"), giving
loot-brain a home in an RTS and a reason to keep playing campaign and
unranked online.

## Design Pillars

1. **Items are data, never runtime dice.** The simulation must never roll
   RNG. An item is `{ base_id, seed, affix_ids }`; stats are a **pure
   function** of that triple plus a versioned affix table. Two peers
   deserialize the same item → identical stats → identical lockstep
   checksums.
2. **Reuse the pinned PRNG.** `cnc-procgen/src/prng.rs` already ships
   SplitMix64 with pinned test vectors, an explicit `Seed(u64)` newtype,
   `Rng::next_range(lo, hi)` for tier/value rolls, and stateless
   `hash_coord(x, y, seed)` for drop rolls. Its philosophy ("pinned here,
   never version-drifted") is exactly the affix-table contract. **Do not add
   a `rand` dependency anywhere in this system.**
3. **Same injection contract as buffs and tree nodes.** Crafted stats mutate
   `RulesDatabase` (`cnc-core/src/rules.rs`: `TypeRule`, `HouseRule`,
   `WeaponRule`, `WarheadRule`) inside `GameSnapshot.rules`
   (`cnc-tiberian-dawn/src/savegame.rs:64`) before frame 0 — the existing
   `RuleTweak` table from the companion docs, extended with per-instance
   values.
4. **Earned, never bought.** No real money, consistent with the series.

---

## Layer 1 — Item Bases & Rarities (D2 mapping)

| D2 concept | This game |
|---|---|
| White base item | **Base platform**: stock chassis/weapon from `ClassId` (`cnc-core/src/class_id.rs`) with no affixes |
| Magic (blue) | Base + 1 prefix + 1 suffix |
| Rare (yellow) | Base + 3–6 affixes + **procedural two-part name** (see below) |
| Unique (gold) | Hand-authored fixed legendaries with signature quirks |
| Set (green) | 3–4 item sets, one per TECHTREE doctrine branch — completing a set activates its branch keystone's flavor effect in unranked |
| Runewords | **Sockets**: chassis have 0–2 module sockets; specific *ordered* module combinations unlock a runeword bonus |
| Horadric Cube | **Workshop** transmutation recipes (Layer 3) |
| Gamble | **Requisitions**: credits → random item of a chosen base class (Layer 3) |

### Affix system (RTS translation of D2 affixes)

Static JSON table, versioned with a `affix_fingerprint` exchanged in the
match handshake (same mechanism as TECHTREE's `ruleset_fingerprint` —
mismatch aborts cleanly).

- **Prefixes (offense/utility):** `Reinforced` (+HP%), `Overclocked`
  (+speed%), `Precision` (+range in cells), `Loaded` (+warhead damage%),
  `Rapid` (−reload%), `Lightened` (−cost%, −HP%)
- **Suffixes (defense/utility):** `of the Bull` (+armor), `of Haste`
  (−build time%), `of Leeching` (repair-on-kill, D2 life-leech),
  `of Warding` (−superweapon damage taken), `of Scavenging` (+salvage drop%)
- **Each affix:** 3–5 tiers × value ranges; the roll within tier uses
  `Rng::next_range` seeded by the item's `Seed`. All values land in
  `Fixed16_16` space (the sim's math domain) at table-build time.
- **Curse affixes** (tradeoff, D2-esque): `brittle` (−10% HP), `thirsty`
  (+10% ammo/fuel cost) — possible on rares; they're what makes a "god roll"
  feel earned.
- **Procedural rare names:** D2's iconic two-part generator —
  `name = prefix_word[seed] + "of " + suffix_word[seed]` rolled from the same
  item seed ("Vorpal Sentinel of the Iron Ledger"). Cheap, memorable,
  screenshot-able.

### Uniques

~5 per faction at launch, hand-authored (no rolls — fixed stats + one
signature quirk, e.g. **"Mammoth Prime": +40% HP, crushes walls, cannot be
repaired**). Drop sources: campaign intel caches (PROGRESSION.md §campaign),
weekly ladder season reward, ultra-rare workshop outcome with a **pity
counter** (guaranteed unique every N crafts — deterministic, fair, and a
retention hook).

## Layer 2 — Where Items Live (online/campaign scope)

- **Meta layer (default, ships first):** items are equipped to a loadout
  pre-match; equipped item effects are applied as `RuleTweaks` to the owning
  player's scoped rules inside the snapshot. Campaign: same, per-mission
  loadout. This is TECHTREE §4a's mechanism with per-instance values.
- **In-match battlefield salvage (phase 2):** wrecks occasionally drop
  **crates**; drop rolls are a pure function —
  `hash_coord(wreck_id, frame, match_seed)` — so both peers agree on what
  dropped (D2 loot beams, lockstep-safe). Crates picked up via a
  `Command::Pickup { unit, crate }` following the `Command::Sabotage`
  determinism contract (`cnc-tiberian-dawn/src/network.rs:190-209`).
  V1 crates contain parts/credits (bookkeeping); full affix items are
  granted at match end from the crate seed (identical for both peers).
- **Ranked:** pinned to stock — no items, no crafting effects (identical
  policy to buffs/tree; `cnc-net-directory` matchmaking marks the queue).

## Layer 3 — The Workshop (economy loop; D2 Cube + Gamble)

- **Salvage in:** destroyed enemy units in any mode yield parts + credits
  (harvested from the telemetry `GameSnapshot` already tracks —
  `projectiles_fired`, units destroyed, `harvested_credits`; credited at
  match end via `cnc-meta`).
- **Workshop recipes:**
  - **Reforge** (chaos orb): re-roll all affixes, same base. The core
    dopamine spin — cheap enough to spam, seeded per craft.
  - **Upgrade rarity** (fuse 3 same-base items → next rarity, new seed).
  - **Socket / Add module** (D2 sockets), enabling runewords.
  - **Targeted reroll** (expensive): keep affixes, re-roll one — the
    "almost god-roll" chase that keeps players grinding one more match.
  - **Caster/Blood/Hitpower-style archetypes:** recipe = base + specific
    catalyst + gem-equivalent ⇒ **guaranteed one affix from a family**
    (e.g. "Blood recipe" always rolls a HP affix) plus randoms — D2's actual
    crafting recipes, directly portable.
- **Requisitions (gamble):** spend credits for an unidentified item of a
  chosen base class; revealed in a workshop "identify" beat with rising
  rarity sfx (`cnc-audio` `AudioMixer` sting; `cnc-sfx` for the reveal
  motif). Gambling is D2's credit sink and ours — it drains the economy the
  harvester loop inflates.

## UI & Presentation (the dopamine delivery layer)

- **D2 color language, instantly readable:** white/blue/yellow/gold item
  names in every list (renderer text color — near-free to implement, iconic).
- **Workshop screen:** new `Screen` variant (`cnc-launcher/src/menu.rs`);
  cube-style 3-slot recipe grid, roll animation, affix lines flipping with
  tick sfx before settling (variable-ratio reveal).
- **Tooltip:** affix lines color-coded (blue/yellow), curse affixes red,
  runeword bonuses green; exact `Fixed16_16`-derived numbers, no hidden math.
- **Match-end salvage report:** crates earned → workshop preview — the
  "open the stash" moment that pulls players into the next session
  (PROGRESSION.md §7 feedback principles).

## Economy Balance & Anti-Abuse

- **Fingerprint handshake:** affix table + item data hashed; mismatch aborts
  pre-match (TECHTREE §integrity mechanism, reused verbatim).
- **Per-node determinism tests:** every affix's RuleTweak application must
  produce identical `LockstepSession::frame_checksum` sequences in a
  two-sim test (same harness as buffs/tree).
- **Pity counter** is deterministic (in profile JSON, `cnc-meta`) — auditable,
  and doubles as a "come back" hook (visible progress toward guaranteed
  unique).
- **No trading at launch** (dupe/RMT risk with local-trust profiles); future
  trading can ride `cnc-net-directory`'s content distribution
  (`torrent.rs` precedent) with server-authoritative escrow, clearly
  modded-labeled, never rated.

## Suggested Build Order

| Phase | Deliverable | Depends on |
|---|---|---|
| 1 | Affix table JSON + `cnc-meta` item model `{base, seed, affixes}` + name generator | `cnc-meta` (PROGRESSION.md P1), `cnc-procgen` prng |
| 2 | Workshop screen: reforge/upgrade/identify + rarity-colored UI | launcher menu, renderer |
| 3 | Salvage economy from match telemetry + pity counter | `cnc-meta`, launcher match-end |
| 4 | Loadout equipping → scoped RuleTweaks in snapshot | TECHTREE P4 RuleTweak table |
| 5 | Fingerprint handshake + determinism test harness per affix | `NetworkConfig`, lockstep tests |
| 6 | Uniques + set items + runewords | content pass |
| 7 | In-match crate drops (`hash_coord`) + `Command::Pickup` | network.rs command contract |
| 8 | Requisitions gambling + trading (future, directory-backed) | net-directory |

## Non-Goals

- No runtime RNG inside the simulation; no pay-for-items; no items in ranked;
  no per-client loot tables (drops must be pure functions of shared seeds);
  no unbounded affix tables (every affix is a code-reviewed RuleTweak with a
  determinism test); no trading until server-authoritative escrow exists.
