"""Per-cell template/icon selection.

Strategies, both deterministic and seeded:

- ``naive`` — random base candidate of the cell's material, ignoring
  neighbours. The baseline that shows seams.
- ``matched`` — greedy scanline with exact edge-run alignment:
  cells are placed row-major; the North and West neighbours are already
  placed, so their facing edge class-runs are known exactly and the
  candidate's N/W edge runs are compared pixel-for-pixel (mirrored).
  East/South and diagonals score by the coarse fractional demands from
  ``join.edge_demand``. Ties break toward the most solid tile, then seeded
  random among near-equal, so shorelines vary instead of repeating one icon.

This is a Wang-tile placement in the space of the shipped art's edge runs.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

import numpy as np

from .join import (
    JoinTable,
    Material,
    edge_demand,
    normalize_template,
)
from .terrain import TerrainField


class Strategy(Enum):
    NAIVE = "naive"
    MATCHED = "matched"


@dataclass
class CellPick:
    template: str
    icon: int


# TD shore resolution by 4-bit land mask (N=1, E=2, S=4, W=8) from
# `RUST/cnc-map-editor/src/original_tileset.rs` (remaster-art tuned; kept for
# the mask-restrict mode).
SHORE_BY_MASK: tuple[tuple[str, int], ...] = (
    ("SH11", 7), ("SH2", 6), ("SH12", 6), ("SH11", 6),
    ("SH10", 4), ("SH1", 0), ("SH12", 6), ("SH14", 5),
    ("SH2", 4), ("SH17", 2), ("SH11", 4), ("SH1", 6),
    ("SH2", 4), ("SH1", 0), ("SH11", 4), ("SH1", 1),
)

_LAND_MASK_SIDES = ((0, -1, 1), (1, 0, 2), (0, 1, 4), (-1, 0, 8))


def land_mask(field: TerrainField, x: int, y: int) -> int:
    size = field.size
    mask = 0
    for dx, dy, bit in _LAND_MASK_SIDES:
        nx, ny = x + dx, y + dy
        if not (0 <= nx < size and 0 <= ny < size):
            continue
        if field.materials[ny, nx] in (Material.LAND, Material.SHORE):
            mask |= bit
    return mask


def _run_mismatch(a: np.ndarray, b: np.ndarray) -> float:
    """Fraction of differing pixels between facing edge runs.

    Both runs enumerate in the same screen direction (x for N/S seams,
    y for W/E seams), so they compare directly — no mirroring.
    """
    if a is None or b is None or len(a) != len(b):
        return 0.0
    return float((a != b).mean())


def _demand_for(field: TerrainField, x: int, y: int, material: Material, mat: Material, side: str) -> str:
    # Within-band (SHORE|SHORE) arc phase is chosen by the scanline's exact
    # N/W run alignment; unplaced sides stay field-driven.
    return edge_demand(material, mat)


def _coarse_error(cand, side: str, demand: str) -> float:
    return cand.edges.__getattribute__(side.lower()).disagreement(demand)


def _diagonal_coarse(cand, material: Material, neighbours8, weight: float) -> float:
    total = 0.0
    for mat, side in neighbours8:
        if side not in ("NE", "SE", "SW", "NW"):
            continue
        demand = edge_demand(material, mat)
        a, b = side[0], side[1]
        approx = 0.5 * (
            _coarse_error(cand, a, demand) + _coarse_error(cand, b, demand)
        )
        total += weight * approx
    return total


def scanline_assign(
    field: TerrainField,
    table: JoinTable,
    seed: int = 0,
    families: dict[Material, list[str]] | None = None,
    diagonal_weight: float = 0.5,
    pool_families: list[str] | None = None,
) -> np.ndarray:
    """Greedy scanline placement with exact N/W run alignment."""
    size = field.size
    rng = np.random.default_rng(seed ^ 0x5EED)
    placed: np.ndarray = np.empty((size, size), dtype=object)

    def allowed_for(material: Material, x: int, y: int) -> list[str] | None:
        allowed = families.get(material) if families else None
        if (
            pool_families is not None
            and field.pool is not None
            and field.pool[y, x]
            and material in (Material.SHORE, Material.WATER)
        ):
            allowed = pool_families
        return allowed

    for y in range(size):
        for x in range(size):
            material = field.materials[y, x]
            allowed = allowed_for(material, x, y)
            candidates = table.bases(material)
            if (
                allowed is not None
                and field.pool is not None
                and field.pool[y, x]
                and material in (Material.SHORE, Material.WATER)
            ):
                # Pool cells draw from the pond templates' FULL icon sets:
                # their sand-ring icons are CLEAR-land by CDATA but are the
                # correct pond-shore art.
                want = {normalize_template(t).upper() for t in allowed}
                candidates = [
                    c
                    for c in table.bases(Material.SHORE)
                    + table.bases(Material.WATER)
                    + table.bases(Material.LAND)
                    if normalize_template(c.template).upper() in want
                ]
            elif allowed is not None:
                want = {normalize_template(t).upper() for t in allowed}
                candidates = [c for c in candidates if normalize_template(c.template).upper() in want]
            if not candidates:
                raise ValueError(f"no candidates for {material}")

            north_run = placed[y - 1, x].runs.get("S") if y > 0 else None
            west_run = placed[y, x - 1].runs.get("E") if x > 0 else None

            neighbours8 = field.neighbours(x, y)

            def cost(c):
                total = 0.0
                # Exact alignments (weight 2: pixel-exact seams matter most)
                if north_run is not None:
                    total += 2.0 * _run_mismatch(c.runs.get("N"), north_run)
                if west_run is not None:
                    total += 2.0 * _run_mismatch(c.runs.get("W"), west_run)
                # Coarse demands for unplaced sides
                for mat, side in neighbours8:
                    if side in ("N", "W") and (
                        (side == "N" and north_run is not None)
                        or (side == "W" and west_run is not None)
                    ):
                        continue  # already exactly scored
                    if side in ("NE", "SE", "SW", "NW"):
                        continue
                    demand = _demand_for(field, x, y, material, mat, side)
                    total += _coarse_error(c, side, demand)
                total += _diagonal_coarse(c, material, neighbours8, diagonal_weight)
                return total

            scored = sorted(candidates, key=cost)
            best = cost(scored[0])
            pool = [c for c in scored if abs(cost(c) - best) < 1e-6]
            chosen = pool[int(rng.integers(len(pool)))] if len(pool) > 1 else scored[0]
            placed[y, x] = chosen
    return placed


def pick(
    strategy: Strategy,
    field: TerrainField,
    table: JoinTable,
    x: int,
    y: int,
    rng: np.random.Generator | None = None,
    diagonal_weight: float = 0.5,
    families: dict[Material, list[str]] | None = None,
    shore_mask_restrict: bool = True,
) -> CellPick:
    """Single-cell pick (no scanline context) — used by naive and tests."""
    material = field.materials[y, x]
    neighbours = field.neighbours(x, y)
    allowed: list[str] | None = families.get(material) if families else None
    if strategy is Strategy.MATCHED and material is Material.SHORE and shore_mask_restrict:
        allowed = [SHORE_BY_MASK[land_mask(field, x, y)][0]]
    candidates = table.bases(material)
    if allowed is not None:
        want = {normalize_template(t).upper() for t in allowed}
        candidates = [c for c in candidates if normalize_template(c.template).upper() in want]
    if not candidates:
        raise ValueError(f"no base candidates for {material}")
    if strategy is Strategy.NAIVE:
        chosen = candidates[int(rng.integers(len(candidates)))] if rng is not None else candidates[0]
        return CellPick(chosen.template, chosen.icon)
    # Coarse-only matched pick (fractions + diagonals)
    def coarse_cost(c):
        orth = sum(
            c.edges.__getattribute__(s.lower()).disagreement(edge_demand(material, m))
            for m, s in neighbours
            if s in ("N", "E", "S", "W") and m is not None
        )
        return orth + _diagonal_coarse(c, material, neighbours, diagonal_weight)

    scored = sorted(candidates, key=lambda c: (coarse_cost(c), -c.solidity, c.template, c.icon))
    best = coarse_cost(scored[0])
    pool = [c for c in scored if abs(coarse_cost(c) - best) < 1e-6]
    chosen = pool[int(rng.integers(len(pool)))] if (rng is not None and len(pool) > 1) else scored[0]
    return CellPick(chosen.template, chosen.icon)


def assign(
    strategy: Strategy,
    field: TerrainField,
    table: JoinTable,
    seed: int = 0,
    diagonal_weight: float = 0.5,
    families: dict[Material, list[str]] | None = None,
    shore_mask_restrict: bool = True,
    pool_families: list[str] | None = None,
) -> np.ndarray:
    if strategy is Strategy.MATCHED:
        return scanline_assign(
            field,
            table,
            seed=seed,
            families=families,
            diagonal_weight=diagonal_weight,
            pool_families=pool_families,
        )
    size = field.size
    rng = np.random.default_rng(seed ^ 0x5EED)
    picks = np.empty((size, size), dtype=object)
    for y in range(size):
        for x in range(size):
            picks[y, x] = pick(
                strategy,
                field,
                table,
                x,
                y,
                rng=rng,
                diagonal_weight=diagonal_weight,
                families=families,
                shore_mask_restrict=shore_mask_restrict,
            )
    return picks
