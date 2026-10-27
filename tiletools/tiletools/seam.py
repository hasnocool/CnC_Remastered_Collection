"""Quantitative seam metric, material-aware.

The original DOS tiles are fully opaque, so raw color distance can't
separate a *defect* from a *coastline*: land meeting water at a boundary is
intended. A defect is a class change across a seam where the material field
says both cells are the same material, or any transparent-vs-opaque pair
(a hole or a cut-out). Classes: transparent (alpha < cutoff), water
(blue-dominant), land (everything else).
"""

from __future__ import annotations

import json
from dataclasses import asdict, dataclass

import numpy as np

from .matcher import CellPick

ALPHA_CUTOFF = 32


@dataclass
class SeamStats:
    holes: int  # transparent-vs-opaque across a seam: real defect
    transitions: int  # class changes along the seam: coastlines and arc notches
    checked_edges: int
    hole_fraction: float
    worst_edge: float
    worst_cells: list[str]


def _classify(strip: np.ndarray) -> np.ndarray:
    """0 = transparent, 1 = water, 2 = land; [N] uint8."""
    alpha = strip[:, 3]
    rgb = strip[:, :3].astype(np.int16)
    out = np.full(len(strip), 2, dtype=np.uint8)
    out[alpha < ALPHA_CUTOFF] = 0
    body = rgb[alpha >= ALPHA_CUTOFF]
    water = (body[:, 2] > body[:, 0] + 20) & (body[:, 2] > body[:, 1] + 5)
    idx = np.where(alpha >= ALPHA_CUTOFF)[0]
    out[idx[water]] = 1
    return out


def _strip(rgba: np.ndarray, side: str) -> np.ndarray:
    return {"N": rgba[0], "S": rgba[-1], "W": rgba[:, 0], "E": rgba[:, -1]}[side]


def edge_pixels(art, pick: CellPick, side: str) -> np.ndarray:
    return np.asarray(art.icon_rgba(pick.template, pick.icon), dtype=np.int16)[
        {"N": (0,), "S": (-1,), "W": (slice(None), 0), "E": (slice(None), -1)}[side]
    ]


def seam_score(
    art,
    picks: np.ndarray,
    materials: np.ndarray | None = None,
    classes: dict[str, np.ndarray] | None = None,
) -> SeamStats:
    size = picks.shape[0]
    holes = 0
    transitions = 0
    checked = 0
    worst = -1.0
    worst_cells: list[str] = []

    def pick_classes(pick: CellPick, side: str) -> np.ndarray:
        if classes is not None:
            cls = classes[pick.template.upper()][pick.icon]
            return {"N": cls[0], "S": cls[-1], "W": cls[:, 0], "E": cls[:, -1]}[side]
        return _classify(edge_pixels(art, pick, side))

    for y in range(size):
        for x in range(size):
            for side, (nx, ny) in (("S", (x, y + 1)), ("E", (x + 1, y))):
                if not (0 <= nx < size and 0 <= ny < size):
                    continue
                a = pick_classes(picks[y, x], side)
                opposite = "N" if side == "S" else "W"
                b = pick_classes(picks[ny, nx], opposite)
                alpha_err = ((a == 0) != (b == 0)).astype(np.float32)
                class_err = (a != b).astype(np.float32)
                mean = float(np.maximum(alpha_err, 0.0).mean())
                mean_class = float(class_err.mean())
                checked += 1
                if mean_class > worst:
                    worst = mean_class
                    worst_cells = [f"({x},{y})|({nx},{ny}) {side}"]
                elif mean_class == worst and len(worst_cells) < 5:
                    worst_cells.append(f"({x},{y})|({nx},{ny}) {side}")
                if mean > 0.02:
                    holes += 1
                if mean_class > 0.02:
                    transitions += 1
    return SeamStats(
        holes=holes,
        transitions=transitions,
        checked_edges=checked,
        hole_fraction=holes / max(1, checked),
        worst_edge=max(0.0, worst),
        worst_cells=worst_cells,
    )


def dumps(stats: SeamStats) -> str:
    return json.dumps(asdict(stats), indent=2)
