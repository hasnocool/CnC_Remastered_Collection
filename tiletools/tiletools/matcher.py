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
from .terrain import (
    TerrainField,
    apply_shore,
    fill_specks,
    label_pools,
    relabel_stale_shore,
)


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


class Patch:
    """One whole coast template (the DOS art's rounded-edge unit).

    Geometry is measured from the art's index-class arrays, not hand-tuned:
    a patch cell is land-look / water-look / mixed by its edge and coverage
    fractions.
    """

    __slots__ = ("template", "w", "h", "kind")

    def __init__(self, template: str, w: int, h: int, kind: str):
        self.template = template
        self.w = w
        self.h = h
        self.kind = kind

    def cell_frac(self, classes: dict[str, np.ndarray], icon: int) -> tuple[float, float]:
        """(land fraction, water fraction) of one patch cell's area."""
        cls = classes[self.template][icon]
        return float((cls == 2).mean()), float((cls == 1).mean())

    def cells(self):
        for cy in range(self.h):
            for cx in range(self.w):
                yield cx, cy, cy * self.w + cx


# Chain kinds:
#   ns-W / ns-E : straight N-S coast, land on the W/E patch edge (SH34/SH35);
#                 chains vertically at 3-cell steps.
#   ew-S        : straight E-W coast, land on the S edge (SH12/SH13);
#                 chains horizontally at 3-cell steps.
#   notch-NW/NE : water pocket in a land corner (SH32/SH33).
#   island      : land blob in water (SH11).
#   diag        : diagonal coast arc (SH10, 2x2).
COAST_PATCHES = (
    Patch("SH34", 3, 3, "ns-W"),
    Patch("SH35", 3, 3, "ns-E"),
    Patch("SH12", 3, 3, "ew-S"),
    Patch("SH13", 3, 3, "ew-S"),
    Patch("SH32", 3, 3, "notch-NW"),
    Patch("SH33", 3, 3, "notch-NE"),
    Patch("SH11", 3, 3, "island"),
    Patch("SH10", 2, 2, "diag"),
)

# Open-water variety: the DOS set ships three 2x2 all-water patches with
# distinct ripple art. Tiling the ocean interior with them (instead of one
# W1 everywhere) breaks the monotony while every shared edge stays water.
WATER_PATCHES = (
    Patch("W2", 2, 2, "water"),
    Patch("SH17", 2, 2, "water"),
    Patch("SH18", 2, 2, "water"),
)


def place_turn_pieces(
    field: TerrainField,
    rng: np.random.Generator,
    fixed: dict[tuple[int, int], tuple[str, int]],
) -> None:
    """Coast turns get the pocket/arc piece their land mask demands.

    A turn cell (land on two adjacent sides) is the inside corner of a
    coast bend: the DOS set ships a pocket piece per two-side geometry —
    SH32:0 (land E+S), SH33:2 (land W+S), SH35:6 (land N+E), SH34:2
    (land N+W, the pure diagonal arc).
    """
    size = field.size
    turn_piece = {
        0b0110: ("SH32", 0),  # land E+S
        0b1100: ("SH33", 2),  # land W+S
        0b0011: ("SH35", 6),  # land N+E
        0b1001: ("SH34", 2),  # land N+W
    }
    for y in range(size):
        for x in range(size):
            if field.materials[y, x] is not Material.SHORE or (x, y) in fixed:
                continue
            piece = turn_piece.get(land_mask(field, x, y))
            if piece is not None:
                fixed[(x, y)] = piece


def place_water_patches(
    field: TerrainField,
    rng: np.random.Generator,
    fixed: dict[tuple[int, int], tuple[str, int]],
) -> None:
    """Tile plain-water areas with 2x2 water patches on the global lattice."""
    size = field.size
    for by in range(0, size - 1, 2):
        for bx in range(0, size - 1, 2):
            cells = [(bx + cx, by + cy) for cy in range(2) for cx in range(2)]
            if any(x >= size or y >= size for x, y in cells):
                continue
            if any((x, y) in fixed for x, y in cells):
                continue
            if not all(field.materials[y, x] is Material.WATER for x, y in cells):
                continue
            patch = WATER_PATCHES[int(rng.integers(len(WATER_PATCHES)))]
            for cx, cy, icon in patch.cells():
                fixed[(bx + cx, by + cy)] = (patch.template, icon)


def find_coast_strips(field: TerrainField) -> list[tuple[str, str, int, int, int]]:
    """Straight coast strips: (orientation, land_side, fixed_coord, start, end).

    A strip = maximal run of coast cells in one column/row with land on the
    same side throughout and open water on the far side. Patches chain along
    these.
    """
    size = field.size
    wet = (Material.WATER, Material.SHORE)
    strips: list[tuple[str, str, int, int, int]] = []

    def land(x, y):
        return 0 <= x < size and 0 <= y < size and field.materials[y, x] is Material.LAND

    def wetish(x, y):
        return 0 <= x < size and 0 <= y < size and field.materials[y, x] in wet

    # N-S strips: coast cell (x,y) = SHORE, land at W, water at E (and beyond).
    for x in range(size):
        y = 0
        while y < size:
            if (
                field.materials[y, x] is Material.SHORE
                and land(x - 1, y)
                and wetish(x + 1, y)
            ):
                start = y
                while (
                    y < size
                    and field.materials[y, x] is Material.SHORE
                    and land(x - 1, y)
                    and wetish(x + 1, y)
                ):
                    y += 1
                if y - start >= 2:
                    strips.append(("NS", "W", x, start, y))
            else:
                y += 1
    # Mirrored: land at E, water at W.
    for x in range(size):
        y = 0
        while y < size:
            if (
                field.materials[y, x] is Material.SHORE
                and land(x + 1, y)
                and wetish(x - 1, y)
            ):
                start = y
                while (
                    y < size
                    and field.materials[y, x] is Material.SHORE
                    and land(x + 1, y)
                    and wetish(x - 1, y)
                ):
                    y += 1
                if y - start >= 2:
                    strips.append(("NS", "E", x, start, y))
            else:
                y += 1
    # E-W strips: land at S (water N) and land at N (water S).
    for y in range(size):
        x = 0
        while x < size:
            if (
                field.materials[y, x] is Material.SHORE
                and land(x, y - 1) is False
                and land(x, y + 1)
            ):
                start = x
                while (
                    x < size
                    and field.materials[y, x] is Material.SHORE
                    and not land(x, y - 1)
                    and land(x, y + 1)
                ):
                    x += 1
                if x - start >= 2:
                    strips.append(("EW", "S", y, start, x))
            else:
                x += 1
    for y in range(size):
        x = 0
        while x < size:
            if (
                field.materials[y, x] is Material.SHORE
                and land(x, y + 1) is False
                and land(x, y - 1)
            ):
                start = x
                while (
                    x < size
                    and field.materials[y, x] is Material.SHORE
                    and not land(x, y + 1)
                    and land(x, y - 1)
                ):
                    x += 1
                if x - start >= 2:
                    strips.append(("EW", "N", y, start, x))
            else:
                x += 1
    return strips


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


def place_coast_patches(
    field: TerrainField,
    classes: dict[str, np.ndarray],
    seed: int = 0,
) -> dict[tuple[int, int], tuple[str, int]]:
    """Chain whole coast templates along straight strips.

    Patches anchor on the global 3-cell lattice (phase-locked so arcs from
    adjacent strips meet in phase), may hang off the map edge (off-map cells
    are simply not rendered), and validate per cell:
    - full-land art needs the field cell to be LAND, full-water art needs
      water/shore, mixed arc cells accept either (the arc defines the local
      boundary);
    - the patch's OUTER edges must agree with the field across them.
    """
    size = field.size
    rng = np.random.default_rng(seed ^ 0x0C0A5)
    fixed: dict[tuple[int, int], tuple[str, int]] = {}

    def patch_template_for(strip, slot):
        orientation, land_side = strip[0], strip[1]
        if orientation == "NS":
            return "SH34" if land_side == "W" else "SH35"
        # Alternate the two E-W sand shelves along the coast for variety.
        return ("SH12", "SH13")[slot % 2]

    def validate(template: str, patch: Patch, px: int, py: int) -> bool:
        for cx, cy, icon in patch.cells():
            x, y = px + cx, py + cy
            if not (0 <= x < size and 0 <= y < size):
                continue  # clipped off-map
            if (x, y) in fixed:
                return False
            cls = classes[template][icon]
            f_land, f_water = float((cls == 2).mean()), float((cls == 1).mean())
            material = field.materials[y, x]
            if f_land > 0.9:
                if material is not Material.LAND:
                    return False
            elif f_water > 0.9:
                if material not in (Material.WATER, Material.SHORE):
                    return False
            # Outer edges: the field across them must agree with the run.
            outer = []
            if cy == 0:
                outer.append("N")
            if cy == patch.h - 1:
                outer.append("S")
            if cx == 0:
                outer.append("W")
            if cx == patch.w - 1:
                outer.append("E")
            for side in outer:
                run = {
                    "N": cls[0], "S": cls[-1], "W": cls[:, 0], "E": cls[:, -1],
                }[side]
                water, land = float((run == 1).mean()), float((run == 2).mean())
                dx, dy = {"N": (0, -1), "E": (1, 0), "S": (0, 1), "W": (-1, 0)}[side]
                nx, ny = x + dx, y + dy
                if not (0 <= nx < size and 0 <= ny < size):
                    continue
                nmat = field.materials[ny, nx]
                if water > 0.75 and nmat not in (Material.WATER, Material.SHORE):
                    return False
                if land > 0.75 and nmat is not Material.LAND:
                    return False
        return True

    for orientation, land_side, coord, start, end in sorted(
        find_coast_strips(field), key=lambda s: s[4] - s[3], reverse=True
    ):
        # Variety comes from template alternation and the island/water
        # patches, not from gaps: a gap in the chain renders as an arc that
        # stops mid-coast.
        slot = 0
        # Global lattice anchors inside the strip; the tail (if any) goes to
        # the per-cell matcher, which run-matches against the placed arcs.
        stride = 3
        for k in range((-start) % stride + start, end - stride + 1, stride):
            slot += 1
            template = patch_template_for((orientation, land_side), slot)
            patch = next(p for p in COAST_PATCHES if p.template == template)
            if orientation == "NS":
                px = coord - (patch.w - 1) if land_side == "W" else coord
                py = k
            elif land_side == "S":
                # Water row (patch top) on the coast row, land below.
                py = coord
                px = k
            else:
                # Land N: patch hangs north; water row on the coast row.
                py = coord - (patch.h - 1)
                px = k
            if not validate(template, patch, px, py):
                continue
            for cx, cy, icon in patch.cells():
                x, y = px + cx, py + cy
                if 0 <= x < size and 0 <= y < size:
                    fixed[(x, y)] = (template, icon)
    place_turn_pieces(field, rng, fixed)
    place_water_patches(field, rng, fixed)
    return fixed


def mask_fitness(table: JoinTable) -> dict[int, float]:
    """Best achievable seam penalty for each 4-bit land mask.

    The demand model for a shore cell with land mask ``m``: sides with the
    bit set demand opaque (land continues), sides without demand water.
    The minimum over all shore candidates = how well the shipped art can
    join that local geometry. Masks no piece fits (> ~0.6) mark coast
    shapes the field should not contain.
    """
    fitness = {}
    shore_candidates = table.bases(Material.SHORE) + table.bases(Material.WATER)
    if not shore_candidates:
        return fitness
    for mask in range(16):
        demands = {
            "N": "land" if mask & 1 else "water",
            "E": "land" if mask & 2 else "water",
            "S": "land" if mask & 4 else "water",
            "W": "land" if mask & 8 else "water",
        }
        fitness[mask] = min(
            sum(
                c.edges.__getattribute__(s.lower()).disagreement(demands[s])
                for s in "NESW"
            )
            for c in shore_candidates
        )
    return fitness


def repair_field(
    field: TerrainField,
    table: JoinTable,
    max_passes: int = 3,
    threshold: float = 0.6,
) -> int:
    """Reshape coast geometry the shipped art cannot join.

    Wet specks fully enclosed by land and elongated gully slivers render as
    detached sand wedges; close both and rerun the shore/pool labelling.
    Also close compact wet regions whose shore cells have a land mask no
    shore-family piece can dress (they would fall back to plain W1 water
    squares floating in grass).
    """
    lut = {}
    for cands in table.by_material.values():
        for c in cands:
            lut[(c.template, c.icon)] = c
    shore_families = {"SH10", "SH12", "SH13", "SH34", "SH35", "SH32", "SH33"}
    size = field.size
    DIR = {"N": (0, -1), "E": (1, 0), "S": (0, 1), "W": (-1, 0)}
    for _ in range(max_passes):
        undressable = []
        for y in range(size):
            for x in range(size):
                if field.materials[y, x] is not Material.SHORE:
                    continue
                # a shore cell must have arc art with water on its water
                # sides; check any candidate in the shore families fits the
                # field demands on all four cardinals
                fits = False
                for t in shore_families:
                    for cand in lut.values():
                        if cand.template.upper() != t:
                            continue
                        ok = True
                        for side, (dx, dy) in DIR.items():
                            nx, ny = x + dx, y + dy
                            if not (0 <= nx < size and 0 <= ny < size):
                                continue
                            nmat = field.materials[ny, nx]
                            demand = edge_demand(field.materials[y, x], nmat)
                            if getattr(cand.edges, side.lower()).disagreement(demand) > 0.45:
                                ok = False
                                break
                        if ok:
                            fits = True
                            break
                    if fits:
                        break
                if not fits:
                    undressable.append((x, y))
        if not undressable:
            break
        for x, y in undressable:
            field.materials[y, x] = Material.LAND
        apply_shore(field)
        label_pools(field)
    closed = fill_specks(field) + close_sliver_regions(field)
    relabel_stale_shore(field)
    return closed


def close_sliver_regions(field: TerrainField, max_area: int = 7, min_fill: float = 0.6) -> int:
    """Close elongated thin wet regions (width-1 gullies).

    Compact ponds (bbox fill >= ``min_fill``) keep their pocket art; gully
    chains render as square wedges because the DOS set ships no diagonal
    channel pieces (rivers RV* are the future extension), so slivers close.
    """
    size = field.size
    closed = 0
    seen: set[tuple[int, int]] = set()
    for sy in range(size):
        for sx in range(size):
            mat = field.materials[sy, sx]
            if mat not in (Material.WATER, Material.SHORE) or (sx, sy) in seen:
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
                        and field.materials[ny, nx] in (Material.WATER, Material.SHORE)
                        and (nx, ny) not in seen
                    ):
                        seen.add((nx, ny))
                        stack.append((nx, ny))
            if len(region) > max_area:
                continue  # big water (lakes, ocean) is never closed
            xs = [p[0] for p in region]
            ys = [p[1] for p in region]
            fill = len(region) / max(1, (max(xs) - min(xs) + 1) * (max(ys) - min(ys) + 1))
            if fill < min_fill:
                for x, y in region:
                    field.materials[y, x] = Material.LAND
                closed += len(region)
    if closed:
        apply_shore(field)
        label_pools(field)
    return closed


def _profile_mismatch(a: np.ndarray | None, b: np.ndarray | None) -> float:
    """Mean |sand-curve difference| between facing edge profiles.

    Both profiles enumerate in the same screen direction, so they compare
    directly; the arc phase (where the boundary sits along the shared
    edge) is what this scores.
    """
    if a is None or b is None or a.shape != b.shape:
        return 0.0
    return float(np.abs(a - b).mean())


def _run_mismatch(a: np.ndarray, b: np.ndarray) -> float:
    """Fraction of differing pixels between facing edge runs.

    Both runs enumerate in the same screen direction (x for N/S seams,
    y for W/E seams), so they compare directly — no mirroring.
    """
    if a is None or b is None or len(a) != len(b):
        return 0.0
    # Sand (3) and grass (2) are both opaque land: a beach arc ending
    # against grass is the intended coast look, not a seam defect. Only
    # material-level disagreement (water vs land, arc shape) counts.
    return float((a % 3 != b % 3).mean())


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
    icon_classes: dict[str, np.ndarray] | None = None,
) -> np.ndarray:
    """Greedy scanline placement with exact N/W run alignment.

    Whole coast patches (`place_coast_patches`) are placed first and act as
    fixed anchors; the per-cell matcher fills the rest around them with
    exact run alignment against every already-placed neighbour.
    """
    size = field.size
    rng = np.random.default_rng(seed ^ 0x5EED)
    placed: np.ndarray = np.empty((size, size), dtype=object)

    lookup: dict[tuple[str, int], JoinCandidate] = {}
    for cands in table.by_material.values():
        for c in cands:
            lookup[(c.template, c.icon)] = c
    repair_field(field, table)
    fixed = (
        place_coast_patches(field, icon_classes, seed=seed)
        if icon_classes is not None
        else {}
    )
    for (x, y), key in fixed.items():
        placed[y, x] = lookup.get(key)

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
            if placed[y, x] is not None:
                continue  # fixed by a coast patch
            material = field.materials[y, x]
            allowed = allowed_for(material, x, y)
            candidates = table.bases(material)
            if (
                material is Material.SHORE
            ):
                # 1-cell ponds (land on all four sides) were closed in
                # repair_field; anything still standing here gets the
                # shipped-map convention (learned from 18 campaign maps):
                # straight coast water cells = plain W1 water with W2
                # ripple variety (~30%); arcs/ponds live only on the
                # special geometry handled by the patch and turn layers.
                # Keep the family restriction (pool cells stay with
                # their pockets).
                candidates = table.bases(Material.WATER)
                if allowed is not None:
                    want = {normalize_template(t).upper() for t in allowed}
                    filtered = [
                        c
                        for c in candidates
                        if normalize_template(c.template).upper() in want
                    ]
                    if not filtered:
                        # A shore cell ringed by land on all four sides is a
                        # 1-cell pond: no arc art fits, close it to grass.
                        if land_mask(field, x, y) == 0b1111:
                            field.materials[y, x] = Material.LAND
                            apply_shore(field)
                            label_pools(field)
                            placed[y, x] = CellPick("CLEAR1", 0)
                            continue
                        # otherwise: plain water is the safest fallback
                        want = {"W1", "W2"}
                        filtered = [
                            c
                            for c in candidates
                            if normalize_template(c.template).upper() in want
                        ]
                    candidates = filtered
            elif (
                allowed is not None
                and field.pool is not None
                and field.pool[y, x]
                and material is Material.WATER
            ):
                # Pond cells whose land is only diagonal (not SHORE by the
                # 4-neighbour rule) still get the pond pocket pieces.
                want = {normalize_template(t).upper() for t in pool_families or []}
                candidates = [
                    c
                    for c in table.bases(Material.WATER)
                    if normalize_template(c.template).upper() in want
                ]
            elif allowed is not None:
                want = {normalize_template(t).upper() for t in allowed}
                candidates = [c for c in candidates if normalize_template(c.template).upper() in want]
            if not candidates:
                raise ValueError(f"no candidates for {material}")

            def cand_at(yy, xx):
                if not (0 <= yy < size and 0 <= xx < size) or placed[yy, xx] is None:
                    return None
                p = placed[yy, xx]
                if not hasattr(p, "runs"):
                    p = lookup.get((p.template, p.icon))
                return p

            north_run = (
                cand_at(y - 1, x).runs.get("S") if cand_at(y - 1, x) is not None else None
            )
            west_run = (
                cand_at(y, x - 1).runs.get("E") if cand_at(y, x - 1) is not None else None
            )
            north_candidate_profile = (
                cand_at(y - 1, x).profiles if cand_at(y - 1, x) is not None else {}
            )
            west_candidate_profile = (
                cand_at(y, x - 1).profiles if cand_at(y, x - 1) is not None else {}
            )

            neighbours8 = field.neighbours(x, y)

            def cost(c):
                total = 0.0
                # Placed neighbours: align the sand contour curves (weight
                # 3) plus the pixel run (weight 1) — the curve is what the
                # eye reads as a continuing arc.
                if north_run is not None:
                    north_c = c.profiles.get("N")
                    total += 3.0 * _profile_mismatch(
                        north_c, north_candidate_profile.get("S")
                    )
                    total += 1.0 * _run_mismatch(c.runs.get("N"), north_run)
                if west_run is not None:
                    total += 3.0 * _profile_mismatch(
                        c.profiles.get("W"), west_candidate_profile.get("E")
                    )
                    total += 1.0 * _run_mismatch(c.runs.get("W"), west_run)
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


def repair_contours(
    field: TerrainField,
    table: JoinTable,
    placed: np.ndarray,
    rng: np.random.Generator | None = None,
    passes: int = 3,
) -> np.ndarray:
    """Re-pick curve-carrying cells whose arc phase disagrees with a placed
    neighbour.

    The scanline only scores the exact runs of the placed N/W neighbours;
    a later-placed patch (pond pocket, shelf band) can still face an
    already-picked cell's mixed edge with a different curve phase. This
    pass scores every candidate against ALL placed neighbours' exact runs
    and re-picks any cell on a mismatched mixed-vs-mixed seam. Patch
    anchors (cells whose material demand a full arc — SH14+rock etc.) are
    re-picked too: candidates keep the same material, so the field stays
    consistent.
    """
    lut = {}
    for cands in table.by_material.values():
        for c in cands:
            lut[(c.template, c.icon)] = c
    size = field.size
    OPP = {"N": "S", "E": "W", "S": "N", "W": "E"}
    DIR = {"N": (0, -1), "E": (1, 0), "S": (0, 1), "W": (-1, 0)}

    def mixed(run):
        if run is None:
            return False
        f = float((run == 1).mean())
        return 0.02 < f < 0.98

    for _ in range(passes):
        changed = 0
        for y in range(size):
            for x in range(size):
                if placed[y, x] is None:
                    continue
                pc = placed[y, x]
                if not hasattr(pc, "runs"):
                    here = lut.get((pc.template, pc.icon))
                else:
                    here = pc
                if here is None:
                    continue
                bad_sides = []
                nb_runs = {}
                for side, (dx, dy) in DIR.items():
                    nx, ny = x + dx, y + dy
                    if not (0 <= nx < size and 0 <= ny < size):
                        continue
                    if placed[ny, nx] is None:
                        continue
                    oc = placed[ny, nx]
                    other = oc if hasattr(oc, "runs") else lut.get((oc.template, oc.icon))
                    if other is None:
                        continue
                    r_here, r_there = here.runs.get(side), other.runs.get(OPP[side])
                    if not (mixed(r_here) and mixed(r_there)):
                        continue
                    nb_runs[side] = r_there
                    if (r_here != r_there).any():
                        bad_sides.append(side)
                if not bad_sides:
                    continue
                material = field.materials[y, x]
                candidates = table.bases(material)
                if not candidates:
                    continue
                def cost(c):
                    total = 0.0
                    for side, r_there in nb_runs.items():
                        r_cand = c.runs.get(side)
                        if r_cand is None:
                            total += 0.5
                            continue
                        total += float((r_cand != r_there).mean())
                    # keep the material demands of the free sides
                    for m2, s2 in field_neighbours(field, x, y):
                        if s2 in nb_runs or m2 is None or s2 not in ("N", "E", "S", "W"):
                            continue
                        total += getattr(c.edges, s2.lower()).disagreement(
                            edge_demand(material, m2)
                        )
                    return total
                current_cost = cost(here)
                scored = sorted(candidates, key=lambda c: (cost(c), -c.solidity, c.template, c.icon))
                if cost(scored[0]) + 1e-9 < current_cost:
                    placed[y, x] = CellPick(scored[0].template, scored[0].icon)
                    changed += 1
        if changed == 0:
            break
    return placed


def field_neighbours(field: TerrainField, x: int, y: int):
    return field.neighbours(x, y)


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
    icon_classes: dict[str, np.ndarray] | None = None,
) -> np.ndarray:
    if strategy is Strategy.MATCHED:
        placed = scanline_assign(
            field,
            table,
            seed=seed,
            families=families,
            diagonal_weight=diagonal_weight,
            pool_families=pool_families,
            icon_classes=icon_classes,
        )
        return repair_contours(field, table, placed, rng=np.random.default_rng(seed ^ 0xC0DE))
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
