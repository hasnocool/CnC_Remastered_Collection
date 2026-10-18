//! End-to-end tests for the asset converter CLI (Phase 4).
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cnc-asset-cli"))
}

fn test_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cnc_cli_{}_{}_{}",
        name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn palette_bytes() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(768);
    for index in 0..256u32 {
        bytes.push((index % 256) as u8);
        bytes.push(((index * 3) % 256) as u8);
        bytes.push(((index * 7) % 256) as u8);
    }
    bytes
}

fn shp_bytes(frames: &[(u16, u16, u8)]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(frames.len() as u16).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    for (width, height, _) in frames {
        out.extend_from_slice(&width.to_le_bytes());
        out.extend_from_slice(&height.to_le_bytes());
        out.extend_from_slice(&0i16.to_le_bytes());
        out.extend_from_slice(&0i16.to_le_bytes());
    }
    let header = 8 + frames.len() * 8 + frames.len() * 4;
    let mut cursor = header as u32;
    for (width, height, _) in frames {
        out.extend_from_slice(&cursor.to_le_bytes());
        cursor += u32::from(*width) * u32::from(*height);
    }
    for (width, height, color) in frames {
        out.extend(std::iter::repeat(*color).take(usize::from(*width) * usize::from(*height)));
    }
    out
}

fn voc_bytes() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(b"Creative Voice File\x1a");
    data.extend_from_slice(&26u16.to_le_bytes());
    data.extend_from_slice(&0x0100u16.to_le_bytes());
    data.extend_from_slice(&0u16.to_le_bytes());
    data.push(0x00);
    data.extend_from_slice(&[0, 0, 0, 0]);
    data
}

fn aud_bytes() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&22050u32.to_le_bytes());
    data.extend_from_slice(&1u16.to_le_bytes());
    data.extend_from_slice(&0u16.to_le_bytes());
    data.extend_from_slice(&7i16.to_le_bytes());
    data
}


fn aud_bytes_with_samples(frames: usize) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&22050u32.to_le_bytes());
    data.extend_from_slice(&1u16.to_le_bytes());
    data.extend_from_slice(&0u16.to_le_bytes());
    for index in 0..frames {
        let sample = ((index as f32 * 440.0 * std::f32::consts::TAU / 22050.0).sin() * 12000.0) as i16;
        data.extend_from_slice(&sample.to_le_bytes());
    }
    data
}

fn mix_bytes() -> Vec<u8> {
    let entries: &[(&str, &[u8])] = &[("HELLO.TXT", b"hello mix"), ("DATA.BIN", b"12345678")];
    let mut out = Vec::new();
    out.extend_from_slice(b"MIX\x00");
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    let mut names = Vec::new();
    let mut offsets = Vec::new();
    for (name, _) in entries {
        offsets.push(names.len() as u32);
        names.extend_from_slice(name.as_bytes());
        names.push(0);
    }
    for offset in offsets { out.extend_from_slice(&offset.to_le_bytes()); }
    for (_, data) in entries { out.extend_from_slice(&(data.len() as u32).to_le_bytes()); }
    out.extend_from_slice(&names);
    for (_, data) in entries { out.extend_from_slice(data); }
    out
}

fn assert_success(command: &mut Command) -> String {
    let output = command.output().expect("failed to spawn cnc-asset-cli");
    assert!(
        output.status.success(),
        "command failed: {command:?}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn assert_failure(command: &mut Command) {
    let output = command.output().expect("failed to spawn cnc-asset-cli");
    assert!(!output.status.success(), "command unexpectedly succeeded: {command:?}");
}

#[test]
fn pal_export_formats() {
    let dir = test_dir("pal");
    let input = dir.join("TEMPERAT.PAL");
    fs::write(&input, palette_bytes()).unwrap();
    for format in ["json", "png", "css"] {
        let out = dir.join(format);
        let stdout = assert_success(cli().args(["pal", "--input"]).arg(&input).arg("--output").arg(&out).args(["--format", format]));
        assert!(stdout.contains("palette"), "stdout: {stdout}");
    }
    assert!(dir.join("json/palette.json").exists());
    assert!(dir.join("png/palette.png").exists());
    assert!(fs::read_to_string(dir.join("css/palette.css")).unwrap().contains("--palette-255"));
    assert_failure(cli().args(["pal", "--input"]).arg(&input).arg("--output").arg(dir.join("bad")).args(["--format", "xml"]));
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn shp_frames_and_sprite_sheet() {
    let dir = test_dir("shp");
    let shp = dir.join("E1.SHP");
    let pal = dir.join("TEMPERAT.PAL");
    fs::write(&shp, shp_bytes(&[(2, 2, 1), (3, 1, 2)])).unwrap();
    fs::write(&pal, palette_bytes()).unwrap();

    let frames = dir.join("frames");
    let stdout = assert_success(cli().args(["shp", "--input"]).arg(&shp).arg("--output").arg(&frames).arg("--palette").arg(&pal));
    assert!(stdout.contains("2 frame"), "stdout: {stdout}");
    assert!(frames.join("E1_frame000.png").exists());
    assert!(frames.join("E1_frame001.png").exists());

    let sheet = dir.join("sheet").join("E1_sheet.png");
    assert_success(cli().args(["shp", "--input"]).arg(&shp).arg("--output").arg(&sheet).arg("--palette").arg(&pal).arg("--sprite-sheet"));
    assert!(sheet.exists());
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(sheet.with_extension("json")).unwrap()).unwrap();
    assert_eq!(manifest["width"], 5);
    assert_eq!(manifest["frames"].as_array().unwrap().len(), 2);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn mix_list_and_extract() {
    let dir = test_dir("mix");
    let mix = dir.join("TEST.MIX");
    fs::write(&mix, mix_bytes()).unwrap();

    let out_dir = dir.join("extracted");
    let list = assert_success(cli().args(["mix", "--input"]).arg(&mix).arg("--output").arg(&out_dir).arg("--list"));
    assert!(list.contains("HELLO.TXT") && list.contains("DATA.BIN"), "list: {list}");
    assert!(!out_dir.exists());

    let stdout = assert_success(cli().args(["mix", "--input"]).arg(&mix).arg("--output").arg(&out_dir));
    assert!(stdout.contains("2 entries"), "stdout: {stdout}");
    assert_eq!(fs::read(out_dir.join("HELLO.TXT")).unwrap(), b"hello mix");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn voc_and_aud_to_wav() {
    let dir = test_dir("audio");
    let voc = dir.join("beep.voc");
    let aud = dir.join("shot.aud");
    fs::write(&voc, voc_bytes()).unwrap();
    fs::write(&aud, aud_bytes()).unwrap();
    let voc_wav = dir.join("beep.wav");
    let aud_wav = dir.join("shot.wav");
    assert_success(cli().args(["voc", "--input"]).arg(&voc).arg("--output").arg(&voc_wav));
    assert_success(cli().args(["voc", "--input"]).arg(&aud).arg("--output").arg(&aud_wav));
    assert_eq!(&fs::read(&voc_wav).unwrap()[..4], b"RIFF");
    assert_eq!(&fs::read(&aud_wav).unwrap()[..4], b"RIFF");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn batch_converts_tree_and_reports_counts() {
    let dir = test_dir("batch");
    let input = dir.join("game");
    fs::create_dir_all(input.join("data")).unwrap();
    fs::write(input.join("E1.SHP"), shp_bytes(&[(1, 1, 3)])).unwrap();
    fs::write(input.join("data").join("beep.voc"), voc_bytes()).unwrap();
    fs::write(input.join("data").join("shot.aud"), aud_bytes()).unwrap();
    fs::write(input.join("data").join("TEMPERAT.PAL"), palette_bytes()).unwrap();
    fs::write(input.join("data").join("ARCHIVE.MIX"), mix_bytes()).unwrap();
    fs::write(input.join("notes.txt"), b"ignored").unwrap();

    let out = dir.join("out");
    let stdout = assert_success(cli().args(["batch", "--input"]).arg(&input).arg("--output").arg(&out).args(["--game", "td"]));
    assert!(stdout.contains("converted 5 file(s), skipped 1, failed 0"), "stdout: {stdout}");
    for kind in ["mix: 1", "shp: 1", "voc: 1", "aud: 1", "pal: 1"] {
        assert!(stdout.contains(kind), "missing {kind} in: {stdout}");
    }
    assert!(out.join("E1_frame000.png").exists());
    assert!(out.join("data").join("beep.wav").exists());
    assert!(out.join("data").join("ARCHIVE").join("HELLO.TXT").exists());
    assert_failure(cli().args(["batch", "--input"]).arg(&input).arg("--output").arg(dir.join("bad")).args(["--game", "duke3d"]));
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn generate_atlas_writes_manifest() {
    let dir = test_dir("atlas");
    let out = dir.join("atlas");
    let stdout = assert_success(cli().args(["generate-atlas", "--out"]).arg(&out).args(["--style", "classic"]));
    assert!(stdout.contains("frames"), "stdout: {stdout}");
    assert!(out.join("atlas.png").exists());
    assert!(out.join("atlas.json").exists());
    assert_failure(cli().args(["generate-atlas", "--out"]).arg(dir.join("bad")).args(["--style", "vaporwave"]));
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn batch_returns_error_when_a_file_fails() {
    let dir = test_dir("batch_fail");
    let input = dir.join("in");
    fs::create_dir_all(&input).unwrap();
    fs::write(input.join("BROKEN.SHP"), b"garbage").unwrap();
    let output = cli()
        .args(["batch", "--input"])
        .arg(&input)
        .arg("--output")
        .arg(dir.join("out"))
        .args(["--game", "ra"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("BROKEN.SHP"), "stderr: {stderr}");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn aud_to_ogg_with_loop_points_and_batch_ogg() {
    let dir = test_dir("ogg");
    let aud = dir.join("music.aud");
    fs::write(&aud, aud_bytes_with_samples(4096)).unwrap();

    let ogg = dir.join("music.ogg");
    let stdout = assert_success(
        cli()
            .args(["voc", "--input"])
            .arg(&aud)
            .arg("--output")
            .arg(&ogg)
            .args(["--format", "ogg", "--loop-start", "100", "--loop-end", "3000"]),
    );
    assert!(stdout.contains("ogg"), "stdout: {stdout}");
    let bytes = fs::read(&ogg).unwrap();
    assert_eq!(&bytes[..4], b"OggS");
    assert!(bytes.windows(13).any(|window| window == b"LOOPSTART=100"));
    assert!(bytes.windows(12).any(|window| window == b"LOOPEND=3000"));

    // Extension-based format inference also selects Ogg.
    let inferred = dir.join("inferred.ogg");
    assert_success(cli().args(["voc", "--input"]).arg(&aud).arg("--output").arg(&inferred));
    assert_eq!(&fs::read(&inferred).unwrap()[..4], b"OggS");

    // Batch converts to Ogg when requested.
    let input = dir.join("in");
    fs::create_dir_all(&input).unwrap();
    fs::write(input.join("shot.aud"), aud_bytes_with_samples(1024)).unwrap();
    let out = dir.join("out");
    let stdout = assert_success(
        cli()
            .args(["batch", "--input"])
            .arg(&input)
            .arg("--output")
            .arg(&out)
            .args(["--game", "td", "--audio-format", "ogg"]),
    );
    assert!(stdout.contains("aud: 1"), "stdout: {stdout}");
    assert!(out.join("shot.ogg").exists());
    assert!(!out.join("shot.wav").exists());

    // Loop bounds must be supplied together.
    assert_failure(
        cli()
            .args(["voc", "--input"])
            .arg(&aud)
            .arg("--output")
            .arg(dir.join("bad.ogg"))
            .args(["--loop-start", "10"]),
    );
    fs::remove_dir_all(&dir).unwrap();
}
