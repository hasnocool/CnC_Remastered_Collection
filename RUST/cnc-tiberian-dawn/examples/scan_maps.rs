//! Decode every loose TD scenario .BIN in a directory tree with decode_bin.
use std::path::Path;

fn main() {
    let dir = Path::new("/tmp/td_original_data/CAMPAIGN");
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .flatten()
        .map(|p| p.path())
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bin")))
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    names.sort();
    for name in &names {
        let bin = dir.join(name);
        let stem = name.trim_end_matches(".bin").trim_end_matches(".BIN");
        match cnc_tiberian_dawn::scenario_loader::load_bin(&bin, None) {
            Ok(map) => {
                let used = map.cells.iter().filter(|c| c.template != 255).count();
                println!("{stem}: {}x{} used{used}", map.width, map.height);
            }
            Err(e) => println!("{stem}: ERR {e}"),
        }
    }
}
