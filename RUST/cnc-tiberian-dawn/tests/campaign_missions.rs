//! The authored TD campaign missions must stay loadable.

use std::path::{Path, PathBuf};

use cnc_tiberian_dawn::Mission;

#[test]
fn authored_td_campaign_missions_parse() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("td_campaign directory missing")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("ini"))
        })
        .collect();
    paths.sort();
    assert_eq!(paths.len(), 9, "{paths:?}");

    let names: Vec<String> = paths
        .iter()
        .map(|path| {
            let mission = Mission::from_file(path)
                .unwrap_or_else(|error| panic!("{}: {error:#}", path.display()));
            assert!(!mission.objectives.is_empty());
            assert!(mission.triggers.len() >= 3);
            assert!(
                !mission.briefing.is_empty(),
                "{}: missing briefing",
                path.display()
            );
            mission.name
        })
        .collect();
    assert!(names[0].contains("Holding the Line"));
    assert!(names[1].contains("Breakout"));
    assert!(names[2].contains("Counterstrike"));
    assert!(names[3].contains("Scorched Earth"));
    assert!(names[4].contains("Silent Extraction"));
    assert!(names[5].contains("Hammer Fall"));
    assert!(names[6].contains("Blackout"));
    assert!(names[7].contains("Steel Rain"));
    assert!(names[8].contains("Broken Arrow"));
}
