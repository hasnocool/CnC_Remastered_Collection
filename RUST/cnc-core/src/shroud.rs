//! Fog of war / shroud, modelled on the original per-cell visibility bits.
//!
//! `TIBERIANDAWN/CELL.H` keeps two bits per cell per house:
//!
//! - `IsMapped`  - the cell has been *seen at least once* (permanent).
//! - `IsVisible` - the cell is *currently* in sight (recomputed as units move).
//!
//! Each is stored as a per-house bit in `IsMappedByPlayerMask` /
//! `IsVisibleByPlayerMask`. This module implements the same split with one
//! bitset per house, so targeting and AI can ask "can house N see cell X?"
//! without re-walking every unit.

use crate::math::Vec2i;

/// Houses supported by the per-house bitmasks.
///
/// The original used a 32-bit mask; C&C1/RA games never exceed eight houses.
pub const MAX_HOUSES: usize = 8;

/// Largest sight radius the original accepts (MAP.CPP Sight_From returns
/// early for sightrange > 10).
pub const MAX_SIGHT_RADIUS: i32 = 10;

/// Current and historical visibility for one map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shroud {
    width: u32,
    height: u32,
    /// Bitset per house: `cells` words of `u64`, row-major.
    visible: Vec<Vec<u64>>,
    /// Bitset per house: cells seen at least once.
    mapped: Vec<Vec<u64>>,
}

impl Shroud {
    /// Build a fully-shrouded map (nothing visible, nothing mapped).
    pub fn new(width: u32, height: u32) -> Self {
        let words = Self::words_for(width, height);
        Self {
            width,
            height,
            visible: vec![vec![0u64; words]; MAX_HOUSES],
            mapped: vec![vec![0u64; words]; MAX_HOUSES],
        }
    }

    /// Build a map where every house has already seen everything.
    ///
    /// Useful for the "no shroud" skirmish option and for tests that are not
    /// about visibility.
    pub fn fully_revealed(width: u32, height: u32) -> Self {
        let mut shroud = Self::new(width, height);
        for house in 0..MAX_HOUSES {
            shroud.mapped[house].fill(u64::MAX);
            shroud.visible[house].fill(u64::MAX);
        }
        shroud
    }

    fn words_for(width: u32, height: u32) -> usize {
        ((u64::from(width) * u64::from(height)).div_ceil(64)) as usize
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn bit_index(&self, cell: Vec2i) -> Option<(usize, u64)> {
        if cell.x < 0 || cell.y < 0 || cell.x as u32 >= self.width || cell.y as u32 >= self.height {
            return None;
        }
        let flat = (cell.y as u64) * u64::from(self.width) + cell.x as u64;
        Some(((flat / 64) as usize, 1u64 << (flat % 64)))
    }

    /// Is `cell` currently within `house`'s sight?
    pub fn is_visible(&self, house: usize, cell: Vec2i) -> bool {
        let Some((word, bit)) = self.bit_index(cell) else {
            return false;
        };
        self.visible
            .get(house)
            .is_some_and(|bits| bits.get(word).is_some_and(|w| w & bit != 0))
    }

    /// Has `house` ever seen `cell`?
    pub fn is_mapped(&self, house: usize, cell: Vec2i) -> bool {
        let Some((word, bit)) = self.bit_index(cell) else {
            return false;
        };
        self.mapped
            .get(house)
            .is_some_and(|bits| bits.get(word).is_some_and(|w| w & bit != 0))
    }

    /// `true` when `house` may act on the cell (visible right now).
    pub fn can_target(&self, house: usize, cell: Vec2i) -> bool {
        self.is_visible(house, cell)
    }

    /// Clear every house's *current* sight, keeping the mapped history.
    ///
    /// Call this at the start of each visibility refresh so units that moved
    /// away stop revealing cells behind them.
    pub fn clear_visible(&mut self) {
        for bits in &mut self.visible {
            bits.fill(0);
        }
    }

    /// Reveal every cell within `radius` of `centre` for `house`.
    ///
    /// The shape is the original diamond, not a square: MAP.CPP builds
    /// RadiusOffset from dist = max/2 + min over (|dx|, |dy|), so a radius-1
    /// sight covers the 8 neighbours but leaves the 3x3 corners dark. Sight
    /// still passes through walls, matching Sight_From, which does no
    /// line-of-sight blocking.
    pub fn reveal(&mut self, house: usize, centre: Vec2i, radius: i32) {
        if house >= MAX_HOUSES {
            return;
        }
        let radius = radius.clamp(0, MAX_SIGHT_RADIUS);
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let (xd, yd) = (dx.abs(), dy.abs());
                // MAP.CPP: dist = larger/2 + smaller.
                let dist = if xd > yd { yd / 2 + xd } else { xd / 2 + yd };
                if dist > radius {
                    continue;
                }
                let cell = Vec2i::new(centre.x + dx, centre.y + dy);
                let Some((word, bit)) = self.bit_index(cell) else {
                    continue;
                };
                self.visible[house][word] |= bit;
                self.mapped[house][word] |= bit;
            }
        }
    }

    /// Count cells currently visible to `house` (diagnostics / tests).
    pub fn visible_count(&self, house: usize) -> u32 {
        self.visible
            .get(house)
            .map(|bits| bits.iter().map(|word| word.count_ones()).sum())
            .unwrap_or(0)
    }

    /// Count cells ever mapped by `house`.
    pub fn mapped_count(&self, house: usize) -> u32 {
        self.mapped
            .get(house)
            .map(|bits| bits.iter().map(|word| word.count_ones()).sum())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(x: i32, y: i32) -> Vec2i {
        Vec2i::new(x, y)
    }

    #[test]
    fn a_new_map_is_fully_shrouded() {
        let shroud = Shroud::new(16, 16);
        assert!(!shroud.is_visible(0, at(5, 5)));
        assert!(!shroud.is_mapped(0, at(5, 5)));
        assert_eq!(shroud.visible_count(0), 0);
        assert_eq!(shroud.mapped_count(0), 0);
    }

    #[test]
    fn reveal_marks_both_visible_and_mapped() {
        let mut shroud = Shroud::new(16, 16);
        shroud.reveal(0, at(5, 5), 0);
        assert!(shroud.is_visible(0, at(5, 5)));
        assert!(shroud.is_mapped(0, at(5, 5)));
        assert_eq!(shroud.visible_count(0), 1);
    }

    #[test]
    fn radius_reveals_a_square_block() {
        let mut shroud = Shroud::new(16, 16);
        shroud.reveal(0, at(5, 5), 1);
        // The 3x3 block around (5,5).
        assert_eq!(shroud.visible_count(0), 9);
        assert!(shroud.is_visible(0, at(4, 4)));
        assert!(shroud.is_visible(0, at(6, 6)));
        assert!(!shroud.is_visible(0, at(7, 7)));
    }

    #[test]
    fn clearing_visibility_keeps_the_mapped_history() {
        let mut shroud = Shroud::new(16, 16);
        shroud.reveal(0, at(5, 5), 0);
        shroud.clear_visible();

        // No longer seen, but still remembered.
        assert!(!shroud.is_visible(0, at(5, 5)));
        assert!(shroud.is_mapped(0, at(5, 5)));
        assert_eq!(shroud.visible_count(0), 0);
        assert_eq!(shroud.mapped_count(0), 1);
    }

    #[test]
    fn visibility_is_per_house() {
        let mut shroud = Shroud::new(16, 16);
        shroud.reveal(0, at(3, 3), 0);
        assert!(shroud.is_visible(0, at(3, 3)));
        assert!(
            !shroud.is_visible(1, at(3, 3)),
            "house 1 has not looked there"
        );
    }

    #[test]
    fn targeting_requires_current_visibility_not_just_memory() {
        let mut shroud = Shroud::new(16, 16);
        shroud.reveal(0, at(2, 2), 0);
        shroud.clear_visible();
        assert!(shroud.is_mapped(0, at(2, 2)));
        assert!(
            !shroud.can_target(0, at(2, 2)),
            "a remembered cell cannot be targeted while unseen"
        );
    }

    #[test]
    fn radius_one_is_the_full_3x3() {
        // MAP.CPP's radius-1 row lists all eight neighbours, diagonals
        // included: (-MCW*1)-1 ... (MCW*1)+1. So radius 1 really is a 3x3.
        let mut shroud = Shroud::new(16, 16);
        shroud.reveal(0, at(8, 8), 1);
        assert_eq!(shroud.visible_count(0), 9, "centre plus 8 neighbours");
        assert!(shroud.is_visible(0, at(9, 9)), "diagonal corner included");
    }

    #[test]
    fn larger_radii_are_diamonds() {
        // The diamond shows from radius 2: MAP.CPP's row omits (2,2) and
        // keeps (2,1), because dist = larger/2 + smaller.
        let mut shroud = Shroud::new(16, 16);
        shroud.reveal(0, at(8, 8), 2);
        assert!(!shroud.is_visible(0, at(10, 10)), "(2,2) is outside");
        assert!(shroud.is_visible(0, at(10, 9)), "(2,1) is inside");
        assert!(shroud.is_visible(0, at(8, 10)), "straight down");
    }

    #[test]
    fn reveal_clips_at_the_map_edge() {
        let mut shroud = Shroud::new(8, 8);
        shroud.reveal(0, at(0, 0), 2);
        // Corner reveal, clipped by two map edges. The original radius-2
        // diamond has 13 cells; five fall off-map from (0,0), leaving 8:
        // (0..2,0), (0..2,1), (0..1,2).
        assert_eq!(shroud.visible_count(0), 8);
        assert!(shroud.is_visible(0, at(2, 0)), "along the diamond tip");
        assert!(
            !shroud.is_visible(0, at(2, 2)),
            "the 3x3 corner is outside the diamond"
        );
        assert!(shroud.is_visible(0, at(0, 0)));
        assert!(!shroud.is_visible(0, at(-1, 0)));
    }

    #[test]
    fn out_of_range_cells_are_never_visible() {
        let shroud = Shroud::new(8, 8);
        assert!(!shroud.is_visible(0, at(-1, 0)));
        assert!(!shroud.is_visible(0, at(999, 0)));
        assert!(!shroud.is_visible(0, at(0, 999)));
    }

    #[test]
    fn fully_revealed_shows_everything() {
        let shroud = Shroud::fully_revealed(16, 16);
        assert!(shroud.is_visible(0, at(0, 0)));
        assert!(shroud.is_visible(7, at(15, 15)));
    }

    #[test]
    fn bitsets_span_multiple_words() {
        // 200 cells needs four 64-bit words.
        let mut shroud = Shroud::new(20, 10);
        shroud.reveal(0, at(19, 9), 0);
        assert!(shroud.is_visible(0, at(19, 9)), "last cell of the map");
        assert_eq!(shroud.visible_count(0), 1);
    }
}
