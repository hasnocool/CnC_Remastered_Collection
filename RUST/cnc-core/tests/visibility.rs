//! Integration tests for the canonical visibility/shroud primitive.
//!
//! Covers the contract consumers rely on: idempotent reveal/conceal,
//! order-independent deterministic updates, validated snapshot round-trips
//! through both binary and serde encodings, and bounds safety on every entry
//! point.

use cnc_core::shroud::Shroud;
use cnc_core::visibility::{
    CellState, FogPolicy, SightSource, Visibility, MAX_HOUSES, MAX_SIGHT_RADIUS,
};
use cnc_core::Vec2i;
use proptest::prelude::*;

fn at(x: i32, y: i32) -> Vec2i {
    Vec2i::new(x, y)
}

fn grid(width: u32, height: u32, policy: FogPolicy) -> Visibility {
    Visibility::new(width, height, policy)
}

/// Header is magic(4) + version(4) + width(4) + height(4) + policy(1) + words(4).
const HEADER_LEN: usize = 21;

#[test]
fn reveal_and_conceal_are_idempotent() {
    let mut vis = grid(16, 16, FogPolicy::Recovers);

    assert!(!vis.is_explored(0, at(4, 4)));
    assert!(vis.reveal_cell(0, at(4, 4)), "first reveal changes state");
    assert!(vis.is_visible(0, at(4, 4)));
    assert!(!vis.reveal_cell(0, at(4, 4)), "repeated reveal is a no-op");
    assert_eq!(vis.visible_count(0), 1);
    assert_eq!(vis.explored_count(0), 1);

    assert!(vis.conceal_cell(0, at(4, 4)), "first conceal changes state");
    assert_eq!(vis.state(0, at(4, 4)), CellState::Explored);
    assert!(
        !vis.conceal_cell(0, at(4, 4)),
        "repeated conceal is a no-op"
    );
    assert_eq!(vis.visible_count(0), 0);
    assert_eq!(vis.explored_count(0), 1, "memory survives conceal");

    // Area ops are idempotent too, and never lose explored memory.
    vis.reveal(0, at(8, 8), 2);
    let checksum = vis.checksum();
    vis.reveal(0, at(8, 8), 2);
    assert_eq!(vis.checksum(), checksum);
    vis.conceal(0, at(8, 8), 2);
    vis.conceal(0, at(8, 8), 2);
    assert_eq!(vis.state(0, at(8, 8)), CellState::Explored);
}

#[test]
fn update_is_order_independent_and_deterministic() {
    let sources = vec![
        SightSource::new(0, at(5, 5), 2),
        SightSource::new(0, at(20, 8), 3),
        SightSource::new(1, at(9, 9), 1),
        SightSource::new(1, at(2, 25), 4),
        SightSource::new(3, at(30, 30), 2),
        SightSource::new(0, at(5, 5), 2),                // duplicate
        SightSource::new(MAX_HOUSES as u8, at(1, 1), 2), // out-of-range house
    ];

    let mut reference = grid(40, 40, FogPolicy::Recovers);
    reference.update(&sources);
    let expected = reference.checksum();

    let permutations: Vec<Vec<SightSource>> = vec![
        sources.iter().rev().copied().collect(),
        {
            let mut v = sources.clone();
            v.rotate_left(3);
            v
        },
        {
            let mut v = sources.clone();
            v.sort_by_key(|s| (s.centre.x, s.house, s.radius));
            v
        },
    ];

    for permutation in permutations {
        let mut other = grid(40, 40, FogPolicy::Recovers);
        other.update(&permutation);
        assert_eq!(other, reference, "grid must not depend on source order");
        assert_eq!(other.checksum(), expected);
    }

    // Re-running an update on an already updated grid is stable.
    reference.update(&sources);
    assert_eq!(reference.checksum(), expected);
}

proptest! {
    #[test]
    fn arbitrary_source_order_produces_the_same_grid(
        raw in prop::collection::vec((0u8..12, -50i32..50, -50i32..50, -5i32..15), 0..32)
    ) {
        let sources: Vec<SightSource> = raw
            .iter()
            .map(|&(house, y, x, radius)| SightSource::new(house, at(x, y), radius))
            .collect();
        let mut forward = grid(64, 64, FogPolicy::Recovers);
        forward.update(&sources);

        let mut reverse_sources = sources.clone();
        reverse_sources.reverse();
        let mut backward = grid(64, 64, FogPolicy::Recovers);
        backward.update(&reverse_sources);

        prop_assert_eq!(&forward, &backward);
        prop_assert_eq!(forward.checksum(), backward.checksum());
    }
}

#[test]
fn fog_policies_differ_on_lost_sight() {
    // Permanent (Tiberian Dawn): sight is never taken back by update.
    let mut td = grid(32, 32, FogPolicy::Permanent);
    td.update(&[SightSource::new(0, at(10, 10), 2)]);
    assert!(td.is_visible(0, at(10, 10)));
    td.update(&[]);
    assert!(
        td.is_visible(0, at(10, 10)),
        "permanent reveal keeps terrain visible"
    );
    assert_eq!(td.state(0, at(10, 10)), CellState::Visible);

    // Recovers (Red Alert): fog re-covers, explored memory stays.
    let mut ra = grid(32, 32, FogPolicy::Recovers);
    ra.update(&[SightSource::new(0, at(10, 10), 2)]);
    assert!(ra.is_visible(0, at(10, 10)));
    ra.update(&[]);
    assert!(!ra.is_visible(0, at(10, 10)), "RA fog re-covers");
    assert_eq!(ra.state(0, at(10, 10)), CellState::Explored);
    assert!(ra.is_explored(0, at(10, 10)));
}

#[test]
fn snapshot_round_trips_through_binary_and_serde() {
    let mut vis = grid(37, 23, FogPolicy::Recovers);
    vis.reveal(0, at(3, 4), 3);
    vis.reveal(2, at(30, 20), 5);
    vis.reveal(7, at(0, 0), 1);
    vis.conceal(2, at(30, 20), 1);
    vis.conceal(0, at(3, 4), 2);

    let bytes = vis.to_bytes().expect("snapshot encodes");
    let decoded = Visibility::from_bytes(&bytes).expect("snapshot decodes");
    assert_eq!(decoded, vis);
    assert_eq!(decoded.checksum(), vis.checksum());
    assert_eq!(decoded.width(), 37);
    assert_eq!(decoded.height(), 23);
    assert_eq!(decoded.policy(), FogPolicy::Recovers);

    let json = serde_json::to_string(&vis).expect("json serialize");
    let from_json: Visibility = serde_json::from_str(&json).expect("json deserialize");
    assert_eq!(from_json, vis);

    let bin = bincode::serialize(&vis).expect("bincode serialize");
    let from_bin: Visibility = bincode::deserialize(&bin).expect("bincode deserialize");
    assert_eq!(from_bin, vis);

    // Re-encoding the decoded grid is byte-identical (stable snapshot).
    assert_eq!(decoded.to_bytes().expect("re-encode"), bytes);
}

#[test]
fn empty_and_tiny_maps_round_trip() {
    for (w, h) in [(0u32, 0u32), (0, 7), (7, 0), (1, 1), (64, 1), (65, 1)] {
        let vis = grid(w, h, FogPolicy::Recovers);
        let bytes = vis.to_bytes().expect("encodes");
        let decoded = Visibility::from_bytes(&bytes).expect("decodes");
        assert_eq!(decoded, vis, "grid {w}x{h}");
        assert_eq!(decoded.visible_count(0), 0);
    }
}

#[test]
fn corrupt_snapshots_are_rejected() {
    let mut vis = grid(8, 8, FogPolicy::Recovers);
    vis.reveal(0, at(3, 3), 2);
    let bytes = vis.to_bytes().expect("snapshot encodes");
    let words = 1usize; // 64 cells fit in one u64
    assert_eq!(bytes.len(), HEADER_LEN + MAX_HOUSES * words * 8 * 2);

    let bad_magic = {
        let mut b = bytes.clone();
        b[0] ^= 0xff;
        b
    };
    assert!(Visibility::from_bytes(&bad_magic).is_err());

    let bad_version = {
        let mut b = bytes.clone();
        b[4..8].copy_from_slice(&2u32.to_le_bytes());
        b
    };
    assert!(Visibility::from_bytes(&bad_version).is_err());

    let bad_word_count = {
        let mut b = bytes.clone();
        b[17..21].copy_from_slice(&0u32.to_le_bytes());
        b
    };
    assert!(Visibility::from_bytes(&bad_word_count).is_err());

    let bad_policy = {
        let mut b = bytes.clone();
        b[16] = 0x7f;
        b
    };
    assert!(Visibility::from_bytes(&bad_policy).is_err());

    let truncated = &bytes[..bytes.len() - 1];
    assert!(Visibility::from_bytes(truncated).is_err());

    // Visible bit without the matching explored bit is internally invalid.
    let visible_without_explored = {
        let mut b = bytes.clone();
        let explored_start = HEADER_LEN + MAX_HOUSES * words * 8;
        b[explored_start..explored_start + words * 8].fill(0);
        b
    };
    assert!(Visibility::from_bytes(&visible_without_explored).is_err());

    // Padding bits beyond the last map cell are non-canonical.
    let mut small = grid(10, 10, FogPolicy::Recovers);
    small.reveal(0, at(1, 1), 0);
    let mut padded = small.to_bytes().expect("snapshot encodes");
    let words = 2usize; // 100 cells need two u64 words
    let visible_word = HEADER_LEN + 8;
    let explored_word = HEADER_LEN + MAX_HOUSES * words * 8 + 8;
    for offset in [visible_word, explored_word] {
        padded[offset..offset + 8].copy_from_slice(&(1u64 << 40).to_le_bytes());
    }
    assert!(Visibility::from_bytes(&padded).is_err());

    // An empty slice is rejected rather than panicking.
    assert!(Visibility::from_bytes(&[]).is_err());
}

#[test]
fn out_of_bounds_access_never_panics() {
    let mut vis = grid(10, 10, FogPolicy::Recovers);
    let probes = [
        at(-1, 0),
        at(0, -1),
        at(10, 0),
        at(0, 10),
        at(i32::MIN, i32::MIN),
        at(i32::MAX, i32::MAX),
    ];
    for cell in probes {
        assert_eq!(vis.state(0, cell), CellState::Unseen);
        assert!(!vis.is_visible(0, cell));
        assert!(!vis.is_explored(0, cell));
        assert!(!vis.reveal_cell(0, cell));
        assert!(!vis.conceal_cell(0, cell));
    }
    assert_eq!(vis.checksum(), grid(10, 10, FogPolicy::Recovers).checksum());

    // Out-of-range houses are ignored, not indexing errors.
    for house in [MAX_HOUSES, MAX_HOUSES + 1, 999, usize::MAX] {
        assert_eq!(vis.state(house, at(1, 1)), CellState::Unseen);
        assert_eq!(vis.visible_count(house), 0);
        assert_eq!(vis.explored_count(house), 0);
        assert!(!vis.reveal_cell(house, at(1, 1)));
        vis.reveal(house, at(1, 1), 3);
    }
    assert_eq!(vis.visible_count(0), 0);

    // Radius is clamped, including negative and absurd values.
    vis.reveal(0, at(5, 5), i32::MIN);
    assert!(vis.is_visible(0, at(5, 5)), "negative radius behaves as 0");
    vis.reveal(0, at(5, 5), i32::MAX);
    let mut clamped = grid(10, 10, FogPolicy::Recovers);
    clamped.reveal(0, at(5, 5), MAX_SIGHT_RADIUS);
    assert_eq!(
        vis.visible_count(0),
        clamped.visible_count(0),
        "absurd radii clamp to MAX_SIGHT_RADIUS"
    );

    // Revealing near an edge clips instead of wrapping or panicking.
    let mut edge = grid(4, 4, FogPolicy::Recovers);
    edge.reveal(0, at(0, 0), 2);
    assert!(edge.is_visible(0, at(0, 0)));
    assert!(!edge.is_visible(0, at(-1, 0)));

    // Zero-sized grids answer conservatively.
    let mut empty = grid(0, 0, FogPolicy::Recovers);
    assert_eq!(empty.state(0, at(0, 0)), CellState::Unseen);
    empty.reveal(0, at(0, 0), 2);
    empty.update(&[SightSource::new(0, at(0, 0), 2)]);
    assert_eq!(empty.visible_count(0), 0);
}

#[test]
fn sight_shape_matches_the_original_diamond() {
    let mut vis = grid(32, 32, FogPolicy::Recovers);
    vis.reveal(0, at(8, 8), 1);
    assert_eq!(vis.visible_count(0), 9, "radius 1 covers the 3x3 block");

    let mut wide = grid(32, 32, FogPolicy::Recovers);
    wide.reveal(0, at(16, 16), 2);
    assert!(
        !wide.is_visible(0, at(18, 18)),
        "(2,2) lies outside the diamond"
    );
    assert!(wide.is_visible(0, at(18, 17)), "(2,1) lies inside it");
    assert!(wide.is_visible(0, at(16, 14)), "the vertical tip");
}

#[test]
fn from_shroud_mirrors_an_engine_grid() {
    let mut shroud = Shroud::new(12, 12);
    shroud.reveal(0, at(4, 4), 2);
    shroud.reveal(3, at(9, 9), 1);
    shroud.clear_visible();
    shroud.reveal(3, at(9, 9), 0);

    let vis = Visibility::from_shroud(&shroud, FogPolicy::Recovers);

    // House 0 keeps explored memory but has no current sight.
    assert!(vis.is_explored(0, at(4, 4)));
    assert!(!vis.is_visible(0, at(4, 4)));
    assert_eq!(vis.explored_count(0), shroud.mapped_count(0));

    // House 3 still sees its last cell.
    assert!(vis.is_visible(3, at(9, 9)));
    assert_eq!(vis.visible_count(3), shroud.visible_count(3));
    assert_eq!(vis.visible_count(1), 0);

    // The mirrored grid is a normal primitive: it round-trips.
    let decoded = Visibility::from_bytes(&vis.to_bytes().expect("encodes")).expect("decodes");
    assert_eq!(decoded, vis);
}

#[test]
fn fully_revealed_covers_every_cell() {
    let vis = Visibility::fully_revealed(20, 10, FogPolicy::Recovers);
    let total = 20 * 10;
    for house in 0..MAX_HOUSES {
        assert_eq!(vis.visible_count(house), total);
        assert_eq!(vis.explored_count(house), total);
        assert_eq!(vis.state(house, at(19, 9)), CellState::Visible);
    }
    assert_eq!(vis.state(0, at(20, 9)), CellState::Unseen);
}
