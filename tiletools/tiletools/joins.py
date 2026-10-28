"""Render the SH-family join-compatibility matrix.

For the coast/water vocabulary (SH34, SH35, SH12, SH13, SH10, SH11, W1),
compute for every ordered icon pair whether the facing edges agree exactly
(sand-normalized class runs). Render two matrices — S->N seams (vertical)
and E->W seams (horizontal) — so a human can read at a glance which icons
continue which, i.e. how the tileset fits best.

Cell colour: dark green = exact run match, amber = partial, red = mismatch.
Rows = the WEST (or NORTH) neighbour's EAST (or SOUTH) edge owner; columns
= the candidate on the other side.
"""

from __future__ import annotations

import numpy as np
from PIL import Image, ImageDraw

VOCAB = ("W1", "SH34", "SH35", "SH12", "SH13", "SH10", "SH11")
OPP = {"N": "S", "E": "W", "S": "N", "W": "E"}

GREEN = (34, 102, 34)
AMBER = (168, 130, 42)
RED = (140, 40, 40)
BG = (24, 24, 32)


def _icons_in(table, tpl: str) -> list[int]:
    return [c.icon for c in table.by_material.get(__import__("tiletools.join", fromlist=["Material"]).Material.WATER, []) + table.by_material.get(__import__("tiletools.join", fromlist=["Material"]).Material.SHORE, []) if c.template.upper() == tpl and c.icon not in [x for x in []]]


def render_join_matrix(table, out_path, cell: int = 22) -> dict:
    """Two seam matrices (vertical, horizontal) over the coast vocabulary.

    Returns {"exact": {(tplA, i, tplB, j): (v, h)}} for reuse in docs.
    """
    from .join import Material

    # Collect candidates per template from shore+water material pools.
    pool = table.by_material.get(Material.SHORE, []) + table.by_material.get(
        Material.WATER, []
    )
    entries: dict[str, list] = {}
    for c in pool:
        entries.setdefault(c.template.upper(), []).append(c)
    icons: list = []
    for tpl in VOCAB:
        for c in sorted(entries.get(tpl, []), key=lambda c: c.icon):
            icons.append(c)
    n = len(icons)

    def frac(a, b) -> float:
        if a is None or b is None or len(a) != len(b):
            return 1.0
        return float((a % 3 != b % 3).mean())

    vert = np.zeros((n, n))  # row = north neighbour (its S run), col = candidate (N run)
    horiz = np.zeros((n, n))  # row = west neighbour (E run), col = candidate (W run)
    exact = {}
    for r, ra in enumerate(icons):
        for c_, rb in enumerate(icons):
            v = frac(ra.runs.get("S"), rb.runs.get("N"))
            h = frac(ra.runs.get("E"), rb.runs.get("W"))
            vert[r, c_] = v
            horiz[r, c_] = h
            exact[(ra.template, ra.icon, rb.template, rb.icon)] = (1 - v, 1 - h)

    def draw(mat: np.ndarray, title: str, y0: int) -> int:
        img = Image.new("RGB", (cell * n + 320, cell * n + 60), BG)
        d = ImageDraw.Draw(img)
        for r, ra in enumerate(icons):
            d.text((4, y0 + 8 + r * cell), f"{ra.template}.{ra.icon}", fill=(200, 200, 200))
            d.text((320 + r * cell + 2, y0 + 8 + n * cell + 2), f"{ra.icon}", fill=(200, 200, 200))
        for r in range(n):
            for c_ in range(n):
                v = mat[r, c_]
                col = GREEN if v == 0 else (RED if v > 0.5 else AMBER)
                d.rectangle(
                    [320 + c_ * cell, y0 + 8 + r * cell,
                     320 + (c_ + 1) * cell - 1, y0 + 8 + (r + 1) * cell - 1],
                    fill=col,
                )
        d.text((320, y0 - 20 if y0 >= 20 else 4), title, fill=(230, 230, 230))
        return y0 + 8 + n * cell + 24

    height = 40 + (60 + cell * n + 60) * 2
    img = Image.new("RGB", (cell * n + 340, height), BG)
    d = ImageDraw.Draw(img)
    y = 10
    d.text((4, y), "Which S-edge continues onto which N-edge (vertical seams)", fill=(230, 230, 230))
    y += 20
    for r, ra in enumerate(icons):
        d.text((4, y + 8 + r * cell), f"{ra.template}.{ra.icon}", fill=(200, 200, 200))
        d.text((320 + r * cell + 2, y + 8 + n * cell + 2), str(ra.icon), fill=(200, 200, 200))
    d.text((320, y), "candidate (columns, same order)", fill=(180, 180, 180))
    for r in range(n):
        for c_ in range(n):
            v = vert[r, c_]
            col = GREEN if v == 0 else (RED if v > 0.5 else AMBER)
            d.rectangle([320 + c_ * cell, y + 8 + r * cell,
                         320 + (c_ + 1) * cell - 1, y + 8 + (r + 1) * cell - 1], fill=col)
    y += 8 + n * cell + 30
    d.text((4, y), "Which E-edge continues onto which W-edge (horizontal seams)", fill=(230, 230, 230))
    y += 20
    for r, ra in enumerate(icons):
        d.text((4, y + 8 + r * cell), f"{ra.template}.{ra.icon}", fill=(200, 200, 200))
        d.text((320 + r * cell + 2, y + 8 + n * cell + 2), str(ra.icon), fill=(200, 200, 200))
    for r in range(n):
        for c_ in range(n):
            v = horiz[r, c_]
            col = GREEN if v == 0 else (RED if v > 0.5 else AMBER)
            d.rectangle([320 + c_ * cell, y + 8 + r * cell,
                         320 + (c_ + 1) * cell - 1, y + 8 + (r + 1) * cell - 1], fill=col)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    img.save(out_path)
    return {"icons": [(c.template, c.icon) for c in icons], "exact": exact}
