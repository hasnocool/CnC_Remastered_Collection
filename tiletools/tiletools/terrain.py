"""Seeded terrain material field: land / water / shore.

Value-noise fBm with an island falloff, pure function of ``(seed, size)``,
mirroring the determinism convention of ``RUST/cnc-procgen``. The material
field is the ground truth every matcher scores against; the pixel art is
never consulted here.
"""

from __future__ import annotations

from dataclasses import dataclass

import numpy as np

from .join import Material


def _value_noise(rng: np.random.Generator, size: int, cells: int) -> np.ndarray:
    """Bilinear-interpolated lattice noise on an (size, size) grid."""
    lattice = rng.random((cells + 1, cells + 1))
    ys = np.linspace(0.0, cells, size, endpoint=False)
    xs = np.linspace(0.0, cells, size, endpoint=False)
    y0 = np.floor(ys).astype(int)[:, None]
    x0 = np.floor(xs).astype(int)[None, :]
    fy = (ys - np.floor(ys))[:, None]
    fx = (xs - np.floor(xs))[None, :]
    a = lattice[y0, x0]
    b = lattice[y0, x0 + 1]
    c = lattice[y0 + 1, x0]
    d = lattice[y0 + 1, x0 + 1]
    top = a * (1 - fx) + b * fx
    bottom = c * (1 - fx) + d * fx
    return top * (1 - fy) + bottom * fy


def fbm(seed: int, size: int, octaves: int = 4, base_cells: int = 4) -> np.ndarray:
    rng = np.random.default_rng(seed)
    out = np.zeros((size, size))
    amp, total = 1.0, 0.0
    cells = base_cells
    for _ in range(octaves):
        out += amp * _value_noise(rng, size, cells)
        total += amp
        amp *= 0.5
        cells *= 2
    return out / total


@dataclass
class TerrainField:
    """Per-cell material classification for one map."""

    size: int
    materials: np.ndarray  # [size, size] of Material (object array)
    pool: np.ndarray = None  # [size, size] bool: water/shore not border-connected

    def __post_init__(self):
        if self.pool is None:
            self.pool = np.zeros((self.size, self.size), dtype=bool)

    def material_at(self, x: int, y: int) -> Material | None:
        if 0 <= x < self.size and 0 <= y < self.size:
            return self.materials[y, x]
        return None

    def neighbours(self, x: int, y: int) -> list[tuple[Material | None, str]]:
        """[(material, side)] for 8 directions; off-map counts as same."""
        here = self.material_at(x, y)
        out: list[tuple[Material | None, str]] = []
        for side, (dx, dy) in OFFSETS.items():
            other = self.material_at(x + dx, y + dy)
            out.append((other if other is not None else here, side))
        return out


OFFSETS = {
    "N": (0, -1),
    "NE": (1, -1),
    "E": (1, 0),
    "SE": (1, 1),
    "S": (0, 1),
    "SW": (-1, 1),
    "W": (-1, 0),
    "NW": (-1, -1),
}


def island_falloff(size: int, margin: int = 4) -> np.ndarray:
    """0 at the border ring, 1 inside: map borders are open ocean."""
    falloff = np.ones((size, size))
    ramp = np.ones(size)
    if margin > 0:
        ramp = np.linspace(0.0, 1.0, margin + 1)[1:]
        falloff[:margin, :] *= ramp[:, None]
        falloff[-margin:, :] *= ramp[::-1][:, None]
        falloff[:, :margin] *= ramp[None, :]
        falloff[:, -margin:] *= ramp[::-1][None, :]
    return falloff


def generate_field(seed: int, size: int, sea_level: float = 0.35) -> TerrainField:
    height = fbm(seed, size, octaves=4, base_cells=2)
    height = (height - height.min()) / max(1e-9, height.max() - height.min())
    height = height * island_falloff(size)
    materials = np.where(
        height < sea_level,
        np.array(Material.WATER, dtype=object),
        np.array(Material.LAND, dtype=object),
    )
    return TerrainField(size=size, materials=materials)


def apply_shore(field: TerrainField) -> None:
    """Water cells with a land 4-neighbour become Material.SHORE.

    The original shoreline look: the beach arc lives on the WATER side
    (SH* templates are water-based with beach icons on their land edge);
    grass stays grass right up to the boundary.
    """
    mats = field.materials
    water = mats == Material.WATER
    land = mats == Material.LAND
    land_4neighbour = _any_cardinal(land)
    mats[water & land_4neighbour] = Material.SHORE


def label_pools(field: TerrainField, max_pool: int = 4) -> None:
    """Mark SMALL water/shore regions not 4-connected to a border as pools.

    A pool (≤ ``max_pool`` cells) renders with the grass-side pond templates
    (SH32-35); a bigger inner lake reads as ocean and uses the water-based
    SH10-13 band like the map-border ocean.
    """
    size = field.size
    wet = np.isin(field.materials, [Material.WATER, Material.SHORE])
    field.pool = np.zeros_like(wet)
    seen: set[tuple[int, int]] = set()
    for sy in range(size):
        for sx in range(size):
            if not wet[sy, sx] or (sx, sy) in seen:
                continue
            region = []
            stack = [(sx, sy)]
            seen.add((sx, sy))
            touches_border = False
            while stack:
                x, y = stack.pop()
                region.append((x, y))
                if x in (0, size - 1) or y in (0, size - 1):
                    touches_border = True
                for dx, dy in ((0, -1), (1, 0), (0, 1), (-1, 0)):
                    nx, ny = x + dx, y + dy
                    if (
                        0 <= nx < size
                        and 0 <= ny < size
                        and wet[ny, nx]
                        and (nx, ny) not in seen
                    ):
                        seen.add((nx, ny))
                        stack.append((nx, ny))
            if not touches_border and len(region) <= max_pool:
                for x, y in region:
                    field.pool[y, x] = True


def smooth(field: TerrainField, passes: int = 2) -> None:
    """Majority-filter the land/water mask so coasts run straight.

    The DOS coast art chains 3x3/2x2 patches along straight strips; a jagged
    fBm coastline fragments into 1-2 cell runs where no patch fits, and the
    per-cell fallback renders it as square wedges. A 3x3 majority filter
    lengthens the runs.
    """
    mats = field.materials
    for _ in range(passes):
        land = mats == Material.LAND
        shore = mats == Material.SHORE
        solid = land | shore
        # Wet cells with >= 5 wet 8-neighbours (counting off-map as wet)
        # become water; wet cells with >= 6 solid neighbours become land.
        wet_neighbours = _any_neighbour_count(solid)
        solid_neighbours = _any_neighbour_count(~solid)
        wet = ~solid
        new_water = wet & (wet_neighbours >= 5)
        new_land = wet & (solid_neighbours >= 6)
        mats[new_water] = Material.WATER
        mats[new_land] = Material.LAND
    apply_shore(field)
    label_pools(field)


def _any_neighbour_count(mask: np.ndarray) -> np.ndarray:
    """Count of 8-neighbours satisfying ``mask``; off-map counts as True."""
    out = np.zeros(mask.shape, dtype=int)
    padded = np.pad(mask, 1, constant_values=True)
    for dx, dy in OFFSETS.values():
        out += padded[1 + dy : 1 + dy + mask.shape[0], 1 + dx : 1 + dx + mask.shape[1]]
    return out


def _any_cardinal(mask: np.ndarray, pad_value: bool = False) -> np.ndarray:
    out = np.zeros_like(mask)
    padded = np.pad(mask, 1, constant_values=pad_value)
    for dx, dy in ((0, -1), (1, 0), (0, 1), (-1, 0)):
        out |= padded[1 + dy : 1 + dy + mask.shape[0], 1 + dx : 1 + dx + mask.shape[1]]
    return out


def _any_neighbour(mask: np.ndarray, pad_value: bool = False) -> np.ndarray:
    """True where any 8-neighbour satisfies ``mask``; off-map is ``pad_value``."""
    out = np.zeros_like(mask)
    padded = np.pad(mask, 1, constant_values=pad_value)
    for dx, dy in OFFSETS.values():
        out |= padded[1 + dy : 1 + dy + mask.shape[0], 1 + dx : 1 + dx + mask.shape[1]]
    return out
