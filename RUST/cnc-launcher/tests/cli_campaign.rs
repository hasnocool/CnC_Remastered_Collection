//! The RA campaign listing CLI must stay window-free and complete.

use std::path::Path;
use std::process::Command;

#[test]
fn list_campaign_reports_every_ra_mission() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/ra_campaign");
    let output = Command::new(env!("CARGO_BIN_EXE_cnc-launcher"))
        .arg("--list-campaign")
        .arg(&dir)
        .output()
        .expect("failed to run cnc-launcher");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("listed 10 RA campaign scenario(s)"), "{text}");
    assert!(text.contains("01 - Holding the Line"), "{text}");
    assert!(text.contains("07 - Breakwater"), "{text}");
    assert!(text.contains("08 - Iron Coast"), "{text}");
    assert!(text.contains("09 - Frost Line"), "{text}");
    assert!(text.contains("10 - Red Horizon"), "{text}");
    assert!(text.contains("player Allies"), "{text}");
    assert!(text.contains("player Soviet"), "{text}");
    let missions: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("0") || line.starts_with("1"))
        .collect();
    assert_eq!(missions.len(), 10, "{text}");
    for line in missions {
        assert!(line.contains("2 starts"), "{line}");
        assert!(line.contains("objectives"), "{line}");
        assert!(line.contains("triggers"), "{line}");
    }
}
