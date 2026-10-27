//! Aggregate (TType, TIcon) neighbour statistics from decoded scenario maps.
//! Water templates = 1/2 (WATER/WATER2); land = everything else in use.
//! Produces: shore mask table (water cell, land on bits), shelf-side table,
//! template transition matrix, icon histograms.
use std::collections::{BTreeMap, HashMap};

use std::path::Path;

fn is_water(t: u8) -> bool {
    t == 1 || t == 2
}

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

    let mut stats: BTreeMap<u8, (u32, u32, u32)> = BTreeMap::new();
    // shore = water cell with >=2 land neighbours: landmask -> (t,i) -> count
    let mut shore: BTreeMap<u8, BTreeMap<(u8, u8), u32>> = BTreeMap::new();
    let mut shore_arcs: BTreeMap<u8, BTreeMap<(u8, u8), u32>> = BTreeMap::new();
    // 1-land-side coast cell (the straight-coast case): land side -> (t,i)
    let mut coast_side: [BTreeMap<(u8, u8), u32>; 4] = Default::default();
    let mut shelf: BTreeMap<u8, BTreeMap<(u8, u8), u32>> = BTreeMap::new();
    // template transitions: (t_center, t_neighbor) -> count
    let mut trans: HashMap<(u8, u8), u32> = HashMap::new();
    // icon usage per template
    let mut icons: BTreeMap<u8, BTreeMap<u8, u32>> = BTreeMap::new();
    let mut total_cells = 0u32;
    let mut maps_decoded = 0;

    for name in &names {
        let map = match cnc_tiberian_dawn::scenario_loader::load_bin(dir.join(name), None) {
            Ok(m) => m,
            Err(e) => {
                println!("{name}: ERR {e}");
                continue;
            }
        };
        maps_decoded += 1;
        let w = map.width as usize;
        let h = map.height as usize;
        let cells = &map.cells;
        for y in 0..h {
            for x in 0..w {
                let c = &cells[y * w + x];
                if c.template == 255 {
                    continue;
                }
                total_cells += 1;
                *icons.entry(c.template).or_default().entry(c.icon).or_insert(0) += 1;
                let e = stats.entry(c.template).or_insert((0, 0, 0));
                for (side_idx, (dx, dy)) in [(0, (0i32, -1i32)), (1, (1, 0)), (2, (0, 1)), (3, (-1, 0))] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    let n_tpl = if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                        255
                    } else {
                        cells[ny as usize * w + nx as usize].template
                    };
                    let slot = if n_tpl == 255 {
                        &mut e.2
                    } else if n_tpl == c.template {
                        &mut e.0
                    } else {
                        &mut e.1
                    };
                    *slot += 1;
                    if n_tpl != 255 {
                        *trans.entry((c.template, n_tpl)).or_insert(0) += 1;
                    }
                    if is_water(c.template) && n_tpl != 255 && !is_water(n_tpl) {
                        // the neighbouring cell's land art: record the placed
                        // (template, icon) on the LAND side of this edge
                        let n_cell = &cells[ny as usize * w + nx as usize];
                        *shelf
                            .entry(1 << side_idx)
                            .or_default()
                            .entry((n_cell.template, n_cell.icon))
                            .or_insert(0) += 1;
                    }
                }
                let mut mask = 0u8;
                let mut land_count = 0u8;
                let mut side_bits = [0u8; 4];
                if c.template != 255 {
                    for (side_idx, (dx, dy)) in [(0, (0i32, -1i32)), (1, (1, 0)), (2, (0, 1)), (3, (-1, 0))].iter().enumerate() {
                        let (dx, dy): (i32, i32) = *dy;
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        let n_tpl = if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                            255
                        } else {
                            cells[ny as usize * w + nx as usize].template
                        };
                        let land_here = n_tpl != 255 && !is_water(n_tpl);
                        if land_here {
                            mask |= 1 << side_idx;
                            land_count += 1;
                            side_bits[side_idx] = 1;
                        } else {
                            side_bits[side_idx] = 0;
                        }
                    }
                }
                let land_mask_val = mask;
                if is_water(c.template) {
                    if land_count == 1 {
                        let side = side_bits.iter().position(|&b| b == 1).unwrap();
                        *coast_side[side].entry((c.template, c.icon)).or_insert(0) += 1;
                    }
                }
                // Shore pieces (water cells with >=2 land neighbours) and
                // where the arc templates land.
                if is_water(c.template) && land_count >= 2 {
                    *shore.entry(land_mask_val).or_default().entry((c.template, c.icon)).or_insert(0) += 1;
                }
                if c.template >= 3 && c.template <= 15 {
                    *shore_arcs.entry(land_mask_val).or_default().entry((c.template, c.icon)).or_insert(0) += 1;
                }
            }
        }
    }

    let _ = stats.len();
    println!("maps: {maps_decoded}, cells: {total_cells}");
    println!("\ntemplate neighbour stats (template: same% diff% edge%):");
    for (t, (s, d, n)) in &stats {
        let total = s + d + n;
        println!(
            "  {t:3}: same {:.1}% diff {:.1}% edge {:.1}%",
            100.0 * *s as f32 / total as f32,
            100.0 * *d as f32 / total as f32,
            100.0 * *n as f32 / total as f32
        );
    }
    let side_names = ["N", "E", "S", "W"];
    let mut all: Vec<String> = Vec::new();
    for (idx, side) in side_names.iter().enumerate() {
        let mut top: Vec<_> = coast_side[idx].iter().collect();
        top.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let total: u32 = coast_side[idx].values().sum();
        let tops: Vec<_> = top.iter().take(5).map(|((t, i), n)| format!("{t}:{i}={n}")).collect();
        all.push(format!("  {side} ({total}): {}", tops.join(", ")));
    }
    println!("\n1-land-side coast cells (water cell on the straight coast):");
    for line in all { println!("{line}"); }
    println!("\nland-side shelf placements (land cell, water mask):");
    for (mask, entries) in &shelf {
        let mut top: Vec<_> = entries.iter().collect();
        top.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let top: Vec<_> = top.iter().take(3).map(|((t, i), n)| format!("{t}:{i}={n}")).collect();
        println!("  {mask:02X}: {}", top.join(", "));
    }
    println!("\nshore pieces (water cell, land_mask_val with >=2 land sides):");
    for (mask, entries) in &shore {
        let mut top: Vec<_> = entries.iter().collect();
        top.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let top: Vec<_> = top.iter().take(3).map(|((t, i), n)| format!("{t}:{i}={n}")).collect();
        println!("  {mask:02X}: {}", top.join(", "));
    }
    println!("\narc-template placements (templates 3..=15 by land mask):");
    for (mask, entries) in &shore_arcs {
        let mut top: Vec<_> = entries.iter().collect();
        top.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let top: Vec<_> = top.iter().take(3).map(|((t, i), n)| format!("{t}:{i}={n}")).collect();
        println!("  {mask:02X}: {}", top.join(", "));
    }
    println!("\nicon histograms (top templates):");
    for (t, hist) in icons.iter().take(12).collect::<Vec<_>>() {
        let total: u32 = hist.values().sum();
        let mut top: Vec<_> = hist.iter().collect();
        top.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let tops: Vec<_> = top.iter().take(8).map(|(i, n)| format!("{i}={n}")).collect();
        println!("  t{t} ({total}): {}", tops.join(", "));
    }
}

fn shore_entry_by_land_mask(
    table: &mut BTreeMap<u8, BTreeMap<(u8, u8), u32>>,
    land_mask: u8,
    piece: (u8, u8),
) {
    *table.entry(land_mask).or_default().entry(piece).or_insert(0) += 1;
}
