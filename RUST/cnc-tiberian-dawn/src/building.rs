//! Buildings for Tiberian Dawn

use cnc_core::{ARGB8888, ClassId, Fixed16_16, Rect, Vec2};
use serde::{Deserialize, Serialize};
use cnc_asset::{AssetRegistry, SpriteAtlas};
use cnc_renderer::{Renderer, Sprite};

use crate::td_rules::{ArmorType, StructureType};

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
        }
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }

    pub fn repair(&mut self, amount: Fixed16_16) {
        self.health = (self.health + amount).min(self.max_health);
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
        building
    }

    pub fn structure_type(&self) -> Option<StructureType> {
        StructureType::from_class_id(self.class_id)
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
                    1,
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
        assets.get(&format!("buildings/{:?}.shp", self.class_id)).cloned()
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
        Self { base, production_queue: Vec::new() }
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
        Self { base, stored_credits: 0, capacity }
    }
}