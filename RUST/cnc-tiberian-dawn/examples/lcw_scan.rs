//! Scan LCW streams: decode each payload at several candidate grid sizes and
//! print the ones that decode, for the CONQUER.MIX reverse engineering.
use std::fs;
use std::path::Path;

fn main() {
    let dir = Path::new("/tmp/conquer_payloads");
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .flatten()
        .map(|p| p.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let mut decoded = 0;
    for name in &names {
        let raw = fs::read(dir.join(name)).unwrap();
        let mut hit: Vec<String> = Vec::new();
        for side in [64u32, 48, 32, 56, 40, 24, 16, 62, 50] {
            let want = (side * side * 2) as usize;
            if want < raw.len() {
                continue;
            }
            if let Ok(out) = cnc_asset::format80::decompress(&raw, want) {
                let used = out.iter().step_by(2).filter(|&&t| t != 255).count();
                hit.push(format!("{side}x{side} used{used}"));
            }
        }
        if !hit.is_empty() {
            decoded += 1;
            println!("{name}: {}", hit.join(" | "));
        }
    }
    println!("decoded {decoded} of {}", names.len());
}
