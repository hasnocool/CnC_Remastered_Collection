//! Top-level Tiberian Dawn game state.
//!
//! Owns the map, houses, units and buildings, advances a small demo
//! simulation and renders the whole scene through cnc_renderer.

use cnc_asset::SpriteAtlas;
use cnc_core::rules::RulesDatabase;
use cnc_core::{ClassId, Fixed16_16, Vec2, Vec2i};
use cnc_renderer::Renderer;

use crate::ai::AI;
use crate::building::Building;
use crate::house::House;
use crate::house::ProductionItem;
use crate::map::{Map, TILE_SIZE};
use crate::mission::{Action, Mission, MissionOutcome};
use crate::pathfind::find_path;
use crate::td_rules::{
    default_structure_owner, default_unit_owner, leptons_to_px, owner_token_allows,
    prerequisite_token_met, production_seconds, px_to_leptons, tiberium_stage, unit_class_id,
    unit_type_for_class, ProductionKind, StructureType, UnitType, WarheadType, WeaponType,
    EXPLOSION_RANGE_LEPTONS, HARVESTER_STEP_COUNT, HARVESTER_STEP_CREDITS,
};
use crate::unit::Unit;
use crate::weapon::{Projectile, Warhead, Weapon};

/// Short-lived superweapon blast visual.
#[derive(Clone, Copy, Debug)]
pub struct Blast {
    pub position: Vec2,
    pub radius: f32,
    pub ttl: Fixed16_16,
    pub max_ttl: Fixed16_16,
    pub color: [f32; 4],
}

/// Missile launch animation from a structure to the target.
#[derive(Clone, Copy, Debug)]
pub struct Strike {
    pub from: Vec2,
    pub to: Vec2,
    pub progress: Fixed16_16,
    pub duration: Fixed16_16,
}

/// Ion Cannon beam visual.
#[derive(Clone, Copy, Debug)]
pub struct Beam {
    pub position: Vec2,
    pub ttl: Fixed16_16,
    pub max_ttl: Fixed16_16,
    pub width: f32,
}

/// Damage scheduled with a short delay for staggered per-cell blasts.
pub(crate) struct PendingBlast {
    position: Vec2,
    radius: f32,
    damage: i32,
    warhead: WarheadType,
    delay: Fixed16_16,
    color: [f32; 4],
}

/// Snapshot of HUD-relevant game state, owned by the game rather than the UI.
#[derive(Clone, Debug, Default)]
pub struct GameStatus {
    pub mission: String,
    pub units: usize,
    pub buildings: usize,
    pub selected: usize,
    pub gdi_credits: i32,
    pub nod_credits: i32,
    pub production: usize,
    pub gdi_power: i32,
    pub gdi_drain: i32,
    pub nod_power: i32,
    pub nod_drain: i32,
    pub gdi_tiberium: i32,
    pub gdi_capacity: i32,
    pub nod_tiberium: i32,
    pub nod_capacity: i32,
    pub gdi_ion: i32,
    pub gdi_nuke: i32,
    pub nod_ion: i32,
    pub nod_nuke: i32,
    pub ready_buildings: usize,
    /// Campaign objectives shown in the HUD.
    pub objectives_total: usize,
    pub objectives_done: usize,
    /// True once a `win`/`lose` action resolved the mission.
    pub mission_over: bool,
    pub mission_victory: bool,
}

/// A type the player can put into a production queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buildable {
    Unit(UnitType),
    Structure(StructureType),
}

impl Buildable {
    pub const fn ini_name(self) -> &'static str {
        match self {
            Self::Unit(unit) => unit.ini_name(),
            Self::Structure(structure) => structure.ini_name(),
        }
    }

    pub const fn class_id(self) -> ClassId {
        match self {
            Self::Unit(unit) => unit_class_id(unit),
            Self::Structure(structure) => structure.class_id(),
        }
    }
}

/// One row of the production sidebar, including why it is hidden or disabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarOption {
    pub buildable: Buildable,
    /// Factory category (infantry, vehicle, aircraft or building).
    pub category: ProductionKind,
    pub cost: i32,
    /// Rules.ini TechLevel override or TDATA/BDATA fallback.
    pub tech_level: u8,
    /// False when TechLevel or prerequisites keep the row out of the sidebar.
    pub visible: bool,
    /// True when the house currently has enough credits.
    pub affordable: bool,
    /// True when the matching factory (or Construction Yard) is alive.
    pub has_producer: bool,
    /// Unmet Prerequisite= tokens, for tooltips and diagnostics.
    pub missing_prerequisites: Vec<String>,
}

/// The playable game session: simulation state plus rendering entry point.
pub struct Game {
    pub map: Map,
    pub houses: Vec<House>,
    /// House controlled by the local player (mission `Player`, default GDI).
    pub player_owner: u8,
    pub units: Vec<Unit>,
    pub buildings: Vec<Building>,
    pub mission: Mission,
    pub elapsed: Fixed16_16,
    pub patrol_center: Vec2,
    pub patrol_radius: Fixed16_16,
    pub projectiles: Vec<Projectile>,
    /// Total projectiles spawned this session (useful for tests/HUD).
    pub projectiles_fired: u64,
    /// Ticks since the last tiberium growth cycle.
    pub tiberium_tick: u32,
    /// Units completed through production (for HUD/logging).
    pub units_produced: u64,
    /// Total credits delivered by harvesters (for HUD/logging).
    pub harvested_credits: i64,
    /// Completed structures waiting to be placed on the map.
    pub ready_buildings: Vec<(u8, StructureType)>,
    /// Aircraft shots fired (telemetry for tests/HUD).
    pub aircraft_shots: u64,
    /// Aircraft rearm steps completed at helipads.
    pub aircraft_rearmed: u64,
    /// Shots fired with a secondary weapon (telemetry).
    pub secondary_shots: u64,
    /// Active superweapon blasts (visual only).
    pub blasts: Vec<Blast>,
    /// Missile launches in flight (visual; damage is scheduled separately).
    pub strikes: Vec<Strike>,
    /// Ion Cannon beams (visual).
    pub beams: Vec<Beam>,
    /// Optional Rules.ini overrides. An empty database falls back to the
    /// hard-coded TD stats ported from UDATA/BDATA/CONST.CPP.
    pub rules: RulesDatabase,
    pub(crate) pending_blasts: Vec<PendingBlast>,
}

impl Game {
    pub fn new(map_name: &str, width: u32, height: u32) -> Self {
        let mut map = Map::new(map_name.to_string(), width, height);

        // Deterministic tiberium patches so the demo has resource fields.
        for y in 0..height {
            for x in 0..width {
                if ((x / 3) + (y / 3) + x) % 13 == 0 {
                    map.grid
                        .set_tiberium(Vec2i::new(x as i32, y as i32), 160, 1);
                }
            }
        }

        Self {
            map,
            houses: Vec::new(),
            player_owner: 0,
            units: Vec::new(),
            buildings: Vec::new(),
            mission: Mission::new(map_name.to_string()),
            elapsed: Fixed16_16::ZERO,
            patrol_center: Self::cell_center(26, 26),
            patrol_radius: Fixed16_16::from_int(80),
            projectiles: Vec::new(),
            projectiles_fired: 0,
            tiberium_tick: 0,
            units_produced: 0,
            harvested_credits: 0,
            ready_buildings: Vec::new(),
            aircraft_shots: 0,
            aircraft_rearmed: 0,
            secondary_shots: 0,
            blasts: Vec::new(),
            strikes: Vec::new(),
            beams: Vec::new(),
            rules: RulesDatabase::default(),
            pending_blasts: Vec::new(),
        }
    }

    /// Build a game with Rules.ini overrides applied.
    pub fn with_rules(map_name: &str, width: u32, height: u32, rules: RulesDatabase) -> Self {
        let mut game = Self::new(map_name, width, height);
        game.rules = rules;
        game
    }

    /// Replace the active rules database (for hot-reload or mission setup).
    /// Existing units immediately pick up the new weapon/warhead values.
    pub fn set_rules(&mut self, rules: RulesDatabase) {
        self.rules = rules;
        self.refresh_weapon_rules();
    }

    /// Load Rules.ini from disk and apply it to this game.
    pub fn load_rules(&mut self, path: &std::path::Path) -> anyhow::Result<usize> {
        let rules = RulesDatabase::load(path)?;
        let count = rules.type_count();
        self.set_rules(rules);
        Ok(count)
    }

    /// Cost of a unit, honouring a Rules.ini override when present.
    pub fn unit_cost(&self, unit_type: UnitType) -> i32 {
        self.rules
            .type_cost(unit_type.ini_name())
            .unwrap_or_else(|| unit_type.stats().cost)
    }

    /// Cost of a structure, honouring a Rules.ini override when present.
    pub fn building_cost(&self, structure: StructureType) -> i32 {
        self.rules
            .type_cost(structure.ini_name())
            .unwrap_or_else(|| structure.stats().cost)
    }

    /// Maximum health of a unit, honouring a Rules.ini override when present.
    pub fn unit_strength(&self, unit_type: UnitType) -> i32 {
        self.rules
            .type_strength(unit_type.ini_name())
            .unwrap_or_else(|| unit_type.stats().strength)
    }

    /// Maximum health of a structure, honouring a Rules.ini override when present.
    pub fn building_strength(&self, structure: StructureType) -> i32 {
        self.rules
            .type_strength(structure.ini_name())
            .unwrap_or_else(|| structure.stats().strength)
    }

    /// Build a weapon with the current Rules.ini Damage/ROF/Range/Warhead data.
    pub fn make_weapon(&self, weapon_type: WeaponType) -> Weapon {
        Weapon::from_type_with_rules(weapon_type, &self.rules)
    }

    /// Warhead profile for explosions, honouring Rules.ini Verses/Spread.
    pub fn warhead_profile(&self, warhead: WarheadType) -> Warhead {
        match self.rules.warhead(warhead.ini_name()) {
            Some(rule) => Warhead::from_rule(warhead, rule),
            None => Warhead::from_type(warhead),
        }
    }

    /// Rebuild every unit's weapons from the current rules database.
    pub fn refresh_weapon_rules(&mut self) {
        let rules = self.rules.clone();
        for unit in &mut self.units {
            unit.apply_weapon_rules(&rules);
        }
    }

    /// Does the house own a living structure of this type?
    pub fn owns_structure(&self, owner: u8, structure: StructureType) -> bool {
        self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type() == Some(structure)
        })
    }

    /// Every distinct living structure owned by a house, for prerequisites.
    pub fn owned_structures(&self, owner: u8) -> Vec<StructureType> {
        let mut owned = Vec::new();
        for building in &self.buildings {
            if building.owner != owner || !building.is_alive() {
                continue;
            }
            if let Some(structure) = building.structure_type() {
                if !owned.contains(&structure) {
                    owned.push(structure);
                }
            }
        }
        owned
    }

    fn is_gdi(&self, owner: u8) -> bool {
        self.houses
            .get(owner as usize)
            .map_or(false, |house| house.id == ClassId::HouseGDI)
    }

    /// Rules.ini Prerequisite= enforcement (RULES.CPP / TYPE.CPP tech tree).
    ///
    /// A type with no rules entry, or an empty prerequisite list, is always
    /// allowed. Unknown tokens do not pass, matching the original engine.
    pub fn meets_prerequisites(&self, owner: u8, type_name: &str) -> bool {
        let Some(rule) = self.rules.type_rule(type_name) else {
            return true;
        };
        let is_gdi = self.is_gdi(owner);
        let owned = self.owned_structures(owner);
        rule.prerequisite
            .iter()
            .all(|token| prerequisite_token_met(token, is_gdi, &owned))
    }

    /// Human-readable list of unmet prerequisite tokens for UI/diagnostics.
    pub fn missing_prerequisites(&self, owner: u8, type_name: &str) -> Vec<String> {
        let Some(rule) = self.rules.type_rule(type_name) else {
            return Vec::new();
        };
        let is_gdi = self.is_gdi(owner);
        let owned = self.owned_structures(owner);
        rule.prerequisite
            .iter()
            .filter(|token| !prerequisite_token_met(token, is_gdi, &owned))
            .cloned()
            .collect()
    }

    /// Build level of a unit: Rules.ini TechLevel override or TDATA fallback.
    pub fn unit_tech_level(&self, unit_type: UnitType) -> u8 {
        self.rules
            .type_rule(unit_type.ini_name())
            .and_then(|rule| rule.tech_level)
            .map(|level| level.clamp(0, u8::MAX as i32) as u8)
            .unwrap_or(unit_type.stats().tech_level)
    }

    /// Rules.ini `Owner=` gate for a unit, with TD fallback when omitted.
    pub fn unit_owned_by_faction(&self, owner: u8, unit_type: UnitType) -> bool {
        let is_gdi = self.is_gdi(owner);
        match self.rules.type_rule(unit_type.ini_name()) {
            Some(rule) if !rule.owner.is_empty() => owner_token_allows(&rule.owner, is_gdi),
            _ => default_unit_owner(unit_type, is_gdi),
        }
    }

    /// Rules.ini `Owner=` gate for a structure, with TD fallback when omitted.
    pub fn structure_owned_by_faction(&self, owner: u8, structure: StructureType) -> bool {
        let is_gdi = self.is_gdi(owner);
        match self.rules.type_rule(structure.ini_name()) {
            Some(rule) if !rule.owner.is_empty() => owner_token_allows(&rule.owner, is_gdi),
            _ => default_structure_owner(structure, is_gdi),
        }
    }

    /// Build level of a structure: Rules.ini TechLevel or BDATA fallback.
    pub fn building_tech_level(&self, structure: StructureType) -> u8 {
        self.rules
            .type_rule(structure.ini_name())
            .and_then(|rule| rule.tech_level)
            .map(|level| level.clamp(0, u8::MAX as i32) as u8)
            .unwrap_or(structure.stats().tech_level)
    }

    /// HOUSE.CPP Can_Build: the house build level must cover the type level.
    pub fn meets_tech_level(&self, owner: u8, required: u8) -> bool {
        self.houses
            .get(owner as usize)
            .map_or(false, |house| required <= house.tech_level)
    }

    fn cell_center(cx: i32, cy: i32) -> Vec2 {
        Vec2::new(
            Fixed16_16::from_f32(cx as f32 * TILE_SIZE + TILE_SIZE * 0.5),
            Fixed16_16::from_f32(cy as f32 * TILE_SIZE + TILE_SIZE * 0.5),
        )
    }

    fn cell(cx: i32, cy: i32) -> Vec2 {
        Vec2::new(
            Fixed16_16::from_f32(cx as f32 * TILE_SIZE),
            Fixed16_16::from_f32(cy as f32 * TILE_SIZE),
        )
    }

    /// Spawn a small GDI/Nod skirmish so the launcher renders real game state.
    pub fn setup_demo(&mut self) {
        self.houses.clear();
        self.units.clear();
        self.buildings.clear();
        self.projectiles.clear();

        let mut gdi = House::new(ClassId::HouseGDI, "GDI".to_string());
        gdi.add_credits(5000);
        let mut nod = House::new(ClassId::HouseNod, "Nod".to_string());
        nod.add_credits(3000);

        let structures = [
            (StructureType::ConstructionYard, 12, 12, 0u8),
            (StructureType::PowerPlant, 17, 12, 0u8),
            (StructureType::Refinery, 22, 12, 0u8),
            (StructureType::Barracks, 12, 17, 0u8),
            (StructureType::WarFactory, 17, 17, 0u8),
            (StructureType::Helipad, 22, 17, 0u8),
            (StructureType::ConstructionYard, 42, 42, 1u8),
            (StructureType::PowerPlant, 47, 42, 1u8),
            (StructureType::Refinery, 42, 47, 1u8),
        ];

        for (structure, cx, cy, owner) in structures {
            let mut building = Building::from_type(structure, Self::cell(cx, cy), owner);
            building.set_powered(true);
            let id = self.buildings.len() as u32;
            if owner == 0 {
                gdi.add_building(id);
            } else {
                nod.add_building(id);
            }
            self.buildings.push(building);
        }

        for index in 0..12 {
            let angle = index as f32 * 0.55;
            let cx = 26.0 + angle.cos() * 7.0;
            let cy = 26.0 + angle.sin() * 7.0;
            let unit_type = match index {
                4 => UnitType::MammothTank,
                _ if index % 4 == 0 => UnitType::MediumTank,
                _ => UnitType::InfantryE1,
            };
            let position = Vec2::new(
                Fixed16_16::from_f32(cx * TILE_SIZE),
                Fixed16_16::from_f32(cy * TILE_SIZE),
            );
            let mut unit = Unit::new(unit_type, position, 0);
            unit.is_selected = index == 0;
            let id = self.units.len() as u32;
            gdi.add_unit(id);
            self.units.push(unit);
        }

        // A dense field next to the refinery keeps the demo economy active.
        for (x, y) in [(19i32, 22i32), (20, 22), (21, 22), (20, 23), (21, 21)] {
            self.map.grid.set_tiberium(Vec2i::new(x, y), 255, 1);
        }

        // One harvester starts the GDI economy loop.
        let harvester = Unit::new(UnitType::Harvester, Self::cell_center(20, 22), 0);
        let id = self.units.len() as u32;
        gdi.add_unit(id);
        self.units.push(harvester);

        // A patrolling Orca showcases the aircraft layer.
        let orca = Unit::new(UnitType::Orca, Self::cell_center(28, 20), 0);
        let id = self.units.len() as u32;
        gdi.add_unit(id);
        self.units.push(orca);

        // A small Nod patrol so attack orders have a target. The first one is
        // a grenadier so the demo shows arcing projectiles.
        for (index, (cx, cy)) in [(33i32, 24i32), (35, 27), (32, 29)].into_iter().enumerate() {
            let mut unit = Unit::new(UnitType::InfantryE1, Self::cell(cx, cy), 1);
            if index == 0 {
                unit.equip_weapon(Weapon::from_type_with_rules(
                    WeaponType::Grenade,
                    &self.rules,
                ));
            }
            let id = self.units.len() as u32;
            nod.add_unit(id);
            self.units.push(unit);
        }

        self.patrol_center = Self::cell_center(26, 26);
        self.houses.push(gdi);
        self.houses.push(nod);
        self.refresh_house_power();
        self.refresh_weapon_rules();
    }

    /// Recompute each house power from its structures (BDATA.CPP POWER/DRAIN).
    pub fn refresh_house_power(&mut self) {
        for (owner, house) in self.houses.iter_mut().enumerate() {
            let mut generated = 0;
            let mut drain = 0;
            let mut capacity = 0;
            for building in &self.buildings {
                if building.owner as usize != owner || !building.is_alive() {
                    continue;
                }
                if let Some(structure) = building.structure_type() {
                    let stats = structure.stats();
                    generated += stats.power;
                    drain += stats.drain;
                    capacity += stats.capacity;
                }
            }
            house.power = generated;
            house.max_power = drain;
            house.capacity = capacity;
            if house.tiberium > house.capacity {
                house.tiberium = house.capacity;
            }
        }
    }

    fn unit_factory_kind(unit_type: UnitType) -> ProductionKind {
        let stats = unit_type.stats();
        if stats.is_aircraft {
            ProductionKind::Aircraft
        } else if stats.is_infantry {
            ProductionKind::Infantry
        } else {
            ProductionKind::Unit
        }
    }

    fn has_producer(&self, owner: u8, kind: ProductionKind) -> bool {
        self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building
                    .structure_type()
                    .map_or(false, |structure| structure.stats().produces == Some(kind))
        })
    }

    pub fn can_queue_unit(&self, owner: u8, unit_type: UnitType) -> bool {
        self.meets_tech_level(owner, self.unit_tech_level(unit_type))
            && self.meets_prerequisites(owner, unit_type.ini_name())
            && self.has_producer(owner, Self::unit_factory_kind(unit_type))
            && self
                .houses
                .get(owner as usize)
                .map_or(false, |house| house.can_afford(self.unit_cost(unit_type)))
    }

    pub fn can_queue_building(&self, owner: u8, structure: StructureType) -> bool {
        self.meets_tech_level(owner, self.building_tech_level(structure))
            && self.meets_prerequisites(owner, structure.ini_name())
            && self.owns_structure(owner, StructureType::ConstructionYard)
            && self.houses.get(owner as usize).map_or(false, |house| {
                house.can_afford(self.building_cost(structure))
            })
    }

    /// Full production sidebar for a house.
    ///
    /// Entries stay in this list even when hidden so callers can distinguish
    /// "locked by TechLevel/prerequisites" (visible == false) from "shown but
    /// unaffordable or missing a factory" (visible == true).
    pub fn sidebar_options(&self, owner: u8) -> Vec<SidebarOption> {
        let Some(house) = self.houses.get(owner as usize) else {
            return Vec::new();
        };

        let mut options = Vec::new();
        for unit_type in UnitType::ALL {
            let category = Self::unit_factory_kind(unit_type);
            let tech_level = self.unit_tech_level(unit_type);
            let missing = self.missing_prerequisites(owner, unit_type.ini_name());
            let cost = self.unit_cost(unit_type);
            options.push(SidebarOption {
                buildable: Buildable::Unit(unit_type),
                category,
                cost,
                tech_level,
                visible: tech_level <= house.tech_level
                    && missing.is_empty()
                    && self.unit_owned_by_faction(owner, unit_type),
                affordable: house.can_afford(cost),
                has_producer: self.has_producer(owner, category),
                missing_prerequisites: missing,
            });
        }

        let has_yard = self.owns_structure(owner, StructureType::ConstructionYard);
        for structure in StructureType::ALL {
            let tech_level = self.building_tech_level(structure);
            let missing = self.missing_prerequisites(owner, structure.ini_name());
            let cost = self.building_cost(structure);
            options.push(SidebarOption {
                buildable: Buildable::Structure(structure),
                category: ProductionKind::Building,
                cost,
                tech_level,
                visible: tech_level <= house.tech_level
                    && missing.is_empty()
                    && self.structure_owned_by_faction(owner, structure),
                affordable: house.can_afford(cost),
                has_producer: has_yard,
                missing_prerequisites: missing,
            });
        }

        options
    }

    /// Sidebar rows the player should actually see right now.
    pub fn visible_sidebar_options(&self, owner: u8) -> Vec<SidebarOption> {
        self.sidebar_options(owner)
            .into_iter()
            .filter(|option| option.visible)
            .collect()
    }

    /// Deterministic AI production pick driven by the rules registry.
    pub fn ai_production_choice(&self, owner: u8, ai: &AI, roll: u32) -> Option<UnitType> {
        let house = self.houses.get(owner as usize)?;
        let owned = self.owned_structures(owner);
        let weights = ai.production_weights(
            &self.rules,
            house.tech_level,
            house.id == ClassId::HouseGDI,
            &owned,
            house.available_money(),
        );
        AI::choose_weighted(&weights, roll)
    }

    /// Pick and queue an AI unit, returning the type when it was accepted.
    pub fn ai_queue_production(&mut self, owner: u8, ai: &AI, roll: u32) -> Option<UnitType> {
        let choice = self.ai_production_choice(owner, ai, roll)?;
        if self.queue_unit(owner, choice) {
            Some(choice)
        } else {
            None
        }
    }

    /// Queue a unit at the house that owns the matching factory.
    pub fn queue_unit(&mut self, owner: u8, unit_type: UnitType) -> bool {
        let cost = self.unit_cost(unit_type);
        if !self.can_queue_unit(owner, unit_type) {
            return false;
        }

        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if !house.can_afford(cost) {
            return false;
        }

        // TECHNO.CPP Time_To_Build uses the house power fraction.
        let seconds = production_seconds(cost, house.power, house.max_power);
        if !house.spend_credits(cost) {
            return false;
        }
        house.production_queue.push_back(ProductionItem {
            class_id: unit_class_id(unit_type),
            progress: Fixed16_16::ZERO,
            total_time: Fixed16_16::from_f32(seconds),
            cost,
        });
        true
    }

    /// Cancel the front production item and refund its cost.
    pub fn cancel_production(&mut self, owner: u8) -> Option<i32> {
        let house = self.houses.get_mut(owner as usize)?;
        let item = house.production_queue.pop_front()?;
        house.add_credits(item.cost);
        Some(item.cost)
    }

    /// Repair a damaged building to full health, paying for the missing health.
    pub fn repair_building(&mut self, owner: u8, index: usize) -> Option<i32> {
        let building = self.buildings.get(index)?;
        if building.owner != owner || !building.is_alive() || building.health >= building.max_health {
            return None;
        }
        let structure = building.structure_type()?;
        let missing = (building.max_health - building.health).to_f32()
            / building.max_health.to_f32().max(1.0);
        let cost = ((self.building_cost(structure) as f32 * missing).ceil() as i32).max(1);
        let house = self.houses.get_mut(owner as usize)?;
        if !house.spend_credits(cost) {
            return None;
        }
        if let Some(building) = self.buildings.get_mut(index) {
            building.health = building.max_health;
        }
        Some(cost)
    }

    /// Sell a placed building: refund half its cost and mark it destroyed.
    pub fn sell_building(&mut self, owner: u8, index: usize) -> Option<i32> {
        let building = self.buildings.get(index)?;
        if building.owner != owner || !building.is_alive() {
            return None;
        }
        let structure = building.structure_type()?;
        let refund = self.building_cost(structure) / 2;
        {
            let building = &mut self.buildings[index];
            building.start_selling();
            building.health = Fixed16_16::ZERO;
        }
        if let Some(house) = self.houses.get_mut(owner as usize) {
            house.add_credits(refund);
            house.remove_building(index as u32);
        }
        Some(refund)
    }

    /// Queue a structure at the Construction Yard.
    pub fn queue_building(&mut self, owner: u8, structure: StructureType) -> bool {
        let cost = self.building_cost(structure);
        if !self.can_queue_building(owner, structure) {
            return false;
        }

        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if !house.can_afford(cost) {
            return false;
        }
        let seconds = production_seconds(cost, house.power, house.max_power);
        if !house.spend_credits(cost) {
            return false;
        }
        house.production_queue.push_back(ProductionItem {
            class_id: structure.class_id(),
            progress: Fixed16_16::ZERO,
            total_time: Fixed16_16::from_f32(seconds),
            cost,
        });
        true
    }

    pub fn ready_building_for(&self, owner: u8) -> Option<StructureType> {
        self.ready_buildings
            .iter()
            .find(|(building_owner, _)| *building_owner == owner)
            .map(|(_, structure)| *structure)
    }

    /// BDATA.CPP Legal_Placement: every foundation cell must be buildable and
    /// free of objects. Rust structures are one cell.
    pub fn can_place_building(&self, cell: Vec2i) -> bool {
        if !self.map.grid.in_bounds(cell.x, cell.y) {
            return false;
        }
        let index = (cell.y as u32 * self.map.grid.width + cell.x as u32) as usize;
        let terrain = &self.map.grid.cells[index];
        if !terrain.is_buildable || terrain.has_tiberium {
            return false;
        }
        if self
            .buildings
            .iter()
            .any(|building| building.is_alive() && Self::building_cell(building) == cell)
        {
            return false;
        }
        if self
            .units
            .iter()
            .any(|unit| unit.is_alive() && Self::unit_cell(unit) == cell)
        {
            return false;
        }
        true
    }

    pub fn place_ready_building(&mut self, owner: u8, cell: Vec2i) -> bool {
        let Some(index) = self
            .ready_buildings
            .iter()
            .position(|(building_owner, _)| *building_owner == owner)
        else {
            return false;
        };
        if !self.can_place_building(cell) {
            return false;
        }

        let (_, structure) = self.ready_buildings.remove(index);
        let mut building = Building::from_type(structure, Self::cell_center(cell.x, cell.y), owner);
        let strength = self.building_strength(structure);
        let max_health = Fixed16_16::from_int(strength);
        building.health = max_health;
        building.max_health = max_health;
        building.set_powered(owner == 0);
        let id = self.buildings.len() as u32;
        self.buildings.push(building);
        if let Some(house) = self.houses.get_mut(owner as usize) {
            house.add_building(id);
        }
        self.refresh_house_power();
        true
    }

    fn update_superweapons(&mut self, delta_time: Fixed16_16) {
        for (owner, house) in self.houses.iter_mut().enumerate() {
            let ion_ready = self.buildings.iter().any(|building| {
                building.owner as usize == owner
                    && building.is_alive()
                    && building.structure_type() == Some(StructureType::IonCannon)
            });
            let temple_ready = self.buildings.iter().any(|building| {
                building.owner as usize == owner
                    && building.is_alive()
                    && building.structure_type() == Some(StructureType::TempleOfNod)
            });
            house.charge_superweapons(ion_ready, temple_ready, delta_time);
        }
    }

    fn update_blasts(&mut self, delta_time: Fixed16_16) {
        let mut index = 0;
        while index < self.pending_blasts.len() {
            if self.pending_blasts[index].delay <= Fixed16_16::ZERO {
                let pending = self.pending_blasts.remove(index);
                self.apply_blast(
                    pending.position,
                    pending.radius,
                    pending.damage,
                    pending.warhead,
                );
                self.blasts.push(Blast {
                    position: pending.position,
                    radius: pending.radius,
                    ttl: Fixed16_16::from_f32(0.5),
                    max_ttl: Fixed16_16::from_f32(0.5),
                    color: pending.color,
                });
            } else {
                self.pending_blasts[index].delay =
                    (self.pending_blasts[index].delay - delta_time).max(Fixed16_16::ZERO);
                index += 1;
            }
        }

        for blast in &mut self.blasts {
            blast.ttl = (blast.ttl - delta_time).max(Fixed16_16::ZERO);
        }
        self.blasts.retain(|blast| blast.ttl > Fixed16_16::ZERO);
    }

    fn update_strikes(&mut self, delta_time: Fixed16_16) {
        for strike in &mut self.strikes {
            if strike.duration > Fixed16_16::ZERO {
                strike.progress =
                    (strike.progress + delta_time / strike.duration).min(Fixed16_16::ONE);
            } else {
                strike.progress = Fixed16_16::ONE;
            }
        }
        self.strikes
            .retain(|strike| strike.progress < Fixed16_16::ONE);

        for beam in &mut self.beams {
            beam.ttl = (beam.ttl - delta_time).max(Fixed16_16::ZERO);
        }
        self.beams.retain(|beam| beam.ttl > Fixed16_16::ZERO);
    }

    /// ANIM.CPP ANIM_ION_CANNON: 600 damage with WARHEAD_PB at the centre.
    pub fn fire_ion_cannon(&mut self, owner: u8, cell: Vec2i) -> bool {
        let has_building = self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type() == Some(StructureType::IonCannon)
        });
        if !has_building {
            return false;
        }
        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if house.ion_charge < Fixed16_16::ONE {
            return false;
        }

        house.ion_charge = Fixed16_16::ZERO;
        let center = Self::cell_center(cell.x, cell.y);
        self.apply_blast(
            center,
            leptons_to_px(EXPLOSION_RANGE_LEPTONS),
            600,
            WarheadType::Pb,
        );
        self.beams.push(Beam {
            position: center,
            ttl: Fixed16_16::from_f32(0.5),
            max_ttl: Fixed16_16::from_f32(0.5),
            width: 26.0,
        });
        self.blasts.push(Blast {
            position: center,
            radius: leptons_to_px(EXPLOSION_RANGE_LEPTONS),
            ttl: Fixed16_16::from_f32(0.6),
            max_ttl: Fixed16_16::from_f32(0.6),
            color: [0.60, 0.85, 1.00, 1.0],
        });
        true
    }

    /// ANIM.CPP ANIM_ATOM_BLAST: 4-cell radius, 1000 raw damage, WARHEAD_FIRE.
    pub fn fire_nuke(&mut self, owner: u8, cell: Vec2i) -> bool {
        let has_temple = self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type() == Some(StructureType::TempleOfNod)
        });
        if !has_temple {
            return false;
        }
        let launch_site = self
            .buildings
            .iter()
            .find(|building| {
                building.owner == owner
                    && building.is_alive()
                    && building.structure_type() == Some(StructureType::TempleOfNod)
            })
            .map(|building| {
                building.position + Vec2::new(Fixed16_16::from_int(12), Fixed16_16::from_int(12))
            })
            .unwrap_or_else(|| Self::cell_center(cell.x, cell.y));

        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if house.nuke_charge < Fixed16_16::ONE {
            return false;
        }

        house.nuke_charge = Fixed16_16::ZERO;
        let center = Self::cell_center(cell.x, cell.y);
        let radius = TILE_SIZE * 4.0;

        // Missile launch: the warhead travels from the Temple to the target.
        let dx = center.x.to_f32() - launch_site.x.to_f32();
        let dy = center.y.to_f32() - launch_site.y.to_f32();
        let distance = (dx * dx + dy * dy).sqrt();
        let flight_seconds = (distance / 150.0).clamp(1.0, 4.0);
        let duration = Fixed16_16::from_f32(flight_seconds);
        self.strikes.push(Strike {
            from: launch_site,
            to: center,
            progress: Fixed16_16::ZERO,
            duration,
        });
        self.blasts.push(Blast {
            position: launch_site,
            radius: 14.0,
            ttl: Fixed16_16::from_f32(0.4),
            max_ttl: Fixed16_16::from_f32(0.4),
            color: [1.0, 0.70, 0.30, 1.0],
        });

        // ANIM.CPP ANIM_ATOM_BLAST: 9x9 cells, 1000 / ((4/2)+1) = 333 damage
        // per cell with WARHEAD_FIRE and Explosion_Damage around each cell.
        // The damage is staggered outward over ~0.15 s like the animation.
        let blast_radius = leptons_to_px(EXPLOSION_RANGE_LEPTONS);
        let per_cell_damage = 1000 / ((4 / 2) + 1);
        for dy in -4..=4 {
            for dx in -4..=4 {
                let target = Vec2i::new(cell.x + dx, cell.y + dy);
                if !self.map.grid.in_bounds(target.x, target.y) {
                    continue;
                }
                let cell_center = Self::cell_center(target.x, target.y);
                let delay = flight_seconds + ((dx.abs() + dy.abs()) as f32 * 0.015).min(0.15);
                self.pending_blasts.push(PendingBlast {
                    position: cell_center,
                    radius: blast_radius,
                    damage: per_cell_damage,
                    warhead: WarheadType::Fire,
                    delay: Fixed16_16::from_f32(delay),
                    color: [1.0, 0.55, 0.2, 1.0],
                });
            }
        }

        self.blasts.push(Blast {
            position: center,
            radius,
            ttl: Fixed16_16::from_f32(1.2),
            max_ttl: Fixed16_16::from_f32(1.2),
            color: [1.0, 0.60, 0.25, 1.0],
        });
        true
    }

    fn apply_blast(&mut self, center: Vec2, radius: f32, damage: i32, warhead: WarheadType) {
        let warhead = self.warhead_profile(warhead);
        for unit in &mut self.units {
            if !unit.is_alive() {
                continue;
            }
            let distance = unit.position.distance(center).to_f32();
            if distance > radius {
                continue;
            }
            let modified = warhead
                .damage(
                    Fixed16_16::from_int(damage),
                    unit.armor,
                    px_to_leptons(distance),
                )
                .int();
            unit.take_damage(Fixed16_16::from_int(modified));
        }

        for building in &mut self.buildings {
            if !building.is_alive() {
                continue;
            }
            let center_position =
                building.position + Vec2::new(Fixed16_16::from_int(12), Fixed16_16::from_int(12));
            let distance = center_position.distance(center).to_f32();
            if distance > radius {
                continue;
            }
            let modified = warhead
                .damage(
                    Fixed16_16::from_int(damage),
                    building.armor,
                    px_to_leptons(distance),
                )
                .int();
            building.health =
                (building.health - Fixed16_16::from_int(modified)).max(Fixed16_16::ZERO);
        }
    }

    /// Advance production queues and spawn completed units at a factory.
    pub fn update_production(&mut self, delta_time: Fixed16_16) {
        let mut completed = Vec::new();
        for (owner, house) in self.houses.iter_mut().enumerate() {
            if let Some(item) = house.update_production(delta_time) {
                completed.push((owner as u8, item));
            }
        }

        for (owner, item) in completed {
            if let Some(structure) = crate::td_rules::StructureType::from_class_id(item.class_id) {
                self.ready_buildings.push((owner, structure));
                continue;
            }
            let Some(unit_type) = unit_type_for_class(item.class_id) else {
                continue;
            };
            let position = self.spawn_position(owner);
            let mut unit = Unit::new(unit_type, position, owner);
            unit.apply_weapon_rules(&self.rules);
            let strength = self.unit_strength(unit_type);
            let max_health = Fixed16_16::from_int(strength);
            unit.health = max_health;
            unit.max_health = max_health;
            let id = self.units.len() as u32;
            if let Some(house) = self.houses.get_mut(owner as usize) {
                house.add_unit(id);
            }
            self.units.push(unit);
            self.units_produced += 1;
        }
    }

    fn spawn_position(&self, owner: u8) -> Vec2 {
        let factory = self.buildings.iter().find(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type().map_or(false, |structure| {
                    matches!(
                        structure.stats().produces,
                        Some(ProductionKind::Infantry)
                            | Some(ProductionKind::Unit)
                            | Some(ProductionKind::Aircraft)
                    )
                })
        });

        let (cell_x, cell_y) = match factory {
            Some(building) => {
                let cell = Self::building_cell(building);
                (cell.x, cell.y)
            }
            None => (self.map.width as i32 / 2, self.map.height as i32 / 2),
        };

        let used: Vec<(i32, i32)> = self
            .occupied_cells(None)
            .iter()
            .map(|cell| (cell.x, cell.y))
            .collect();
        let (x, y) = self.nearby_cell(cell_x + 2, cell_y + 2, &used);
        Self::cell_center(x, y)
    }

    /// Advance the demo simulation.
    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.elapsed += delta_time;

        for unit in &mut self.units {
            if unit.reload > Fixed16_16::ZERO {
                unit.reload = (unit.reload - delta_time).max(Fixed16_16::ZERO);
            }
        }

        self.update_production(delta_time);
        self.update_superweapons(delta_time);
        self.update_blasts(delta_time);
        self.update_strikes(delta_time);
        self.update_harvesters(delta_time);
        self.update_aircraft(delta_time);
        self.update_auto_acquire();
        self.update_facings(delta_time);
        self.update_attacks();

        let t = self.elapsed.to_f32();
        let center_x = self.patrol_center.x.to_f32();
        let center_y = self.patrol_center.y.to_f32();
        let radius = self.patrol_radius.to_f32();
        let player_owner = self.player_owner;

        for (index, unit) in self.units.iter_mut().enumerate() {
            // Player move orders take priority; ordered units never go back
            // to the idle patrol, so they hold their destination.
            if unit.ordered {
                unit.advance_order(delta_time);
                unit.reserved_cell = unit
                    .path
                    .get(unit.path_index)
                    .map(|point| Self::world_cell(*point));
                if unit.is_aircraft {
                    unit.rotor_phase =
                        (unit.rotor_phase + delta_time.to_f32() * 25.0) % std::f32::consts::TAU;
                } else if !unit.is_infantry() {
                    unit.track_phase = (unit.track_phase
                        + delta_time.to_f32() * unit.speed.to_f32() * 0.2)
                        % std::f32::consts::TAU;
                }
                continue;
            }
            // Harvesters run their own mission; only idle player combat units
            // drift on the patrol ring. Enemy units hold their spawn positions.
            if unit.is_harvester || unit.is_aircraft || unit.owner != player_owner {
                continue;
            }
            let phase = t * 0.6 + index as f32 * 0.55;
            unit.position = Vec2::new(
                Fixed16_16::from_f32(center_x + phase.cos() * radius),
                Fixed16_16::from_f32(center_y + phase.sin() * radius),
            );
        }

        self.update_projectiles(delta_time);
        self.update_mission(delta_time);

        // MAP.CPP grows/spreads tiberium over time rather than every tick.
        self.tiberium_tick = self.tiberium_tick.wrapping_add(1);
        if self.tiberium_tick % 30 == 0 {
            self.map.grid.tiberium_cycle();
        }
    }

    /// Attach a campaign mission and apply its scenario settings.
    pub fn start_mission(&mut self, mission: Mission) {
        self.player_owner = mission.player;
        if let Some(credits) = mission.starting_credits {
            if let Some(house) = self.houses.get_mut(mission.player as usize) {
                house.credits = credits;
            }
        }
        if let Some(tech) = mission.tech_level {
            for house in &mut self.houses {
                house.tech_level = tech;
            }
        }
        self.mission = mission;
    }

    /// Load a mission INI file and start it.
    pub fn load_mission(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        let mission = Mission::from_file(path)?;
        self.start_mission(mission);
        Ok(())
    }

    /// Evaluate campaign triggers and execute their actions.
    ///
    /// Conditions are level-triggered and checked every simulation tick; the
    /// trigger list is snapshotted before any action runs so a trigger cannot
    /// observe a half-applied action list. Actions that end the mission stop
    /// the remaining due triggers for this tick.
    pub fn update_mission(&mut self, _delta_time: Fixed16_16) {
        if self.mission.is_over() {
            return;
        }
        let due = self.mission.due(self);
        for id in due {
            let actions = self.mission.fire(id);
            for action in actions {
                self.apply_mission_action(&action);
                if self.mission.is_over() {
                    return;
                }
            }
        }
    }

    fn apply_mission_action(&mut self, action: &Action) {
        match action {
            Action::Win => self.mission.finish(MissionOutcome::Victory),
            Action::Lose => self.mission.finish(MissionOutcome::Defeat),
            Action::Credits { owner, amount } => {
                if let Some(house) = self.houses.get_mut(*owner as usize) {
                    house.add_credits(*amount);
                }
            }
            Action::SpawnUnits {
                owner,
                unit_type,
                count,
                cell,
            } => {
                for index in 0..*count {
                    // Spread reinforcements over a small block so they do not
                    // all occupy exactly the same cell.
                    let x = cell.x + (index % 3) as i32;
                    let y = cell.y + (index / 3) as i32;
                    let unit = Unit::new(*unit_type, Self::cell_center(x, y), *owner);
                    let entity_id = self.units.len() as u32;
                    if let Some(house) = self.houses.get_mut(*owner as usize) {
                        house.add_unit(entity_id);
                    }
                    self.units.push(unit);
                }
            }
            Action::SpawnStructure {
                owner,
                structure,
                cell,
            } => {
                let mut building = Building::from_type(
                    *structure,
                    Self::cell_center(cell.x, cell.y),
                    *owner,
                );
                building.set_powered(true);
                let entity_id = self.buildings.len() as u32;
                if let Some(house) = self.houses.get_mut(*owner as usize) {
                    house.add_building(entity_id);
                }
                self.buildings.push(building);
                self.refresh_house_power();
            }
            Action::Message(text) => self.mission.messages.push(text.clone()),
            Action::SetFlag(name) => {
                self.mission.flags.insert(name.clone());
            }
            Action::EnableTrigger(id) => self.mission.enable(*id),
            Action::DisableTrigger(id) => self.mission.disable(*id),
            Action::CompleteObjective(id) => self.mission.complete_objective(*id),
            Action::RunScript(name) => {
                if let Some(script) = self.mission.script(name).cloned() {
                    for action in &script.actions {
                        self.apply_mission_action(action);
                        if self.mission.is_over() {
                            return;
                        }
                    }
                }
            }
        }
    }

    /// UNIT.CPP Mission_Harvest: harvest tiberium, then dock at a refinery.
    fn update_harvesters(&mut self, delta_time: Fixed16_16) {
        for index in 0..self.units.len() {
            if !self.units[index].is_harvester || !self.units[index].is_alive() {
                continue;
            }

            if self.units[index].harvest_timer > Fixed16_16::ZERO {
                self.units[index].harvest_timer =
                    (self.units[index].harvest_timer - delta_time).max(Fixed16_16::ZERO);
            }

            let owner = self.units[index].owner;
            let position = self.units[index].position;
            let cell = Self::unit_cell(&self.units[index]);
            if self.units[index].tiberium >= HARVESTER_STEP_COUNT {
                self.units[index].unloading = true;
            }
            let unloading = self.units[index].unloading;
            let timer_ready = self.units[index].harvest_timer <= Fixed16_16::ZERO;

            if unloading {
                let refinery_cell = self
                    .buildings
                    .iter()
                    .find(|building| {
                        building.owner == owner
                            && building.is_alive()
                            && building.structure_type() == Some(StructureType::Refinery)
                    })
                    .map(Self::building_cell);
                let Some(refinery_cell) = refinery_cell else {
                    continue;
                };

                let refinery_center = Self::cell_center(refinery_cell.x, refinery_cell.y);
                let distance = position.distance(refinery_center).to_f32();
                if distance <= TILE_SIZE * 1.5 && timer_ready {
                    self.units[index].tiberium = self.units[index].tiberium.saturating_sub(1);
                    self.units[index].harvest_timer = Fixed16_16::from_f32(0.15);
                    self.units[index].clear_path();
                    self.units[index].target = None;
                    if let Some(house) = self.houses.get_mut(owner as usize) {
                        house.harvested(HARVESTER_STEP_CREDITS);
                    }
                    self.harvested_credits += HARVESTER_STEP_CREDITS as i64;
                    if self.units[index].tiberium == 0 {
                        self.units[index].unloading = false;
                    }
                } else if self.units[index].path.is_empty() && self.units[index].target.is_none() {
                    self.move_unit_to_cell(index, refinery_cell);
                }
                continue;
            }

            let on_tiberium = self
                .map
                .grid
                .get(cell)
                .map_or(false, |cell| cell.has_tiberium);
            if on_tiberium && timer_ready {
                let stage = self
                    .map
                    .grid
                    .get(cell)
                    .map(|cell| tiberium_stage(cell.tiberium_amount))
                    .unwrap_or(0);
                let reducer = (stage % 6) + 1;
                let capacity_left = HARVESTER_STEP_COUNT.saturating_sub(self.units[index].tiberium);
                let levels = reducer.min(capacity_left);
                let reduced = self.map.grid.reduce_tiberium(cell, levels);
                self.units[index].tiberium = self.units[index].tiberium.saturating_add(reduced);
                self.units[index].harvest_timer = Fixed16_16::from_f32(1.0);
                self.units[index].clear_path();
                self.units[index].target = None;
            } else if timer_ready
                && self.units[index].path.is_empty()
                && self.units[index].target.is_none()
            {
                if let Some(target_cell) = self.nearest_tiberium_cell(cell) {
                    self.move_unit_to_cell(index, target_cell);
                }
            }
        }
    }

    /// AIRCRAFT.CPP flight/rearm slice: aircraft with no ammunition return to
    /// an owned helipad and reload one shot every 0.25 s.
    fn update_aircraft(&mut self, delta_time: Fixed16_16) {
        for index in 0..self.units.len() {
            if !self.units[index].is_aircraft || !self.units[index].is_alive() {
                continue;
            }
            if self.units[index].rearm_timer > Fixed16_16::ZERO {
                self.units[index].rearm_timer =
                    (self.units[index].rearm_timer - delta_time).max(Fixed16_16::ZERO);
            }

            let owner = self.units[index].owner;
            let ammo = self.units[index].ammo;
            let max_ammo = self.units[index].max_ammo;
            if max_ammo <= 0 || ammo >= max_ammo {
                continue;
            }

            let helipad = self
                .buildings
                .iter()
                .find(|building| {
                    building.owner == owner
                        && building.is_alive()
                        && building.structure_type() == Some(StructureType::Helipad)
                })
                .map(Self::building_cell);
            let Some(helipad) = helipad else {
                continue;
            };

            let center = Self::cell_center(helipad.x, helipad.y);
            let distance = self.units[index].position.distance(center).to_f32();
            if distance <= TILE_SIZE * 1.5 {
                if self.units[index].rearm_timer <= Fixed16_16::ZERO {
                    self.units[index].ammo = (self.units[index].ammo + 1).min(max_ammo);
                    self.units[index].rearm_timer = Fixed16_16::from_f32(0.25);
                    self.aircraft_rearmed += 1;
                    self.units[index].clear_path();
                    self.units[index].target = None;
                }
            } else if ammo == 0
                && self.units[index].path.is_empty()
                && self.units[index].target.is_none()
            {
                self.move_unit_to_cell(index, helipad);
            }
        }
    }

    fn nearest_tiberium_cell(&self, from: Vec2i) -> Option<Vec2i> {
        let width = self.map.width as i32;
        let height = self.map.height as i32;
        let mut best: Option<(Vec2i, i32)> = None;
        for y in 0..height {
            for x in 0..width {
                let cell = self.map.grid.get(Vec2i::new(x, y))?;
                if !cell.has_tiberium {
                    continue;
                }
                let distance = (x - from.x).abs() + (y - from.y).abs();
                if best.map_or(true, |(_, current)| distance < current) {
                    best = Some((Vec2i::new(x, y), distance));
                }
            }
        }
        best.map(|(cell, _)| cell)
    }

    fn move_unit_to_cell(&mut self, index: usize, goal: Vec2i) {
        let path = self.path_to_cell(index, goal);
        let reserved = path.first().map(|point| Self::world_cell(*point));
        let destination = Self::cell_center(goal.x, goal.y);
        let unit = &mut self.units[index];
        unit.set_path(path);
        unit.reserved_cell = reserved;
        unit.target = Some(destination);
        unit.ordered = true;
    }

    /// FOOT.CPP Mission_Guard / Mission_Hunt: idle units scan for a threat.
    ///
    /// Human (GDI) units use THREAT_RANGE (twice weapon range, clamped to
    /// 0x0A00 leptons); AI units use THREAT_NORMAL (weapon range only).
    fn update_auto_acquire(&mut self) {
        for index in 0..self.units.len() {
            if !self.units[index].is_alive()
                || self.units[index].ordered
                || self.units[index].attack_target.is_some()
            {
                continue;
            }
            if self.units[index].max_ammo > 0 && self.units[index].ammo == 0 {
                continue;
            }
            let Some(weapon) = self.units[index].weapon.clone() else {
                continue;
            };
            let owner = self.units[index].owner;
            let position = self.units[index].position;
            let scan_range = if owner == 0 {
                (weapon.range.to_f32() * 2.0).min(leptons_to_px(0x0A00))
            } else {
                weapon.range.to_f32()
            };
            let scan_squared = scan_range * scan_range;

            let mut best: Option<(usize, f32)> = None;
            for (other, candidate) in self.units.iter().enumerate() {
                if other == index || candidate.owner == owner || !candidate.is_alive() {
                    continue;
                }
                let dx = candidate.position.x.to_f32() - position.x.to_f32();
                let dy = candidate.position.y.to_f32() - position.y.to_f32();
                let distance_squared = dx * dx + dy * dy;
                if distance_squared <= scan_squared
                    && best.map_or(true, |(_, current)| distance_squared < current)
                {
                    best = Some((other, distance_squared));
                }
            }

            if let Some((target, _)) = best {
                self.units[index].attack_target = Some(target);
                self.units[index].target = None;
                self.units[index].ordered = true;
            }
        }
    }

    /// Rotate turrets (or bodies for non-turret vehicles) toward the target.
    fn update_facings(&mut self, delta_time: Fixed16_16) {
        for index in 0..self.units.len() {
            let Some(target_index) = self.units[index].attack_target else {
                continue;
            };
            let Some(target) = self.units.get(target_index) else {
                continue;
            };
            if !target.is_alive() {
                continue;
            }

            let dx = target.position.x.to_f32() - self.units[index].position.x.to_f32();
            let dy = target.position.y.to_f32() - self.units[index].position.y.to_f32();
            if dx == 0.0 && dy == 0.0 {
                continue;
            }
            let desired = dy.atan2(dx);
            self.units[index].rotate_toward(desired, delta_time);
        }
    }

    fn update_attacks(&mut self) {
        for index in 0..self.units.len() {
            let Some(target_index) = self.units[index].attack_target else {
                continue;
            };
            let target_info = self
                .units
                .get(target_index)
                .map(|target| (target.position, target.owner, target.is_alive()));
            let Some((target_position, target_owner, target_alive)) = target_info else {
                self.units[index].attack_target = None;
                continue;
            };

            let (position, owner, primary, secondary, reload) = {
                let unit = &self.units[index];
                (
                    unit.position,
                    unit.owner,
                    unit.weapon.clone(),
                    unit.secondary_weapon.clone(),
                    unit.reload,
                )
            };

            if !target_alive || target_owner == owner {
                self.units[index].attack_target = None;
                continue;
            }

            let Some(primary) = primary else {
                continue;
            };

            let distance = position.distance(target_position);
            let secondary_range = secondary
                .as_ref()
                .map(|weapon| weapon.range)
                .unwrap_or(Fixed16_16::ZERO);
            let max_range = primary.range.max(secondary_range);

            let chosen = if distance <= primary.range {
                Some((primary.clone(), true))
            } else if let Some(secondary) = secondary.clone() {
                if distance <= secondary.range {
                    Some((secondary, true))
                } else {
                    Some((primary.clone(), false))
                }
            } else {
                Some((primary.clone(), false))
            };
            let Some((weapon, in_range)) = chosen else {
                continue;
            };

            if in_range {
                self.units[index].target = None;
                let desired = (target_position.y.to_f32() - position.y.to_f32())
                    .atan2(target_position.x.to_f32() - position.x.to_f32());

                let has_ammo = self.units[index].max_ammo == 0 || self.units[index].ammo > 0;
                if reload <= Fixed16_16::ZERO && has_ammo && self.units[index].aiming_at(desired) {
                    self.projectiles.push(Projectile::new(
                        weapon.clone(),
                        position,
                        target_position,
                        index as u32,
                        Some(target_index as u32),
                    ));
                    self.projectiles_fired += 1;
                    if weapon.weapon_type != primary.weapon_type {
                        self.secondary_shots += 1;
                    }
                    // TECHNO.CPP Rearm_Delay: single shooters always use the
                    // normal ROF + 3; two-shooters alternate 9 ticks and ROF + 3.
                    let delay = self.units[index].next_rearm_delay(&weapon);
                    self.units[index].reload = if delay > Fixed16_16::ZERO {
                        delay
                    } else {
                        Fixed16_16::from_f32(0.5)
                    };
                    self.units[index].note_shot_fired();
                    if self.units[index].is_aircraft {
                        self.aircraft_shots += 1;
                    }
                    if self.units[index].max_ammo > 0 {
                        self.units[index].ammo = self.units[index].ammo.saturating_sub(1);
                        if self.units[index].ammo == 0 {
                            self.units[index].attack_target = None;
                        }
                    }
                }
            } else {
                // FOOT.CPP Approach_Target: stop short of weapon range so the
                // unit can fire without walking into the target.
                let approach = (max_range - Fixed16_16::from_f32(leptons_to_px(0x00B7)))
                    .max(Fixed16_16::from_f32(8.0));
                let offset = target_position - position;
                let distance = offset.length();
                let destination = if distance > approach {
                    target_position - offset.normalize() * approach
                } else {
                    position
                };

                if self.units[index].path.is_empty() && self.units[index].target.is_none() {
                    let goal_cell = Vec2i::new(
                        (destination.x.to_f32() / TILE_SIZE).floor() as i32,
                        (destination.y.to_f32() / TILE_SIZE).floor() as i32,
                    );
                    let used: Vec<(i32, i32)> = self
                        .occupied_cells(Some(index))
                        .iter()
                        .map(|cell| (cell.x, cell.y))
                        .collect();
                    let terrain_ok = self.map.grid.in_bounds(goal_cell.x, goal_cell.y)
                        && self.map.grid.cells[(goal_cell.y as u32 * self.map.grid.width
                            + goal_cell.x as u32)
                            as usize]
                            .is_passable;
                    let goal = if terrain_ok && !used.contains(&(goal_cell.x, goal_cell.y)) {
                        goal_cell
                    } else {
                        let (x, y) = self.nearby_cell(goal_cell.x, goal_cell.y, &used);
                        Vec2i::new(x, y)
                    };
                    let path = self.path_to_cell(index, goal);
                    self.units[index].set_path(path);
                }

                self.units[index].target = Some(destination);
                self.units[index].ordered = true;
            }
        }
    }

    fn update_projectiles(&mut self, delta_time: Fixed16_16) {
        // BULLET.CPP homing missiles re-aim at the target every other frame.
        let target_positions: Vec<Option<Vec2>> = self
            .units
            .iter()
            .map(|unit| {
                if unit.is_alive() {
                    Some(unit.position)
                } else {
                    None
                }
            })
            .collect();

        let mut hits: Vec<(Fixed16_16, Warhead, Vec2, usize)> = Vec::new();
        self.projectiles.retain_mut(|projectile| {
            if projectile.homing && projectile.age % 2 == 0 {
                if let Some(target) = projectile.target_entity {
                    if let Some(Some(position)) = target_positions.get(target as usize) {
                        projectile.target = *position;
                    }
                }
            }
            if projectile.update(delta_time) {
                hits.push((
                    projectile.weapon.damage,
                    projectile.weapon.warhead,
                    projectile.position,
                    projectile.source as usize,
                ));
                false
            } else {
                true
            }
        });

        // COMBAT.CPP Explosion_Damage: everything within 1.5 cells of the
        // impact takes warhead-modified damage, attenuated by distance.
        let blast_radius = leptons_to_px(EXPLOSION_RANGE_LEPTONS);
        for (base_damage, warhead, impact, source) in hits {
            for (index, unit) in self.units.iter_mut().enumerate() {
                if index == source || !unit.is_alive() {
                    continue;
                }
                let distance = unit.position.distance(impact).to_f32();
                if distance > blast_radius {
                    continue;
                }
                let damage = warhead.damage(base_damage, unit.armor, px_to_leptons(distance));
                unit.take_damage(damage);
            }
        }
    }

    /// Render the full scene: terrain, buildings, then units.
    pub fn render(&self, renderer: &mut dyn Renderer) {
        self.render_with_sprites(renderer, None);
    }

    pub fn render_with_sprites(&self, renderer: &mut dyn Renderer, atlas: Option<&SpriteAtlas>) {
        self.map.render(renderer);
        for building in &self.buildings {
            building.render_with(renderer, atlas);
        }
        for unit in &self.units {
            unit.render_with(renderer, atlas);
        }
        for projectile in &self.projectiles {
            let x = projectile.position.x.to_f32();
            let y = projectile.position.y.to_f32();
            let altitude = projectile.altitude.to_f32();
            renderer.draw_rect(x - 2.0, y - 2.0, 4.0, 4.0, [0.15, 0.15, 0.15, 0.6]);
            renderer.draw_rect(x - 2.0, y - 2.0 - altitude, 4.0, 4.0, [1.0, 0.85, 0.2, 1.0]);
        }
        for blast in &self.blasts {
            let ratio = (blast.ttl.to_f32() / blast.max_ttl.to_f32().max(1.0e-6)).clamp(0.0, 1.0);
            let radius = blast.radius * (1.15 - 0.35 * ratio);
            let x = blast.position.x.to_f32();
            let y = blast.position.y.to_f32();
            let color = blast.color;
            renderer.draw_rect(
                x - radius,
                y - radius,
                radius * 2.0,
                radius * 2.0,
                [color[0], color[1], color[2], 0.30 * ratio],
            );
            let inner = radius * 0.6;
            renderer.draw_rect(
                x - inner,
                y - inner,
                inner * 2.0,
                inner * 2.0,
                [color[0], color[1], color[2], 0.45 * ratio],
            );
            let core = radius * 0.3;
            renderer.draw_rect(
                x - core,
                y - core,
                core * 2.0,
                core * 2.0,
                [1.0, 1.0, 1.0, 0.35 * ratio],
            );
        }

        for beam in &self.beams {
            let ratio = (beam.ttl.to_f32() / beam.max_ttl.to_f32().max(1.0e-6)).clamp(0.0, 1.0);
            let x = beam.position.x.to_f32();
            let y = beam.position.y.to_f32();
            renderer.draw_rect(
                x - beam.width * 0.5,
                y - 260.0,
                beam.width,
                260.0,
                [0.60, 0.85, 1.00, 0.35 * ratio],
            );
            renderer.draw_rect(
                x - beam.width * 0.25,
                y - 260.0,
                beam.width * 0.5,
                260.0,
                [1.0, 1.0, 1.0, 0.50 * ratio],
            );
        }

        for strike in &self.strikes {
            let progress = strike.progress.to_f32();
            let from_x = strike.from.x.to_f32();
            let from_y = strike.from.y.to_f32();
            let to_x = strike.to.x.to_f32();
            let to_y = strike.to.y.to_f32();
            for trail in 0..4 {
                let t = (progress - 0.03 * (trail as f32 + 1.0)).max(0.0);
                let x = from_x + (to_x - from_x) * t;
                let y = from_y + (to_y - from_y) * t;
                renderer.draw_rect(
                    x - 2.0,
                    y - 2.0,
                    4.0,
                    4.0,
                    [1.0, 0.65, 0.25, 0.85 - 0.15 * trail as f32],
                );
            }
        }

        // Move-order destination markers.
        for unit in &self.units {
            if let Some(target) = unit.target {
                renderer.draw_rect(
                    target.x.to_f32() - 3.0,
                    target.y.to_f32() - 3.0,
                    6.0,
                    6.0,
                    [1.0, 0.9, 0.2, 1.0],
                );
            }
        }
    }

    /// World-space centre of the map, useful for centring a camera.
    pub fn world_center(&self) -> Vec2 {
        Vec2::new(
            Fixed16_16::from_f32(self.map.width as f32 * TILE_SIZE * 0.5),
            Fixed16_16::from_f32(self.map.height as f32 * TILE_SIZE * 0.5),
        )
    }

    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    pub fn building_count(&self) -> usize {
        self.buildings.len()
    }

    /// Select the nearest owned unit within a click radius of a world position.
    ///
    /// Returns the index of the newly selected unit, if any.
    pub fn select_at(&mut self, world: Vec2) -> Option<usize> {
        self.select_at_additive(world, false)
    }

    /// Click-selection with optional additive (shift) behaviour.
    pub fn select_at_additive(&mut self, world: Vec2, additive: bool) -> Option<usize> {
        const PICK_RADIUS: f32 = 22.0;
        let mut best: Option<(usize, f32)> = None;
        for (index, unit) in self.units.iter().enumerate() {
            if unit.owner != self.player_owner || !unit.is_alive() {
                continue;
            }
            let dx = unit.position.x.to_f32() - world.x.to_f32();
            let dy = unit.position.y.to_f32() - world.y.to_f32();
            let distance_squared = dx * dx + dy * dy;
            if distance_squared <= PICK_RADIUS * PICK_RADIUS
                && best.map_or(true, |(_, current)| distance_squared < current)
            {
                best = Some((index, distance_squared));
            }
        }

        if !additive {
            for unit in &mut self.units {
                unit.is_selected = false;
            }
        }
        if let Some((index, _)) = best {
            self.units[index].is_selected = true;
            Some(index)
        } else {
            None
        }
    }

    /// Box-select every living owned unit inside the world-space rectangle.
    pub fn select_in_rect(&mut self, a: Vec2, b: Vec2, additive: bool) -> usize {
        let min_x = a.x.to_f32().min(b.x.to_f32());
        let max_x = a.x.to_f32().max(b.x.to_f32());
        let min_y = a.y.to_f32().min(b.y.to_f32());
        let max_y = a.y.to_f32().max(b.y.to_f32());

        if !additive {
            for unit in &mut self.units {
                unit.is_selected = false;
            }
        }

        let mut count = 0;
        for unit in &mut self.units {
            if unit.owner != self.player_owner || !unit.is_alive() {
                continue;
            }
            let x = unit.position.x.to_f32();
            let y = unit.position.y.to_f32();
            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                unit.is_selected = true;
                count += 1;
            }
        }
        count
    }

    /// Issue a move order to every selected unit, spread into a formation.
    ///
    /// Returns how many units were ordered.
    pub fn order_move(&mut self, world: Vec2) -> usize {
        let selected: Vec<usize> = self
            .units
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.is_selected && unit.is_alive())
            .map(|(index, _)| index)
            .collect();
        self.order_move_indices(&selected, world)
    }

    /// Move exactly the listed unit indices (network command application).
    fn order_move_indices(&mut self, selected: &[usize], world: Vec2) -> usize {
        let count = selected.len();
        if count == 0 {
            return 0;
        }

        let target_x = (world.x.to_f32() / TILE_SIZE).floor() as i32;
        let target_y = (world.y.to_f32() / TILE_SIZE).floor() as i32;

        let mut used: Vec<(i32, i32)> = self
            .occupied_cells(None)
            .iter()
            .map(|cell| (cell.x, cell.y))
            .collect();
        for &index in selected {
            let (cell_x, cell_y) = self.nearby_cell(target_x, target_y, &used);
            used.push((cell_x, cell_y));
            let goal = Vec2i::new(cell_x, cell_y);
            let path = self.path_to_cell(index, goal);
            let reserved = path.first().map(|point| Self::world_cell(*point));
            let destination = Self::cell_center(cell_x, cell_y);
            let unit = &mut self.units[index];
            unit.set_path(path);
            unit.reserved_cell = reserved;
            unit.target = Some(destination);
            unit.ordered = true;
            unit.attack_target = None;
        }
        count
    }

    /// Unit indices owned by `owner` among the given stable ids.
    fn owned_unit_indices(&self, owner: u8, unit_ids: &[u32]) -> Vec<usize> {
        unit_ids
            .iter()
            .filter_map(|id| {
                let index = *id as usize;
                self.units
                    .get(index)
                    .filter(|unit| unit.owner == owner && unit.is_alive())
                    .map(|_| index)
            })
            .collect()
    }

    /// Move a specific player's units by stable id (lockstep network use).
    pub fn order_units_move(&mut self, owner: u8, unit_ids: &[u32], world: Vec2) -> usize {
        let indices = self.owned_unit_indices(owner, unit_ids);
        self.order_move_indices(&indices, world)
    }

    fn world_cell(position: Vec2) -> Vec2i {
        Vec2i::new(
            (position.x.to_f32() / TILE_SIZE).floor() as i32,
            (position.y.to_f32() / TILE_SIZE).floor() as i32,
        )
    }

    fn unit_cell(unit: &Unit) -> Vec2i {
        Self::world_cell(unit.position)
    }

    fn building_cell(building: &Building) -> Vec2i {
        Vec2i::new(
            (building.position.x.to_f32() / TILE_SIZE).floor() as i32,
            (building.position.y.to_f32() / TILE_SIZE).floor() as i32,
        )
    }

    /// Cells currently occupied by live units and buildings.
    fn occupied_cells(&self, exclude_unit: Option<usize>) -> Vec<Vec2i> {
        let mut cells = Vec::new();
        for (index, unit) in self.units.iter().enumerate() {
            if Some(index) == exclude_unit || !unit.is_alive() {
                continue;
            }
            cells.push(Self::unit_cell(unit));
            if let Some(reserved) = unit.reserved_cell {
                cells.push(reserved);
            }
        }
        for building in &self.buildings {
            cells.push(Self::building_cell(building));
        }
        cells
    }

    /// Build a cell-centre waypoint path for a unit, avoiding occupied cells.
    fn path_to_cell(&self, unit_index: usize, goal: Vec2i) -> Vec<Vec2> {
        let Some(unit) = self.units.get(unit_index) else {
            return Vec::new();
        };
        if unit.is_aircraft {
            return Vec::new();
        }
        let start = Self::unit_cell(unit);
        let blocked = self.occupied_cells(Some(unit_index));
        match find_path(&self.map.grid, &blocked, start, goal, 8192) {
            Some(cells) => cells
                .iter()
                .map(|cell| Self::cell_center(cell.x, cell.y))
                .collect(),
            None => Vec::new(),
        }
    }

    /// MAP.CPP MapClass::Nearby_Location: scan the expanding box edges outward
    /// and return the first free map cell.
    fn nearby_cell(&self, cell_x: i32, cell_y: i32, used: &[(i32, i32)]) -> (i32, i32) {
        let width = self.map.width as i32;
        let height = self.map.height as i32;
        let is_free = |x: i32, y: i32, used: &[(i32, i32)]| {
            x >= 0
                && y >= 0
                && x < width
                && y < height
                && self.map.grid.cells[(y * width + x) as usize].is_passable
                && !used.contains(&(x, y))
        };

        for radius in 0..=16 {
            for x in (cell_x - radius)..=(cell_x + radius) {
                if radius > 0 {
                    let top = cell_y - radius;
                    if is_free(x, top, used) {
                        return (x, top);
                    }
                }
                let bottom = cell_y + radius;
                if is_free(x, bottom, used) {
                    return (x, bottom);
                }
            }
            if radius > 0 {
                for y in (cell_y - radius + 1)..(cell_y + radius) {
                    let left = cell_x - radius;
                    if is_free(left, y, used) {
                        return (left, y);
                    }
                    let right = cell_x + radius;
                    if is_free(right, y, used) {
                        return (right, y);
                    }
                }
            }
        }
        (cell_x.clamp(0, width - 1), cell_y.clamp(0, height - 1))
    }

    /// Find the nearest enemy unit (relative to the player's house).
    pub fn enemy_at(&self, world: Vec2) -> Option<usize> {
        const PICK_RADIUS: f32 = 22.0;
        let mut best: Option<(usize, f32)> = None;
        for (index, unit) in self.units.iter().enumerate() {
            if unit.owner == self.player_owner || !unit.is_alive() {
                continue;
            }
            let dx = unit.position.x.to_f32() - world.x.to_f32();
            let dy = unit.position.y.to_f32() - world.y.to_f32();
            let distance_squared = dx * dx + dy * dy;
            if distance_squared <= PICK_RADIUS * PICK_RADIUS
                && best.map_or(true, |(_, current)| distance_squared < current)
            {
                best = Some((index, distance_squared));
            }
        }
        best.map(|(index, _)| index)
    }

    /// Order every selected unit to attack the given enemy unit index.
    pub fn order_attack(&mut self, target_index: usize) -> usize {
        let selected: Vec<usize> = self
            .units
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.is_selected && unit.owner == self.player_owner && unit.is_alive())
            .map(|(index, _)| index)
            .collect();
        self.order_attack_indices(self.player_owner, &selected, target_index)
    }

    /// Order the listed unit indices (owned by `owner`) onto an enemy index.
    fn order_attack_indices(
        &mut self,
        owner: u8,
        selected: &[usize],
        target_index: usize,
    ) -> usize {
        let valid_target = self
            .units
            .get(target_index)
            .map_or(false, |target| target.owner != owner && target.is_alive());
        if !valid_target {
            return 0;
        }

        let mut ordered = 0;
        for &index in selected {
            let Some(unit) = self.units.get_mut(index) else {
                continue;
            };
            if unit.owner == owner && unit.is_alive() && unit.weapon.is_some() {
                unit.attack_target = Some(target_index);
                unit.ordered = true;
                ordered += 1;
            }
        }
        ordered
    }

    /// Attack for a specific player's units by stable id (lockstep network use).
    pub fn order_units_attack(
        &mut self,
        owner: u8,
        unit_ids: &[u32],
        target_index: usize,
    ) -> usize {
        let indices = self.owned_unit_indices(owner, unit_ids);
        self.order_attack_indices(owner, &indices, target_index)
    }

    /// Clear move/attack orders for one player's listed units.
    pub fn order_units_stop(&mut self, owner: u8, unit_ids: &[u32]) -> usize {
        let indices = self.owned_unit_indices(owner, unit_ids);
        let mut stopped = 0;
        for index in indices {
            let unit = &mut self.units[index];
            if unit.target.is_some()
                || unit.attack_target.is_some()
                || !unit.path.is_empty()
                || unit.ordered
            {
                stopped += 1;
            }
            unit.target = None;
            unit.attack_target = None;
            unit.path.clear();
            unit.path_index = 0;
            unit.reserved_cell = None;
            unit.ordered = false;
        }
        stopped
    }

    /// Select every living unit owned by the given player (0 = GDI).
    pub fn select_all_owned(&mut self, owner: u8) -> usize {
        let mut count = 0;
        for unit in &mut self.units {
            unit.is_selected = unit.owner == owner && unit.is_alive();
            if unit.is_selected {
                count += 1;
            }
        }
        count
    }

    /// HUD-relevant snapshot of the current game state.
    pub fn status(&self) -> GameStatus {
        GameStatus {
            mission: self.mission.name.clone(),
            units: self.units.len(),
            buildings: self.buildings.len(),
            selected: self.units.iter().filter(|unit| unit.is_selected).count(),
            gdi_credits: self
                .houses
                .iter()
                .find(|house| house.id == ClassId::HouseGDI)
                .map(|house| house.credits)
                .unwrap_or(0),
            nod_credits: self
                .houses
                .iter()
                .find(|house| house.id == ClassId::HouseNod)
                .map(|house| house.credits)
                .unwrap_or(0),
            production: self
                .houses
                .iter()
                .map(|house| house.production_queue.len())
                .sum(),
            gdi_power: self.houses.get(0).map_or(0, |house| house.power),
            gdi_drain: self.houses.get(0).map_or(0, |house| house.max_power),
            nod_power: self.houses.get(1).map_or(0, |house| house.power),
            nod_drain: self.houses.get(1).map_or(0, |house| house.max_power),
            gdi_tiberium: self.houses.get(0).map_or(0, |house| house.tiberium),
            gdi_capacity: self.houses.get(0).map_or(0, |house| house.capacity),
            nod_tiberium: self.houses.get(1).map_or(0, |house| house.tiberium),
            nod_capacity: self.houses.get(1).map_or(0, |house| house.capacity),
            gdi_ion: self
                .houses
                .get(0)
                .map_or(0, |house| (house.ion_charge.to_f32() * 100.0) as i32),
            gdi_nuke: self
                .houses
                .get(0)
                .map_or(0, |house| (house.nuke_charge.to_f32() * 100.0) as i32),
            nod_ion: self
                .houses
                .get(1)
                .map_or(0, |house| (house.ion_charge.to_f32() * 100.0) as i32),
            nod_nuke: self
                .houses
                .get(1)
                .map_or(0, |house| (house.nuke_charge.to_f32() * 100.0) as i32),
            ready_buildings: self.ready_buildings.len(),
            objectives_total: self.mission.objectives.len(),
            objectives_done: self.mission.objectives_done(),
            mission_over: self.mission.is_over(),
            mission_victory: self.mission.outcome == Some(MissionOutcome::Victory),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cnc_asset::{AtlasStyle, SpriteAtlas};
    use cnc_renderer::SoftwareRenderer;

    struct CountingRenderer {
        sprites: usize,
    }

    impl cnc_renderer::Renderer for CountingRenderer {
        fn begin_frame(&mut self) {}
        fn end_frame(&mut self) -> anyhow::Result<()> {
            Ok(())
        }
        fn draw_sprite(&mut self, _sprite: &cnc_renderer::Sprite) {
            self.sprites += 1;
        }
        fn draw_rect(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _color: [f32; 4]) {}
        fn draw_text(&mut self, _text: &str, _x: f32, _y: f32, _size: f32, _color: [f32; 4]) {}
        fn set_camera(&mut self, _camera: &cnc_renderer::Camera) {}
        fn resize(&mut self, _width: u32, _height: u32) {}
        fn set_palette(&mut self, _palette: &cnc_core::Palette) {}
        fn uses_sprites(&self) -> bool {
            true
        }
    }

    #[test]
    fn atlas_covers_demo_classes() {
        let atlas = SpriteAtlas::generate(AtlasStyle::Neon);
        for key in [
            "InfantryE1",
            "VehicleE2",
            "VehicleE4",
            "VehicleE6",
            "AircraftE1",
            "ConstructionYard",
            "PowerPlant",
            "Refinery",
            "Barracks",
            "WarFactory",
            "Helipad",
            "Silo",
        ] {
            assert!(atlas.frame(key).is_some(), "atlas missing {key}");
        }
    }

    #[test]
    fn render_with_sprites_emits_unit_and_building_sprites() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let atlas = SpriteAtlas::generate(AtlasStyle::Neon);
        let mut renderer = CountingRenderer { sprites: 0 };

        game.render_with_sprites(&mut renderer, Some(&atlas));

        let expected = game.buildings.len() + game.units.len();
        assert!(
            renderer.sprites >= expected,
            "expected at least {expected} sprites, got {}",
            renderer.sprites
        );
    }

    #[test]
    fn demo_game_updates_and_renders() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        assert!(game.unit_count() > 0, "demo should spawn units");
        assert!(game.building_count() > 0, "demo should spawn buildings");
        assert!(game.houses.len() >= 2, "demo should spawn houses");

        let enemy_index = game
            .units
            .iter()
            .position(|unit| unit.owner != 0)
            .expect("demo should spawn Nod units");
        let enemy_before = game.units[enemy_index].position;

        let before: Vec<Vec2> = game
            .units
            .iter()
            .filter(|unit| unit.owner == 0)
            .map(|unit| unit.position)
            .collect();
        game.update(Fixed16_16::from_int(1));
        assert!(
            game.units
                .iter()
                .filter(|unit| unit.owner == 0)
                .zip(before.iter())
                .any(|(unit, before)| unit.position != *before),
            "idle GDI units should patrol or reposition"
        );
        assert_eq!(
            game.units[enemy_index].position, enemy_before,
            "enemy units should hold position until ordered"
        );

        let mut renderer = SoftwareRenderer::new(800, 600);
        game.render(&mut renderer);
        assert!(
            renderer.buffer().iter().any(|pixel| *pixel != 0),
            "rendering the game should write pixels"
        );
    }

    #[test]
    fn move_order_routes_around_impassable_terrain() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(1, 8), 0));
        game.units[0].is_selected = true;

        // Solid wall at x = 8 with a gap at the bottom edge.
        for y in 0..16 {
            game.map.grid.set_passable(Vec2i::new(8, y), false);
        }
        game.map.grid.set_passable(Vec2i::new(8, 15), true);

        assert_eq!(game.order_move(Game::cell_center(15, 8)), 1);
        let path_cells: Vec<Vec2i> = game.units[0]
            .path
            .iter()
            .map(|point| {
                Vec2i::new(
                    (point.x.to_f32() / TILE_SIZE).floor() as i32,
                    (point.y.to_f32() / TILE_SIZE).floor() as i32,
                )
            })
            .collect();

        assert!(
            !path_cells.is_empty(),
            "a path around the wall should exist"
        );
        assert!(
            path_cells.iter().all(|cell| cell.x != 8 || cell.y == 15),
            "path must only cross the wall at the gap"
        );
        assert!(path_cells.iter().any(|cell| cell.y == 15));
    }

    #[test]
    fn move_order_avoids_building_cells() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::MediumTank, Game::cell_center(1, 8), 0));
        game.units[0].is_selected = true;
        game.buildings.push(Building::new(
            ClassId::ConstructionYard,
            Game::cell_center(8, 8),
            Fixed16_16::from_int(500),
            0,
        ));

        assert_eq!(game.order_move(Game::cell_center(15, 8)), 1);
        let path_cells: Vec<Vec2i> = game.units[0]
            .path
            .iter()
            .map(|point| {
                Vec2i::new(
                    (point.x.to_f32() / TILE_SIZE).floor() as i32,
                    (point.y.to_f32() / TILE_SIZE).floor() as i32,
                )
            })
            .collect();

        assert!(!path_cells.is_empty());
        assert!(
            !path_cells.contains(&Vec2i::new(8, 8)),
            "path must not go through the building cell"
        );
    }

    #[test]
    fn explosion_damage_hits_nearby_enemies() {
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        let tank_position = Vec2::new(Fixed16_16::from_int(500), Fixed16_16::from_int(600));
        let enemy_a = Vec2::new(Fixed16_16::from_int(600), Fixed16_16::from_int(600));
        let enemy_b = Vec2::new(Fixed16_16::from_int(615), Fixed16_16::from_int(600));

        game.units
            .push(Unit::new(UnitType::MediumTank, tank_position, 0));
        game.units.push(Unit::new(UnitType::InfantryE1, enemy_a, 1));
        game.units.push(Unit::new(UnitType::InfantryE1, enemy_b, 1));
        game.units[0].is_selected = true;

        assert_eq!(game.order_attack(1), 1);
        let start = [game.units[1].health, game.units[2].health];

        for _ in 0..200 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(
            game.units[1].health < start[0],
            "direct target should take damage"
        );
        assert!(
            game.units[2].health < start[1],
            "adjacent enemy should take explosion damage"
        );
    }

    #[test]
    fn idle_units_auto_acquire_nearby_enemy() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.update(Fixed16_16::from_f32(0.05));

        assert!(
            game.units
                .iter()
                .any(|unit| unit.owner == 0 && unit.attack_target.is_some()),
            "guarding GDI units should acquire an enemy within threat range"
        );
    }

    #[test]
    fn world_center_is_map_middle() {
        let game = Game::new("test", 64, 64);
        let center = game.world_center();
        assert!((center.x.to_f32() - 64.0 * TILE_SIZE * 0.5).abs() < 0.01);
        assert!((center.y.to_f32() - 64.0 * TILE_SIZE * 0.5).abs() < 0.01);
    }

    #[test]
    fn click_selects_nearest_unit() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let target = game.units[3].position;
        assert_eq!(game.select_at(target), Some(3));
        assert!(game.units[3].is_selected);
        assert_eq!(game.status().selected, 1);
    }

    #[test]
    fn click_on_empty_ground_clears_selection() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.select_all_owned(0);
        assert!(game.status().selected > 0);

        let empty = Vec2::new(Fixed16_16::from_int(-500), Fixed16_16::from_int(-500));
        assert_eq!(game.select_at(empty), None);
        assert_eq!(game.status().selected, 0);
    }

    #[test]
    fn move_order_drives_unit_to_target() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let index = 2;
        for unit in &mut game.units {
            unit.is_selected = false;
        }
        game.units[index].is_selected = true;
        let start = game.units[index].position;
        let target = Vec2::new(
            start.x + Fixed16_16::from_int(120),
            start.y + Fixed16_16::from_int(60),
        );

        assert_eq!(game.order_move(target), 1);
        assert!(game.units[index].ordered);
        let destination = game.units[index]
            .target
            .expect("move order should set a target");
        assert_ne!(destination, start);

        for _ in 0..2000 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        let dx = game.units[index].position.x.to_f32() - destination.x.to_f32();
        let dy = game.units[index].position.y.to_f32() - destination.y.to_f32();
        assert!(
            dx.abs() < 5.0 && dy.abs() < 5.0,
            "unit should arrive, offset {dx},{dy}"
        );
        assert!(game.units[index].target.is_none());
    }

    #[test]
    fn move_order_draws_target_marker() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let marker_target = Vec2::new(Fixed16_16::from_int(100), Fixed16_16::from_int(100));
        game.units[0].target = Some(marker_target);
        game.units[0].ordered = true;

        let mut renderer = SoftwareRenderer::new(800, 600);
        game.render(&mut renderer);

        // [1.0, 0.9, 0.2, 1.0] packed by SoftwareRenderer as ARGB.
        let marker_color = (255u32 << 24) | (255u32 << 16) | (229u32 << 8) | 51u32;
        assert!(
            renderer.buffer().contains(&marker_color),
            "expected a yellow move-order marker in the rendered frame"
        );
    }

    #[test]
    fn box_selection_selects_multiple_units() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let center = game.patrol_center;
        let offset = Fixed16_16::from_int(300);
        let min = Vec2::new(center.x - offset, center.y - offset);
        let max = Vec2::new(center.x + offset, center.y + offset);
        let count = game.select_in_rect(min, max, false);

        assert!(
            count >= 10,
            "box should select the GDI cluster, got {count}"
        );
        assert_eq!(game.status().selected, count);
        assert!(
            game.units
                .iter()
                .filter(|u| u.owner == 0)
                .filter(|u| u.is_selected)
                .count()
                >= 10
        );
    }

    #[test]
    fn shift_click_adds_to_selection() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let first = game.units[0].position;
        assert_eq!(game.select_at(first), Some(0));
        let second = game.units[5].position;
        assert_eq!(game.select_at_additive(second, true), Some(5));
        assert_eq!(game.status().selected, 2);
        assert!(game.units[0].is_selected);
        assert!(game.units[5].is_selected);
    }

    #[test]
    fn group_move_uses_nearby_cells() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert_eq!(game.select_all_owned(0), 14);

        let world = Vec2::new(Fixed16_16::from_int(700), Fixed16_16::from_int(700));
        assert_eq!(game.order_move(world), 14);

        let gdi: Vec<&Unit> = game.units.iter().filter(|u| u.owner == 0).collect();
        let mut targets = Vec::new();
        for unit in &gdi {
            let target = unit.target.expect("every selected unit gets a target");
            let x = target.x.to_f32();
            let y = target.y.to_f32();
            targets.push((x, y));

            // Nearby_Location returns cell centres.
            let cell_x = ((x - TILE_SIZE * 0.5) / TILE_SIZE).round();
            let cell_y = ((y - TILE_SIZE * 0.5) / TILE_SIZE).round();
            assert!((x - (cell_x * TILE_SIZE + TILE_SIZE * 0.5)).abs() < 0.01);
            assert!((y - (cell_y * TILE_SIZE + TILE_SIZE * 0.5)).abs() < 0.01);
            assert!((x - world.x.to_f32()).abs() <= 16.0 * TILE_SIZE);
            assert!((y - world.y.to_f32()).abs() <= 16.0 * TILE_SIZE);
        }

        for i in 0..targets.len() {
            for j in (i + 1)..targets.len() {
                assert!(
                    targets[i] != targets[j],
                    "formation targets must be distinct"
                );
            }
        }
    }

    #[test]
    fn enemy_at_finds_nod_unit() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let (index, position) = game
            .units
            .iter()
            .enumerate()
            .find(|(_, unit)| unit.owner != 0)
            .map(|(index, unit)| (index, unit.position))
            .expect("demo should spawn Nod units");
        assert_eq!(game.enemy_at(position), Some(index));
    }

    #[test]
    fn attack_order_fires_projectiles_and_damages_enemy() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert_eq!(game.select_all_owned(0), 14);

        let enemy = game
            .units
            .iter()
            .position(|unit| unit.owner != 0)
            .expect("demo should spawn Nod units");
        assert!(game.order_attack(enemy) > 0);

        let start_health = game.units[enemy].health;
        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(
            game.projectiles_fired > 0,
            "attack order should spawn projectiles"
        );
        assert!(
            game.units[enemy].health < start_health,
            "the enemy should have taken damage"
        );
    }

    #[test]
    fn turret_rotates_before_firing() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::MediumTank, Game::cell_center(8, 12), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 4), 1));
        game.units[0].is_selected = true;
        assert_eq!(game.order_attack(1), 1);
        assert_eq!(game.units[0].turret_facing, 0.0);

        // First short step: out of range and still facing east.
        game.update(Fixed16_16::from_f32(0.05));
        assert!(
            game.units[0].turret_facing < 0.0,
            "turret should start rotating toward the northern target"
        );
        assert_eq!(game.projectiles_fired, 0);

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.projectiles_fired > 0 {
                break;
            }
        }

        assert!(
            game.projectiles_fired > 0,
            "tank should fire once aligned and in range"
        );
        assert!(
            (game.units[0].turret_facing + std::f32::consts::FRAC_PI_2).abs() < 0.2,
            "turret should end up facing north"
        );
    }

    #[test]
    fn setup_demo_computes_house_power() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let status = game.status();
        assert_eq!(status.gdi_power, 140);
        assert_eq!(status.gdi_drain, 115);
        assert_eq!(status.nod_power, 140);
        assert_eq!(status.nod_drain, 55);
        assert_eq!(status.gdi_capacity, 1000);
        assert_eq!(status.nod_capacity, 1000);
    }

    #[test]
    fn queue_unit_spends_credits_and_spawns_at_factory() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.units.clear();

        let credits = game.status().gdi_credits;
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        let status = game.status();
        assert_eq!(status.gdi_credits, credits - 100);
        assert_eq!(status.production, 1);

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.units.is_empty() {
                break;
            }
        }

        assert_eq!(game.units.len(), 1, "production should spawn one unit");
        assert_eq!(game.units[0].owner, 0);
        assert_eq!(game.units[0].unit_type, UnitType::InfantryE1);
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn queue_requires_matching_factory() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let nod_credits = game.status().nod_credits;

        assert!(!game.queue_unit(1, UnitType::InfantryE1));
        assert!(!game.queue_unit(1, UnitType::MediumTank));
        assert_eq!(game.status().nod_credits, nod_credits);
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn tiberium_grows_and_spreads() {
        let mut game = Game::new("test", 8, 8);
        for cell in &mut game.map.grid.cells {
            cell.has_tiberium = false;
            cell.tiberium_amount = 0;
        }
        game.map.grid.set_tiberium(Vec2i::new(2, 2), 200, 1);

        let before = game.map.grid.get(Vec2i::new(2, 2)).unwrap().tiberium_amount;
        game.map.grid.tiberium_cycle();

        let grown = game.map.grid.get(Vec2i::new(2, 2)).unwrap().tiberium_amount;
        assert!(grown > before, "existing tiberium should grow");
        assert!(
            game.map.grid.get(Vec2i::new(3, 2)).unwrap().has_tiberium,
            "heavy tiberium should spread to a clear neighbour"
        );
    }

    #[test]
    fn harvester_harvests_tiberium() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        for cell in &mut game.map.grid.cells {
            cell.has_tiberium = false;
            cell.tiberium_amount = 0;
        }
        game.map.grid.set_tiberium(Vec2i::new(5, 5), 255, 1);

        let mut harvester = Unit::new(UnitType::Harvester, Game::cell_center(5, 5), 0);
        harvester.ordered = true;
        game.units.push(harvester);

        let before = game.map.grid.get(Vec2i::new(5, 5)).unwrap().tiberium_amount;
        for _ in 0..50 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.units[0].tiberium > 0 {
                break;
            }
        }

        assert!(
            game.units[0].tiberium > 0,
            "harvester should pick up tiberium"
        );
        let after = game.map.grid.get(Vec2i::new(5, 5)).unwrap().tiberium_amount;
        assert!(after < before, "the tiberium cell should be reduced");
    }

    #[test]
    fn harvester_unloads_at_refinery() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::Refinery,
            Game::cell_center(4, 4),
            0,
        ));
        game.refresh_house_power();

        let mut harvester = Unit::new(UnitType::Harvester, Game::cell_center(6, 4), 0);
        harvester.tiberium = HARVESTER_STEP_COUNT;
        harvester.ordered = true;
        game.units.push(harvester);

        for _ in 0..500 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.units[0].tiberium == 0 {
                break;
            }
        }

        assert_eq!(game.units[0].tiberium, 0, "harvester should unload fully");
        assert_eq!(
            game.houses[0].tiberium,
            HARVESTER_STEP_COUNT as i32 * HARVESTER_STEP_CREDITS,
            "each step should add 25 credits of stored tiberium"
        );
        assert_eq!(
            game.houses[0].credits, 0,
            "initial cash should be untouched"
        );
    }

    #[test]
    fn queue_building_requires_construction_yard() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.buildings
            .retain(|building| building.structure_type() != Some(StructureType::ConstructionYard));

        let credits = game.status().gdi_credits;
        assert!(!game.queue_building(0, StructureType::PowerPlant));
        assert_eq!(game.status().gdi_credits, credits);
    }

    #[test]
    fn queue_building_completes_to_ready() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let credits = game.status().gdi_credits;

        assert!(game.queue_building(0, StructureType::PowerPlant));
        assert_eq!(game.status().gdi_credits, credits - 300);
        assert!(game.ready_building_for(0).is_none());

        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.ready_building_for(0).is_some() {
                break;
            }
        }

        assert_eq!(game.ready_building_for(0), Some(StructureType::PowerPlant));
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn place_ready_building_adds_structure() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert!(game.queue_building(0, StructureType::PowerPlant));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.ready_building_for(0).is_some() {
                break;
            }
        }

        let mut cell = None;
        'search: for y in 0..32 {
            for x in 0..32 {
                let candidate = Vec2i::new(x, y);
                if game.can_place_building(candidate) {
                    cell = Some(candidate);
                    break 'search;
                }
            }
        }
        let cell = cell.expect("there should be a free buildable cell");

        let buildings_before = game.building_count();
        let power_before = game.status().gdi_power;
        assert!(game.place_ready_building(0, cell));
        assert_eq!(game.building_count(), buildings_before + 1);
        assert_eq!(game.status().gdi_power, power_before + 100);
        assert!(game.ready_building_for(0).is_none());
    }

    #[test]
    fn place_building_rejects_occupied_cell() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert!(game.queue_building(0, StructureType::PowerPlant));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.ready_building_for(0).is_some() {
                break;
            }
        }

        let occupied = game
            .buildings
            .iter()
            .find(|building| building.owner == 0)
            .map(Game::building_cell)
            .expect("GDI has buildings");

        assert!(!game.place_ready_building(0, occupied));
        assert_eq!(game.ready_building_for(0), Some(StructureType::PowerPlant));
    }

    #[test]
    fn house_spends_tiberium_before_credits() {
        let mut house = House::new(ClassId::HouseGDI, "GDI".to_string());
        house.add_credits(100);
        house.capacity = 1000;
        house.harvested(500);

        assert_eq!(house.available_money(), 600);
        assert!(house.spend_credits(300));
        assert_eq!(house.tiberium, 200);
        assert_eq!(house.credits, 100);

        assert!(house.spend_credits(300));
        assert_eq!(house.tiberium, 0);
        assert_eq!(house.credits, 0);
        assert!(!house.spend_credits(1));
    }

    #[test]
    fn house_harvested_caps_at_capacity() {
        let mut house = House::new(ClassId::HouseGDI, "GDI".to_string());
        house.capacity = 1000;
        assert_eq!(house.harvested(1500), 1000);
        assert_eq!(house.tiberium, 1000);
        assert_eq!(house.harvested_credits, 1500);
    }

    #[test]
    fn silo_increases_storage_capacity() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert_eq!(game.status().gdi_capacity, 1000);

        game.buildings.push(Building::from_type(
            StructureType::Silo,
            Game::cell_center(6, 6),
            0,
        ));
        game.refresh_house_power();
        assert_eq!(game.status().gdi_capacity, 2500);
    }

    fn mammoth_duel(range: f32, expected: WeaponType) {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        let position = Game::cell_center(5, 8);
        let target = Vec2::new(
            Fixed16_16::from_f32(position.x.to_f32() + range),
            position.y,
        );
        game.units
            .push(Unit::new(UnitType::MammothTank, position, 0));
        game.units.push(Unit::new(UnitType::InfantryE1, target, 1));
        game.units[0].is_selected = true;
        assert_eq!(game.order_attack(1), 1);

        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.projectiles.is_empty() {
                break;
            }
        }

        assert_eq!(
            game.projectiles[0].weapon.weapon_type, expected,
            "expected {:?} at range {range}",
            expected
        );
    }

    #[test]
    fn mammoth_uses_primary_inside_range_and_tusk_outside() {
        mammoth_duel(100.0, WeaponType::Cannon120);
        mammoth_duel(117.0, WeaponType::MammothTusk);
    }

    #[test]
    fn ion_cannon_requires_building_and_charge() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        let target = Vec2i::new(8, 8);
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 8), 1));

        assert!(!game.fire_ion_cannon(0, target), "no ion cannon building");

        game.buildings.push(Building::from_type(
            StructureType::IonCannon,
            Game::cell_center(4, 4),
            0,
        ));
        assert!(!game.fire_ion_cannon(0, target), "not charged");

        game.houses[0].ion_charge = Fixed16_16::ONE;
        let health = game.units[0].health;
        assert!(game.fire_ion_cannon(0, target));
        assert!(
            game.units[0].health < health,
            "ion blast should damage units at the target"
        );
        assert_eq!(game.houses[0].ion_charge, Fixed16_16::ZERO);
        assert!(!game.blasts.is_empty());
        assert_eq!(game.beams.len(), 1, "Ion Cannon should draw a beam");

        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
        }
        assert!(game.beams.is_empty(), "beam fades out");
    }

    #[test]
    fn nuke_hits_wide_area_and_buildings() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.houses
            .push(House::new(ClassId::HouseNod, "Nod".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::TempleOfNod,
            Game::cell_center(2, 2),
            1,
        ));
        game.houses[1].nuke_charge = Fixed16_16::ONE;

        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 8), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(10, 8), 0));
        game.buildings.push(Building::from_type(
            StructureType::Barracks,
            Game::cell_center(9, 8),
            0,
        ));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(14, 8), 0));
        for unit in &mut game.units {
            // Hold position instead of drifting on the idle patrol ring.
            unit.ordered = true;
        }

        let unit_health = game.units[0].health;
        let far_health = game.units[1].health;
        let outside_health = game.units[2].health;
        let building_health = game.buildings[1].health;

        assert!(game.fire_nuke(1, Vec2i::new(8, 8)));
        assert_eq!(
            game.units[0].health, unit_health,
            "per-cell damage is deferred until the blast animation advances"
        );
        assert_eq!(game.houses[1].nuke_charge, Fixed16_16::ZERO);
        assert_eq!(game.strikes.len(), 1, "missile launch should be tracked");

        // During the missile flight the target is still undamaged.
        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
        }
        assert_eq!(game.units[0].health, unit_health, "missile still in flight");

        // After the flight and the staggered per-cell damage window.
        for _ in 0..80 {
            game.update(Fixed16_16::from_f32(0.05));
        }
        assert!(game.strikes.is_empty());
        assert!(game.units[0].health < unit_health);
        assert!(game.units[1].health < far_health, "blast covers 4 cells");
        assert_eq!(
            game.units[2].health, outside_health,
            "6 cells away is outside the atom blast"
        );
        assert!(
            game.buildings[1].health < building_health,
            "buildings take nuke damage"
        );
    }

    #[test]
    fn superweapons_charge_over_time() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::IonCannon,
            Game::cell_center(4, 4),
            0,
        ));

        for _ in 0..599 {
            game.update(Fixed16_16::from_int(1));
        }
        assert!(game.houses[0].ion_charge.to_f32() > 0.99);
    }

    #[test]
    fn group_moves_reserve_distinct_cells_until_arrival() {
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(2, 8), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(4, 8), 0));
        game.select_all_owned(0);

        assert_eq!(game.order_move(Game::cell_center(10, 8)), 2);
        let first = game.units[0].reserved_cell.expect("first reservation");
        let second = game.units[1].reserved_cell.expect("second reservation");
        assert_ne!(first, second, "each unit reserves a different next cell");

        for _ in 0..3000 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].target.is_none() && game.units[1].target.is_none() {
                break;
            }
        }
        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(game.units[0].reserved_cell.is_none());
        assert!(game.units[1].reserved_cell.is_none());
    }

    #[test]
    fn homing_projectile_retargets_on_even_frames() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_center(10, 10),
            1,
        ));

        let initial_target = Game::cell_center(10, 10);
        let projectile = Projectile::new(
            Weapon::from_type(WeaponType::Dragon),
            Game::cell_center(1, 1),
            initial_target,
            0,
            Some(0),
        );
        assert!(projectile.homing);
        game.projectiles.push(projectile);

        // Odd frame: missile keeps its old aim.
        game.update(Fixed16_16::from_f32(0.05));
        assert_eq!(game.projectiles[0].target, initial_target);

        // Move the target, then advance until the missile checks on an even age.
        game.units[0].position = Game::cell_center(12, 12);
        game.update(Fixed16_16::from_f32(0.05));
        game.update(Fixed16_16::from_f32(0.05));
        assert_eq!(game.projectiles[0].target, Game::cell_center(12, 12));
    }

    #[test]
    fn aircraft_fire_consumes_ammo() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::Orca, Game::cell_center(8, 8), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 10), 1));
        game.units[0].is_selected = true;
        assert_eq!(game.order_attack(1), 1);

        let start = game.units[0].ammo;
        for _ in 0..200 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].ammo < start {
                break;
            }
        }

        assert!(game.projectiles_fired > 0, "aircraft should fire");
        assert!(game.units[0].ammo < start, "firing should consume ammo");
    }

    #[test]
    fn aircraft_flies_over_impassable_terrain() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::Orca, Game::cell_center(1, 8), 0));
        game.units[0].is_selected = true;
        for y in 0..16 {
            game.map.grid.set_passable(Vec2i::new(8, y), false);
        }

        assert_eq!(game.order_move(Game::cell_center(15, 8)), 1);
        assert!(
            game.units[0].path.is_empty(),
            "aircraft ignore ground paths"
        );

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].target.is_none() {
                break;
            }
        }

        let x = game.units[0].position.x.to_f32();
        assert!((x - 15.0 * TILE_SIZE - TILE_SIZE * 0.5).abs() < 5.0);
    }

    #[test]
    fn aircraft_rotor_and_vehicle_tracks_animate_while_moving() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::Orca, Game::cell_center(2, 2), 0));
        game.units
            .push(Unit::new(UnitType::MediumTank, Game::cell_center(2, 6), 0));
        game.select_all_owned(0);
        assert_eq!(game.order_move(Game::cell_center(8, 2)), 2);

        for _ in 0..10 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(game.units[0].rotor_phase > 0.0, "rotor should spin");
        assert!(game.units[1].track_phase > 0.0, "tracks should animate");
    }

    #[test]
    fn aircraft_rearms_at_helipad() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.buildings.push(Building::from_type(
            StructureType::Helipad,
            Game::cell_center(4, 4),
            0,
        ));
        let mut orca = Unit::new(UnitType::Orca, Game::cell_center(6, 4), 0);
        orca.ammo = 0;
        game.units.push(orca);

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].ammo > 0 {
                break;
            }
        }

        assert!(game.units[0].ammo > 0, "helipad should rearm the aircraft");
    }

    #[test]
    fn status_reports_mission_and_credits() {
        let mut game = Game::new("HUD test", 16, 16);
        game.setup_demo();

        let status = game.status();
        assert_eq!(status.mission, "HUD test");
        assert!(status.units >= 1);
        assert!(status.buildings >= 1);
        assert!(status.gdi_credits > 0);
        assert!(status.nod_credits > 0);
    }
}

#[cfg(test)]
mod rules_integration_tests {
    use super::*;
    use cnc_core::rules::RulesDatabase;

    fn rules(source: &str) -> RulesDatabase {
        RulesDatabase::from_str(source).expect("rules should parse")
    }

    #[test]
    fn ini_names_match_original_data_files() {
        assert_eq!(UnitType::InfantryE1.ini_name(), "E1");
        assert_eq!(UnitType::MediumTank.ini_name(), "MTNK");
        assert_eq!(UnitType::MammothTank.ini_name(), "HTNK");
        assert_eq!(UnitType::Harvester.ini_name(), "HARV");
        assert_eq!(UnitType::Orca.ini_name(), "ORCA");
        assert_eq!(UnitType::Apache.ini_name(), "HELI");

        assert_eq!(StructureType::ConstructionYard.ini_name(), "FACT");
        assert_eq!(StructureType::PowerPlant.ini_name(), "NUKE");
        assert_eq!(StructureType::Refinery.ini_name(), "PROC");
        assert_eq!(StructureType::Barracks.ini_name(), "PYLE");
        assert_eq!(StructureType::WarFactory.ini_name(), "WEAP");
        assert_eq!(StructureType::Helipad.ini_name(), "HPAD");
        assert_eq!(StructureType::IonCannon.ini_name(), "EYE");
        assert_eq!(StructureType::TempleOfNod.ini_name(), "TMPL");
    }

    #[test]
    fn empty_rules_keep_original_costs() {
        let game = Game::new("test", 32, 32);
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 100);
        assert_eq!(game.unit_cost(UnitType::MammothTank), 1500);
        assert_eq!(game.building_cost(StructureType::PowerPlant), 300);
        assert_eq!(game.unit_strength(UnitType::InfantryE1), 50);
        assert_eq!(game.building_strength(StructureType::PowerPlant), 200);
    }

    #[test]
    fn rules_override_unit_cost_and_charge() {
        let rules = rules("[E1]\nCost=40\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.units.clear();

        assert_eq!(game.unit_cost(UnitType::InfantryE1), 40);
        let credits = game.status().gdi_credits;
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        assert_eq!(game.status().gdi_credits, credits - 40);
    }

    #[test]
    fn rules_override_building_cost_and_charge() {
        let rules = rules("[NUKE]\nCost=100\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();

        assert_eq!(game.building_cost(StructureType::PowerPlant), 100);
        let credits = game.status().gdi_credits;
        assert!(game.queue_building(0, StructureType::PowerPlant));
        assert_eq!(game.status().gdi_credits, credits - 100);
    }

    #[test]
    fn rules_override_strength_when_unit_spawns() {
        let rules = rules("[E1]\nCost=100\nStrength=200\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.units.clear();

        assert!(game.queue_unit(0, UnitType::InfantryE1));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.units.is_empty() {
                break;
            }
        }
        assert_eq!(game.units.len(), 1);
        assert_eq!(game.units[0].max_health, Fixed16_16::from_int(200));
        assert_eq!(game.units[0].health, Fixed16_16::from_int(200));
    }

    #[test]
    fn rules_override_strength_when_building_is_placed() {
        let rules = rules("[NUKE]\nStrength=350\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.ready_buildings.push((0, StructureType::PowerPlant));

        let cell = Vec2i::new(1, 1);
        assert!(game.can_place_building(cell));
        assert!(game.place_ready_building(0, cell));
        let building = game
            .buildings
            .iter()
            .rev()
            .find(|building| building.structure_type() == Some(StructureType::PowerPlant))
            .expect("power plant should exist");
        assert_eq!(building.max_health, Fixed16_16::from_int(350));
        assert_eq!(building.health, Fixed16_16::from_int(350));
    }

    #[test]
    fn unknown_type_falls_back_to_static_stats() {
        let rules = rules("[E1]\nCost=40\n");
        let game = Game::with_rules("test", 32, 32, rules);
        // MTNK is absent from the rules file, so the UDATA.CPP value applies.
        assert_eq!(game.unit_cost(UnitType::MediumTank), 800);
    }

    #[test]
    fn load_rules_from_disk_reports_type_count() {
        let dir = std::env::temp_dir().join(format!(
            "cnc_td_rules_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("rules.ini");
        std::fs::write(&path, "[E1]\nCost=75\n[FACT]\nCost=4000\n").unwrap();

        let mut game = Game::new("test", 32, 32);
        let count = game.load_rules(&path).unwrap();
        assert_eq!(count, 2);
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 75);
        assert_eq!(game.building_cost(StructureType::ConstructionYard), 4000);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn set_rules_replaces_database() {
        let mut game = Game::new("test", 32, 32);
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 100);
        game.set_rules(rules("[E1]\nCost=25\n"));
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 25);
        game.set_rules(RulesDatabase::default());
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 100);
    }
}

#[cfg(test)]
mod weapon_rules_tests {
    use super::*;
    use crate::td_rules::{ticks_to_seconds, ArmorType};
    use cnc_core::rules::RulesDatabase;

    fn rules(source: &str) -> RulesDatabase {
        RulesDatabase::from_str(source).expect("rules should parse")
    }

    #[test]
    fn weapon_ini_names_match_rules_tables() {
        assert_eq!(WeaponType::M16.ini_name(), "M16");
        assert_eq!(WeaponType::Cannon105.ini_name(), "105mm");
        assert_eq!(WeaponType::MammothTusk.ini_name(), "MammothTusk");
        assert_eq!(WeaponType::ObeliskLaser.ini_name(), "ObeliskLaser");
        assert_eq!(WarheadType::Sa.ini_name(), "SA");
        assert_eq!(WarheadType::HollowPoint.ini_name(), "HollowPoint");
        assert_eq!(WarheadType::from_ini_name("ap"), Some(WarheadType::Ap));
        assert_eq!(WarheadType::from_ini_name("NOPE"), None);
    }

    #[test]
    fn weapon_damage_rof_and_range_come_from_rules() {
        let rules = rules("[M16]\nDamage=25\nROF=10\nRange=1280\n");
        let game = Game::with_rules("test", 32, 32, rules);
        let weapon = game.make_weapon(WeaponType::M16);

        assert_eq!(weapon.damage, Fixed16_16::from_int(25));
        assert_eq!(weapon.range, Fixed16_16::from_f32(leptons_to_px(1280)));
        assert_eq!(
            weapon.rate_of_fire,
            Fixed16_16::from_f32(ticks_to_seconds(13))
        );
        // The static M16 row stays untouched for games without a rules file.
        let plain = Weapon::from_type(WeaponType::M16);
        assert_eq!(plain.damage, Fixed16_16::from_int(15));
        assert_ne!(plain.range, weapon.range);
    }

    #[test]
    fn weapon_warhead_override_selects_rules_warhead() {
        let rules = rules("[Grenade]\nWarhead=AP\n[AP]\nSpread=0\nVerses=10,20,30,40,50\n");
        let game = Game::with_rules("test", 32, 32, rules);
        let weapon = game.make_weapon(WeaponType::Grenade);

        assert_eq!(weapon.warhead.warhead_type, WarheadType::Ap);
        let base = Fixed16_16::from_int(100);
        assert_eq!(
            weapon.warhead.damage(base, ArmorType::Steel, 0),
            Fixed16_16::from_int(40)
        );
        assert_eq!(
            weapon.warhead.damage(base, ArmorType::None, 0),
            Fixed16_16::from_int(10)
        );
    }

    #[test]
    fn warhead_verses_override_without_weapon_section() {
        let rules = rules("[AP]\nSpread=0\nVerses=5,10,20,40,80\n");
        let game = Game::with_rules("test", 32, 32, rules);
        // M16's static warhead is SA, but an explicit warhead only applies to AP.
        let m16 = game.make_weapon(WeaponType::M16);
        assert!(m16.warhead.verses.is_none());

        let profile = game.warhead_profile(WarheadType::Ap);
        assert_eq!(
            profile.damage(Fixed16_16::from_int(100), ArmorType::Concrete, 0),
            Fixed16_16::from_int(80)
        );
    }

    #[test]
    fn rules_spread_attenuates_damage_with_distance() {
        let rules = rules("[AP]\nSpread=2\nVerses=100,100,100,100,100\n");
        let game = Game::with_rules("test", 32, 32, rules);
        let profile = game.warhead_profile(WarheadType::Ap);
        let near = profile.damage(Fixed16_16::from_int(100), ArmorType::Steel, 0);
        let far = profile.damage(Fixed16_16::from_int(100), ArmorType::Steel, 8);
        assert_eq!(near, Fixed16_16::from_int(100));
        assert_eq!(far, Fixed16_16::from_int(25));
    }

    #[test]
    fn set_rules_refreshes_existing_unit_weapons() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.set_rules(rules("[M16]\nDamage=99\nRange=2048\n"));

        let infantry = game
            .units
            .iter()
            .find(|unit| unit.unit_type == UnitType::InfantryE1)
            .expect("demo GDI infantry");
        let weapon = infantry.weapon.as_ref().expect("rifle");
        assert_eq!(weapon.damage, Fixed16_16::from_int(99));
        assert_eq!(weapon.range, Fixed16_16::from_f32(leptons_to_px(2048)));
    }

    #[test]
    fn production_spawns_units_with_rules_weapons() {
        let rules = rules("[E1]\nCost=100\n[M16]\nDamage=42\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.units.clear();
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.units.is_empty() {
                break;
            }
        }
        let unit = game.units.first().expect("produced infantry");
        assert_eq!(
            unit.weapon.as_ref().unwrap().damage,
            Fixed16_16::from_int(42)
        );
    }
}

#[cfg(test)]
mod prerequisite_tests {
    use super::*;
    use crate::ai::AIDifficulty;
    use cnc_core::rules::RulesDatabase;

    fn rules(source: &str) -> RulesDatabase {
        RulesDatabase::from_str(source).expect("rules should parse")
    }

    #[test]
    fn types_without_rules_have_no_prerequisites() {
        let game = Game::new("test", 32, 32);
        assert!(game.meets_prerequisites(0, "E1"));
        assert!(game.meets_prerequisites(0, "MISSING"));
        assert!(game.missing_prerequisites(0, "E1").is_empty());
    }

    #[test]
    fn unit_prerequisite_requires_owned_structure() {
        let database = rules("[E1]\nCost=100\nPrerequisite=WEAP\n");
        let mut game = Game::with_rules("test", 32, 32, database);
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));

        game.set_rules(rules("[E1]\nCost=100\nPrerequisite=EYE\n"));
        assert!(!game.meets_prerequisites(0, "E1"));
        assert_eq!(game.missing_prerequisites(0, "E1"), vec!["EYE"]);

        game.units.clear();
        let credits = game.status().gdi_credits;
        assert!(!game.queue_unit(0, UnitType::InfantryE1));
        assert_eq!(game.status().gdi_credits, credits);
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn building_prerequisite_is_enforced() {
        let database = rules("[NUKE]\nCost=300\nPrerequisite=TMPL\n");
        let mut game = Game::with_rules("test", 32, 32, database);
        game.setup_demo();
        assert!(!game.can_queue_building(0, StructureType::PowerPlant));
        assert!(!game.queue_building(0, StructureType::PowerPlant));

        game.set_rules(rules("[NUKE]\nCost=300\nPrerequisite=FACT\n"));
        assert!(game.can_queue_building(0, StructureType::PowerPlant));
        assert!(game.queue_building(0, StructureType::PowerPlant));
    }

    #[test]
    fn none_and_faction_tokens_are_supported() {
        let game = Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=NONE\n"));
        assert!(game.meets_prerequisites(0, "E1"));

        let mut game =
            Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=GDI\n"));
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));
        assert!(!game.meets_prerequisites(1, "E1"));
        assert_eq!(game.missing_prerequisites(1, "E1"), vec!["GDI"]);

        let mut game =
            Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=NOD\n"));
        game.setup_demo();
        assert!(game.meets_prerequisites(1, "E1"));
        assert!(!game.meets_prerequisites(0, "E1"));
    }

    #[test]
    fn gdi_and_nod_barracks_aliases_are_equivalent() {
        let mut game =
            Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=PYLE\n"));
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));

        game.set_rules(rules("[E1]\nCost=100\nPrerequisite=HAND\n"));
        // The demo GDI barracks satisfies the Nod spelling through the alias.
        assert!(game.meets_prerequisites(0, "E1"));
    }

    #[test]
    fn prerequisite_lists_require_every_token() {
        let mut game = Game::with_rules(
            "test",
            32,
            32,
            rules("[E1]\nCost=100\nPrerequisite=WEAP,FACT\n"),
        );
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));

        game.set_rules(rules("[E1]\nCost=100\nPrerequisite=WEAP,EYE\n"));
        assert!(!game.meets_prerequisites(0, "E1"));
        assert_eq!(game.missing_prerequisites(0, "E1"), vec!["EYE"]);
    }

    #[test]
    fn unknown_prerequisite_fails_closed() {
        let mut game = Game::with_rules(
            "test",
            32,
            32,
            rules("[E1]\nCost=100\nPrerequisite=NOT_A_BUILDING\n"),
        );
        game.setup_demo();
        assert!(!game.meets_prerequisites(0, "E1"));
        assert_eq!(game.missing_prerequisites(0, "E1"), vec!["NOT_A_BUILDING"]);
    }

    #[test]
    fn prerequisite_only_sections_are_registered_as_types() {
        let rules = rules("[E1]\nPrerequisite=WEAP\n");
        assert_eq!(rules.type_rule("E1").unwrap().prerequisite, vec!["WEAP"]);
    }

    #[test]
    fn tech_level_gates_unit_and_structure_production() {
        let mut game = Game::new("tech", 32, 32);
        game.setup_demo();

        assert_eq!(game.unit_tech_level(UnitType::MammothTank), 5);
        assert_eq!(game.unit_tech_level(UnitType::InfantryE1), 1);
        assert_eq!(game.building_tech_level(StructureType::Helipad), 6);
        assert_eq!(game.building_tech_level(StructureType::PowerPlant), 0);

        game.houses[0].tech_level = 1;
        assert!(game.can_queue_unit(0, UnitType::InfantryE1));
        assert!(!game.can_queue_unit(0, UnitType::MammothTank));
        assert!(!game.can_queue_building(0, StructureType::Helipad));

        game.houses[0].tech_level = 6;
        assert!(game.can_queue_unit(0, UnitType::MammothTank));
        assert!(game.can_queue_building(0, StructureType::Helipad));
    }

    #[test]
    fn rules_tech_level_override_replaces_the_fallback() {
        let mut game = Game::with_rules(
            "tech-override",
            32,
            32,
            rules("[MTNK]\nTechLevel=7\n[HPAD]\nTechLevel=9\n"),
        );
        game.setup_demo();
        assert_eq!(game.unit_tech_level(UnitType::MediumTank), 7);
        assert_eq!(game.building_tech_level(StructureType::Helipad), 9);

        game.houses[0].tech_level = 6;
        assert!(!game.can_queue_unit(0, UnitType::MediumTank));
        game.houses[0].tech_level = 7;
        assert!(game.can_queue_unit(0, UnitType::MediumTank));
    }

    #[test]
    fn sidebar_hides_tech_and_prerequisite_locked_rows() {
        let mut game = Game::with_rules("sidebar", 32, 32, rules("[MTNK]\nPrerequisite=TMPL\n"));
        game.setup_demo();

        let visible = game.visible_sidebar_options(0);
        assert!(visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::InfantryE1)));
        assert!(!visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MediumTank)));

        let all = game.sidebar_options(0);
        let mtnk = all
            .iter()
            .find(|option| option.buildable == Buildable::Unit(UnitType::MediumTank))
            .expect("medium tank row exists");
        assert!(!mtnk.visible);
        assert_eq!(mtnk.missing_prerequisites, vec!["TMPL"]);

        // TechLevel 1 still shows cheap units but hides the high-tier rows.
        game.houses[0].tech_level = 1;
        let visible = game.visible_sidebar_options(0);
        assert!(visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::InfantryE1)));
        assert!(!visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MammothTank)));
        assert!(!visible
            .iter()
            .any(|option| { option.buildable == Buildable::Structure(StructureType::Helipad) }));
    }

    #[test]
    fn sidebar_keeps_rows_visible_when_unaffordable() {
        let mut game = Game::new("sidebar-cost", 32, 32);
        game.setup_demo();
        game.houses[0].credits = 0;
        game.houses[0].tiberium = 0;

        let e1 = game
            .sidebar_options(0)
            .into_iter()
            .find(|option| option.buildable == Buildable::Unit(UnitType::InfantryE1))
            .expect("infantry row exists");
        assert!(e1.visible);
        assert!(!e1.affordable);
        assert!(e1.has_producer);
    }

    #[test]
    fn sidebar_filters_owner_locked_rows() {
        let mut game = Game::with_rules(
            "sidebar-owner",
            32,
            32,
            rules(
                "[MTNK]\nOwner=GDI\n[HTNK]\nOwner=Nod\n[BGGY]\nOwner=Nod\n\
                 [TMPL]\nOwner=Nod\nTechLevel=1\n[EYE]\nOwner=GDI\nTechLevel=1\n",
            ),
        );
        game.setup_demo();

        let gdi = game.visible_sidebar_options(0);
        assert!(gdi
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MediumTank)));
        assert!(!gdi
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MammothTank)));
        assert!(!gdi
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::Buggy)));
        assert!(!gdi
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::TempleOfNod)));
        assert!(gdi
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::IonCannon)));
        // The full table still lists every row, just with visible = false.
        assert_eq!(game.sidebar_options(0).len(), UnitType::ALL.len() + StructureType::ALL.len());

        let nod = game.visible_sidebar_options(1);
        assert!(!nod
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MediumTank)));
        assert!(nod
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::Buggy)));
        assert!(nod
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::TempleOfNod)));
        assert!(!nod
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::IonCannon)));
    }

    #[test]
    fn faction_ownership_uses_owner_tokens_and_td_fallback() {
        let mut game = Game::with_rules("owner-tokens", 32, 32, rules("[MTNK]\nOwner=Nod,ANY\n"));
        game.setup_demo();

        // Owner=ANY admits both sides, Owner=Nod locks the medium tank to Nod.
        assert!(game.unit_owned_by_faction(0, UnitType::MediumTank));
        assert!(game.unit_owned_by_faction(1, UnitType::MediumTank));

        // No rules entry: TD defaults (Mammoth is GDI-only, Buggy Nod-only).
        assert!(game.unit_owned_by_faction(0, UnitType::MammothTank));
        assert!(!game.unit_owned_by_faction(1, UnitType::MammothTank));
        assert!(!game.unit_owned_by_faction(0, UnitType::Buggy));
        assert!(game.unit_owned_by_faction(1, UnitType::Buggy));
        assert!(game.structure_owned_by_faction(0, StructureType::IonCannon));
        assert!(!game.structure_owned_by_faction(1, StructureType::IonCannon));
        assert!(game.structure_owned_by_faction(1, StructureType::TempleOfNod));
    }

    #[test]
    fn ai_production_uses_rules_cost_and_tech_level() {
        let ai = AI::new(0, AIDifficulty::Normal);
        let mut game = Game::with_rules(
            "ai-production",
            32,
            32,
            rules("[MTNK]\nCost=800\nTechLevel=3\n[HTNK]\nCost=10\nTechLevel=7\n"),
        );
        game.setup_demo();

        game.houses[0].tech_level = 6;
        assert_ne!(
            game.ai_production_choice(0, &ai, 3000),
            Some(UnitType::MammothTank),
            "rules TechLevel 7 must stay locked at build level 6"
        );

        // At full tech the 10-credit Mammoth dominates the weighted table.
        game.houses[0].tech_level = 7;
        assert_eq!(
            game.ai_production_choice(0, &ai, 3000),
            Some(UnitType::MammothTank)
        );
        assert!(game.ai_queue_production(0, &ai, 0).is_some());
    }

    #[test]
    fn cancelling_td_production_refunds_the_front_item() {
        let mut game = Game::new("cancel", 64, 64);
        game.setup_demo();
        let before = game.houses[0].credits;
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        assert!(game.houses[0].credits < before);
        let refund = game.cancel_production(0).expect("front item");
        assert!(refund > 0);
        assert_eq!(game.houses[0].credits, before);
        assert!(game.houses[0].production_queue.is_empty());
        assert_eq!(game.cancel_production(0), None);
    }

    #[test]
    fn repairing_a_building_costs_and_heals() {
        let mut game = Game::new("repair", 64, 64);
        game.setup_demo();
        let index = game
            .buildings
            .iter()
            .position(|building| building.owner == 0 && building.is_alive())
            .unwrap();
        game.buildings[index].health = game.buildings[index].max_health / Fixed16_16::from_int(2);
        let credits = game.houses[0].credits;
        let cost = game.repair_building(0, index).expect("repair should succeed");
        assert!(cost > 0);
        assert_eq!(game.houses[0].credits, credits - cost);
        assert_eq!(game.buildings[index].health, game.buildings[index].max_health);
        assert_eq!(game.repair_building(0, index), None, "already full");
        game.buildings[index].health = game.buildings[index].max_health / Fixed16_16::from_int(4);
        assert_eq!(game.repair_building(1, index), None, "wrong owner");
    }

    #[test]
    fn selling_a_building_refunds_half() {
        let mut game = Game::new("sell", 64, 64);
        game.setup_demo();
        let index = game
            .buildings
            .iter()
            .position(|building| building.owner == 0 && building.is_alive())
            .unwrap();
        let structure = game.buildings[index].structure_type().unwrap();
        let cost = game.building_cost(structure);
        let credits = game.houses[0].credits;
        let refund = game.sell_building(0, index).unwrap();
        assert_eq!(refund, cost / 2);
        assert_eq!(game.houses[0].credits, credits + refund);
        assert!(!game.buildings[index].is_alive());
        assert!(game.buildings[index].is_selling);
        assert_eq!(game.sell_building(0, index), None, "already sold");
        let enemy = game
            .buildings
            .iter()
            .position(|building| building.owner == 1 && building.is_alive())
            .unwrap();
        assert_eq!(game.sell_building(0, enemy), None, "wrong owner");
    }

    #[test]
    fn nod_mission_controls_the_nod_house() {
        let maps = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
        let mut game = Game::new("nod-test", 64, 64);
        game.setup_demo();
        game.load_mission(&maps.join("02.ini")).unwrap();
        assert_eq!(game.player_owner, 1);

        let position = game
            .units
            .iter()
            .find(|unit| unit.owner == 1 && unit.is_alive())
            .map(|unit| unit.position)
            .expect("Nod units missing");
        assert!(
            game.select_at_additive(position, false).is_some(),
            "clicking a Nod unit selects it"
        );
        assert!(game.select_all_owned(1) > 0);

        let gdi_position = game
            .units
            .iter()
            .find(|unit| unit.owner == 0 && unit.is_alive())
            .map(|unit| unit.position)
            .expect("GDI units missing");
        assert!(game.enemy_at(gdi_position).is_some());

        let path = std::env::temp_dir().join(format!("nod-player-{}.sav", std::process::id()));
        game.save_to_file(&path).unwrap();
        let loaded = Game::load_from_file(&path).unwrap();
        assert_eq!(loaded.player_owner, 1, "the save keeps the mission side");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn owner_targeted_orders_only_touch_listed_units() {
        let mut game = Game::new("net", 32, 32);
        game.setup_demo();
        let (gdi_id, gdi_start) = game
            .units
            .iter()
            .enumerate()
            .find(|(_, unit)| unit.owner == 0 && unit.is_alive())
            .map(|(index, unit)| (index as u32, unit.position))
            .unwrap();
        let (nod_id, nod_start) = game
            .units
            .iter()
            .enumerate()
            .find(|(_, unit)| unit.owner == 1 && unit.is_alive())
            .map(|(index, unit)| (index as u32, unit.position))
            .unwrap();
        let target = Vec2::new(
            Fixed16_16::from_f32(gdi_start.x.to_f32() + 6.0 * TILE_SIZE),
            Fixed16_16::from_f32(gdi_start.y.to_f32() + 2.0 * TILE_SIZE),
        );
        assert_eq!(game.order_units_move(0, &[gdi_id], target), 1);
        assert!(game.units[gdi_id as usize].target.is_some());
        assert!(game.units[nod_id as usize].target.is_none(), "other owner untouched");

        // A mismatched owner id is ignored.
        assert_eq!(game.order_units_move(1, &[gdi_id], target), 0);

        // Stop clears the order but not the unit.
        assert_eq!(game.order_units_stop(0, &[gdi_id]), 1);
        assert!(game.units[gdi_id as usize].target.is_none());
        assert!(game.units[gdi_id as usize].path.is_empty());

        // Attack only accepts an enemy of the commanding owner.
        assert_eq!(game.order_units_attack(0, &[gdi_id], nod_id as usize), 1);
        assert_eq!(
            game.units[gdi_id as usize].attack_target,
            Some(nod_id as usize)
        );
        assert_eq!(
            game.order_units_attack(1, &[nod_id], nod_id as usize),
            0,
            "cannot attack yourself"
        );

        // A GDI id cannot be commanded by the Nod player.
        assert_eq!(game.order_units_attack(1, &[gdi_id], nod_id as usize), 0);

        // Dead units in the list are skipped.
        let _ = nod_start;
        game.units[gdi_id as usize].health = Fixed16_16::ZERO;
        assert_eq!(game.order_units_stop(0, &[gdi_id]), 0);
    }
}
