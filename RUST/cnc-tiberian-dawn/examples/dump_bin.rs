//! Dump one scenario .BIN's cells + theater to JSON for the Python tooling.
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let bin = Path::new(&args[1]);
    let map = cnc_tiberian_dawn::scenario_loader::load_bin(bin, None)?;
    let cells: Vec<(u8, u8)> = map.cells.iter().map(|c| (c.template, c.icon)).collect();
    let json = serde_json::json!({
        "width": map.width,
        "height": map.height,
            "cells": cells,
    });
    println!("{json}");
    Ok(())
}
