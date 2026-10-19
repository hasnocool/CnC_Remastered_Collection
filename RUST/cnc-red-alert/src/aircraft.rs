//! Red Alert aircraft, helipads and airfields.
//!
//! Aircraft carry a finite ammunition load, must dock to rearm and cannot fire
//! while parked. A helipad holds one aircraft; an airfield holds up to four.

use cnc_core::{ClassId, Fixed16_16, Vec2};

/// Default seconds to rearm one full load.
pub const REARM_SECONDS: i32 = 4;
/// Default ammunition for a helicopter.
pub const DEFAULT_AMMO: u32 = 2;
/// Airfield parking capacity.
pub const AIRFIELD_CAPACITY: usize = 4;

/// A flying unit.
#[derive(Debug, Clone)]
pub struct Aircraft {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub ammo: u32,
    pub max_ammo: u32,
    /// Pad index while docked (None while airborne).
    pub at_helipad: Option<u32>,
    pub rearm_progress: Fixed16_16,
    pub rearm_time: Fixed16_16,
}

impl Aircraft {
    pub fn new(class_id: ClassId, position: Vec2, max_ammo: u32, rearm_seconds: i32) -> Self {
        Self {
            class_id,
            position,
            health: Fixed16_16::from_int(150),
            max_health: Fixed16_16::from_int(150),
            ammo: max_ammo,
            max_ammo,
            at_helipad: None,
            rearm_progress: Fixed16_16::ZERO,
            rearm_time: Fixed16_16::from_int(rearm_seconds.max(1)),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    pub fn docked(&self) -> bool {
        self.at_helipad.is_some()
    }

    /// Fire one shot; returns false when empty, parked or dead.
    pub fn fire(&mut self) -> bool {
        if !self.is_alive() || self.docked() || self.ammo == 0 {
            return false;
        }
        self.ammo -= 1;
        true
    }

    /// Dock at a pad index; starts rearming.
    pub fn dock(&mut self, pad: u32) -> bool {
        if !self.is_alive() || self.docked() {
            return false;
        }
        self.at_helipad = Some(pad);
        self.rearm_progress = Fixed16_16::ZERO;
        true
    }

    /// Leave the pad; only allowed with a full or partially loaded magazine.
    pub fn take_off(&mut self) -> bool {
        if !self.docked() {
            return false;
        }
        self.at_helipad = None;
        true
    }

    /// Advance rearming; returns true on the frame the load is full.
    pub fn update(&mut self, delta_time: Fixed16_16) -> bool {
        if !self.docked() || !self.is_alive() {
            return false;
        }
        if self.ammo >= self.max_ammo {
            return false;
        }
        self.rearm_progress += delta_time;
        if self.rearm_progress >= self.rearm_time {
            self.ammo = self.max_ammo;
            self.rearm_progress = Fixed16_16::ZERO;
            return true;
        }
        false
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }
}

/// A helipad that parks exactly one aircraft.
#[derive(Debug, Clone)]
pub struct Helipad {
    pub position: Vec2,
    pub aircraft: Option<u32>,
}

impl Helipad {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            aircraft: None,
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.aircraft.is_some()
    }

    pub fn dock(&mut self, aircraft: u32) -> bool {
        if self.is_occupied() {
            return false;
        }
        self.aircraft = Some(aircraft);
        true
    }

    pub fn release(&mut self) -> Option<u32> {
        self.aircraft.take()
    }
}

/// An airfield that parks up to [`AIRFIELD_CAPACITY`] aircraft.
#[derive(Debug, Clone)]
pub struct Airfield {
    pub position: Vec2,
    pub aircraft: Vec<u32>,
    pub capacity: usize,
}

impl Airfield {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            aircraft: Vec::new(),
            capacity: AIRFIELD_CAPACITY,
        }
    }

    pub fn is_full(&self) -> bool {
        self.aircraft.len() >= self.capacity
    }

    pub fn dock(&mut self, aircraft: u32) -> bool {
        if self.is_full() {
            return false;
        }
        self.aircraft.push(aircraft);
        true
    }

    pub fn release(&mut self, aircraft: u32) -> bool {
        let before = self.aircraft.len();
        self.aircraft.retain(|candidate| *candidate != aircraft);
        self.aircraft.len() != before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f32, y: f32) -> Vec2 {
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y))
    }

    #[test]
    fn aircraft_burns_ammo_then_rearms_only_while_docked() {
        let mut aircraft = Aircraft::new(
            ClassId::AircraftE1,
            point(0.0, 0.0),
            DEFAULT_AMMO,
            REARM_SECONDS,
        );
        assert!(aircraft.fire());
        assert!(aircraft.fire());
        assert!(!aircraft.fire());
        assert_eq!(aircraft.ammo, 0);

        assert!(!aircraft.update(Fixed16_16::from_int(10)));
        assert!(aircraft.dock(1));
        assert!(!aircraft.fire(), "parked aircraft cannot fire");
        assert!(!aircraft.update(Fixed16_16::from_int(REARM_SECONDS - 1)));
        assert!(aircraft.update(Fixed16_16::ONE));
        assert_eq!(aircraft.ammo, DEFAULT_AMMO);
        assert!(aircraft.take_off());
        assert!(aircraft.fire());
    }

    #[test]
    fn helipad_holds_one_aircraft() {
        let mut pad = Helipad::new(point(0.0, 0.0));
        assert!(pad.dock(7));
        assert!(pad.is_occupied());
        assert!(!pad.dock(8));
        assert_eq!(pad.release(), Some(7));
        assert!(pad.dock(8));
    }

    #[test]
    fn airfield_enforces_capacity() {
        let mut field = Airfield::new(point(0.0, 0.0));
        for index in 0..AIRFIELD_CAPACITY as u32 {
            assert!(field.dock(index));
        }
        assert!(field.is_full());
        assert!(!field.dock(99));
        assert!(field.release(2));
        assert!(field.dock(99));
    }

    #[test]
    fn dead_aircraft_cannot_operate() {
        let mut aircraft = Aircraft::new(
            ClassId::AircraftE2,
            point(0.0, 0.0),
            DEFAULT_AMMO,
            REARM_SECONDS,
        );
        aircraft.take_damage(Fixed16_16::from_int(10_000));
        assert!(!aircraft.is_alive());
        assert!(!aircraft.fire());
        assert!(!aircraft.dock(0));
    }
}
