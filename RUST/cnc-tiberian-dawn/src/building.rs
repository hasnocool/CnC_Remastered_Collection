//! Buildings for Tiberian Dawn

use cnc_asset::{AssetRegistry, SpriteAtlas};
use cnc_core::{ClassId, Fixed16_16, Rect, Vec2, ARGB8888};
use cnc_renderer::{Renderer, Sprite};
use serde::{Deserialize, Serialize};

use crate::td_rules::{ArmorType, StructureType, SABOTAGE_COUNTDOWN_SECONDS, SABOTAGE_DAMAGE};
use crate::weapon::Warhead;

/// Base building structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub class_id: ClassId,
    pub armor: ArmorType,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub owner: u8,
    pub is_powered: bool,
    pub is_selling: bool,
    /// BUILDING.H:95-102 `IsGoingToBlow`: a commando planted vertical C4 here.
    pub is_going_to_blow: bool,
    /// BUILDING.H:124-125 `CountDown`: seconds until the charge fires.
    pub count_down: Fixed16_16,
    /// BUILDING.H:140-142 `WhomToRepay`: unit index of the saboteur, kept for
    /// kill credit once the building burns down.
    pub whom_to_repay: Option<u32>,
    /// BUILDING.H:118-121 `IsCaptured`: an engineer took this structure over.
    pub is_captured: bool,
    /// Seconds until a defensive structure may fire again.
    pub reload: Fixed16_16,
    /// Weapon carried by a defensive structure (None for non-defences).
    pub weapon: Option<crate::weapon::Weapon>,
    /// Turret facing angle in radians for defensive structures with turrets.
    pub turret_facing: f32,
    /// The authored structure type.
    ///
    /// Several TD structures share a ClassId (all defences collapse onto
    /// `AA_Gun`/`TeslaCoil`), so `class_id` alone cannot round-trip. Keeping
    /// the type here preserves the distinction.
    pub structure: StructureType,
}

impl Building {
    pub fn new(class_id: ClassId, position: Vec2, max_health: Fixed16_16, owner: u8) -> Self {
        let armor = StructureType::from_class_id(class_id)
            .map(|structure| structure.stats().armor)
            .unwrap_or(ArmorType::Wood);
        Self {
            class_id,
            armor,
            position,
            health: max_health,
            max_health,
            owner,
            is_powered: false,
            is_selling: false,
            is_going_to_blow: false,
            count_down: Fixed16_16::ZERO,
            whom_to_repay: None,
            is_captured: false,
            reload: Fixed16_16::ZERO,
            weapon: StructureType::from_class_id(class_id)
                .and_then(|s| s.defence_weapon())
                .map(crate::weapon::Weapon::from_type),
            turret_facing: 0.0,
            structure: StructureType::from_class_id(class_id)
                .unwrap_or(StructureType::ConstructionYard),
        }
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }

    pub fn repair(&mut self, amount: Fixed16_16) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// BDATA.CPP "Can this building be captured?" column, i.e.
    /// `BuildingTypeClass::IsCaptureable` (BUILDING.CPP:3873-3885).
    pub fn is_captureable(&self) -> bool {
        self.structure_type()
            .is_some_and(|structure| structure.stats().is_captureable)
    }

    /// INFANTRY.CPP:717-735 Per_Cell_Process `MISSION_SABOTAGE` arm: a commando
    /// just entered the cell carrying vertical C4.
    pub fn plant_c4(&mut self, saboteur: u32) {
        self.is_going_to_blow = true;
        self.count_down = Fixed16_16::from_f32(SABOTAGE_COUNTDOWN_SECONDS);
        self.whom_to_repay = Some(saboteur);
    }

    /// BUILDING.CPP:1229 detonation gate: `IsGoingToBlow && CountDown.Expired()`.
    pub fn demolition_expired(&self) -> bool {
        self.is_going_to_blow && self.count_down <= Fixed16_16::ZERO
    }

    /// Armour-modified C4 damage for this structure.
    ///
    /// BUILDING.CPP:1245-1246 applies `int damage = 5000` through
    /// `Take_Damage(damage, 0, WARHEAD_FIRE, ...)`, so the armour verses apply
    /// exactly like any other fire warhead hit.
    pub fn sabotage_damage(&self, warhead: &Warhead) -> i32 {
        warhead
            .damage(Fixed16_16::from_int(SABOTAGE_DAMAGE), self.armor, 0)
            .int()
    }

    /// BUILDING.CPP:1229-1246 demolition timeout: the planted charge fires.
    /// Returns the damage actually applied (0 when not armed).
    pub fn detonate(&mut self, warhead: &Warhead) -> i32 {
        if !self.demolition_expired() || !self.is_alive() {
            return 0;
        }
        self.count_down = Fixed16_16::ZERO;
        self.is_going_to_blow = false;
        let damage = self.sabotage_damage(warhead);
        self.health = (self.health - Fixed16_16::from_int(damage)).max(Fixed16_16::ZERO);
        damage
    }

    /// INFANTRY.CPP:695-697 Per_Cell_Process `MISSION_CAPTURE` arm:
    /// `tech->Captured(House); Delete_This();`.
    pub fn capture(&mut self, house: u8) {
        self.owner = house;
        self.is_captured = true;
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    /// Spawn a building from the BDATA.CPP structure stats.
    pub fn from_type(structure: StructureType, position: Vec2, owner: u8) -> Self {
        let stats = structure.stats();
        let mut building = Self::new(
            structure.class_id(),
            position,
            Fixed16_16::from_int(stats.strength),
            owner,
        );
        building.armor = stats.armor;
        building.weapon = structure
            .defence_weapon()
            .map(crate::weapon::Weapon::from_type);
        building.structure = structure;
        building
    }

    pub fn structure_type(&self) -> Option<StructureType> {
        Some(self.structure)
    }

    pub fn set_powered(&mut self, powered: bool) {
        self.is_powered = powered;
    }

    pub fn start_selling(&mut self) {
        self.is_selling = true;
    }

    /// Draw this building through any cnc_renderer::Renderer implementation.
    pub fn render(&self, renderer: &mut dyn Renderer) {
        self.render_with(renderer, None);
    }

    pub fn render_with(&self, renderer: &mut dyn Renderer, atlas: Option<&SpriteAtlas>) {
        if let Some(atlas) = atlas {
            if let Some(frame) = atlas.frame(&format!("{:?}", self.class_id)) {
                let tint = if !self.is_alive() {
                    [0.25, 0.25, 0.25, 1.0]
                } else if self.is_powered {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    [0.6, 0.6, 0.6, 1.0]
                };
                let mut sprite = Sprite::new(
                    0, // texture slot 0 = procedural unit atlas
                    self.position,
                    Vec2::new(Fixed16_16::from_int(32), Fixed16_16::from_int(32)),
                );
                sprite.uv_rect = Rect::new(
                    frame.x as i32,
                    frame.y as i32,
                    frame.w as i32,
                    frame.h as i32,
                );
                sprite.color = ARGB8888::new(
                    (tint[3] * 255.0) as u8,
                    (tint[0] * 255.0) as u8,
                    (tint[1] * 255.0) as u8,
                    (tint[2] * 255.0) as u8,
                );
                renderer.draw_sprite(&sprite);
                return;
            }
        }

        let color = if self.is_powered {
            [0.2, 0.6, 1.0, 1.0]
        } else {
            [0.6, 0.6, 0.6, 1.0]
        };
        renderer.draw_rect(
            self.position.x.to_f32(),
            self.position.y.to_f32(),
            32.0,
            32.0,
            color,
        );
    }

    /// Resolve this building's sprite bytes from the asset registry.
    pub fn asset(&self, assets: &AssetRegistry) -> Option<Vec<u8>> {
        assets
            .get(&format!("buildings/{:?}.shp", self.class_id))
            .cloned()
    }
}

/// Factory building
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factory {
    pub base: Building,
    pub production_queue: Vec<u32>,
}

impl Factory {
    pub fn new(base: Building) -> Self {
        Self {
            base,
            production_queue: Vec::new(),
        }
    }
}

/// Power plant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerPlant {
    pub base: Building,
    pub power_output: i32,
}

impl PowerPlant {
    pub fn new(base: Building, power_output: i32) -> Self {
        Self { base, power_output }
    }
}

/// Refinery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refinery {
    pub base: Building,
    pub stored_credits: i32,
    pub capacity: i32,
}

impl Refinery {
    pub fn new(base: Building, capacity: i32) -> Self {
        Self {
            base,
            stored_credits: 0,
            capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enemy_building() -> Building {
        Building::from_type(StructureType::ConstructionYard, Vec2::zero(), 1)
    }

    #[test]
    fn new_buildings_carry_no_sabotage_state() {
        let building = enemy_building();
        assert!(!building.is_going_to_blow);
        assert_eq!(building.count_down, Fixed16_16::ZERO);
        assert_eq!(building.whom_to_repay, None);
        assert!(!building.is_captured);
    }

    #[test]
    fn plant_c4_sets_going_to_blow_and_the_building_h_countdown() {
        let mut building = enemy_building();
        building.plant_c4(7);
        assert!(building.is_going_to_blow);
        assert_eq!(building.count_down, Fixed16_16::from_f32(20.0));
        assert_eq!(building.whom_to_repay, Some(7));
        assert!(!building.demolition_expired());
    }

    #[test]
    fn detonate_applies_warhead_fire_damage_of_5000() {
        let mut building = enemy_building();
        let fire = Warhead::from_type(crate::td_rules::WarheadType::Fire);
        building.plant_c4(0);

        // Half the countdown still leaves the charge armed.
        building.count_down = Fixed16_16::from_f32(10.0);
        assert_eq!(building.detonate(&fire), 0);
        assert!(building.is_alive());

        building.count_down = Fixed16_16::ZERO;
        assert!(building.demolition_expired());
        let damage = building.detonate(&fire);
        assert!(damage >= 399, "C4 must out-damage a 400-strength yard");
        assert!(!building.is_alive());
        assert!(!building.is_going_to_blow, "the charge is spent");
    }

    #[test]
    fn capture_transfers_ownership_and_marks_the_building() {
        let mut building = enemy_building();
        assert!(
            building.is_captureable(),
            "construction yards are captureable"
        );
        building.capture(0);
        assert_eq!(building.owner, 0);
        assert!(building.is_captured);

        let eye = Building::from_type(StructureType::IonCannon, Vec2::zero(), 0);
        assert!(
            !eye.is_captureable(),
            "BDATA.CPP marks the Eye uncaptureable"
        );
    }
}
