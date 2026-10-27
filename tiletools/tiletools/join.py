"""Edge-profile join matching, ported from ``RUST/cnc-asset/src/tile_join.rs``.

The shipped art carries no join metadata: a generator must pick
`(template, icon)` pairs whose shared edges agree on material. For every
shipped icon this module measures each edge's outermost pixel run as
transparent / opaque / water fractions, then scores candidates against the
neighbour material field:

- same-material neighbour -> the facing edge must carry material (opaque or
  water), or the material visibly stops at the seam;
- different-material neighbour -> the facing edge must yield (be
  transparent), or it paints its own material over the neighbour's.

Water-blue edges facing land on a WATER tile are an outright seam break and
get an extra penalty. Ties break toward the most solid tile, then by
(template, icon) so results are deterministic.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum

import numpy as np

ALPHA_CUTOFF = 32

ORTHOGONAL = ("N", "E", "S", "W")
DIAGONAL = ("NE", "SE", "SW", "NW")


class Material(Enum):
    LAND = 1
    WATER = 2
    SHORE = 3
    ROAD = 4
    ROCK = 5
    RIVER = 6


# Original `LandType` (CDATA.CPP) -> join material.
LAND_TYPE_MATERIAL = {
    "CLEAR": Material.LAND,
    "CLEAR2": Material.LAND,
    "TIBERIUM": Material.LAND,
    "WATER": Material.WATER,
    "BEACH": Material.SHORE,
    "ROAD": Material.ROAD,
    "ROCK": Material.ROCK,
    "RIVER": Material.RIVER,
}


@dataclass
class EdgeProfile:
    transparent: float = 0.0
    opaque: float = 0.0
    water: float = 0.0

    @property
    def wants_opaque(self) -> float:
        return self.opaque + self.water

    def disagreement(self, demand: str) -> float:
        """``demand`` in {'opaque', 'water', 'any', 'transparent'}."""
        if demand == "opaque":
            return 1.0 - self.opaque
        if demand == "water":
            return 1.0 - self.water
        if demand == "any":
            return 1.0 - self.wants_opaque
        return self.wants_opaque  # transparent


@dataclass
class EdgeProfiles:
    n: EdgeProfile = field(default_factory=EdgeProfile)
    e: EdgeProfile = field(default_factory=EdgeProfile)
    s: EdgeProfile = field(default_factory=EdgeProfile)
    w: EdgeProfile = field(default_factory=EdgeProfile)

    def side(self, side: str) -> EdgeProfile:
        return getattr(self, side.lower())


def edge_demand(self_material: Material, neighbour: Material) -> str:
    """What the shared edge must look like, from both sides.

    - facing the same or compatible material: carry it (opaque land, or
      water colour);
    - facing a different material: yield (transparent), except shore art,
      which is opaque water with a beach on its land edge, so a shore tile
      facing water demands WATER colour and facing land demands OPAQUE.
    """
    if self_material is neighbour:
        return "any"
    pair = {self_material, neighbour}
    if Material.WATER in pair and Material.LAND in pair:
        # Water directly against land: the water side yields (the shore
        # system draws the transition on the water side).
        return "transparent" if self_material is Material.WATER else "opaque"
    if self_material is Material.SHORE:
        return "water" if neighbour is Material.WATER else "opaque"
    if self_material is Material.WATER:
        return "water" if neighbour in (Material.WATER, Material.SHORE) else "transparent"
    if self_material is Material.LAND:
        return "opaque" if neighbour in (Material.LAND, Material.SHORE) else "transparent"
    return "transparent"


def measure_edge(rgba: np.ndarray, coords: np.ndarray) -> EdgeProfile:
    """Classify one edge's pixel run; ``coords`` is an [N, 2] array of (x, y)."""
    px = rgba[coords[:, 1], coords[:, 0]].astype(np.int16)
    alpha = px[:, 3]
    transparent = int((alpha < ALPHA_CUTOFF).sum())
    body = px[alpha >= ALPHA_CUTOFF]
    water = int(
        ((body[:, 2] > body[:, 0] + 20) & (body[:, 2] > body[:, 1] + 5)).sum()
    )
    opaque = len(body) - water
    total = max(1, transparent + opaque + water)
    return EdgeProfile(transparent / total, opaque / total, water / total)


def measure_icon(rgba: np.ndarray) -> EdgeProfiles:
    h, w = rgba.shape[:2]
    if w < 4 or h < 4:
        raise ValueError(f"icon too small: {w}x{h}")
    xs = np.arange(w)
    ys = np.arange(h)
    return EdgeProfiles(
        n=measure_edge(rgba, np.column_stack((xs, np.zeros(w, dtype=int)))),
        s=measure_edge(rgba, np.column_stack((xs, np.full(w, h - 1)))),
        w=measure_edge(rgba, np.column_stack((np.zeros(h, dtype=int), ys))),
        e=measure_edge(rgba, np.column_stack((np.full(h, w - 1), ys))),
    )


def measure_icon_classes(cls: np.ndarray) -> tuple[EdgeProfiles, dict[str, np.ndarray]]:
    """Measure from index-class arrays (0=transparent, 1=water, 2=land).

    Returns the fractional profiles plus the exact per-edge class runs
    (N/E/S/W), which the scanline matcher aligns pixel-for-pixel.
    """
    h, w = cls.shape
    if w < 4 or h < 4:
        raise ValueError(f"icon too small: {w}x{h}")

    def profile(run: np.ndarray) -> EdgeProfile:
        total = max(1, len(run))
        return EdgeProfile(
            transparent=float((run == 0).sum()) / total,
            water=float((run == 1).sum()) / total,
            opaque=float((run == 2).sum()) / total,
        )

    runs = {
        "N": cls[0],
        "S": cls[-1],
        "W": cls[:, 0],
        "E": cls[:, -1],
    }
    return EdgeProfiles(
        n=profile(runs["N"]),
        s=profile(runs["S"]),
        w=profile(runs["W"]),
        e=profile(runs["E"]),
    ), runs


@dataclass
class JoinCandidate:
    template: str
    icon: int
    edges: EdgeProfiles
    material: Material
    runs: dict[str, np.ndarray] = field(default_factory=dict)

    def score(
        self,
        neighbours: list[tuple[Material, str]],
        diagonal_weight: float = 0.5,
    ) -> float:
        """Total seam disagreement against the neighbour material field.

        Orthogonal sides weigh 1.0; diagonals (mattering for corner-shaped
        art) weigh ``diagonal_weight``, approximated by the mean of the two
        adjacent orthogonal edges.
        """
        total = 0.0
        for mat, side in neighbours:
            if side in ORTHOGONAL:
                demand = edge_demand(self.material, mat)
                total += self.edges.side(side).disagreement(demand)
            elif side in DIAGONAL and diagonal_weight:
                a, b = DIAGONAL_ADJ[side]
                demand = edge_demand(self.material, mat)
                approx = 0.5 * (
                    self.edges.side(a).disagreement(demand)
                    + self.edges.side(b).disagreement(demand)
                )
                total += diagonal_weight * approx
        return total

    @property
    def solidity(self) -> float:
        e = self.edges
        return e.n.wants_opaque + e.e.wants_opaque + e.s.wants_opaque + e.w.wants_opaque

    def is_base(self) -> bool:
        """A drawn base tile, not an overlay sliver (Rust ``select`` filter)."""
        e = self.edges
        ns = e.n.wants_opaque + e.s.wants_opaque
        ew = e.e.wants_opaque + e.w.wants_opaque
        sides = [e.n.wants_opaque, e.e.wants_opaque, e.s.wants_opaque, e.w.wants_opaque]
        return ns >= 1.0 or ew >= 1.0 or sum(s > 0.2 for s in sides) >= 3


# Diagonal -> the two orthogonal edges touching it.
DIAGONAL_ADJ = {"NE": ("N", "E"), "SE": ("E", "S"), "SW": ("S", "W"), "NW": ("W", "N")}


# Template family -> material. Longest matching prefix wins. Names follow
# `TemplateTypeClass` in `original/TIBERIANDAWN/CDATA.CPP` (RA zero-pads).
#
# Ground truth for the remaster's art (verified against
# `RUST/cnc-asset/src/tileset_pack.rs` semantic slots + DOS SHP frame counts):
# base ground is the `P` patch family (`P01` grass, `P03` dirt, `P04` sand,
# `P07` rock, `P08` rough, `P13` mud, `P14` concrete); `W` is open water;
# `SH` is shoreline; `CLEAR1` is a translucent clear-overlay the engine draws
# UNDER a base; `RV`/`FALLS`/`FORD` are river features with their own banks;
# `S` are grass/rock slopes; `D` are roads; `B`/`BR` translucent boulders.
# Overlay-ish families (CLEAR1, B*, S* slivers) drop out via `is_base`.
MATERIAL_FOR_PREFIX: tuple[tuple[str, Material], ...] = (
    ("SH", Material.SHORE),
    ("FORD", Material.SHORE),
    ("RV", Material.WATER),
    ("FALLS", Material.WATER),
    ("W", Material.WATER),
    ("CLEAR", Material.LAND),
    ("D", Material.LAND),
    ("S", Material.LAND),
    ("P", Material.LAND),
    ("BR", Material.LAND),
    ("B", Material.LAND),
    ("BRIDGE", Material.LAND),
)


def material_for(template: str) -> Material | None:
    hits = [(p, m) for p, m in MATERIAL_FOR_PREFIX if template.upper().startswith(p)]
    if not hits:
        return None
    return max(hits, key=lambda pm: len(pm[0]))[1]


@dataclass
class JoinTable:
    by_material: dict[Material, list[JoinCandidate]] = field(default_factory=dict)

    def push(self, c: JoinCandidate) -> None:
        self.by_material.setdefault(c.material, []).append(c)

    def candidates(self, material: Material) -> list[JoinCandidate]:
        return self.by_material.get(material, [])

    def bases(self, material: Material) -> list[JoinCandidate]:
        bases = [c for c in self.candidates(material) if c.is_base()]
        return bases if bases else self.candidates(material)

    def __len__(self) -> int:
        return sum(len(v) for v in self.by_material.values())


def build_table_from_icons(
    measured: dict[tuple[str, int], EdgeProfiles],
    materials: dict[tuple[str, int], Material],
    runs: dict[tuple[str, int], dict[str, np.ndarray]] | None = None,
) -> JoinTable:
    table = JoinTable()
    for key, edges in measured.items():
        material = materials.get(key)
        if material is None:
            continue
        template, icon = key
        table.push(JoinCandidate(
            template=template,
            icon=icon,
            edges=edges,
            material=material,
            runs=(runs or {}).get(key, {}),
        ))
    return table


def build_table(measured: dict[tuple[str, int], EdgeProfiles]) -> JoinTable:
    table = JoinTable()
    for (template, icon), edges in measured.items():
        material = material_for(template)
        if material is None:
            continue
        table.push(JoinCandidate(template=template, icon=icon, edges=edges, material=material))
    return table


def normalize_template(name: str) -> str:
    """RA zero-pads two-digit families (`SH1` -> `SH01`); map data uses the
    unpadded spelling, so compare on the padded form."""
    if len(name) > 2:
        prefix, digits = name[:-2], name[-2:]
        if digits.isdigit() and not digits.startswith("0"):
            return f"{prefix}0{digits}"
    return name


def select(
    table: JoinTable,
    material: Material,
    neighbours: list[tuple[Material, str]],
    allowed_templates: list[str] | None = None,
    diagonal_weight: float = 0.5,
    rng: np.random.Generator | None = None,
) -> JoinCandidate | None:
    """Best base tile of ``material`` against the neighbour material field.

    Ties (within an epsilon) break toward the most solid tile; among
    near-equal solid candidates a seeded rng varies the pick so shorelines
    don't repeat one icon monotonously.
    """
    candidates = table.bases(material)
    if allowed_templates is not None:
        allowed = {normalize_template(t).upper() for t in allowed_templates}
        candidates = [c for c in candidates if normalize_template(c.template).upper() in allowed]
    if not candidates:
        return None
    scored = sorted(
        candidates,
        key=lambda c: (
            c.score(neighbours, diagonal_weight),
            -c.solidity,
            c.template,
            c.icon,
        ),
    )
    best_score = scored[0].score(neighbours, diagonal_weight)
    pool = [c for c in scored if abs(c.score(neighbours, diagonal_weight) - best_score) < 1e-6]
    if rng is not None and len(pool) > 1:
        return pool[int(rng.integers(len(pool)))]
    return scored[0]
