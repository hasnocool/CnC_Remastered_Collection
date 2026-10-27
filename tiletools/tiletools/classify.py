"""Per-tile visual classification for the join vocabulary.

Every icon in the vocabulary is classified from its index-class array:

- material coverage: land / water / transparent fractions;
- per-edge type: ``land`` / ``water`` / ``mixed``, and for mixed edges the
  arc flip positions along the run (where sand transitions to water) plus
  which side of the flip is sand — the contour data the matcher aligns;
- interior features: the fraction of "clutter" pixels (rock/grass detail
  indices inside a water area — the DOS shore art's rocks and grass
  clumps), detected as non-water pixels more than one pixel in from the
  edges.

Outputs ``tiles.json`` (machine-readable, feeds the matcher) and
``tile_atlas.png`` (labeled contact sheet, one panel per icon, for visual
verification of the classification).
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field

import numpy as np
from PIL import Image, ImageDraw

from .dos import DosTheater


@dataclass
class EdgeClass:
    kind: str  # "land" | "water" | "mixed" | "transparent"
    flips: list[tuple[int, str]] = field(default_factory=list)
    # flips: (position along the run, "land->water" | "water->land")


@dataclass
class TileClass:
    template: str
    icon: int
    land_frac: float
    water_frac: float
    transparent_frac: float
    edges: dict[str, EdgeClass]
    clutter: float  # interior detail density (rocks/grass clumps in water)


def classify_edges(run: np.ndarray) -> EdgeClass:
    n = len(run)
    land = float((run == 2).mean())
    water = float((run == 1).mean())
    trans = float((run == 0).mean())
    if trans > 0.95:
        return EdgeClass("transparent")
    if land > 0.95:
        return EdgeClass("land")
    if water > 0.95:
        return EdgeClass("water")
    flips: list[tuple[int, str]] = []
    for i in range(1, n):
        a, b = run[i - 1], run[i]
        if a == 2 and b != 2:
            flips.append((i, "land->water" if b == 1 else "land->trans"))
        elif a == 1 and b != 1:
            flips.append((i, "water->land" if b == 2 else "water->trans"))
    return EdgeClass("mixed", flips=flips)


def clutter_fraction(cls: np.ndarray) -> float:
    """Non-water pixels strictly interior (>= 2 px from every edge)."""
    inner = cls[2:-2, 2:-2]
    if inner.size == 0:
        return 0.0
    return float(((inner == 2) | (inner == 0)).mean())


def classify_theater(theater: DosTheater, templates: set[str] | None = None) -> dict[str, TileClass]:
    classes = theater.index_classes()
    out: dict[str, TileClass] = {}
    for template, frames in classes.items():
        if templates is not None and template not in templates:
            continue
        for icon in range(frames.shape[0]):
            cls = frames[icon]
            out[f"{template}:{icon}"] = TileClass(
                template=template,
                icon=icon,
                land_frac=float((cls == 2).mean()),
                water_frac=float((cls == 1).mean()),
                transparent_frac=float((cls == 0).mean()),
                edges={
                    side: classify_edges(run)
                    for side, run in (
                        ("N", cls[0]),
                        ("S", cls[-1]),
                        ("W", cls[:, 0]),
                        ("E", cls[:, -1]),
                    )
                },
                clutter=clutter_fraction(cls),
            )
    return out


def dumps(classification: dict[str, TileClass]) -> str:
    def enc(ec: EdgeClass):
        return {"kind": ec.kind, "flips": ec.flips}

    def enc_tile(t: TileClass):
        return {
            "template": t.template,
            "icon": t.icon,
            "land_frac": round(t.land_frac, 3),
            "water_frac": round(t.water_frac, 3),
            "transparent_frac": round(t.transparent_frac, 3),
            "edges": {s: enc(e) for s, e in t.edges.items()},
            "clutter": round(t.clutter, 3),
        }

    return json.dumps({k: enc_tile(v) for k, v in sorted(classification.items())}, indent=1)


def atlas(
    theater: DosTheater,
    classification: dict[str, TileClass],
    columns: int = 6,
    cell: int = 128,
    out_path: str | None = None,
) -> Image.Image:
    """Labeled contact sheet: every tile with its classification summary."""
    keys = sorted(classification)
    rows = (len(keys) + columns - 1) // columns
    banner = 36
    sheet = Image.new("RGB", (cell * columns, (cell + banner) * rows), (60, 60, 80))
    draw = ImageDraw.Draw(sheet)
    for i, key in enumerate(keys):
        t = classification[key]
        x = (i % columns) * cell
        y = (i // columns) * (cell + banner)
        img = theater.icon_rgba(t.template, t.icon)
        if img.shape[0] != cell:
            img = np.asarray(
                Image.fromarray(img, "RGBA").resize((cell, cell), Image.NEAREST)
            )
        sheet.paste(Image.fromarray(img, "RGBA").convert("RGB"), (x, y))
        kinds = "".join(
            {
                "land": "L",
                "water": "W",
                "mixed": "M",
                "transparent": "T",
            }[t.edges[s].kind]
            for s in "NESW"
        )
        draw.text(
            (x + 3, y + cell + 4),
            f"{t.template}:{t.icon}  cov{t.land_frac:.2f}  NESW={kinds}",
            fill=(255, 255, 0),
        )
    if out_path:
        sheet.save(out_path)
    return sheet
