//! MCV deployment into a Construction Yard.
//!
//! Deployment takes a short unpack time, can be cancelled while packing is in
//! progress and (in the original game) requires a clear footprint. The block
//! map is abstracted as a closure so the same logic serves campaign scenarios
//! and tests.

use cnc_core::{ClassId, Fixed16_16, Vec2};

/// Seconds from "deploy" to a usable Construction Yard.
pub const MCV_UNPACK_SECONDS: i32 = 5;
/// Health of a fresh MCV.
pub const MCV_HEALTH: i32 = 600;
/// Footprint of the deployed Construction Yard (cells).
pub const CONSTRUCTION_YARD_CELLS: i32 = 2;

/// A Mobile Construction Vehicle.
#[derive(Debug, Clone)]
pub struct MCV {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub deployed: bool,
    /// True while the unpack animation runs.
    pub deploying: bool,
    pub unpack_progress: Fixed16_16,
}

impl MCV {
    pub fn new(position: Vec2) -> Self {
        Self {
            class_id: ClassId::VehicleE7,
            position,
            health: Fixed16_16::from_int(MCV_HEALTH),
            max_health: Fixed16_16::from_int(MCV_HEALTH),
            deployed: false,
            deploying: false,
            unpack_progress: Fixed16_16::ZERO,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    /// Start unpacking if the MCV is alive and idle.
    pub fn begin_deploy(&mut self) -> bool {
        if self.deployed || self.deploying || !self.is_alive() {
            return false;
        }
        self.deploying = true;
        self.unpack_progress = Fixed16_16::ZERO;
        true
    }

    /// Abort an in-progress deployment.
    pub fn cancel_deploy(&mut self) -> bool {
        if !self.deploying {
            return false;
        }
        self.deploying = false;
        self.unpack_progress = Fixed16_16::ZERO;
        true
    }

    /// Advance unpacking; returns the Construction Yard position once complete.
    pub fn update(&mut self, delta_time: Fixed16_16) -> Option<Vec2> {
        if !self.deploying || !self.is_alive() {
            return None;
        }
        self.unpack_progress += delta_time;
        if self.unpack_progress >= Fixed16_16::from_int(MCV_UNPACK_SECONDS) {
            self.deploying = false;
            self.deployed = true;
            self.unpack_progress = Fixed16_16::from_int(MCV_UNPACK_SECONDS);
            return Some(self.position);
        }
        None
    }

    /// Instant deploy helper kept for scripts and tests.
    pub fn deploy(&mut self) -> Option<Vec2> {
        if self.deployed || !self.is_alive() {
            return None;
        }
        self.deployed = true;
        self.deploying = false;
        Some(self.position)
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }
}

/// Footprint cells covered by a deployed yard, anchored at its position cell.
pub fn yard_footprint(cell_x: i32, cell_y: i32) -> Vec<(i32, i32)> {
    let mut cells = Vec::with_capacity((CONSTRUCTION_YARD_CELLS * CONSTRUCTION_YARD_CELLS) as usize);
    for y in 0..CONSTRUCTION_YARD_CELLS {
        for x in 0..CONSTRUCTION_YARD_CELLS {
            cells.push((cell_x + x, cell_y + y));
        }
    }
    cells
}

/// Whether a 2x2 deployment is legal; `blocked` reports occupied cells.
pub fn can_deploy_at(cell_x: i32, cell_y: i32, blocked: impl Fn(i32, i32) -> bool) -> bool {
    yard_footprint(cell_x, cell_y)
        .into_iter()
        .all(|(x, y)| !blocked(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f32, y: f32) -> Vec2 {
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y))
    }

    #[test]
    fn deployment_takes_time_and_can_be_cancelled() {
        let mut mcv = MCV::new(point(100.0, 100.0));
        assert!(mcv.begin_deploy());
        assert!(!mcv.begin_deploy(), "already deploying");
        assert!(mcv.update(Fixed16_16::from_int(2)).is_none());

        assert!(mcv.cancel_deploy());
        assert!(!mcv.update(Fixed16_16::from_int(10)).is_some());

        assert!(mcv.begin_deploy());
        let yard = mcv.update(Fixed16_16::from_int(MCV_UNPACK_SECONDS));
        assert_eq!(yard, Some(point(100.0, 100.0)));
        assert!(mcv.deployed);
        assert!(!mcv.deploying);
    }

    #[test]
    fn footprint_and_clearance() {
        let cells = yard_footprint(3, 4);
        assert_eq!(cells.len(), 4);
        assert!(cells.contains(&(3, 4)));
        assert!(cells.contains(&(4, 5)));

        let blocked = |x: i32, y: i32| x == 4 && y == 5;
        assert!(!can_deploy_at(3, 4, blocked));
        assert!(can_deploy_at(10, 10, blocked));
    }

    #[test]
    fn dead_mcv_cannot_deploy() {
        let mut mcv = MCV::new(point(0.0, 0.0));
        mcv.take_damage(Fixed16_16::from_int(10_000));
        assert!(!mcv.begin_deploy());
        assert_eq!(mcv.deploy(), None);
    }
}
