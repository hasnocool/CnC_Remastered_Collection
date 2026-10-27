"""Extract (TType, TIcon) grids from shipped TD/RA scenario .BIN files and
aggregate neighbour statistics.

The .BIN layout (verified against ``original/TIBERIANDAWN/MAP.CPP:663-700``):
sequential ``{u8 TType; u8 TIcon}`` pairs over the full 64x64 cell space
(MAP_CELL_TOTAL = 4096); TType 255 = no template. The scenario INI's
``[MAP]`` section gives the used window (X, Y, Width, Height) inside that
grid.
"""

from __future__ import annotations

import re
from collections import Counter
from dataclasses import dataclass
from pathlib import Path

import numpy as np

MAP_CELL_SIDE = 64


@dataclass
class ScenarioMap:
    name: str
    window: tuple[int, int, int, int]  # X, Y, W, H
    theater: str
    ttype: np.ndarray  # [H, W] template enum (255 = none)
    ticon: np.ndarray  # [H, W] icon index


def parse_window(ini_text: str) -> tuple[int, int, int, int] | None:
    m = re.search(r"\[MAP\]([^\[]*)", ini_text, re.I)
    if not m:
        return None
    body = m.group(1)

    def get(key: str) -> int | None:
        mm = re.search(rf"^{key}\s*=\s*(\d+)\s*$", body, re.M | re.I)
        return int(mm.group(1)) if mm else None

    x, y, w, h = get("X"), get("Y"), get("Width"), get("Height")
    if None in (x, y, w, h):
        return None
    return x, y, w, h


def parse_theater(ini_text: str) -> str:
    m = re.search(r"\[MAP\]([^\[]*)", ini_text, re.I)
    if m:
        t = re.search(r"^Theater\s*=\s*(\w+)", m.group(1), re.M | re.I)
        if t:
            return t.group(1)
    return "TEMPERATE"


def read_scenario(bin_path: Path, ini_path: Path) -> ScenarioMap | None:
    ini_text = ini_path.read_text(errors="replace")
    window = parse_window(ini_text)
    if window is None:
        return None
    data = bin_path.read_bytes()
    if len(data) < MAP_CELL_SIDE * MAP_CELL_SIDE * 2:
        return None
    grid = np.frombuffer(data, dtype=np.uint8).reshape(-1, 2)[: MAP_CELL_SIDE ** 2]
    tt = grid[:, 0].reshape(MAP_CELL_SIDE, MAP_CELL_SIDE)
    ic = grid[:, 1].reshape(MAP_CELL_SIDE, MAP_CELL_SIDE)
    x, y, w, h = window
    return ScenarioMap(
        name=bin_path.stem.upper(),
        window=window,
        theater=parse_theater(ini_text),
        ttype=tt[y : y + h, x : x + w],
        ticon=ic[y : y + h, x : x + w],
    )


def neighbour_statistics(maps: list[ScenarioMap]) -> dict:
    """Aggregate how templates and their icons relate to their neighbours.

    For every placed cell (TType != 255): the 4-neighbour template histogram
    (same template / different / none) and, per template, the distribution
    of the icon grid position (col % template-width is not known here, so
    the raw icon index histogram is kept for later template-width fitting).
    """
    same = Counter()
    diff = Counter()
    none = Counter()
    icon_hist: dict[int, Counter] = {}
    usage = Counter()
    for sm in maps:
        tt, ic = sm.ttype, sm.ticon
        h, w = tt.shape
        for y in range(h):
            for x in range(w):
                t = int(tt[y, x])
                if t == 255:
                    continue
                usage[t] += 1
                icon_hist.setdefault(t, Counter())[int(ic[y, x])] += 1
                for dx, dy in ((0, -1), (1, 0), (0, 1), (-1, 0)):
                    nx, ny = x + dx, y + dy
                    if not (0 <= nx < w and 0 <= ny < h):
                        none[t] += 1
                        continue
                    n = int(tt[ny, nx])
                    if n == 255:
                        none[t] += 1
                    elif n == t:
                        same[t] += 1
                    else:
                        diff[t] += 1
    return {
        "same": same,
        "diff": diff,
        "none": none,
        "icon_hist": icon_hist,
        "usage": usage,
    }
