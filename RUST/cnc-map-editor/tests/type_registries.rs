//! TDD test for INI-driven type registries (todo 50).
//!
//! Verifies that editor catalogs are loaded from RulesDatabase via cnc-core
//! instead of hardcoded ClassId vectors.

use cnc_core::RulesDatabase;
use cnc_map_editor::{GamePlugin, RedAlertPlugin, TiberianDawnPlugin};
use std::path::Path;

fn load_rules_db(relative: &str) -> RulesDatabase {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures")
        .join(relative);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read fixture: {}", path.display()));
    RulesDatabase::from_str(&text).unwrap_or_else(|e| panic!("Failed to parse rules: {}", e))
}

/// Test that TD catalog is derived from rules fixture sections.
/// This will FAIL initially because TiberianDawnPlugin still uses hardcoded vectors.
#[test]
fn catalog_from_rules_matches_fixture_td() {
    // Load TD rules fixture
    let rules = load_rules_db("td/rules_fixture.ini");

    let mut plugin = TiberianDawnPlugin::default();
    plugin.load_catalog_from_rules(&rules);
    let catalog = plugin.available_objects();

    // The catalog should be derived from the rules fixture categories
    // (Infantry, Units, Aircraft, Buildings, Structures)
    // and should contain ClassIds corresponding to those type names.

    // At minimum, these types should exist in any TD rules
    assert!(
        catalog.contains(&cnc_core::ClassId::InfantryE1),
        "E1 should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::ConstructionYard),
        "ConstructionYard should be in catalog"
    );

    // CRITICAL CHECK: The catalog should match the rules fixture exactly
    // The fixture has: E1, E2, E3, E6, LTNK, MTNK, MCV, HARV, APC, HELI, ORCA, FACT, POWR, BARR, WARF, HPAD, SAM, SILO
    // That's 18 types

    // Hardcoded version has 14 items - if it matches 14, it's still hardcoded
    // If it matches 18, it's rules-driven
    println!(
        "TD catalog size: {} (expected 18 if rules-driven)",
        catalog.len()
    );
    println!("TD catalog: {:?}", catalog);

    // This assertion will FAIL with hardcoded 14, PASS with rules-driven 18
    assert_eq!(
        catalog.len(),
        18,
        "Catalog should have 18 types from rules fixture, not hardcoded 14"
    );
}

/// Test that RA catalog is derived from rules fixture sections.
#[test]
fn catalog_from_rules_matches_fixture_ra() {
    let rules = load_rules_db("ra/rules_fixture.ini");

    let mut plugin = RedAlertPlugin::default();
    plugin.load_catalog_from_rules(&rules);
    let catalog = plugin.available_objects();

    // RA fixture has: E1, LTNK, MCV, HARV, HELI, POWR, BARR, WARF = 8 types
    // Hardcoded version has 26 items
    assert!(
        catalog.contains(&cnc_core::ClassId::InfantryE1),
        "E1 should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::VehicleE1),
        "LTNK should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::VehicleE7),
        "MCV should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::VehicleE6),
        "HARV should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::AircraftE1),
        "HELI should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::PowerPlant),
        "POWR should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::Barracks),
        "BARR should be in catalog"
    );
    assert!(
        catalog.contains(&cnc_core::ClassId::WarFactory),
        "WARF should be in catalog"
    );

    println!(
        "RA catalog size: {} (expected 8 if rules-driven, 26 if hardcoded)",
        catalog.len()
    );
    println!("RA catalog: {:?}", catalog);

    // This assertion will FAIL with hardcoded 26, PASS with rules-driven 8
    assert_eq!(
        catalog.len(),
        8,
        "Catalog should have 8 types from rules fixture, not hardcoded 26"
    );
}

/// Test that category entries without type sections are skipped gracefully.
/// The RulesDatabase stores them in categories but they don't appear in types.
#[test]
fn category_entries_without_sections_are_skipped() {
    let rules_text = r#"
[Infantry]
1=E1
2=MISSING_TYPE

[E1]
Cost=120
Strength=60
"#;

    let rules = RulesDatabase::from_str(rules_text).unwrap();

    // The category still contains both entries
    let infantry = rules.categories.get("Infantry").unwrap();
    assert_eq!(infantry.len(), 2);
    assert!(infantry.contains(&"E1".to_string()));
    assert!(infantry.contains(&"MISSING_TYPE".to_string()));

    // But only E1 appears in types (MISSING_TYPE has no section with Cost/Strength)
    assert_eq!(rules.type_count(), 1);
    assert_eq!(rules.type_cost("E1"), Some(120));
    assert_eq!(rules.type_cost("MISSING_TYPE"), None);

    // The editor plugin should skip unknown types with a warning
    let mut plugin = TiberianDawnPlugin::default();
    plugin.load_catalog_from_rules(&rules);
    let catalog = plugin.available_objects();

    // Only E1 should be in the catalog
    assert_eq!(catalog.len(), 1);
    assert!(catalog.contains(&cnc_core::ClassId::InfantryE1));
}
