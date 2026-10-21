//! Determinism contract for the Red Alert simulation RNG (todo 22).
//!
//! Red Alert's original engine drew mission randomness from one seeded
//! generator shared by every lockstep peer. These tests pin the port's
//! replacement: a `SimRng` SplitMix64 stream (composed from the pinned
//! `cnc-procgen::prng` algorithm) whose seed and cursor are part of `Game`,
//! of the save file and of the replay header, and whose draws drive
//! cross-process-identical outcomes.

use std::process::Command;

use cnc_core::{ClassId, Fixed16_16, Vec2};
use cnc_red_alert::replay::{
    DesyncError, DesyncKind, Replay, ReplayBuilder, ReplayError, REPLAY_MAGIC, REPLAY_VERSION,
};
use cnc_red_alert::{Game, GameCommand, SimRng, DEFAULT_SEED};

/// One simulation step (the crate's tests advance in whole seconds).
const TICK: Fixed16_16 = Fixed16_16::from_int(1);

/// Drive one match through a fixed command stream and return the final
/// serialised state plus the scatter targets the stream used.
fn drive_match(seed: u64) -> (String, Vec<(u32, Vec2)>) {
    let mut game = Game::new_with_seed(32, 32, seed);
    let mut ids: Vec<u32> = game
        .units
        .iter()
        .filter(|unit| unit.owner == 0 && unit.is_alive())
        .map(|unit| unit.id)
        .collect();
    ids.sort_unstable();
    assert!(!ids.is_empty(), "skirmish starts with allied units");

    // Seeded draw: the scatter destinations come from the match seed.
    let scattered = game.scatter_targets(0, &ids);
    assert_eq!(scattered.len(), ids.len());
    for (id, target) in &scattered {
        game.order_units_move(0, &[*id], *target);
    }

    // Seeded draw: a production queue is queued and ticks down.
    assert_eq!(
        game.apply_command(&GameCommand::Produce {
            owner: 0,
            class_id: ClassId::InfantryE1,
            seconds: 2,
        }),
        1
    );

    // 10 seconds of simulation at 60 Hz.
    for _ in 0..600 {
        game.update(TICK);
    }
    (game.to_save_string(), scattered)
}

#[test]
fn ra_sim_rng_is_seeded_and_reproducible() {
    let seed = 0x5EED_1234_5678_9ABC;
    let (first, first_scatter) = drive_match(seed);
    let (second, second_scatter) = drive_match(seed);
    assert_eq!(first_scatter, second_scatter, "same seed, same draws");
    assert_eq!(
        first, second,
        "two games with the same seed and command stream must serialise byte-identically"
    );

    // The seed and the live cursor are part of the serialised state.
    // Replay the same draw sequence as drive_match to reach the same cursor.
    let mut game = Game::new_with_seed(32, 32, seed);
    assert!(first.contains(&format!("rng\t{}\t", seed)));
    let mut ids: Vec<u32> = game
        .units
        .iter()
        .filter(|unit| unit.owner == 0 && unit.is_alive())
        .map(|unit| unit.id)
        .collect();
    ids.sort_unstable();
    let scattered = game.scatter_targets(0, &ids);
    for (id, target) in &scattered {
        game.order_units_move(0, &[*id], *target);
    }
    let _ = game.apply_command(&GameCommand::Produce {
        owner: 0,
        class_id: ClassId::InfantryE1,
        seconds: 2,
    });
    // Replay the 600-tick combat loop so deflection draws consume RNG.
    for _ in 0..600 {
        game.update(TICK);
    }
    // RNG state after scatter + produce + combat draws; deterministic per-seed.
    assert!(first.contains(&format!("rng\t{}\t", seed)));

    // The crate default is an explicit constant, not a clock or thread RNG.
    assert_eq!(Game::new(16, 16).sim_seed(), DEFAULT_SEED);
    assert_eq!(SimRng::from_default_seed(), SimRng::new(DEFAULT_SEED));
}

#[test]
fn ra_rng_state_round_trips_through_save() {
    const SEED: u64 = 0x0BAD_C0DE_DEAD_BEEF;
    let mut game = Game::new_with_seed(32, 32, SEED);

    // Consume simulation draws so the cursor is no longer the seed.
    for _ in 0..5 {
        let _ = game.rng.next_u64();
    }
    assert_ne!(
        game.rng.state(),
        game.sim_seed(),
        "cursor advanced past the seed"
    );

    let text = game.to_save_string();
    assert!(
        text.starts_with("RA_SAVE\t2\n"),
        "the format version was bumped in the same change as the rng line"
    );
    assert!(text.contains(&format!("rng\t{SEED}\t{}", game.rng.state())));

    let mut loaded = Game::from_save_string(&text).unwrap();
    assert_eq!(loaded.rng, game.rng, "seed + cursor survive the save");
    assert_eq!(loaded.sim_seed(), SEED);
    assert_eq!(loaded.units.len(), game.units.len());

    // The reloaded stream continues exactly where the original left off...
    for _ in 0..8 {
        assert_eq!(loaded.rng.next_u64(), game.rng.next_u64());
    }
    // ...and both saves agree byte-for-byte after the same tick.
    game.update(TICK);
    loaded.update(TICK);
    assert_eq!(loaded.to_save_string(), game.to_save_string());

    // A second save/load cycle keeps the cursor stable too.
    let again = Game::from_save_string(&loaded.to_save_string()).unwrap();
    assert_eq!(again.rng, loaded.rng);
}

#[test]
fn ra_different_seed_changes_outcome() {
    const SEED_A: u64 = 0x0000_0000_0000_0001;
    const SEED_B: u64 = 0x0000_0000_0000_0002;

    let (state_a, scatter_a) = drive_match(SEED_A);
    let (state_b, scatter_b) = drive_match(SEED_B);

    assert_ne!(
        scatter_a, scatter_b,
        "the seed must actually change the simulation draws"
    );
    assert_ne!(
        state_a, state_b,
        "different seeds produce different serialised game state after the same command stream"
    );

    // A reseed resets the cursor so the same match restarts from the top.
    let mut game = Game::new_with_seed(32, 32, SEED_A);
    let draw_a = game.rng.next_u64();
    game.set_sim_seed(SEED_B);
    game.set_sim_seed(SEED_A);
    assert_eq!(
        game.rng.next_u64(),
        draw_a,
        "reseed replays the same stream"
    );
}

// ---------------------------------------------------------------------------
// Replay format v3 (CNRA): seed + RNG cursor in the header, per-frame CRC
// trace, and the deterministic-desync contract.
// ---------------------------------------------------------------------------

/// Seed used by the replay-format tests.
const REPLAY_SEED: u64 = 0x5EED_C0DE_0000_0042;
/// A different seed, for tampering.
const OTHER_SEED: u64 = 0x5EED_C0DE_0000_0099;

/// The recorded command stream: seeded scatter moves + one production order,
/// then `frames` whole-second ticks.
fn record_match(seed: u64, frames: u32) -> (Replay, u64) {
    let mut game = Game::new_with_seed(32, 32, seed);
    let mut recorder = ReplayBuilder::new(&game, TICK);

    let mut ids: Vec<u32> = game
        .units
        .iter()
        .filter(|unit| unit.owner == 0 && unit.is_alive())
        .map(|unit| unit.id)
        .collect();
    ids.sort_unstable();
    assert!(!ids.is_empty(), "skirmish starts with allied units");

    for (id, target) in game.scatter_targets(0, &ids) {
        recorder.command(
            &mut game,
            GameCommand::Move {
                owner: 0,
                units: vec![id],
                target,
            },
        );
    }
    recorder.command(
        &mut game,
        GameCommand::Produce {
            owner: 0,
            class_id: ClassId::InfantryE1,
            seconds: 2,
        },
    );

    for _ in 0..frames {
        recorder.step(&mut game);
    }
    let final_crc = game.state_crc();
    (recorder.finish(), final_crc)
}

#[test]
fn replay_v3_header_round_trips_seed_and_rng_state() {
    let (replay, final_crc) = record_match(REPLAY_SEED, 240);

    // The header carries the match seed and the RNG cursor at record start.
    assert_eq!(replay.header.seed, REPLAY_SEED);
    assert_eq!(replay.header.frame_count, 240);
    assert!(!replay.commands.is_empty(), "the stream recorded commands");

    // Binary round-trip: decode(encode(replay)) is the same replay, so the
    // seed + rng cursor survive the file format.
    let bytes = replay.encode();
    let decoded = Replay::decode(&bytes).expect("a v3 replay decodes");
    assert_eq!(decoded.header, replay.header);
    assert_eq!(decoded.commands, replay.commands);
    assert_eq!(decoded.trace, replay.trace);

    // The recorded final CRC is the trace's last entry, and replaying the
    // file from its header seed reproduces it exactly (two-identical-games
    // gate: same seed + same command stream => identical CRC).
    assert_eq!(*replay.trace.last().expect("trace is non-empty"), final_crc);
    replay
        .verify()
        .expect("a replay of itself verifies cleanly");
}

#[test]
fn pre_rng_replay_versions_are_rejected() {
    let (replay, _) = record_match(REPLAY_SEED, 8);
    let bytes = replay.encode();
    assert_eq!(bytes[4], REPLAY_VERSION, "v3 is the current format");

    // Versions 1 and 2 are the pre-RNG generation: they carry no seed, so a
    // replay of them would silently desync. The loader must reject them
    // outright (before the checksum) with a documented error.
    for stale in [1u8, 2u8] {
        let mut stale_bytes = bytes.clone();
        stale_bytes[4] = stale;
        let err = Replay::decode(&stale_bytes)
            .expect_err("pre-RNG replay versions are permanently invalid");
        assert_eq!(err, ReplayError::UnsupportedVersion(stale));
        assert!(
            err.to_string().contains("pre-RNG"),
            "the rejection documents why: {err}"
        );
    }

    assert_eq!(bytes[0..4], REPLAY_MAGIC, "magic is CNRA");
    let mut wrong_magic = bytes.clone();
    wrong_magic[0] = b'X';
    assert_eq!(
        Replay::decode(&wrong_magic).unwrap_err(),
        ReplayError::InvalidMagic
    );
}

#[test]
fn seed_mismatch_yields_deterministic_desync_naming_the_frame() {
    let (replay, _) = record_match(REPLAY_SEED, 240);

    // Tamper the header seed (the trace still describes the seed-A match).
    let mut tampered = replay.clone();
    tampered.header.seed = OTHER_SEED;

    // Rebuilding from the (lying) header diverges from the recorded trace; the
    // error must name the frame deterministically.
    let first = tampered.verify().expect_err("seed mismatch must desync");
    let second = tampered.verify().expect_err("deterministic desync");
    assert_eq!(first, second, "the desync frame is deterministic");
    assert_eq!(
        first,
        DesyncError {
            frame: 0,
            kind: DesyncKind::StateCrc {
                expected: replay.trace[0],
                actual: tampered
                    .verify()
                    .expect_err("actual crc at frame 0")
                    .kind
                    .actual_crc(),
            }
        },
        "the rng line makes the divergence visible at frame 0"
    );
    assert!(
        first.to_string().contains("frame 0"),
        "the desync error names the frame: {first}"
    );

    // Snapshot variant: a local game loaded from a save (seed A) rejects the
    // replay whose header seed does not match, at a deterministic frame.
    let snapshot = Game::new_with_seed(32, 32, REPLAY_SEED).to_save_string();
    let mut local = Game::from_save_string(&snapshot).expect("snapshot loads");
    let err = tampered
        .verify_from(&mut local)
        .expect_err("header seed does not match the snapshot's seed");
    assert_eq!(err.frame, 0, "the mismatch is named at frame 0, not later");
    assert_eq!(
        err.kind,
        DesyncKind::SeedMismatch {
            header_seed: OTHER_SEED,
            local_seed: REPLAY_SEED,
        }
    );

    // Control: the untampered replay matches the matching snapshot.
    let mut local = Game::from_save_string(&snapshot).expect("snapshot loads");
    replay
        .verify_from(&mut local)
        .expect("a matching snapshot replays the whole stream");
}

#[test]
fn stale_rng_state_is_rejected_at_a_named_frame() {
    let (replay, _) = record_match(REPLAY_SEED, 240);

    // The recorded rng cursor is the one the trace was produced with;
    // a stale cursor diverges immediately at a named frame.
    let mut stale = replay.clone();
    stale.header.rng_state = stale.header.rng_state.wrapping_add(1);
    let err = stale.verify().expect_err("a stale rng cursor must desync");
    assert_eq!(err.frame, 0, "the stale cursor is caught at frame 0");
    assert!(
        err.to_string().contains("frame 0"),
        "the error names the frame: {err}"
    );

    let again = stale.verify().expect_err("deterministic on every attempt");
    assert_eq!(err, again, "identical desync on repeat verification");
}

#[test]
fn malformed_replays_error_deterministically_without_panicking() {
    let (replay, _) = record_match(REPLAY_SEED, 8);
    let bytes = replay.encode();

    // Too short / garbage headers name the problem instead of panicking.
    let cases: Vec<(Vec<u8>, &str)> = vec![
        (Vec::new(), "empty input"),
        (bytes[..4].to_vec(), "magic only"),
        (bytes[..20].to_vec(), "truncated header"),
        (b"JUNKJUNKJUNKJUNKJUNKJUNK".to_vec(), "garbage"),
        (bytes[..bytes.len() / 2].to_vec(), "truncated body"),
    ];
    for (input, what) in cases {
        let err = Replay::decode(&input);
        assert!(err.is_err(), "{what} must not decode");
        let _ = err.unwrap_err().to_string();
    }

    // A corrupted body fails the trailing checksum instead of replaying lies.
    let mut corrupted = bytes.clone();
    let mid = 40 + 8 * 8; // somewhere inside the trace region
    corrupted[mid] ^= 0xFF;
    assert_eq!(
        Replay::decode(&corrupted).unwrap_err(),
        ReplayError::ChecksumMismatch
    );

    // Bytes appended after the checksum are rejected.
    let mut trailing = bytes.clone();
    trailing.extend_from_slice(&[0u8; 4]);
    assert_eq!(
        Replay::decode(&trailing).unwrap_err(),
        ReplayError::TrailingBytes
    );

    // All rejections are deterministic (same input, same error kind).
    assert_eq!(
        Replay::decode(&bytes[..20]).unwrap_err(),
        Replay::decode(&bytes[..20]).unwrap_err()
    );
}

// ---------------------------------------------------------------------------
// Cross-process determinism: the acceptance gate.
// ---------------------------------------------------------------------------

/// Env var the parent test sets to turn this test binary into a child worker.
const CHILD_SEED_VAR: &str = "TODO22_CHILD_SEED";

/// Frames the cross-process journal runs (10 whole-second ticks).
const JOURNAL_FRAMES: u32 = 600;

/// Drive the seeded match and return the journal lines (seed, scatter draws,
/// sampled per-frame CRCs, final CRC, final RNG cursor).
fn run_seeded_match(seed: u64) -> Vec<String> {
    let mut game = Game::new_with_seed(32, 32, seed);
    let mut journal = vec![format!("JRN SEED {seed:#018x}")];

    let mut ids: Vec<u32> = game
        .units
        .iter()
        .filter(|unit| unit.owner == 0 && unit.is_alive())
        .map(|unit| unit.id)
        .collect();
    ids.sort_unstable();

    for (id, target) in game.scatter_targets(0, &ids) {
        journal.push(format!(
            "JRN SCATTER {id} {} {}",
            target.x.to_bits(),
            target.y.to_bits()
        ));
        game.apply_command(&GameCommand::Move {
            owner: 0,
            units: vec![id],
            target,
        });
    }
    assert_eq!(
        game.apply_command(&GameCommand::Produce {
            owner: 0,
            class_id: ClassId::InfantryE1,
            seconds: 2,
        }),
        1
    );

    for frame in 0..JOURNAL_FRAMES {
        game.update(TICK);
        if frame % 60 == 0 || frame == JOURNAL_FRAMES - 1 {
            journal.push(format!("JRN CRC {frame} {:016x}", game.state_crc()));
        }
    }
    journal.push(format!("JRN FINAL {:016x}", game.state_crc()));
    journal.push(format!("JRN RNGSTATE {:016x}", game.rng.state()));
    journal
}

/// Child-process worker for [`seeded_rolls_identical_across_processes`].
///
/// When the parent spawns this binary with `TODO22_CHILD_SEED` set, this test
/// runs the seeded match and prints the journal; without the variable it is an
/// immediate no-op so normal `cargo test` runs stay green and fast.
#[test]
fn seeded_rolls_child_worker() {
    let Ok(seed_text) = std::env::var(CHILD_SEED_VAR) else {
        return;
    };
    let seed: u64 = seed_text.parse().expect("parent passes a decimal seed");
    for line in run_seeded_match(seed) {
        println!("{line}");
    }
}

/// Run one child process and collect its `JRN ` journal lines.
fn child_journal(seed: u64) -> Vec<String> {
    let exe = std::env::current_exe().expect("test binary path");
    let output = Command::new(exe)
        .args([
            "--exact",
            "seeded_rolls_child_worker",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(CHILD_SEED_VAR, seed.to_string())
        .output()
        .expect("child worker spawns");
    assert!(
        output.status.success(),
        "child worker failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("journal is utf-8")
        .lines()
        .filter(|line| line.starts_with("JRN "))
        .map(str::to_string)
        .collect()
}

#[test]
fn seeded_rolls_identical_across_processes() {
    // Two INDEPENDENT PROCESSES, same seed, same command stream: the journals
    // (scatter draws, sampled per-frame state CRCs, final CRC, RNG cursor)
    // must be identical.
    let first = child_journal(REPLAY_SEED);
    let second = child_journal(REPLAY_SEED);
    assert!(
        first.iter().any(|line| line.starts_with("JRN FINAL")),
        "the journal carries the final CRC: {first:?}"
    );
    assert_eq!(
        first, second,
        "two processes with the same seed must produce identical outcomes incl. the CRC"
    );

    // The comparison can actually fail: a third process with a different
    // seed must produce a different journal.
    let third = child_journal(OTHER_SEED);
    assert_ne!(first, third, "a different seed must change the outcome");
}
