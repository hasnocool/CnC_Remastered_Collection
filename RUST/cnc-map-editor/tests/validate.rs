//! End-to-end checks for the --validate scenario CLI.

use std::path::PathBuf;
use std::process::Command;

fn run(args: &[&std::ffi::OsStr]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cnc-map-editor"))
        .args(args)
        .output()
        .expect("failed to run the editor binary")
}

#[test]
fn validate_accepts_a_valid_map_and_reports_issues() {
    let dir: PathBuf =
        std::env::temp_dir().join(format!("cnc-editor-validate-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let valid = dir.join("valid.ini");
    std::fs::write(
        &valid,
        "[Map]\nWidth=8\nHeight=8\nTheater=Temperate\n[PlayerStarts]\n0=(1,1)\n",
    )
    .unwrap();
    let output = run(&["--validate".as_ref(), valid.as_os_str()]);
    assert!(
        output.status.success(),
        "valid map failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK: no validation issues"), "{stdout}");

    let invalid = dir.join("invalid.ini");
    std::fs::write(&invalid, "[Map]\nWidth=8\nHeight=8\nTheater=Temperate\n").unwrap();
    let output = run(&["--validate".as_ref(), invalid.as_os_str()]);
    assert!(!output.status.success(), "invalid map should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no player start"), "{stderr}");

    // Batch mode validates every INI in a directory and fails if any is bad.
    let output = run(&["--validate".as_ref(), dir.as_os_str()]);
    assert!(
        !output.status.success(),
        "batch with an invalid map should fail"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("valid.ini: OK"), "{stdout}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid.ini"), "{stderr}");

    // Template gallery exports parseable scenarios.
    let templates = dir.join("templates");
    let output = run(&["--export-templates".as_ref(), templates.as_os_str()]);
    assert!(
        output.status.success(),
        "template export failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(std::fs::read_dir(&templates).unwrap().flatten().count(), 3);
    let output = run(&["--validate".as_ref(), templates.as_os_str()]);
    assert!(
        output.status.success(),
        "exported templates failed validation: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}
