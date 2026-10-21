//! Original Red Alert `.MPR` scenarios: load the retail INI dialect and
//! build a playable [`Game`].
//!
//! The parsing lives in [`crate::original_scenario`] (one shared
//! `OriginalScenario` for every original-format consumer); this module is the
//! Game-facing entry point. It layers the retail house list and the `[Trigs]`
//! table on top of [`Game::from_original_scenario`], which already places
//! terrain, objects and `[TeamTypes]`.
//!
//! Sections covered: `[Basic]`, `[MapPack]`, `[OverlayPack]`, `[Houses]`,
//! `[Infantry]`, `[Structures]`, `[Units]`, `[Aircraft]`, `[Ships]`,
//! `[Waypoints]`, `[Trigs]`, `[TeamTypes]`, `[Base]`, `[SMUDGE]`, `[TERRAIN]`.
//!
//! References: `REDALERT/SCENARIO.CPP` `Read_INI` (section dispatch),
//! `REDALERT/DISPLAY.CPP` / `MAP.CPP` (MapPack), `REDALERT/OVERLAY.CPP`
//! (OverlayPack), `REDALERT/TRIGTYPE.CPP` (Triggers), `REDALERT/BASE.CPP`,
//! `REDALERT/SMUDGE.CPP`, `REDALERT/TERRAIN.CPP`, `REDALERT/HOUSE.CPP`.

use std::path::Path;

use anyhow::Result;

use cnc_core::Vec2i;

use crate::game::{Game, RunningTrigger};
use crate::house::Faction;
use crate::original_scenario::{OriginalScenario, OriginalTrigger};

/// Load an original `.MPR`/`.INI` scenario from disk.
pub fn load_mpr(path: impl AsRef<Path>) -> Result<OriginalScenario> {
    OriginalScenario::from_file(path)
}

/// Load an original `.MPR`/`.INI` scenario and build its game in one step.
pub fn load_mpr_game(path: impl AsRef<Path>) -> Result<Game> {
    Ok(build_game(&load_mpr(path)?))
}

/// Build a playable game from a parsed original scenario.
///
/// [`Game::from_original_scenario`] handles terrain, object placement,
/// `[TeamTypes]` and `[CellTriggers]`; this adds the `[Houses]` roster and the
/// `[Trigs]` table, then binds cell triggers to the newly created triggers.
pub fn build_game(scenario: &OriginalScenario) -> Game {
    let mut game = Game::from_original_scenario(scenario);
    apply_houses(&mut game, scenario);
    push_triggers(&mut game, scenario);
    game
}

/// Rename/credit the two faction slots from the `[Houses]` list.
///
/// The port keeps one Allied and one Soviet house, so each parsed house is
/// applied to the slot with its faction; a third country has no slot and is
/// ignored (retail campaign maps field at most one country per side).
fn apply_houses(game: &mut Game, scenario: &OriginalScenario) {
    for house in &scenario.houses {
        let Some(faction) = faction_for(&house.name) else {
            continue;
        };
        let Some(slot) = game.houses.iter().position(|slot| slot.faction == faction) else {
            continue;
        };
        let target = &mut game.houses[slot];
        target.name = house.name.clone();
        target.credits = house.credits;
        if let Some(tech_level) = house.tech_level {
            target.tech_level = tech_level.min(u32::from(u8::MAX)) as u8;
        }
    }
}

/// Append `[Trigs]` entries to the game trigger table and bind their cells.
fn push_triggers(game: &mut Game, scenario: &OriginalScenario) {
    for trigger in &scenario.triggers {
        game.triggers.push(RunningTrigger {
            id: trigger.id,
            name: trigger.name.clone(),
            condition: trigger_condition(trigger),
            action: trigger_action(trigger),
            fired: false,
            cell: None,
        });
    }
    // `DISPLAY.CPP` binds `[CellTriggers]` by trigger name; the same lookup
    // runs here against the triggers just added.
    let width = game.map.width.max(1);
    for (cell_index, name) in &scenario.cell_triggers {
        let Some(trigger) = game
            .triggers
            .iter_mut()
            .find(|trigger| trigger.name.eq_ignore_ascii_case(name))
        else {
            continue;
        };
        trigger.cell = Some(Vec2i::new(
            (cell_index % width) as i32,
            (cell_index / width) as i32,
        ));
    }
}

impl RunningTrigger {
    /// Container this trigger binds to in the shared attachment model
    /// (`cnc_core::trigger_attach`): a `[CellTriggers]` entry binds to its
    /// cell, every other running trigger is scenario-global.
    ///
    /// Kept beside [`push_triggers`] (which produces both bindings) so the
    /// runtime type stays free of scenario-format knowledge.
    pub fn attach_target(&self) -> cnc_core::trigger_attach::AttachTarget {
        match self.cell {
            Some(cell) => cnc_core::trigger_attach::AttachTarget::Cell(cell),
            None => cnc_core::trigger_attach::AttachTarget::Global,
        }
    }
}

/// Translate a retail event token into the port's condition mini-language.
///
/// `Time` is the one event the port evaluates directly (`elapsed:<seconds>`
/// reads `Data`). Other event mnemonics are preserved lower-case so a loader
/// consumer can inspect them; they stay inert rather than firing spuriously.
fn trigger_condition(trigger: &OriginalTrigger) -> String {
    match trigger.event.trim().to_ascii_lowercase().as_str() {
        "time" | "timer" => format!("elapsed:{}", trigger.data.max(0)),
        other => other.to_string(),
    }
}

/// Translate a retail action token into the port's action vocabulary.
fn trigger_action(trigger: &OriginalTrigger) -> String {
    match trigger.action.trim().to_ascii_lowercase().as_str() {
        "win" => "win".to_string(),
        "lose" => "lose".to_string(),
        other => other.to_string(),
    }
}

/// Map a retail house name onto its faction.
fn faction_for(name: &str) -> Option<Faction> {
    match name.trim().to_ascii_lowercase().as_str() {
        "greece" | "england" | "germany" | "france" | "spain" | "turkey" | "allies" | "allied" => {
            Some(Faction::Allied)
        }
        "ussr" | "russia" | "ukraine" | "soviet" | "soviets" => Some(Faction::Soviet),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::house::Faction;
    use crate::original_scenario::ObjectKind;

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures/ra/FIXTURE.MPR")
    }

    #[test]
    fn running_trigger_attachment_target_reflects_its_binding() {
        let cell = Vec2i::new(15, 15);
        let mut trigger = RunningTrigger {
            id: 1,
            name: "CELL1".to_string(),
            condition: "never".to_string(),
            action: "win".to_string(),
            fired: false,
            cell: Some(cell),
        };
        assert_eq!(
            trigger.attach_target(),
            cnc_core::trigger_attach::AttachTarget::Cell(cell),
            "a [CellTriggers] binding maps to a cell attachment"
        );
        trigger.cell = None;
        assert_eq!(
            trigger.attach_target(),
            cnc_core::trigger_attach::AttachTarget::Global,
            "an unbound running trigger is scenario-global"
        );
    }

    #[test]
    fn original_ra_mpr_builds_game() {
        let path = fixture();
        let scenario =
            load_mpr(&path).unwrap_or_else(|error| panic!("{}: {error:#}", path.display()));

        // [Basic]
        assert_eq!(scenario.name, "Synthetic RA Golden Fixture");
        assert_eq!(scenario.player, "Greece");

        // [MapPack] / [OverlayPack]: numbered base64 -> LCW -> 3-byte records.
        assert_eq!(scenario.map_cells.len(), 256, "16x16 terrain cells");
        assert!(scenario.has_terrain());
        assert_eq!(scenario.overlay_cells.len(), 2);
        let ids: std::collections::BTreeSet<u8> =
            scenario.map_cells.iter().map(|cell| cell.id).collect();
        assert_eq!(ids, [0u8, 11, 22].into_iter().collect());

        // [Houses] + per-house sections.
        assert_eq!(scenario.houses.len(), 2);
        assert_eq!(scenario.houses[0].name, "Greece");
        assert_eq!(scenario.houses[0].credits, 4_000);
        assert_eq!(scenario.houses[1].name, "USSR");

        // [Units] / [Infantry] / [Structures].
        assert_eq!(scenario.objects_of(ObjectKind::Unit).count(), 2);
        assert_eq!(scenario.objects_of(ObjectKind::Infantry).count(), 1);
        assert_eq!(scenario.objects_of(ObjectKind::Structure).count(), 2);

        // [Waypoints], [Trigs].
        assert_eq!(scenario.waypoints.len(), 3);
        assert_eq!(scenario.waypoints[2].cell, 100);
        assert_eq!(scenario.triggers.len(), 2);
        assert_eq!(scenario.triggers[0].name, "TRIGA");
        assert_eq!(scenario.triggers[0].display_name, "Alpha");
        assert_eq!(scenario.triggers[1].name, "TRIGB");

        // The optional [Base]/[SMUDGE]/[TERRAIN] sections are absent here.
        assert!(scenario.base_nodes.is_empty());
        assert!(scenario.smudges.is_empty());
        assert!(scenario.terrain_objects.is_empty());

        let game = build_game(&scenario);
        assert_eq!(game.map.width, 16);
        assert_eq!(game.map.height, 16);
        assert_eq!(
            game.map.terrain_at(Vec2i::new(1, 4)),
            Some(crate::Terrain::Grass)
        );
        assert_eq!(game.units.len(), 3, "2 vehicles + 1 infantry");
        assert_eq!(game.structures.len(), 2, "power plant + barracks");
        assert_eq!(game.houses.len(), 2);
        assert_eq!(game.houses[0].name, "Greece");
        assert_eq!(game.houses[0].faction, Faction::Allied);
        assert_eq!(game.houses[0].credits, 4_000);
        assert_eq!(game.houses[1].name, "USSR");
        assert_eq!(game.houses[1].faction, Faction::Soviet);
        assert_eq!(game.player_owner, 0, "Greece is the allied player");
        assert_eq!(game.triggers.len(), 2);
        assert_eq!(game.triggers[0].condition, "elapsed:300");
        assert_eq!(game.triggers[0].action, "win");
        assert_eq!(game.triggers[1].action, "lose");
        assert_eq!(game.briefing, "Defend the coast road.");
    }

    #[test]
    fn bad_mappack_errors_without_panicking() {
        // Valid base64/INI, but the payload is not an LCW stream.
        let text = "[Basic]\nName=Broken\n[MapPack]\n1=AAECAwQFBgc=\n";
        let error = match load_game_from_text(text) {
            Ok(_) => panic!("bad MapPack must fail"),
            Err(error) => error,
        };
        let message = format!("{error:#}");
        assert!(
            message.contains("MapPack"),
            "error must name the section: {message}"
        );
    }

    #[test]
    fn bad_overlaypack_errors_without_panicking() {
        let text = "[Basic]\nName=Broken\n[OverlayPack]\n1=!!!not-base64!!!\n";
        let error = match load_game_from_text(text) {
            Ok(_) => panic!("bad OverlayPack must fail"),
            Err(error) => error,
        };
        assert!(format!("{error:#}").contains("OverlayPack"));
    }

    fn load_game_from_text(text: &str) -> Result<Game> {
        Ok(build_game(&OriginalScenario::from_ini(text)?))
    }
}
