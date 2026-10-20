//! Canonical shroud / visibility primitive shared by both engines.
//!
//! Both games keep a per-cell, per-house fog grid (TIBERIANDAWN/CELL.H
//! IsMapped / IsVisible, REDALERT/CELL.CPP per-side shroud). Each engine
//! already has its own grid ported for rendering; this module is the single
//! *observational* primitive that consumers (targeting, AI, GPS, gap
//! generator, replay snapshots) can share:
//!
//! - three explicit per-cell states: CellState::Unseen (never seen),
//!   CellState::Explored (seen before, not in sight now) and
//!   CellState::Visible (in sight right now);
//! - reveal / conceal operations that are idempotent;
//! - a deterministic Visibility::update whose result does not depend on the
//!   order the caller supplies sight sources in, and which uses only integer
//!   arithmetic (no hash-order iteration, no floats, no threads);
//! - a versioned, validated binary snapshot plus serde support, so a grid can
//!   be saved, checksummed and compared across lockstep peers.
//!
//! Two fog policies cover the engine difference: Tiberian Dawn reveals
//! permanently, Red Alert lets fog re-cover cells that leave sight (what the
//! gap generator relies on). The policy changes only what Visibility::update
//! does; reveal/conceal stay explicit operations.
//!
//! Sight shape follows the original diamond from MAP.CPP (dist = larger/2 +
//! smaller) and shares its constants with crate::shroud, so this primitive and
//! the existing render grids cannot drift apart.

use crate::math::Vec2i;
use crate::serialization::{BinaryReader, BinaryWriter, Serializable};
use crate::shroud::Shroud;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

pub use crate::shroud::{MAX_HOUSES, MAX_SIGHT_RADIUS};

/// Version written into (and required from) binary snapshots.
pub const VISIBILITY_SNAPSHOT_VERSION: u32 = 1;

/// Magic at the head of a binary snapshot: the bytes V, I, S, 1.
const SNAPSHOT_MAGIC: u32 = u32::from_le_bytes(*b"VIS1");

/// Per-cell visibility state, ordered from least to most information.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum CellState {
    /// Never seen by this house: black shroud.
    #[default]
    Unseen = 0,
    /// Seen at least once but not in sight now: shaded "explored" terrain.
    Explored = 1,
    /// Currently in sight of this house.
    Visible = 2,
}

impl CellState {
    /// True for both explored and visible cells (terrain memory present).
    pub const fn is_explored(self) -> bool {
        !matches!(self, Self::Unseen)
    }

    /// True only while the cell is in sight.
    pub const fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }

    /// Single-byte code used by binary snapshots.
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Parse a snapshot byte; None for anything outside the three states.
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Unseen),
            1 => Some(Self::Explored),
            2 => Some(Self::Visible),
            _ => None,
        }
    }
}

/// How a game treats cells that leave sight.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum FogPolicy {
    /// Red Alert: visibility is recomputed every update, so fog re-covers
    /// cells that no source covers anymore (explored memory persists).
    #[default]
    Recovers = 0,
    /// Tiberian Dawn: once a cell has been sighted it stays visible.
    Permanent = 1,
}

impl FogPolicy {
    /// Single-byte code used by binary snapshots.
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Parse a snapshot byte; None for unknown policies.
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Recovers),
            1 => Some(Self::Permanent),
            _ => None,
        }
    }
}

/// One observer contributing sight for one house this update.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SightSource {
    /// House that gains the sight (values >= MAX_HOUSES are ignored).
    pub house: u8,
    /// Centre cell in map coordinates; out-of-bounds centres reveal nothing.
    pub centre: Vec2i,
    /// Sight radius in cells, clamped to 0..=MAX_SIGHT_RADIUS.
    pub radius: i32,
}

impl SightSource {
    /// Build a sight source.
    pub const fn new(house: u8, centre: Vec2i, radius: i32) -> Self {
        Self {
            house,
            centre,
            radius,
        }
    }
}

/// Serialisable wire form of a Visibility grid (serde + binary snapshots).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Snapshot {
    width: u32,
    height: u32,
    policy: u8,
    visible: Vec<u64>,
    explored: Vec<u64>,
}

/// Per-house explored/visible grid for one map.
///
/// Storage is row-major u64 bitsets: MAX_HOUSES runs of words words, visible
/// first, then explored. Every accessor is bounds-checked and returns the
/// conservative answer for out-of-range input, so callers may pass raw unit
/// coordinates without panicking.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Snapshot", into = "Snapshot")]
pub struct Visibility {
    width: u32,
    height: u32,
    policy: FogPolicy,
    /// Words per house in visible / explored.
    words: usize,
    visible: Vec<u64>,
    explored: Vec<u64>,
}

impl Visibility {
    /// A fully shrouded grid with the given fog policy.
    pub fn new(width: u32, height: u32, policy: FogPolicy) -> Self {
        let words = words_for(width, height);
        Self {
            width,
            height,
            policy,
            words,
            visible: vec![0; MAX_HOUSES * words],
            explored: vec![0; MAX_HOUSES * words],
        }
    }

    /// A grid where every house already sees every cell.
    ///
    /// This is the "no shroud" skirmish/headless state.
    pub fn fully_revealed(width: u32, height: u32, policy: FogPolicy) -> Self {
        let mut grid = Self::new(width, height, policy);
        let cells = u64::from(width) * u64::from(height);
        let full_words = usize::try_from(cells / 64).unwrap_or(usize::MAX);
        let remainder = (cells % 64) as u32;
        let tail = if remainder == 0 {
            0
        } else {
            (1u64 << remainder) - 1
        };
        for house in 0..MAX_HOUSES {
            let base = house * grid.words;
            for bits in [&mut grid.visible, &mut grid.explored] {
                let slice = &mut bits[base..base + grid.words];
                slice.fill(u64::MAX);
                // Padding bits beyond the last cell are not part of the map;
                // keep them clear so counts and snapshots stay canonical.
                for word in &mut slice[full_words..] {
                    *word = tail;
                }
            }
        }
        grid
    }

    /// Snapshot an engine's existing render grid into this primitive.
    ///
    /// Lets a consumer read the same state the renderer draws without owning
    /// the grid or changing engine code.
    pub fn from_shroud(shroud: &Shroud, policy: FogPolicy) -> Self {
        let mut grid = Self::new(shroud.width(), shroud.height(), policy);
        for house in 0..MAX_HOUSES {
            for y in 0..shroud.height() as i32 {
                for x in 0..shroud.width() as i32 {
                    let cell = Vec2i::new(x, y);
                    if !shroud.is_mapped(house, cell) {
                        continue;
                    }
                    // Mirror the engine grid: mapped cells are explored, and
                    // its visible bits are trusted.
                    grid.set_explored_bit(house, cell);
                    if shroud.is_visible(house, cell) {
                        grid.set_bit(house, cell, true);
                    }
                }
            }
        }
        grid
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn policy(&self) -> FogPolicy {
        self.policy
    }

    /// Change the fog policy without touching any cell state.
    pub fn set_policy(&mut self, policy: FogPolicy) {
        self.policy = policy;
    }

    /// Flat bit position of cell, or None when outside the map.
    fn bit_index(&self, cell: Vec2i) -> Option<usize> {
        if cell.x < 0 || cell.y < 0 || cell.x as u32 >= self.width || cell.y as u32 >= self.height {
            return None;
        }
        let flat = u64::from(cell.y as u32) * u64::from(self.width) + u64::from(cell.x as u32);
        Some(flat as usize)
    }

    fn house_index(&self, house: usize) -> Option<usize> {
        if house >= MAX_HOUSES || self.words == 0 {
            return None;
        }
        Some(house * self.words)
    }

    fn bit(&self, bits: &[u64], house: usize, cell: Vec2i) -> bool {
        let Some(index) = self.bit_index(cell) else {
            return false;
        };
        let Some(base) = self.house_index(house) else {
            return false;
        };
        bits.get(base + index / 64)
            .is_some_and(|word| word & (1u64 << (index % 64)) != 0)
    }

    fn set_bit(&mut self, house: usize, cell: Vec2i, visible: bool) -> bool {
        let Some(index) = self.bit_index(cell) else {
            return false;
        };
        let Some(base) = self.house_index(house) else {
            return false;
        };
        let mask = 1u64 << (index % 64);
        let Some(word) = self.visible.get_mut(base + index / 64) else {
            return false;
        };
        let changed = if visible {
            *word & mask == 0
        } else {
            *word & mask != 0
        };
        if visible {
            *word |= mask;
        } else {
            *word &= !mask;
        }
        changed
    }

    fn set_explored_bit(&mut self, house: usize, cell: Vec2i) {
        let Some(index) = self.bit_index(cell) else {
            return;
        };
        let Some(base) = self.house_index(house) else {
            return;
        };
        let mask = 1u64 << (index % 64);
        if let Some(word) = self.explored.get_mut(base + index / 64) {
            *word |= mask;
        }
    }

    /// Current state of cell for house; out-of-range input is Unseen.
    pub fn state(&self, house: usize, cell: Vec2i) -> CellState {
        if self.bit(&self.visible, house, cell) {
            CellState::Visible
        } else if self.bit(&self.explored, house, cell) {
            CellState::Explored
        } else {
            CellState::Unseen
        }
    }

    /// Is cell in house's sight right now?
    pub fn is_visible(&self, house: usize, cell: Vec2i) -> bool {
        self.bit(&self.visible, house, cell)
    }

    /// Has house ever seen cell (visible or explored)?
    pub fn is_explored(&self, house: usize, cell: Vec2i) -> bool {
        self.bit(&self.explored, house, cell)
    }

    /// Number of cells currently in sight for house.
    pub fn visible_count(&self, house: usize) -> u32 {
        let Some(base) = self.house_index(house) else {
            return 0;
        };
        self.visible[base..base + self.words]
            .iter()
            .map(|word| word.count_ones())
            .sum()
    }

    /// Number of cells ever seen by house.
    pub fn explored_count(&self, house: usize) -> u32 {
        let Some(base) = self.house_index(house) else {
            return 0;
        };
        self.explored[base..base + self.words]
            .iter()
            .map(|word| word.count_ones())
            .sum()
    }

    /// Mark one cell visible (and explored). Idempotent; returns whether the
    /// visible state changed.
    pub fn reveal_cell(&mut self, house: usize, cell: Vec2i) -> bool {
        if !self.bit(&self.explored, house, cell) {
            self.set_explored_bit(house, cell);
        }
        self.set_bit(house, cell, true)
    }

    /// Drop one cell out of sight, keeping it explored. Idempotent; returns
    /// whether the visible state changed.
    pub fn conceal_cell(&mut self, house: usize, cell: Vec2i) -> bool {
        self.set_bit(house, cell, false)
    }

    /// Reveal every cell in radius of centre (the original diamond shape,
    /// clipped to the map and to MAX_SIGHT_RADIUS).
    pub fn reveal(&mut self, house: usize, centre: Vec2i, radius: i32) {
        for cell in self.sight_cells(centre, radius) {
            self.reveal_cell(house, cell);
        }
    }

    /// Hide every cell in radius of centre, keeping explored memory.
    pub fn conceal(&mut self, house: usize, centre: Vec2i, radius: i32) {
        for cell in self.sight_cells(centre, radius) {
            self.conceal_cell(house, cell);
        }
    }

    /// Cells covered by a sight source, in a fixed row-major order and clipped
    /// to the map. Negative radii behave as zero; huge radii clamp to
    /// MAX_SIGHT_RADIUS, matching MAP.CPP Sight_From.
    fn sight_cells(&self, centre: Vec2i, radius: i32) -> Vec<Vec2i> {
        let radius = radius.clamp(0, MAX_SIGHT_RADIUS);
        let mut cells = Vec::new();
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let (xd, yd) = (dx.abs(), dy.abs());
                // MAP.CPP: dist = larger/2 + smaller (integer division).
                let dist = if xd > yd { yd / 2 + xd } else { xd / 2 + yd };
                if dist > radius {
                    continue;
                }
                let cell = Vec2i::new(centre.x.saturating_add(dx), centre.y.saturating_add(dy));
                if self.bit_index(cell).is_some() {
                    cells.push(cell);
                }
            }
        }
        cells
    }

    /// Deterministically recompute sight from sources.
    ///
    /// Sources are applied in a canonical order derived from their own values
    /// (house, then y, then x, then radius), never in caller or hash order, and
    /// duplicates are harmless. Under FogPolicy::Recovers all current sight is
    /// dropped first, so cells no source covers fall back to Explored; under
    /// FogPolicy::Permanent sight is only ever added.
    pub fn update(&mut self, sources: &[SightSource]) {
        let mut ordered = sources.to_vec();
        ordered.sort_by_key(|s| (s.house, s.centre.y, s.centre.x, s.radius));
        if self.policy == FogPolicy::Recovers {
            self.visible.fill(0);
        }
        for source in ordered {
            if usize::from(source.house) >= MAX_HOUSES {
                continue;
            }
            self.reveal(usize::from(source.house), source.centre, source.radius);
        }
    }

    /// Deterministic FNV-1a checksum of every stored bit.
    ///
    /// Two peers whose grids agree produce the same value regardless of the
    /// order sight sources were supplied in.
    pub fn checksum(&self) -> u64 {
        let mut hash = FNV_OFFSET;
        hash = fnv_mix(hash, u64::from(self.width));
        hash = fnv_mix(hash, u64::from(self.height));
        hash = fnv_mix(hash, u64::from(self.policy.to_u8()));
        for word in &self.visible {
            hash = fnv_mix(hash, *word);
        }
        for word in &self.explored {
            hash = fnv_mix(hash, *word);
        }
        hash
    }

    /// Versioned binary snapshot (magic + version + validated word counts).
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut writer = BinaryWriter::new(VISIBILITY_SNAPSHOT_VERSION);
        self.write(&mut writer)?;
        Ok(writer.into_bytes())
    }

    /// Decode a snapshot produced by Visibility::to_bytes, rejecting
    /// truncated, mis-versioned or internally inconsistent data.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut reader = BinaryReader::new(bytes, VISIBILITY_SNAPSHOT_VERSION);
        Self::read(&mut reader)
    }

    /// Build from raw parts, enforcing word-count and subset invariants.
    fn assemble(
        width: u32,
        height: u32,
        policy: FogPolicy,
        visible: Vec<u64>,
        explored: Vec<u64>,
    ) -> Result<Self> {
        let words = words_for(width, height);
        let expected = MAX_HOUSES
            .checked_mul(words)
            .ok_or_else(|| anyhow::anyhow!("visibility snapshot: word count overflow"))?;
        if visible.len() != expected || explored.len() != expected {
            bail!(
                "visibility snapshot: expected {expected} words per mask, got {} visible / {} explored",
                visible.len(),
                explored.len()
            );
        }
        if visible
            .iter()
            .zip(&explored)
            .any(|(seen, known)| seen & !known != 0)
        {
            bail!("visibility snapshot: visible bit set on an unexplored cell");
        }
        let cells = u64::from(width) * u64::from(height);
        let remainder = (cells % 64) as u32;
        if remainder != 0 && words > 0 {
            let padding_mask = !((1u64 << remainder) - 1);
            let last_word = usize::try_from(cells / 64).unwrap_or(usize::MAX);
            for house in 0..MAX_HOUSES {
                let index = house * words + last_word;
                if visible
                    .get(index)
                    .is_some_and(|word| word & padding_mask != 0)
                    || explored
                        .get(index)
                        .is_some_and(|word| word & padding_mask != 0)
                {
                    bail!("visibility snapshot: padding bits beyond the map are set");
                }
            }
        }
        Ok(Self {
            width,
            height,
            policy,
            words,
            visible,
            explored,
        })
    }
}

impl From<Visibility> for Snapshot {
    fn from(value: Visibility) -> Self {
        Self {
            width: value.width,
            height: value.height,
            policy: value.policy.to_u8(),
            visible: value.visible,
            explored: value.explored,
        }
    }
}

impl TryFrom<Snapshot> for Visibility {
    type Error = anyhow::Error;

    fn try_from(value: Snapshot) -> Result<Self> {
        let policy = FogPolicy::from_u8(value.policy).ok_or_else(|| {
            anyhow::anyhow!(
                "visibility snapshot: unknown fog policy 0x{:02x}",
                value.policy
            )
        })?;
        Self::assemble(
            value.width,
            value.height,
            policy,
            value.visible,
            value.explored,
        )
    }
}

impl Serializable for Visibility {
    fn read(reader: &mut BinaryReader) -> Result<Self> {
        let magic = reader.read_u32()?;
        if magic != SNAPSHOT_MAGIC {
            bail!("visibility snapshot: bad magic 0x{magic:08x}");
        }
        let version = reader.read_u32()?;
        if version != VISIBILITY_SNAPSHOT_VERSION {
            bail!(
                "visibility snapshot: unsupported version {version} (want {VISIBILITY_SNAPSHOT_VERSION})"
            );
        }
        let width = reader.read_u32()?;
        let height = reader.read_u32()?;
        let policy_byte = reader.read_u8()?;
        let policy = FogPolicy::from_u8(policy_byte).ok_or_else(|| {
            anyhow::anyhow!("visibility snapshot: unknown fog policy {policy_byte}")
        })?;
        let word_count = reader.read_u32()?;
        let words = words_for(width, height);
        if u32::try_from(words).ok() != Some(word_count) {
            bail!("visibility snapshot: word count {word_count} does not match {words}");
        }
        let total = MAX_HOUSES
            .checked_mul(words)
            .ok_or_else(|| anyhow::anyhow!("visibility snapshot: word count overflow"))?;
        let needed = total
            .checked_mul(8)
            .and_then(|bytes| bytes.checked_mul(2))
            .ok_or_else(|| anyhow::anyhow!("visibility snapshot: byte length overflow"))?;
        if reader.remaining() < needed {
            bail!(
                "visibility snapshot: truncated ({} bytes left, {needed} needed)",
                reader.remaining()
            );
        }
        let mut visible = Vec::with_capacity(total);
        for _ in 0..total {
            visible.push(reader.read_u64()?);
        }
        let mut explored = Vec::with_capacity(total);
        for _ in 0..total {
            explored.push(reader.read_u64()?);
        }
        Self::assemble(width, height, policy, visible, explored)
    }

    fn write(&self, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_u32(SNAPSHOT_MAGIC)?;
        writer.write_u32(VISIBILITY_SNAPSHOT_VERSION)?;
        writer.write_u32(self.width)?;
        writer.write_u32(self.height)?;
        writer.write_u8(self.policy.to_u8())?;
        writer.write_u32(
            u32::try_from(self.words)
                .map_err(|_| anyhow::anyhow!("visibility snapshot: too many words"))?,
        )?;
        for word in &self.visible {
            writer.write_u64(*word)?;
        }
        for word in &self.explored {
            writer.write_u64(*word)?;
        }
        Ok(())
    }
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn fnv_mix(mut hash: u64, value: u64) -> u64 {
    for shift in (0..64).step_by(8) {
        hash ^= (value >> shift) & 0xff;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Words per house needed for a width * height grid.
fn words_for(width: u32, height: u32) -> usize {
    let cells = u64::from(width) * u64::from(height);
    usize::try_from(cells.div_ceil(64)).unwrap_or(usize::MAX)
}
