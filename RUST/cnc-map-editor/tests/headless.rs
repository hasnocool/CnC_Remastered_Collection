//! End-to-end check that the editor binary's headless mode generates a map,
//! writes scenario INI and produces a PNG screenshot. Runs the real binary.

use std::path::PathBuf;
use std::process::Command;

#[test]
fn headless_binary_generates_map_ini_and_screenshot() {
    let dir: PathBuf =
        std::env::temp_dir().join(format!("cnc-editor-headless-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let map_path = dir.join("map.ini");
    let png_path = dir.join("editor.png");

    let status = Command::new(env!("CARGO_BIN_EXE_cnc-map-editor"))
        .args([
            "--headless",
            "--seed",
            "42",
            "--size",
            "48",
            "--players",
            "2",
            "--name",
            "Headless Scenario",
            "--player",
            "Allies",
            "--briefing",
            "Hold the line",
            "--objective",
            "Build a base",
            "--objective",
            "Destroy radar",
            "--out",
        ])
        .arg(&map_path)
        .arg("--screenshot")
        .arg(&png_path)
        .status()
        .expect("failed to run the editor binary");
    assert!(status.success(), "headless editor exited with {status}");

    let ini = std::fs::read_to_string(&map_path).expect("scenario INI was not written");
    for section in ["[Map]", "[Terrain]", "[Overlay]", "[Objects]", "[Triggers]"] {
        assert!(ini.contains(section), "missing {section} in scenario INI");
    }
    assert!(ini.contains("Width=48"));
    assert!(ini.contains("Height=48"));
    assert!(ini.contains("Name=Headless Scenario"), "{ini}");
    assert!(ini.contains("Player=Allies"), "{ini}");
    assert!(ini.contains("Briefing=Hold the line"), "{ini}");
    assert!(ini.contains("[Objectives]"), "{ini}");
    assert!(ini.contains("0=Build a base"), "{ini}");
    assert!(ini.contains("1=Destroy radar"), "{ini}");

    let png = std::fs::read(&png_path).expect("screenshot was not written");
    assert!(png.len() > 1_000, "screenshot is suspiciously small");
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n", "not a PNG file");

    let _ = std::fs::remove_dir_all(&dir);
}
