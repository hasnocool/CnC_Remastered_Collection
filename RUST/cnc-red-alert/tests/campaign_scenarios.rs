//! The authored RA campaign scenarios must stay loadable.

use std::path::{Path, PathBuf};

use cnc_red_alert::{Game, Scenario};

#[test]
fn authored_ra_campaign_scenarios_load() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/ra_campaign");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("ra_campaign directory missing")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("ini"))
        })
        .collect();
    paths.sort();
    assert_eq!(paths.len(), 10, "{paths:?}");

    let mut names = Vec::new();
    for path in &paths {
        let scenario = Scenario::from_file(path)
            .unwrap_or_else(|error| panic!("{}: {error:#}", path.display()));
        assert!(scenario.width > 0 && scenario.height > 0);
        assert_eq!(
            scenario.terrain.len(),
            (scenario.width * scenario.height) as usize,
            "{}: terrain grid size",
            path.display()
        );
        assert_eq!(
            scenario.starts.len(),
            2,
            "{}: two player starts",
            path.display()
        );
        assert!(
            scenario.objectives.len() >= 3,
            "{}: objectives",
            path.display()
        );
        assert!(scenario.triggers.len() >= 4, "{}: triggers", path.display());
        assert!(
            !scenario.briefing.is_empty(),
            "{}: briefing",
            path.display()
        );
        let game = Game::from_scenario(&scenario);
        assert_eq!(game.map.width, scenario.width);
        assert!(!game.units.is_empty());
        assert!(!game.structures.is_empty());
        names.push(scenario.name);
    }
    assert!(names[0].contains("Holding the Line"));
    assert!(names[6].contains("Breakwater"));
    assert!(names[7].contains("Iron Coast"));
    assert!(names[8].contains("Frost Line"));
    assert!(names[9].contains("Red Horizon"));
}
