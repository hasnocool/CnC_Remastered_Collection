//! Detailed review of a generated map: terrain composition, water connectivity,
//! shore framing, overlays, starts and balance.

use cnc_map_editor::generator::{self, MapGenSettings};

fn main() {
    let seed: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(7);
    let mut s = MapGenSettings::new(seed);
    s.width = 128;
    s.height = 128;
    s.player_count = 4;
    let m = generator::generate(&s);
    let n = (m.width * m.height) as usize;

    println!(
        "== {} {}x{} seed {} players {} ==",
        m.theater,
        m.width,
        m.height,
        seed,
        m.player_starts.len() as u8
    );
    println!("overlay cells: {}", m.overlay.len());
    println!("objects: {}", m.objects.len());

    // --- terrain composition
    let mut counts = std::collections::BTreeMap::new();
    for i in 0..m.terrain.len() {
        *counts.entry(m.terrain[i]).or_insert(0usize) += 1;
    }
    println!("\nterrain composition:");
    let names = [
        "Grass", "Water", "Sand", "Dirt", "Rock", "Snow", "Rough", "Mud", "Crater", "Scorched",
        "Swamp", "Concrete", "Forest", "Ice", "Jungle",
    ];
    for (id, c) in &counts {
        let name = names.get(*id as usize).unwrap_or(&"?");
        println!(
            "  {:>9} {:>5}  {:5.1}%",
            name,
            c,
            100.0 * *c as f64 / n as f64
        );
    }

    // --- water connectivity (flood fill from the biggest blob)
    let water = names.iter().position(|n| *n == "Water").unwrap() as u8;
    let idx = |x: i32, y: i32| (y as u32 * m.width + x as u32) as usize;
    let is_w = |x: i32, y: i32| m.terrain[idx(x, y)] == water;
    let mut seen = vec![false; n];
    let (mut biggest, mut blobs) = (0usize, Vec::new());
    let mut total_water = 0usize;
    for y in 0..m.height as i32 {
        for x in 0..m.width as i32 {
            if is_w(x, y) {
                total_water += 1;
            }
            if is_w(x, y) && !seen[idx(x, y)] {
                let mut stack = vec![(x, y)];
                seen[idx(x, y)] = true;
                let mut size = 0usize;
                while let Some((cx, cy)) = stack.pop() {
                    size += 1;
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (nx, ny) = (cx + dx, cy + dy);
                        if nx < 0 || ny < 0 || nx >= m.width as i32 || ny >= m.height as i32 {
                            continue;
                        }
                        if is_w(nx, ny) && !seen[idx(nx, ny)] {
                            seen[idx(nx, ny)] = true;
                            stack.push((nx, ny));
                        }
                    }
                }
                biggest = biggest.max(size);
                blobs.push(size);
            }
        }
    }
    blobs.sort_unstable_by(|a, b| b.cmp(a));
    println!(
        "\nwater: {total_water} cells in {} blob(s); largest {} ({:.1}% of water)",
        blobs.len(),
        biggest,
        100.0 * biggest as f64 / total_water.max(1) as f64
    );
    println!("  blob sizes: {blobs:?}");

    // --- shoreline length (water cells touching land)
    let mut shore = 0usize;
    for y in 0..m.height as i32 {
        for x in 0..m.width as i32 {
            if !is_w(x, y) {
                continue;
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= m.width as i32 || ny >= m.height as i32 {
                    // The map edge counts as land: an enclosed island.
                    shore += 1;
                    break;
                }
                if !is_w(nx, ny) {
                    shore += 1;
                    break;
                }
            }
        }
    }
    println!(
        "\nshoreline: {shore} water cells border land ({:.0}% of water)",
        100.0 * shore as f64 / total_water.max(1) as f64
    );

    // --- overlays
    // OverlayType ids: 0 None, 1 Tiberium (raw ore), 2 Trees, 3 Ore, 4 Road, 5 Wall.
    let (mut tiberium, mut trees, mut ore, mut road, mut wall, mut none) =
        (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    for i in 0..m.overlay.len() {
        match m.overlay[i] {
            1 => tiberium += 1,
            2 => trees += 1,
            3 => ore += 1,
            4 => road += 1,
            5 => wall += 1,
            _ => none += 1,
        }
    }
    let players = (m.player_starts.len().max(1)) as f64;
    println!(
        "\noverlays: tiberium {tiberium}, trees {trees}, ore {ore}, road {road}, wall {wall}, none {none}"
    );
    println!(
        "  tiberium per player: {:.0} cells",
        tiberium as f64 / players
    );

    // --- starts: distance to nearest other start, and land vs water
    let starts = &m.player_starts;
    println!("\nstarts ({}):", starts.len());
    for (i, a) in starts.iter().enumerate() {
        let mut nearest = usize::MAX;
        for (j, b) in starts.iter().enumerate() {
            if i == j {
                continue;
            }
            let d = ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize;
            nearest = nearest.min(d);
        }
        let t = if is_w(a.x, a.y) { "WATER" } else { "land" };
        println!(
            "  ({},{}) {} terrain={} nearest={} (manhattan)",
            a.x,
            a.y,
            t,
            m.terrain[idx(a.x, a.y)],
            nearest
        );
    }

    // --- playability: fraction of land that is walkable (not rock/water)
    let walkable = m
        .terrain
        .iter()
        .enumerate()
        .filter(|(i, t)| **t != water && **t != 4 && m.overlay[*i] != 1)
        .count();
    println!(
        "\nwalkable land: {:.1}% of the map",
        100.0 * walkable as f64 / n as f64
    );

    println!("\nvalidation issues: {}", m.validate().len());
    for i in m.validate().iter().take(20) {
        println!("  - {i}");
    }
}
