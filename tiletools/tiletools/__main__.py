"""CLI: generate a procedural map from original DOS assets and screenshot it.

    python -m tiletools --game td --theater temperate --seed 42 --size 48

Decodes the DOS `.TEM` icon sets straight from the Steam install's
`CNCDATA/<GAME>/CD1/<THEATER>.MIX`, parses the compiled template table from
`original/<GAME>/CDATA.CPP`, measures every icon's edge material profile,
builds a material field, picks `(template, icon)` per cell, composites a
screenshot PNG, and prints the seam metric. `--compare` adds a naive-vs-
matched sheet for visual debugging.
"""

from __future__ import annotations

import argparse
import json
import sys
import time
from pathlib import Path

import numpy as np
from PIL import Image

from .cdata import td_templates, ra_templates
from .dos import DosTheater
from .meg import steam_install_root
from .join import (
    LAND_TYPE_MATERIAL,
    Material,
    build_table_from_icons,
    measure_icon_classes,
)
from .matcher import Strategy, assign, repair_field
from .atlas import render_atlas
from .render import Renderer, comparison_sheet, save, to_preview
from .seam import dumps as seam_dumps, seam_score
from .terrain import (
    apply_shore,
    generate_field,
    label_pools,
    smooth,
    sawtooth_sea_south,
)

THEATERS = {
    "td": ("temperate", "winter", "desert"),
    "ra": ("temperate", "snow", "interior"),
}


def build_dos_join_table(theater: DosTheater, templates):
    """Measure every shipped DOS icon once; materials from the CDATA table."""
    measured = {}
    materials = {}
    runs = {}
    classes = theater.index_classes()
    for template, cls_frames in classes.items():
        definition = templates.get(template)
        if definition is None:
            continue
        for icon in range(cls_frames.shape[0]):
            key = (template, icon)
            try:
                edges, icon_runs = measure_icon_classes(cls_frames[icon])
            except ValueError:
                continue
            measured[key] = edges
            runs[key] = icon_runs
            land = definition.icon_land(icon)
            material = LAND_TYPE_MATERIAL.get(land, Material.LAND)
            # Pond templates (SH32-35: CLEAR land with WATER alt icons) draw
            # water bounded by grass — pond-shore pieces, not open water.
            if (
                material is Material.WATER
                and definition.land == "CLEAR"
                and definition.alt_land == "WATER"
            ):
                material = Material.SHORE
            materials[key] = material
    return build_table_from_icons(measured, materials, runs), measured


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--game", choices=["td", "ra"], default="td")
    parser.add_argument("--theater", default=None, help="default: game's first theater")
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--size", type=int, default=48)
    parser.add_argument("--sea", type=float, default=0.40)
    parser.add_argument("--gamma", type=float, default=3.0, help="palette gamma (1 = DOS look)")
    parser.add_argument("--smooth", type=int, default=2, help="coastline smoothing passes")
    parser.add_argument(
        "--out",
        type=Path,
        default=None,
        help="default: <repo>/RUST/target/scratch-tiletools",
    )
    parser.add_argument("--install-root", type=Path, default=None)
    parser.add_argument("--compare", action="store_true", help="naive vs matched sheet")
    parser.add_argument("--scale", type=float, default=4.0, help="screenshot scale (DOS pixels)")
    parser.add_argument("--atlas", action="store_true", help="render tileset atlas and exit")
    parser.add_argument("--joins", action="store_true", help="render SH-family join-compatibility matrices and exit")
    parser.add_argument("--full", action="store_true", help="use every joinable family, not just the v1 vocabulary")
    parser.add_argument(
        "--zoom-crop",
        type=int,
        nargs=4,
        metavar=("X0", "Y0", "X1", "Y1"),
        action="append",
        help="also render the cell-rect crop at high zoom for seam inspection (repeatable)",
    )
    parser.add_argument("--zoom-scale", type=float, default=8.0, help="crop zoom (DOS-pixel scale)")
    args = parser.parse_args(argv)
    out = args.out
    if out is None:
        out = Path(__file__).resolve().parents[2] / "RUST" / "target" / "scratch-tiletools"

    theater_name = args.theater or THEATERS[args.game][0]
    root = args.install_root or steam_install_root()
    if root is None:
        print("C&C Remastered Steam install not found", file=sys.stderr)
        return 2
    print(f"install: {root}")

    templates = td_templates() if args.game == "td" else ra_templates()
    theater = DosTheater.open(root, args.game, theater_name, gamma=args.gamma)
    print(f"theater: {theater.theater}, {len(theater.frames)} icon sets, "
          f"palette {theater.palette.shape}")

    if args.atlas:
        render_atlas(theater, set(templates), out / "tileset_atlas.png", scale=int(args.scale))
        return 0

    started = time.time()
    table, measured = build_dos_join_table(theater, templates)

    if args.joins:
        from .joins import render_join_matrix

        info = render_join_matrix(table, out / "join_matrix.png")
        n = len(info["icons"])
        exact = sum(
            1
            for v, h in info["exact"].values()
            if (v == 1.0 or h == 1.0)
        )
        print(f"join matrix: {n} icons, {exact} exact continuations "
              f"-> {out / 'join_matrix.png'}")
        return 0

    print(f"join table: {len(table)} candidates from {len(measured)} icons "
          f"in {time.time() - started:.1f}s")

    field = generate_field(args.seed, args.size, sea_level=args.sea)
    apply_shore(field)
    label_pools(field)
    smooth(field, passes=args.smooth)
    repair_field(field, table)
    sawtooth_sea_south(field)
    counts = {m.name: int((field.materials == m).sum()) for m in set(field.materials.flat)}
    print(f"field: {counts}")

    if args.full:
        families = None
    else:
        # v1 vocabulary: CLEAR1 grass, W1/W2 water, water-based SH* shore.
        # SH14/15/16 (rock shore) and SH32-35 (grass-side ponds) join the
        # set with --full; roads, slopes, rivers stay extensions.
        families = {
            Material.LAND: ["CLEAR1"],
            Material.WATER: ["W1"],
            Material.SHORE: ["SH10", "SH12", "SH13", "SH34", "SH35"],
        }
    picks = assign(
        Strategy.MATCHED,
        field,
        table,
        seed=args.seed,
        families=families,
        shore_mask_restrict=False,
        pool_families=["SH32", "SH33", "SH34", "SH35"],
        icon_classes=theater.index_classes(),
    )
    renderer = Renderer(theater)
    screenshot = renderer.compose(picks, scale=args.scale)
    # out resolved above
    save(screenshot, out / "map.png")
    save(to_preview(screenshot), out / "map_preview.png")
    stats = seam_score(theater, picks, materials=field.materials,
                       classes=theater.index_classes())
    (out / "seams.json").write_text(seam_dumps(stats))
    (out / "map_cells.json").write_text(json.dumps(
        [[{"t": p.template, "i": p.icon} for p in row] for row in picks], indent=1
    ))
    print(f"seams: holes {stats.holes}/{stats.checked_edges}, "
          f"transitions {stats.transitions} (coastline/arc), worst {stats.worst_edge:.2f}")

    for crop in args.zoom_crop or []:
        x0, y0, x1, y1 = crop
        size_c = picks.shape[0]
        x0, y0 = max(0, x0), max(0, y0)
        x1, y1 = min(size_c, x1), min(size_c, y1)
        if x1 <= x0 or y1 <= y0:
            continue
        sub = picks[y0:y1, x0:x1]
        zoom = renderer.compose(sub, scale=args.zoom_scale)
        save(zoom, out / f"zoom_{x0}_{y0}_{x1}_{y1}.png")
        print(f"zoom: {x1-x0}x{y1-y0} cells at {args.zoom_scale}x -> "
              f"zoom_{x0}_{y0}_{x1}_{y1}.png ({zoom.width}x{zoom.height})")

    if args.compare:
        naive = assign(Strategy.NAIVE, field, table, seed=args.seed, families=families,
                       shore_mask_restrict=False)
        naive_img = renderer.compose(naive)
        matched_img = renderer.compose(picks)
        sheet = comparison_sheet([
            ("naive (no matching)", naive_img),
            ("matched (edge-profile join)", matched_img),
        ])
        save(to_preview(sheet), out / "compare.png")
        naive_stats = seam_score(theater, naive, materials=field.materials,
                                 classes=theater.index_classes())
        print(f"naive seams: {naive_stats.holes}/{naive_stats.checked_edges} holes "
              f"({naive_stats.hole_fraction:.1%})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
