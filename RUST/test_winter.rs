
use cnc_asset::iconset::load_td_theater;
use std::path::Path;

fn main() {
    let path = Path::new("../../assets/original/Data/CNCDATA/TIBERIAN_DAWN/CD1/WINTER.MIX");
    println!("File exists: {}", path.exists());
    match load_td_theater(path) {
        Ok(maps) => {
            println!("Loaded {} templates", maps.len());
            for (name, map) in &maps {
                println!("  {}: {}x{} frames={} occupancy_len={}", name, map.width, map.height, map.frame_count, map.occupancy.len());
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
