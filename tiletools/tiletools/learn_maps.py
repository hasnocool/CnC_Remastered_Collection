"""Learn coast/patch placement conventions from the shipped scenario maps.

Reads every scenario .BIN/.INI pair in the Steam install's CNCDATA tree,
maps the (TType, TIcon) grid to template INI names via the CDATA enum order,
and learns:

- per-TEMPLATE icon index histograms (how the 3x3/2x2 patches are actually
  oriented in shipped maps);
- coast-cell geometry: for each water cell adjacent to land, which template
  + icon the shipped map placed there, keyed by the 4-bit land mask
  (N=1, E=2, S=4, W=8) — the empirical ``SHORE_BY_MASK`` for every theater;
- the same for land cells adjacent to water (the shelf side).

The learned tables feed the matcher's placement rules so generated maps use
the shipped conventions instead of guessing.
"""

from __future__ import annotations

import json
import re
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from pathlib import Path

import numpy as np

from .scenario import read_scenario
from .cdata import td_templates, ra_templates

_MASK_SIDES = ((0, -1, 1), (1, 0, 2), (0, 1, 4), (-1, 0, 8))


@dataclass
class MapKnowledge:
    shore_by_mask: dict[str, dict[int, Counter]]  # theater -> mask -> (tpl,icon)
    land_water_edge: dict[str, dict[int, Counter]]  # land side, same key
    icon_usage: dict[str, dict[str, Counter]]  # theater -> template -> icon hist
    scenarios: int
    cells: int

    def dump(self, path: Path) -> None:
        def strkeys(d):
            if isinstance(d, dict):
                return {str(k): strkeys(v) for k, v in d.items()}
            return d

        data = {
            "scenarios": self.scenarios,
            "cells": self.cells,
            "shore_by_mask": strkeys(
                {th: {m: dict(c) for m, c in masks.items()}
                 for th, masks in self.shore_by_mask.items()}
            ),
            "land_water_edge": strkeys(
                {th: {m: dict(c) for m, c in masks.items()}
                 for th, masks in self.land_water_edge.items()}
            ),
            "icon_usage": strkeys(
                {th: {t: dict(c) for t, c in per.items()}
                 for th, per in self.icon_usage.items()}
            ),
        }
        path.write_text(json.dumps(data, indent=1, sort_keys=True))


def _enum_names(src_path: Path) -> dict[int, str]:
    src = src_path.read_text(errors="replace")
    order = re.findall(r"TEMPLATE_(\w+),", src)
    return {i: nm for i, nm in enumerate(order)}


def learn_from_maps(
    base: Path,
    game: str = "td",
    gamma_theaters: bool = False,
) -> MapKnowledge:
    """Scan ``CNCDATA/<GAME>`` recursively for SC*.INI/.BIN pairs."""
    templates = td_templates() if game == "td" else ra_templates()
    enum_src = (
        Path(__file__).resolve().parents[2]
        / "original" / ("TIBERIANDAWN" if game == "td" else "REDALERT") / "CDATA.CPP"
    )
    enums = _enum_names(enum_src)
    # enum -> INI name (CDATA ctor table): TEMPLATE_CLEAR1 -> "CLEAR1" etc.
    enum_to_template = {}
    for enum_id, nm in enums.items():
        ini = nm
        if ini in templates:
            enum_to_template[enum_id] = templates[ini]

    shore_by_mask: dict[str, dict[int, Counter]] = defaultdict(lambda: defaultdict(Counter))
    land_water_edge: dict[str, dict[int, Counter]] = defaultdict(lambda: defaultdict(Counter))
    icon_usage: dict[str, dict[str, Counter]] = defaultdict(lambda: defaultdict(Counter))
    scenarios = 0
    cells = 0

    for ini_path in sorted(base.rglob("SC*.INI")):
        bin_path = ini_path.with_suffix(".BIN")
        if not bin_path.exists():
            continue
        sm = read_scenario(bin_path, ini_path)
        if sm is None:
            continue
        scenarios += 1
        theater = sm.theater
        tt, ic = sm.ttype, sm.ticon
        h, w = tt.shape
        for y in range(h):
            for x in range(w):
                t = int(tt[y, x])
                if t == 255:
                    continue
                cells += 1
                definition = enum_to_template.get(t)
                if definition is None:
                    continue
                template = definition.ini_name
                icon = int(ic[y, x])
                icon_usage[theater][template][icon] += 1

                # water cell with land neighbours = shore geometry
                if definition.land == "WATER":
                    mask = 0
                    for dx, dy, bit in _MASK_SIDES:
                        nx, ny = x + dx, y + dy
                        if 0 <= nx < w and 0 <= ny < h:
                            n = int(tt[ny, nx])
                            if n != 255 and enum_to_template.get(n) is not None:
                                if enum_to_template[n].land in ("CLEAR", "BEACH", "ROAD", "ROCK"):
                                    mask |= bit
                    if mask:
                        shore_by_mask[theater][mask][(template, icon)] += 1
                elif definition.land == "CLEAR" and definition.alt_land == "WATER":
                    pass  # pond-side: handled by the shore entry
                # land cell adjacent to water = shelf side
                wet_neighbour = any(
                    int(tt[y + dy, x + dx]) != 255
                    and enum_to_template.get(int(tt[y + dy, x + dx])) is not None
                    and enum_to_template[int(tt[y + dy, x + dx])].land == "WATER"
                    for dx, dy in ((0, -1), (1, 0), (0, 1), (-1, 0))
                    if 0 <= x + dx < w and 0 <= y + dy < h
                )
                if wet_neighbour and definition.land != "WATER":
                    mask = 0
                    for dx, dy, bit in _MASK_SIDES:
                        nx, ny = x + dx, y + dy
                        if 0 <= nx < w and 0 <= ny < h:
                            n = int(tt[ny, nx])
                            if n != 255 and enum_to_template.get(n) is not None:
                                if enum_to_template[n].land == "WATER":
                                    mask |= bit
                    if mask:
                        land_water_edge[theater][mask][(template, icon)] += 1
    return MapKnowledge(shore_by_mask, land_water_edge, icon_usage, scenarios, cells)
