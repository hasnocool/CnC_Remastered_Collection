#!/usr/bin/env python3
"""Deterministic 32x32 RTS tileset generator with connected masks, animation and contrast audits.

Requires: Pillow >= 10.0

Mask convention: bit 0=N, bit 1=E, bit 2=S, bit 3=W. A set bit means that
connection continues in that direction. Terrain transition masks instead use a
set bit to mean the neighboring cell is the same terrain; absent bits receive
an authored transition edge. This is a procedural, C&C-inspired placeholder
pipeline, not a reproduction of proprietary original game artwork.
"""
from __future__ import annotations
import argparse, hashlib, json, math, os, random, shutil, sys, tempfile
from pathlib import Path
from typing import Any
from PIL import Image, ImageDraw

VERSION = "5.5.0"
TILE = 32
DIRECTIONS = ((0, -1), (1, 0), (0, 1), (-1, 0))
DIRECTION_NAMES = ("N", "E", "S", "W")
PALETTE = {
    "grass": (82, 126, 57), "grass_light": (112, 153, 72), "grass_dark": (52, 88, 43),
    "dirt": (139, 99, 59), "dirt_light": (171, 128, 78), "dirt_dark": (91, 63, 43),
    "rock": (113, 113, 107), "rock_light": (157, 157, 147), "rock_dark": (67, 69, 67),
    "water": (39, 91, 139), "water_light": (70, 135, 177), "water_dark": (23, 57, 94),
    "sand": (194, 174, 112), "sand_light": (220, 203, 149), "sand_dark": (145, 123, 76),
    "road": (83, 79, 73), "road_light": (119, 113, 103), "road_dark": (48, 47, 45),
    "gdi": (211, 184, 88), "gdi_dark": (126, 105, 49), "nod": (210, 90, 80), "nod_dark": (99, 35, 36),
    "metal": (160, 171, 173), "metal_dark": (70, 81, 87), "track": (43, 45, 44),
    "infantry": (193, 185, 142), "skin": (199, 150, 111), "muzzle": (255, 238, 132),
    "white": (255, 255, 255), "black": (0, 0, 0), "smoke": (107, 107, 103),
    "fire": (255, 104, 26), "fire_light": (255, 207, 61), "tiberium": (64, 218, 114),
}

# Semantic pairs used for contrast auditing. Ratios use WCAG relative luminance.
CONTRAST_PAIRS = [
    ("unit_gdi_body", "gdi", "grass_dark"), ("unit_gdi_body_vs_water", "gdi", "water_dark"),
    ("unit_nod_body", "nod", "dirt_dark"), ("unit_nod_body_vs_grass", "nod", "grass_dark"),
    ("road_surface_vs_grass", "road", "grass"), ("road_marking_vs_road", "white", "road"),
    ("shore_sand_vs_water", "sand", "water"), ("shore_foam_vs_water", "white", "water"),
    ("metal_detail_vs_body", "metal", "metal_dark"), ("infantry_vs_grass", "infantry", "grass_dark"),
    ("muzzle_flash_vs_unit", "muzzle", "metal_dark"), ("tiberium_vs_dirt", "tiberium", "dirt_dark"),
]

def stable_seed(text: str) -> int:
    return int.from_bytes(hashlib.sha256(text.encode("utf-8")).digest()[:8], "big")

def luminance(rgb: tuple[int, int, int]) -> float:
    def channel(c: int) -> float:
        x = c / 255.0
        return x / 12.92 if x <= 0.04045 else ((x + 0.055) / 1.055) ** 2.4
    r, g, b = (channel(c) for c in rgb)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b

def contrast_ratio(a: tuple[int, int, int], b: tuple[int, int, int]) -> float:
    hi, lo = sorted((luminance(a), luminance(b)), reverse=True)
    return (hi + 0.05) / (lo + 0.05)

def color_hex(c: tuple[int, int, int]) -> str:
    return "#%02X%02X%02X" % c

def check_contrast_pair(name: str, foreground: tuple[int, int, int], background: tuple[int, int, int], threshold: float = 1.5) -> dict[str, Any]:
    """Audit any foreground/background pair, including white-on-white and identical colors."""
    ratio = contrast_ratio(foreground, background)
    return {"name": name, "foreground_hex": color_hex(foreground), "background_hex": color_hex(background),
            "ratio": round(ratio, 3), "threshold": threshold, "pass": ratio >= threshold,
            "suggestion": None if ratio >= threshold else "Use a more distinct foreground/background color or add a contrasting outline."}

def mask_dirs(mask: int) -> list[int]:
    return [i for i in range(4) if mask & (1 << i)]

def new_tile(color: tuple[int, int, int]) -> Image.Image:
    im = Image.new("RGBA", (TILE, TILE), (*color, 255))
    return im

def noise_texture(im: Image.Image, base: str, seed_text: str, amount: int = 9, count: int = 35) -> None:
    rng = random.Random(stable_seed(seed_text)); d = ImageDraw.Draw(im)
    c = PALETTE[base]
    for _ in range(count):
        delta = rng.randint(-amount, amount)
        col = tuple(max(0, min(255, v + delta)) for v in c) + (255,)
        x, y = rng.randrange(TILE), rng.randrange(TILE)
        if rng.random() < .75: d.point((x, y), fill=col)
        else: d.rectangle((x, y, min(31, x + rng.randrange(1, 3)), min(31, y + rng.randrange(1, 3))), fill=col)

def terrain_base(kind: str, seed: str) -> Image.Image:
    """Richer 32x32 pixel-art material tile with structured, deterministic detail."""
    base = {"grass": "grass", "dirt": "dirt", "rock": "rock", "water": "water", "sand": "sand"}[kind]
    im = new_tile(PALETTE[base])
    noise_texture(im, base, seed, 7 if kind == "water" else 9, 70 if kind in ("grass", "dirt", "sand") else 46)
    d = ImageDraw.Draw(im)
    rng = random.Random(stable_seed(seed + ":material-details"))
    if kind == "grass":
        # Small clumps with light tips and dark root pixels; avoid evenly spaced noise.
        for _ in range(6):
            x, y = rng.randrange(2, 30), rng.randrange(2, 30)
            d.point((x, y), fill=(*PALETTE["grass_dark"], 255))
            d.point((min(31, x+1), max(0, y-1)), fill=(*PALETTE["grass_light"], 255))
            if rng.random() < .55:
                d.line((x, y, min(31, x+1), max(0, y-2)), fill=(*PALETTE["grass_light"], 255))
    elif kind == "dirt":
        for _ in range(9):
            x, y = rng.randrange(2, 30), rng.randrange(2, 30)
            col = PALETTE["dirt_light"] if rng.random() < .55 else PALETTE["dirt_dark"]
            d.line((x, y, min(31, x+rng.randrange(1, 3)), y), fill=(*col, 255))
            if rng.random() < .3: d.point((x, min(31,y+1)), fill=(*PALETTE["dirt_dark"],255))
    elif kind == "rock":
        # Faceted strata and small cracks, designed to read at native scale.
        for _ in range(4):
            x, y = rng.randrange(3, 25), rng.randrange(3, 25)
            w, h = rng.randrange(3, 7), rng.randrange(2, 5)
            d.polygon(((x,y),(min(31,x+w),y+1),(min(31,x+w-1),min(31,y+h)),(x+1,min(31,y+h-1))), fill=(*PALETTE["rock_dark" if rng.random()<.45 else "rock_light"],255))
        # Facet speckle: light tips and dark roots, matching the dirt/sand
        # noise families. Fixed-coordinate hatching strokes were removed -
        # they repeated identically on every tile and read as a hatch lattice.
        for _ in range(10):
            x, y = rng.randrange(1, 31), rng.randrange(1, 31)
            d.point((x, y), fill=(*PALETTE["rock_light" if rng.random() < .5 else "rock_dark"], 255))
    elif kind == "water":
        # Broken, multi-length glints rather than repeated uniform dashes.
        for _ in range(8):
            x, y = rng.randrange(1, 28), rng.randrange(2, 30)
            length = rng.randrange(2, 7)
            col = PALETTE["water_light"] if rng.random() < .75 else PALETTE["water_dark"]
            d.line((x, y, min(31, x+length), y), fill=(*col,255))
            if rng.random() < .35: d.point((min(31,x+length),y+1), fill=(*PALETTE["water_light"],255))
    elif kind == "sand":
        for _ in range(10):
            x, y = rng.randrange(1, 31), rng.randrange(1, 31)
            col = PALETTE["sand_dark"] if rng.random() < .5 else PALETTE["sand_light"]
            d.point((x,y), fill=(*col,255))
            if rng.random() < .3: d.point((min(31,x+1),y), fill=(*col,255))
    # A subtle one-pixel top-left highlight and bottom-right shade helps surfaces read as pixel art.
    # Rock carries no per-tile corner highlight: a light/dark dot at (1,1) and
    # (30,30) repeats at every tile corner and reads as a mismatched lattice
    # once tiles butt against each other. Dirt/sand keep their subtle accents.
    if kind in ("dirt", "sand"):
        d.point((1,1), fill=(*PALETTE[base + "_light"],255))
        d.point((30,30), fill=(*PALETTE[base + "_dark"],255))
    return im

# Eight-neighbour bit order: N, NE, E, SE, S, SW, W, NW.
# A diagonal bit is valid only when both adjacent cardinal neighbours are same terrain.
BLOB_DIR_NAMES = ("N", "NE", "E", "SE", "S", "SW", "W", "NW")
BLOB_CARDINAL_BITS = (0, 2, 4, 6)

def valid_blob_masks() -> list[int]:
    valid = []
    for mask in range(256):
        okay = True
        for diagonal, a, b in ((1,0,2),(3,2,4),(5,4,6),(7,6,0)):
            if mask & (1 << diagonal) and not (mask & (1 << a) and mask & (1 << b)):
                okay = False; break
        if okay: valid.append(mask)
    return valid

BLOB_MASKS = valid_blob_masks()  # 47 masks for a canonical 8-neighbour blob autotile.

def terrain_transition(kind: str, same_mask: int, seed: str, alt_kind: str | None = None) -> Image.Image:
    """Eight-neighbour blob autotile with material-specific, jagged boundaries.

    Every missing cardinal edge gets a deterministic irregular transition band;
    diagonal bits control corner cut-outs when both adjacent cardinals connect.
    """
    alt = alt_kind or {"grass":"dirt", "dirt":"grass", "rock":"dirt", "water":"sand", "sand":"water"}[kind]
    if alt == kind or alt not in ("grass", "dirt", "rock", "water", "sand"):
        raise ValueError(f"Invalid alternate terrain pair: {kind} -> {alt}")
    im = terrain_base(alt, seed + ":alternate")
    primary = terrain_base(kind, seed + ":primary")
    rng = random.Random(stable_seed(f"{seed}:{kind}:{alt}:{same_mask}:edge"))
    n, ne, e, se, so, sw, w, nw = [(same_mask >> i) & 1 for i in range(8)]
    # Chunky pixel-art boundary: shallow +/-1 px sawtooth over a fixed base
    # depth (the legacy 16px pack's style), NOT wide organic banks. Steep steps
    # with occasional 2-px jags read as authored rock/terrain edges at map scale.
    def profile(base: int, count: int, lo: int, hi: int) -> list[int]:
        values=[]
        for i in range(count):
            if i % 4 == 0: base = max(lo, min(hi, base + rng.choice((-1, 0, 1))))
            values.append(base)
        return values
    north = profile(7, TILE, 6, 8); south = profile(7, TILE, 6, 8)
    west = profile(7, TILE, 6, 8); east = profile(7, TILE, 6, 8)

    # Corners: a compact chamfered notch (the legacy wall_curve style), not a
    # circle: a 6x6 pocket whose outer tip is a single 45-degree chamfer. This
    # keeps the tile reading as pixel art while softening the 90-degree meet.
    notch = {"nw": rng.randrange(5, 7), "ne": rng.randrange(5, 7),
             "sw": rng.randrange(5, 7), "se": rng.randrange(5, 7)}
    mask = Image.new("L", (TILE,TILE), 0); px=mask.load()
    for y in range(TILE):
        for x in range(TILE):
            inside = True
            if not n and y < north[x]: inside=False
            if not so and y >= TILE-south[x]: inside=False
            if not w and x < west[y]: inside=False
            if not e and x >= TILE-east[y]: inside=False
            # Chamfered corner notches: a compact pocket cut from the corner,
            # axis-aligned with a single 45-degree tip. The legacy pack's
            # wall_curve_ne is [(8,0),(8,8),(15,8)] - the same geometry scaled.
            if not ne and n and e:
                r = notch["ne"]
                if x >= TILE-r and y < r and (TILE-1-x) + y >= r: inside=False
            if not se and e and so:
                r = notch["se"]
                if x >= TILE-r and y >= TILE-r and (TILE-1-x) + (TILE-1-y) >= r: inside=False
            if not sw and so and w:
                r = notch["sw"]
                if x < r and y >= TILE-r and x + (TILE-1-y) >= r: inside=False
            if not nw and w and n:
                r = notch["nw"]
                if x < r and y < r and x + y >= r: inside=False
            if inside: px[x,y]=255
    im.paste(primary, (0,0), mask)
    # Add a one-pixel edge shadow and a few material-specific transition pixels.
    pix=im.load(); mp=mask.load()
    # Edge shadow: a 50% blend of the two terrain darks rather than the raw
    # dark palette entry. A hard black outline around every blob made
    # same-terrain borders read as a dark lattice once tiles butted together.
    dark_kind = PALETTE.get(kind + "_dark", PALETTE[kind])
    dark_alt = PALETTE.get(alt + "_dark", PALETTE[alt])
    edge_col = tuple((a + b) // 2 for a, b in zip(dark_kind, dark_alt))
    for y in range(TILE):
        for x in range(TILE):
            if mp[x,y] and ((x>0 and not mp[x-1,y]) or (x<TILE-1 and not mp[x+1,y]) or (y>0 and not mp[x,y-1]) or (y<TILE-1 and not mp[x,y+1])):
                pix[x,y]=(*edge_col,255)
    d=ImageDraw.Draw(im)
    for _ in range(8):
        x,y=rng.randrange(2,30),rng.randrange(2,30)
        if mp[x,y] and rng.random()<.65:
            light=PALETTE.get(kind+"_light",PALETTE[kind])
            d.point((x,y), fill=(*light,255))
    return im

def connected_road(mask: int, seed: str) -> Image.Image:
    """Connected dirt road with edge-to-edge arms, shoulders, caps and aggregate."""
    im = terrain_base("dirt", seed + ":roadbed")
    d = ImageDraw.Draw(im)
    cx=cy=15
    # Draw darker shoulders first, then the road surface. A connected arm reaches
    # the tile border; a disconnected arm ends at a bevelled cap near the center.
    endpoints = ((15, -1), (32, 15), (15, 32), (-1, 15))
    for i,(ex,ey) in enumerate(endpoints):
        if not mask & (1 << i):
            if i == 0: ey=8
            elif i == 1: ex=23
            elif i == 2: ey=23
            else: ex=8
        d.line((cx,cy,ex,ey), fill=(*PALETTE["road_dark"],255), width=15)
    d.ellipse((cx-7,cy-7,cx+7,cy+7), fill=(*PALETTE["road_dark"],255))
    for i,(ex,ey) in enumerate(endpoints):
        if not mask & (1 << i):
            if i == 0: ey=8
            elif i == 1: ex=23
            elif i == 2: ey=23
            else: ex=8
        d.line((cx,cy,ex,ey), fill=(*PALETTE["road"],255), width=11)
    d.ellipse((cx-5,cy-5,cx+5,cy+5), fill=(*PALETTE["road"],255))
    # Broken gravel highlights, avoiding lane-like lines that look like a grid.
    rng=random.Random(stable_seed(seed+":gravel"))
    for _ in range(15):
        x,y=rng.randrange(3,29),rng.randrange(3,29)
        if im.getpixel((x,y))[:3] in (PALETTE["road"], PALETTE["road_dark"]):
            col=PALETTE["road_light"] if rng.random()<.7 else PALETTE["road_dark"]
            d.point((x,y), fill=(*col,255))
    # Fine edge wear at disconnected road ends.
    for i in range(4):
        if not mask & (1<<i):
            if i==0: d.line((10,8,20,8),fill=(*PALETTE["dirt_dark"],255),width=1)
            elif i==1: d.line((23,10,23,20),fill=(*PALETTE["dirt_dark"],255),width=1)
            elif i==2: d.line((10,23,20,23),fill=(*PALETTE["dirt_dark"],255),width=1)
            else: d.line((8,10,8,20),fill=(*PALETTE["dirt_dark"],255),width=1)
    return im

def independent_turret_frame(faction: str, unit: str, facing: int, recoil_frame: int = 0) -> Image.Image:
    """Transparent turret overlay: aim direction is independent of hull facing.

    recoil_frame: 0=ready, 1=kickback, 2=recovery. Eight-way direction is N,NE,E,SE,S,SW,W,NW.
    """
    im = Image.new("RGBA", (TILE, TILE), (0, 0, 0, 0)); d = ImageDraw.Draw(im)
    dirs = ((0,-1),(1,-1),(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1))
    dx, dy = dirs[facing % 8]
    if dx and dy:
        dx = 1 if dx > 0 else -1; dy = 1 if dy > 0 else -1
    body = PALETTE["gdi" if faction == "gdi" else "nod"]
    dark = PALETTE["gdi_dark" if faction == "gdi" else "nod_dark"]
    # The barrel moves backward only during the firing impulse; hull pixels are not included.
    kick = 2 if recoil_frame == 1 else 1 if recoil_frame == 2 else 0
    cx, cy = 16 - dx*kick, 16 - dy*kick
    radius = 4 if unit == "artillery" else 4
    d.ellipse((cx-radius,cy-radius,cx+radius,cy+radius), fill=(*dark,255), outline=(*PALETTE["metal_dark"],255))
    d.ellipse((cx-2,cy-2,cx+2,cy+2), fill=(*body,255), outline=(*PALETTE["metal"],255))
    length = 12 if unit == "artillery" else 9
    ex, ey = cx+dx*length, cy+dy*length
    d.line((cx,cy,ex,ey), fill=(*PALETTE["metal_dark"],255), width=4)
    d.line((cx,cy,ex,ey), fill=(*PALETTE["metal"],255), width=2)
    if recoil_frame == 1:
        d.polygon(((ex+dy*3,ey-dx*3),(ex+dx*4,ey+dy*4),(ex-dy*3,ey+dx*3)), fill=(*PALETTE["muzzle"],255))
    return im


def road_edge_signature(im: Image.Image, direction: str) -> tuple[bool, ...]:
    """Sample road-material pixels along a tile edge for adjacent-tile QA."""
    pix=im.convert("RGBA").load()
    coords = {"N": [(x,0) for x in range(TILE)], "E": [(TILE-1,y) for y in range(TILE)],
              "S": [(x,TILE-1) for x in range(TILE)], "W": [(0,y) for y in range(TILE)]}[direction]
    road_colors={PALETTE["road"],PALETTE["road_dark"],PALETTE["road_light"]}
    return tuple(pix[x,y][3] > 0 and pix[x,y][:3] in road_colors for x,y in coords)


def audit_road_continuity(seed: str = "road-audit") -> dict[str, Any]:
    """Verify each connected road arm reaches its edge and opposite arms align."""
    roads={m:connected_road(m,f"{seed}:{m}") for m in range(16)}
    checks=[]; failures=[]
    # Edge arms must be present at the central crossing and each edge pair must align.
    edge_bits={"N":(0,"S",2),"E":(1,"W",3),"S":(2,"N",0),"W":(3,"E",1)}
    for mask,im in roads.items():
        for direction,(bit,opposite,opbit) in edge_bits.items():
            sig=road_edge_signature(im,direction)
            connected=bool(mask & (1<<bit))
            # Connected road reaches the edge near its center; disconnected road must not.
            reaches=any(sig[max(0,TILE//2-5):min(TILE,TILE//2+6)])
            ok=(reaches if connected else not reaches)
            item={"mask":mask,"edge":direction,"connected":connected,"edge_reached":reaches,"pass":ok}
            checks.append(item)
            if not ok: failures.append(item)
    return {"checks":len(checks),"failures":failures,"pass":not failures}

def connected_shore(mask: int, seed: str) -> Image.Image:
    """Water tile with irregular sand banks and broken foam along selected edges."""
    im=terrain_base("water", seed + ":water")
    d=ImageDraw.Draw(im)
    rng=random.Random(stable_seed(seed+":shoreline"))
    # Each land-facing edge gets a chunky, irregular bank. Shared corner regions
    # overlap intentionally so adjacent edge combinations form natural coves.
    for edge in range(4):
        if not mask & (1<<edge): continue
        if edge in (0,2):
            ybase=0 if edge==0 else 31
            depths=[rng.randrange(5,9) if i%3==0 else 6 for i in range(32)]
            for x,depth in enumerate(depths):
                if edge==0: y0,y1=0,depth-1
                else: y0,y1=32-depth,31
                d.line((x,y0,x,y1),fill=(*PALETTE["sand"],255))
                if x%4==0:
                    yy=depth-1 if edge==0 else 32-depth
                    d.point((x,yy),fill=(*PALETTE["sand_light"],255))
            # Dark wet-sand contour plus discontinuous foam immediately water-side.
            for x,depth in enumerate(depths):
                boundary=depth if edge==0 else 31-depth
                d.point((x,boundary),fill=(*PALETTE["sand_dark"],255))
                if x%5 in (0,1):
                    fy=boundary+1 if edge==0 else boundary-1
                    if 0<=fy<32: d.point((x,fy),fill=(*PALETTE["white"],255))
        else:
            depths=[rng.randrange(5,9) if i%3==0 else 6 for i in range(32)]
            for y,depth in enumerate(depths):
                if edge==3: x0,x1=0,depth-1
                else: x0,x1=32-depth,31
                d.line((x0,y,x1,y),fill=(*PALETTE["sand"],255))
                if y%4==0:
                    xx=depth-1 if edge==3 else 32-depth
                    d.point((xx,y),fill=(*PALETTE["sand_light"],255))
            for y,depth in enumerate(depths):
                boundary=depth if edge==3 else 31-depth
                d.point((boundary,y),fill=(*PALETTE["sand_dark"],255))
                if y%5 in (0,1):
                    fx=boundary+1 if edge==3 else boundary-1
                    if 0<=fx<32: d.point((fx,y),fill=(*PALETTE["white"],255))
    return im

UNIT_STATES = {
    "tank": ("idle", "move", "turn", "attack", "death"),
    "harvester": ("idle", "move", "turn", "harvest", "unload", "death"),
    "artillery": ("idle", "move", "turn", "deploy", "attack", "death"),
    "infantry": ("idle", "move", "attack", "death"),
    "engineer": ("idle", "move", "repair", "death"),
    "rocket_soldier": ("idle", "move", "attack", "death"),
    "buggy": ("idle", "move", "turn", "attack", "death"),
}
STATE_FRAMES = {"idle":2, "move":4, "turn":4, "attack":4, "harvest":4, "unload":4, "deploy":4, "repair":4, "death":6}

def unit_frame(faction: str, unit: str, facing: int, frame: int, state: str = "move") -> Image.Image:
    """Draw a direction-aware pose without rotating a finished sprite.

    Facing is an eight-way direction. Body axis, weapon direction, limbs, track
    marks, recoil, harvesting and death poses are drawn directly per facing/state.
    This remains procedural pixel art, but frames represent authored pose changes.
    """
    im = Image.new("RGBA", (TILE,TILE), (0,0,0,0)); d = ImageDraw.Draw(im)
    body = PALETTE["gdi" if faction == "gdi" else "nod"]
    dark = PALETTE["gdi_dark" if faction == "gdi" else "nod_dark"]
    dirs = ((0,-1),(1,-1),(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1))
    dx,dy = dirs[facing % 8]
    if dx and dy: dx,dy = (1 if dx>0 else -1, 1 if dy>0 else -1)
    cx, cy = 16, 16
    if state == "death":
        spread = min(5, frame + 1)
        if unit in ("tank","harvester","artillery","buggy"):
            d.polygon(((cx-spread,cy-5),(cx+5,cy-spread),(cx+spread,cy+4),(cx-4,cy+spread)), fill=(*dark,255))
            d.rectangle((cx-5,cy-2,cx+5,cy+2), fill=(*PALETTE["metal_dark"],255))
        else:
            d.ellipse((cx-7-spread//2,cy-3,cx+7+spread//2,cy+3+spread//2), fill=(*dark,255))
            head_x = cx - 3 + (frame % 3)
            d.ellipse((head_x-2,cy-6-spread//2,head_x+2,cy-2), fill=(*PALETTE["skin"],255))
            d.line((cx-4,cy+2,cx+4+(frame%2),cy+3+spread//2), fill=(*body,255), width=2)
        if frame >= 3:
            r = frame - 2
            d.ellipse((cx-r-2,cy-r-5,cx+r+2,cy+r-1), fill=(*PALETTE["smoke"],max(40,190-frame*22)))
        return im
    bob = (0,1,0,-1)[frame % 4] if state != "death" else 0
    if state == "attack": recoil = -2 if frame == 2 else 0
    else: recoil = 0
    if unit in ("tank","harvester","artillery"):
        # Body footprint oriented using direction vectors, not image rotation.
        px, py = -dy, dx
        length = 12 if unit == "artillery" else 10
        width = 7
        def xy(along: float, across: float) -> tuple[int,int]:
            return (int(cx + dx*along + px*across), int(cy + dy*along + py*across + bob))
        corners = [xy(-length,-width),xy(length,-width),xy(length,width),xy(-length,width)]
        d.polygon(corners, fill=(*PALETTE["track"],255), outline=(*PALETTE["metal_dark"],255))
        # Individual track pads shift with the animation phase; these are readable
        # as treads at native size instead of continuous bright stripes.
        phase = frame % 4 if state in ("move", "turn") else 0
        for side in (-1, 1):
            for t in range(-length+2, length, 4):
                a=xy(t,side*(width-1)); b=xy(min(length,t+2),side*(width-1))
                d.line((a,b), fill=(*PALETTE["metal" if (t+phase)%3==0 else "metal_dark"],255), width=1)
        inner=[xy(-length+2,-width+2),xy(length-2,-width+2),xy(length-2,width-2),xy(-length+2,width-2)]
        d.polygon(inner, fill=(*body,255), outline=(*dark,255))
        # Raised engine deck and highlight panels give the hull readable volume.
        rear1=xy(-length+4,-width+3); rear2=xy(-length+4,width-3)
        d.line((rear1,rear2), fill=(*dark,255), width=2)
        front1=xy(length-4,-width+3); front2=xy(length-4,width-3)
        d.line((front1,front2), fill=(*(PALETTE["gdi"] if faction=="gdi" else PALETTE["nod"]),255), width=1)
        # Small faction marking on the rear deck; keep it distinct from the silhouette.
        mark_col = PALETTE["white"] if faction=="gdi" else PALETTE["fire"]
        mx,my=xy(-length+1,0); d.point((mx,my), fill=(*mark_col,255))
        # Turret is a separately shaded layer with a directional barrel.
        tcx,tcy=cx+dx*recoil,cy+dy*recoil+bob
        d.ellipse((tcx-5,tcy-5,tcx+5,tcy+5), fill=(*dark,255), outline=(*PALETTE["metal_dark"],255))
        d.arc((tcx-4,tcy-4,tcx+4,tcy+4), 195, 330, fill=(*PALETTE["metal"],255), width=1)
        d.ellipse((tcx-2,tcy-2,tcx+2,tcy+2), fill=(*body,255), outline=(*dark,255))
        blen = 12 if unit == "artillery" else 8
        ex,ey=tcx+dx*blen,tcy+dy*blen
        d.line((tcx, tcy, ex, ey), fill=(*PALETTE["metal_dark"],255), width=4)
        d.line((tcx, tcy, ex, ey), fill=(*PALETTE["metal"],255), width=2)
        if state == "attack" and frame == 2:
            d.polygon(((ex+dy*3,ey-dx*3),(ex+dx*4,ey+dy*4),(ex-dy*3,ey+dx*3)), fill=(*PALETTE["muzzle"],255))
        if unit == "harvester":
            if state in ("harvest","unload"):
                d.rectangle((cx+dx*7-3,cy+dy*7-3,cx+dx*7+3,cy+dy*7+3), fill=(*PALETTE["tiberium"],255) if state=="harvest" else (*PALETTE["sand"],255))
            d.rectangle((cx-3,cy-3+bob,cx+3,cy+3+bob), fill=(*PALETTE["metal"],255))
        if state == "deploy" and unit == "artillery":
            d.line((cx-dy*8,cy+dx*8,cx+dy*8,cy-dx*8), fill=(*PALETTE["metal"],255), width=2)
    elif unit in ("infantry","engineer","rocket_soldier"):
        # Directional head/body, weapon, arms and alternating stride poses.
        stride = (-3,1,3,-1)[frame % 4] if state == "move" else (0,1,0,-1)[frame % 4]
        hx,hy=cx+dx*3,cy+dy*3+bob
        d.ellipse((hx-3,hy-3,hx+3,hy+3), fill=(*PALETTE["skin"],255))
        d.polygon(((cx-dy*4,cy+dx*4+bob),(cx+dy*4,cy-dx*4+bob),(cx-dx*3,cy-dy*3+bob)), fill=(*body,255))
        for side in (-1,1):
            sx,sy=cx+(-dy*side*2),cy+(dx*side*2)+bob
            ex,ey=sx+dx*4-dy*side*stride//2,sy+dy*4+dx*side*stride//2
            d.line((sx,sy,ex,ey), fill=(*dark,255), width=2)
            lx,ly=cx-dy*side*2,cy+dy*3+bob
            fx,fy=lx+dx*4-dy*side*stride,ly+dy*4+dx*side*stride
            d.line((lx,ly,fx,fy), fill=(*dark,255), width=2)
        wx,wy=cx+dx*(8+recoil),cy+dy*(8+recoil)+bob
        if unit == "rocket_soldier":
            d.line((cx+dx*2,cy+dy*2,wx,wy), fill=(*PALETTE["metal"],255), width=3)
        elif unit == "engineer":
            d.rectangle((wx-2,wy-2,wx+2,wy+2), fill=(*PALETTE["metal"],255))
            if state == "repair" and frame in (1,2): d.point((wx+dx*2,wy+dy*2), fill=(*PALETTE["muzzle"],255))
        else:
            d.line((cx+dx*2,cy+dy*2,wx,wy), fill=(*PALETTE["metal"],255), width=2)
        if state == "attack" and frame == 2:
            d.line((wx,wy,wx+dx*5,wy+dy*5), fill=(*PALETTE["muzzle"],255), width=3)
    elif unit == "buggy":
        px,py=-dy,dx
        pts=((cx-dx*8-px*5,cy-dy*8-py*5+bob),(cx+dx*8-px*5,cy+dy*8-py*5+bob),(cx+dx*8+px*5,cy+dy*8+py*5+bob),(cx-dx*8+px*5,cy-dy*8+py*5+bob))
        d.polygon(pts, fill=(*body,255), outline=(*dark,255))
        for side in (-1,1):
            wx,wy=cx+px*side*5,cy+py*side*5+bob
            d.ellipse((wx-2,wy-3,wx+2,wy+3), fill=(*PALETTE["track"],255))
            if state in ("move", "turn"):
                phase = frame % 4
                d.point((wx + (phase-1), wy + (phase%3)-1), fill=(*PALETTE["metal"],255))
        ex,ey=cx+dx*(9+recoil),cy+dy*(9+recoil)+bob
        d.line((cx,cy,ex,ey), fill=(*PALETTE["metal"],255), width=3)
        if state == "attack" and frame == 2: d.ellipse((ex-2,ey-2,ex+2,ey+2), fill=(*PALETTE["muzzle"],255))
    # State timing accent: tiny engine-light/tool/cargo details animate in idle and work states.
    # These details are deliberate pixel-art cues, not arbitrary hash noise.
    if state == "idle" and frame % 2:
        d.point((cx + dx*5, cy + dy*5), fill=(*PALETTE["muzzle"],255))
    elif state == "harvest":
        d.point((cx + dx*(11 + frame), cy + dy*(11 + frame)), fill=(*PALETTE["tiberium"],255))
    elif state == "unload":
        for j in range(frame + 1):
            d.point((cx + 5 + j, cy + 10), fill=(*PALETTE["sand"],255))
            # Cargo accumulates progressively below the vehicle across frames.
            d.point((2 + j*2, 29), fill=(*PALETTE["sand_light"],255))
    elif state == "deploy":
        d.point((cx + frame - 1, cy+8), fill=(*PALETTE["metal"],255))
    elif state == "repair" and frame % 2:
        d.point((cx + dx*10, cy + dy*10), fill=(*PALETTE["muzzle"],255))
    return im

def unit_condition_frame(faction: str, unit: str, facing: int, condition: str) -> Image.Image:
    """Draw a complete directional unit sprite at a distinct damage condition.

    Conditions are engine-ready full frames, not decals that require an undocumented
    compositor: intact, damaged, critical, and destroyed. Damage marks are deterministic
    and placed relative to the unit's facing so they remain attached to the hull.
    """
    if condition not in ("intact", "damaged", "critical", "destroyed"):
        raise ValueError(f"Unknown unit condition: {condition}")
    if condition == "destroyed":
        # Use the early wreck pose before the death animation's smoke fully blooms;
        # a destroyed selection should read as wreckage rather than an opaque smoke orb.
        im = unit_frame(faction, unit, facing, 2, "death")
        d = ImageDraw.Draw(im)
        # Readable charred hull cracks and scattered debris at native 32x32 scale.
        d.line((7, 21, 12, 17, 16, 23, 22, 18), fill=(*PALETTE["metal_dark"], 255), width=2)
        d.line((11, 13, 15, 15, 18, 11), fill=(*PALETTE["rock_light"], 255), width=1)
        d.point((8, 25), fill=(*PALETTE["fire"], 255))
        d.point((23, 20), fill=(*PALETTE["rock_light"], 255))
        return im

    im = unit_frame(faction, unit, facing, 0, "idle")
    if condition == "intact":
        return im

    d = ImageDraw.Draw(im)
    dirs = ((0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1))
    dx, dy = dirs[facing % 8]
    # Put the impact forward-right relative to the facing, not at one fixed screen pixel.
    px, py = -dy, dx
    hx = max(4, min(27, int(16 + dx * 5 + px * 3)))
    hy = max(4, min(27, int(16 + dy * 5 + py * 3)))
    if condition == "damaged":
        d.ellipse((hx-2, hy-2, hx+2, hy+2), fill=(*PALETTE["rock_dark"], 255))
        d.line((hx-3, hy, hx, hy+2, hx+3, hy-2), fill=(*PALETTE["metal_dark"], 255), width=1)
        d.point((hx+2, hy+1), fill=(*PALETTE["rock_light"], 255))
    else:  # critical: heavy scorch plus small visible fire and smoke accents
        d.ellipse((hx-4, hy-3, hx+4, hy+3), fill=(*PALETTE["black"], 235))
        d.line((hx-4, hy-1, hx-1, hy+2, hx+2, hy-2, hx+4, hy+1), fill=(*PALETTE["rock_light"], 255), width=1)
        d.point((max(1, min(30, hx+dx)), max(1, min(30, hy+dy))), fill=(*PALETTE["fire"], 255))
        d.point((max(1, min(30, hx+dx*2)), max(1, min(30, hy+dy*2))), fill=(*PALETTE["fire_light"], 255))
        sx, sy = max(2, min(29, hx-dx*2)), max(2, min(29, hy-dy*2))
        d.point((sx, sy), fill=(*PALETTE["smoke"], 255))
    return im


def effect_frame(kind: str, frame: int) -> Image.Image:
    im = Image.new("RGBA", (TILE, TILE), (0, 0, 0, 0)); d = ImageDraw.Draw(im)
    if kind == "explosion":
        radii = (4, 8, 13, 10, 6, 3)
        r = radii[frame % len(radii)]; cx = 16 + (frame % 3 - 1); cy = 16 - (frame % 2)
        d.ellipse((cx-r, cy-r, cx+r, cy+r), fill=(*PALETTE["fire"], 230))
        if r > 3: d.ellipse((cx-r//2, cy-r//2, cx+r//2, cy+r//2), fill=(*PALETTE["fire_light"], 255))
        if frame % 2 == 1: d.polygon(((cx, cy-r-2), (cx+3, cy-2), (cx-2, cy+2)), fill=(*PALETTE["muzzle"], 255))
    elif kind == "smoke":
        r = 3 + frame % 5; cx = 14 + frame % 4; cy = 22 - frame * 2
        d.ellipse((cx-r, cy-r, cx+r, cy+r), fill=(*PALETTE["smoke"], max(60, 210-frame*22)))
    elif kind == "muzzle_flash":
        d.polygon(((5, 16), (13, 12 + frame % 3), (19, 6), (18, 15), (28, 16), (18, 19), (20, 26), (12, 20)), fill=(*PALETTE["muzzle" if frame % 2 else "fire"], 255))
    elif kind == "destruction":
        # Multi-stage vehicle/building destruction: blast, debris, then dissipating smoke.
        if frame <= 2:
            r=(5,10,14)[frame]
            d.ellipse((16-r,16-r,16+r,16+r), fill=(*PALETTE["fire"], max(100,240-frame*35)))
            d.ellipse((16-r//2,16-r//2,16+r//2,16+r//2), fill=(*PALETTE["muzzle"],255))
            for i in range(4+frame*2):
                angle=(i*137+frame*29)*math.pi/180
                x=int(16+math.cos(angle)*(r+2)); y=int(16+math.sin(angle)*(r+2))
                d.rectangle((x-1,y-1,x+1,y+1), fill=(*PALETTE["metal_dark"],255))
        else:
            r=3+(frame-3)*2; cx=13+(frame%3); cy=22-(frame-3)*2
            d.ellipse((cx-r,cy-r,cx+r,cy+r), fill=(*PALETTE["smoke"],max(35,190-(frame-3)*28)))
            if frame < 6:
                d.line((7+frame,25,11+frame,22),fill=(*PALETTE["metal_dark"],220),width=2)
    return im

def image_hash(im: Image.Image) -> str:
    return hashlib.sha256(im.tobytes()).hexdigest()

def contrast_audit() -> dict[str, Any]:
    failures, pairs = [], []
    for name, fg_name, bg_name in CONTRAST_PAIRS:
        fg, bg = PALETTE[fg_name], PALETTE[bg_name]
        ratio = contrast_ratio(fg, bg)
        row = {"name": name, "foreground": fg_name, "foreground_hex": color_hex(fg), "background": bg_name,
               "background_hex": color_hex(bg), "ratio": round(ratio, 3), "pass": ratio >= 3.0}
        pairs.append(row)
        if ratio < 3.0:
            failures.append({**row, "suggestion": "Choose a lighter foreground or darker background; increase separation; 1.5:1 is the default material-separation floor."})
    return {"threshold": 1.5, "pairs_checked": len(pairs), "failures": failures, "pairs": pairs,
            "note": "The configured threshold is a practical material-separation floor, not a universal WCAG requirement. Check actual in-game scaling and palette."}

def make_atlas(items: list[dict[str, Any]], cols: int = 16) -> Image.Image:
    rows = math.ceil(len(items) / cols) or 1
    atlas = Image.new("RGBA", (cols * TILE, rows * TILE), (0, 0, 0, 0))
    for i, item in enumerate(items): atlas.alpha_composite(item["image"], ((i % cols) * TILE, (i // cols) * TILE))
    return atlas

def build(out: Path, force: bool = False, seed: str = "cnc32", contrast_threshold: float = 1.5) -> dict[str, Any]:
    if out.exists() and not force: raise FileExistsError(f"Output already exists: {out} (use --force to replace it)")
    out.parent.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix=f".{out.name}.stage-", dir=out.parent))
    sprites: list[dict[str, Any]] = []
    def add(asset_id: str, category: str, image: Image.Image, **meta: Any) -> None:
        rel = Path("sprites") / f"{asset_id}.png"; path = stage / rel; path.parent.mkdir(parents=True, exist_ok=True)
        image.save(path, format="PNG", optimize=False, compress_level=9)
        sprites.append({"id": asset_id, "category": category, "file": rel.as_posix(), "size": [image.width, image.height],
                        "sha256_file": hashlib.sha256(path.read_bytes()).hexdigest(), "pixel_sha256": image_hash(image), **meta})
    try:
        # Complete directed terrain-pair transitions: every terrain against every other terrain,
        # with all 47 valid eight-neighbour blob masks (20 pairs x 47 = 940 tiles).
        terrain_kinds = ("grass", "dirt", "rock", "water", "sand")
        for terrain in terrain_kinds:
            for other in terrain_kinds:
                if other == terrain: continue
                for mask in BLOB_MASKS:
                    im = terrain_transition(terrain, mask, f"{seed}:terrain:{terrain}:{other}:{mask:02x}", alt_kind=other)
                    add(f"terrain/transitions/{terrain}_to_{other}/blob_{mask:02x}", "terrain_transition", im,
                        terrain=terrain, alternate_terrain=other, mask=mask,
                        mask_bits={BLOB_DIR_NAMES[i]: bool(mask & (1 << i)) for i in range(8)},
                        semantics="set bit = same-terrain neighbor; diagonals are valid only when both adjacent cardinal bits are set")
        # Standalone base terrain variants, resources, cliffs, walls, bridges and map decorations.
        for terrain in terrain_kinds:
            for variant in range(4):
                add(f"terrain/base/{terrain}/variant_{variant:02d}", "terrain_base", terrain_base(terrain, f"{seed}:base:{terrain}:{variant}"),
                    terrain=terrain, variant=variant)
        for kind in ("tiberium", "ore"):
            for variant in range(8):
                im = new_tile(PALETTE["grass"]); d = ImageDraw.Draw(im)
                rng = random.Random(stable_seed(f"{seed}:{kind}:{variant}"))
                color = PALETTE["tiberium"] if kind == "tiberium" else PALETTE["sand"]
                highlight = PALETTE["grass_light"] if kind == "tiberium" else PALETTE["sand_light"]
                for _ in range(5 + variant % 4):
                    x, y = rng.randrange(5,27), rng.randrange(5,27); r = rng.randrange(2,5)
                    d.polygon(((x,y-r),(x+r,y),(x,y+r),(x-r,y)), fill=(*color,255), outline=(*highlight,255))
                    d.point((x,y-r+1), fill=(*highlight,255))
                add(f"resources/{kind}/variant_{variant:02d}", "resource", im, resource=kind, variant=variant, footprint=[1,1])
        for mask in range(16):
            # Cliff connectivity follows the same N/E/S/W convention; edges are visible escarpments.
            # The escarpment band is drawn column-by-column with an irregular
            # depth (the shoreline's per-column family) instead of one flat
            # rectangle, and the crest gets broken highlight pixels instead of a
            # straight light line, so cliff tops read as broken rock.
            im = terrain_base("rock", f"{seed}:cliff:{mask}"); d = ImageDraw.Draw(im)
            rng = random.Random(stable_seed(f"{seed}:cliff:{mask}:edge"))
            BAND = 5
            for i, edge in enumerate(("n", "e", "s", "w")):
                if mask & (1 << i):
                    continue
                depths = [
                    rng.randrange(BAND, BAND + 4) if idx % 3 == 0 else BAND
                    for idx in range(TILE)
                ]
                for idx, depth in enumerate(depths):
                    if edge in ("n", "s"):
                        y0, y1 = (0, depth - 1) if edge == "n" else (TILE - depth, TILE - 1)
                        d.line((idx, y0, idx, y1), fill=(*PALETTE["rock_dark"], 255))
                        crest = depth - 1 if edge == "n" else TILE - depth
                        d.point((idx, crest), fill=(*PALETTE["rock_light"], 255))
                        # Broken highlight: skip every third pixel so the crest
                        # never reads as a straight ruled line.
                        if idx % 3:
                            tip = crest + (1 if edge == "n" else -1)
                            if 0 <= tip < TILE:
                                d.point((idx, tip), fill=(*PALETTE["rock_dark"], 255))
                    else:
                        x0, x1 = (0, depth - 1) if edge == "w" else (TILE - depth, TILE - 1)
                        d.line((x0, idx, x1, idx), fill=(*PALETTE["rock_dark"], 255))
                        crest = depth - 1 if edge == "w" else TILE - depth
                        d.point((crest, idx), fill=(*PALETTE["rock_light"], 255))
                        idx_tip = crest + (1 if edge == "w" else -1)
                        if 0 <= idx_tip < TILE:
                            d.point((idx_tip, idx), fill=(*PALETTE["rock_dark"], 255))
            add(f"cliffs/cliff_{mask:02x}", "cliff_connection", im, mask=mask, connections=[DIRECTION_NAMES[i] for i in mask_dirs(mask)])
        for mask in range(16):
            # Wall bands use the same irregular per-column edge family as the
            # cliffs (a flat rectangle reads as a square cut); the masonry core
            # is kept straight so the wall still reads as built.
            im = new_tile(PALETTE["dirt"]); d = ImageDraw.Draw(im)
            rng = random.Random(stable_seed(f"{seed}:wall:{mask}:edge"))
            CORE_LO, CORE_HI = 12, 20
            d.rectangle((11, 0, 20, 31), fill=(*PALETTE["metal_dark"], 255))
            d.rectangle((0, 11, 31, 20), fill=(*PALETTE["metal_dark"], 255))
            # Ragged capping pixels on each open end so the wall tapers into the
            # ground instead of stopping at a square edge.
            for i, edge in enumerate(("n", "e", "s", "w")):
                if mask & (1 << i):
                    continue
                # Taper only the last 5 columns/rows of the capped end, at low
                # density: a dense speckle reads as a broken band at map scale,
                # so the masonry core stays dominant and only the very end
                # feathers into the ground.
                for idx in range(TILE - 5, TILE):
                    if rng.random() < 0.65:
                        continue
                    if edge == "n": d.point((CORE_LO + rng.randrange(9), 7 + rng.randrange(3)), fill=(*PALETTE["dirt"], 255))
                    elif edge == "s": d.point((CORE_LO + rng.randrange(9), 22 - rng.randrange(3)), fill=(*PALETTE["dirt"], 255))
                    elif edge == "w": d.point((7 + rng.randrange(3), CORE_LO + rng.randrange(9)), fill=(*PALETTE["dirt"], 255))
                    else: d.point((24 - rng.randrange(3), CORE_LO + rng.randrange(9)), fill=(*PALETTE["dirt"], 255))
            d.rectangle((13,13,18,18), fill=(*PALETTE["metal"],255))
            add(f"walls/wall_{mask:02x}", "wall_connection", im, mask=mask, connections=[DIRECTION_NAMES[i] for i in mask_dirs(mask)])
        for orientation in ("horizontal", "vertical"):
            for condition in ("intact", "damaged", "destroyed"):
                im = terrain_base("water", f"{seed}:bridge:{orientation}:{condition}"); d = ImageDraw.Draw(im)
                if orientation == "horizontal": box=(0,9,31,22); plank_boxes=[(x,10,x+2,21) for x in range(0,32,4)]
                else: box=(9,0,22,31); plank_boxes=[(10,y,21,y+2) for y in range(0,32,4)]
                d.rectangle(box, fill=(*PALETTE["dirt_dark" if condition=="destroyed" else "dirt"],255), outline=(*PALETTE["metal_dark"],255))
                if condition != "destroyed":
                    for b in plank_boxes: d.rectangle(b, fill=(*PALETTE["dirt_light" if condition=="intact" else "dirt_dark"],255))
                if condition != "intact":
                    for x,y in ((7,7),(24,24),(8,23)): d.line((x,y,x+4,y+4), fill=(*PALETTE["rock_dark"],255), width=2)
                add(f"bridges/{orientation}/{condition}", "bridge", im, orientation=orientation, condition=condition)
        for prop in ("tree", "boulder", "crater", "wreck", "bush", "debris"):
            for variant in range(4):
                im = terrain_base("grass", f"{seed}:prop:{prop}:{variant}"); d = ImageDraw.Draw(im)
                rng = random.Random(stable_seed(f"{seed}:prop:{prop}:{variant}"))
                x,y=rng.randrange(9,23),rng.randrange(9,23)
                if prop == "tree":
                    d.rectangle((x-1,y,x+1,y+8), fill=(*PALETTE["dirt_dark"],255)); d.ellipse((x-6,y-7,x+6,y+4), fill=(*PALETTE["grass_dark"],255), outline=(*PALETTE["grass_light"],255))
                elif prop == "boulder": d.ellipse((x-5,y-4,x+5,y+4), fill=(*PALETTE["rock_dark"],255), outline=(*PALETTE["rock_light"],255))
                elif prop == "crater": d.ellipse((x-7,y-4,x+7,y+4), fill=(*PALETTE["dirt_dark"],255), outline=(*PALETTE["dirt_light"],255))
                elif prop == "wreck": d.polygon(((x-6,y-2),(x+3,y-5),(x+6,y+3),(x-3,y+5)), fill=(*PALETTE["metal_dark"],255), outline=(*PALETTE["metal"],255))
                elif prop == "bush": d.ellipse((x-5,y-3,x+5,y+3), fill=(*PALETTE["grass_dark"],255), outline=(*PALETTE["grass_light"],255))
                else:
                    for _ in range(8): d.point((x+rng.randrange(-5,6),y+rng.randrange(-4,5)), fill=(*PALETTE["rock_dark"],255))
                add(f"decorations/{prop}/variant_{variant:02d}", "decoration", im, prop=prop, variant=variant, footprint=[1,1])
        # Faction-specific structures with explicit construction and damage states.
        # Silhouettes, roof profiles, faction colors and footprints differ by role.
        structure_specs = {
            "power_plant": ("rect", (5, 8, 27, 25)), "barracks": ("wide", (3, 10, 29, 26)),
            "refinery": ("industrial", (3, 7, 29, 27)), "war_factory": ("hangar", (2, 8, 30, 27)),
            "communications": ("tower", (9, 5, 23, 27)), "defense_turret": ("turret", (7, 8, 25, 27)),
        }
        for faction in ("gdi", "nod"):
            faction_color = PALETTE[faction]; faction_dark = PALETTE[f"{faction}_dark"]
            for structure, (shape, (x0, y0, x1, y1)) in structure_specs.items():
                for condition in ("construction", "intact", "damaged", "destroyed"):
                    im = new_tile(PALETTE["grass"]); d = ImageDraw.Draw(im)
                    if condition == "destroyed":
                        d.polygon(((x0+2,y1-2),(x0+5,y0+8),(x0+10,y1-5),(x1-3,y0+10),(x1-1,y1-1)), fill=(*PALETTE["metal_dark"],255), outline=(*PALETTE["rock_dark"],255))
                        d.line((x0+4,y1-4,x1-4,y0+8), fill=(*PALETTE["rock_light"],255), width=1)
                    else:
                        if condition == "construction":
                            d.rectangle((x0,y0,x1,y1), outline=(*PALETTE["sand_light"],255), width=2)
                            for xx in range(x0+2,x1,5): d.line((xx,y0,xx,y1), fill=(*PALETTE["sand"],255), width=1)
                            d.rectangle((x0+2,y1-4,x1-2,y1-2), fill=(*PALETTE["rock_dark"],255))
                        else:
                            d.rectangle((x0,y0,x1,y1), fill=(*faction_dark,255), outline=(*PALETTE["metal_dark"],255))
                            roof = faction_color if faction == "gdi" else PALETTE["nod"]
                            if shape == "tower":
                                d.rectangle((x0+4,y0+4,x1-4,y1-3), fill=(*roof,255)); d.line((16,y0-2,16,y0+5), fill=(*PALETTE["metal"],255), width=2)
                            elif shape == "turret":
                                d.polygon(((x0+2,y1-2),(x0+4,y0+6),(16,y0+1),(x1-4,y0+6),(x1-2,y1-2)), fill=(*roof,255))
                                d.ellipse((12,y0+5,20,y0+13), fill=(*PALETTE["metal"],255), outline=(*PALETTE["metal_dark"],255))
                                d.line((16,y0+9,16,y0+1), fill=(*PALETTE["metal"],255), width=2)
                            elif shape == "hangar":
                                d.polygon(((x0+1,y0+6),(16,y0),(x1-1,y0+6)), fill=(*roof,255)); d.rectangle((12,y0+10,20,y1-1), fill=(*PALETTE["metal_dark"],255))
                            elif shape == "industrial":
                                d.rectangle((x0+3,y0+3,x0+9,y0+10), fill=(*PALETTE["metal"],255)); d.rectangle((x1-9,y0+2,x1-4,y0+12), fill=(*PALETTE["metal_dark"],255))
                                d.rectangle((x0+4,y1-8,x1-4,y1-3), fill=(*roof,255))
                            else:
                                d.rectangle((x0+3,y0+4,x1-3,y1-4), fill=(*roof,255)); d.rectangle((14,y1-8,18,y1-1), fill=(*PALETTE["metal_dark"],255))
                            d.rectangle((x0+2,y0+2,x1-2,y0+4), fill=(*PALETTE["metal"],255))
                            if condition == "damaged":
                                d.polygon(((x0+3,y0+2),(x0+9,y0+5),(x0+6,y0+11),(x0+12,y0+14),(x0+8,y0+18)), fill=(*PALETTE["rock_dark"],255))
                                d.point((x1-4,y0+5), fill=(*PALETTE["fire"],255))
                    add(f"buildings/{faction}/{structure}/{condition}", "building", im, faction=faction, building=structure,
                        condition=condition, footprint=[2,2] if structure in ("refinery","war_factory","barracks") else [1,1],
                        silhouette=shape, construction_stage=condition)
        # Roads: all 16 cardinal connectivity states (including isolated/end/corner/T/cross).
        for mask in range(16):
            names = {0: "isolated", 1: "north_end", 2: "east_end", 4: "south_end", 8: "west_end", 5: "vertical", 10: "horizontal", 3: "north_east_corner", 6: "east_south_corner", 12: "south_west_corner", 9: "west_north_corner", 7: "t_north_missing_west", 11: "t_east_missing_south", 13: "t_south_missing_east", 14: "t_west_missing_north", 15: "cross"}
            add(f"roads/road_{mask:02x}_{names[mask]}", "road_connection", connected_road(mask, f"{seed}:road:{mask}"),
                mask=mask, connections=[DIRECTION_NAMES[i] for i in mask_dirs(mask)],
                semantics="set bit = road continues through tile edge")
        # Shoreline: all combinations of land-facing edges.
        for mask in range(16):
            add(f"shoreline/water_landmask_{mask:02x}", "shoreline", connected_shore(mask, f"{seed}:shore:{mask}"),
                land_edges=[DIRECTION_NAMES[i] for i in mask_dirs(mask)], semantics="set bit = land/sand shore on that edge")
        # Full state/facing/frame matrix; every supported unit state has explicit frame metadata.
        units = tuple(UNIT_STATES)
        for faction in ("gdi", "nod"):
            for unit in units:
                for state in UNIT_STATES[unit]:
                    count = STATE_FRAMES[state]
                    for facing in range(8):
                        for frame in range(count):
                            im = unit_frame(faction, unit, facing, frame, state)
                            add(f"units/{faction}/{unit}/{state}/facing_{facing}/frame_{frame:02d}", "unit_animation", im,
                                faction=faction, unit=unit, state=state, facing=facing, frame=frame, frame_count=count,
                                duration_ms=(180 if state == "idle" else 110 if state in ("attack", "turn") else 140 if state == "death" else 120),
                                loop=state in ("idle", "move", "harvest"), pivot=[16, 16], animation=state,
                                pose_variant=f"{state}_{frame:02d}")
        # Full-frame condition matrix: damage is represented in the sprite art itself,
        # so engines need no undocumented overlay compositing to display it.
        for faction in ("gdi", "nod"):
            for unit in units:
                for facing in range(8):
                    for condition in ("intact", "damaged", "critical", "destroyed"):
                        im = unit_condition_frame(faction, unit, facing, condition)
                        add(f"units/{faction}/{unit}/condition_{condition}/facing_{facing}", "unit_condition", im,
                            faction=faction, unit=unit, condition=condition, facing=facing,
                            pivot=[16, 16], frame_count=1, duration_ms=120, loop=False,
                            semantics="complete 32x32 sprite for unit damage condition")
        # Independent turret layers for tracked vehicles: game code can rotate/aim the
        # turret without changing hull facing, and select firing recoil frames separately.
        for faction in ("gdi", "nod"):
            for unit in ("tank", "harvester", "artillery"):
                for turret_facing in range(8):
                    for recoil_frame in range(3):
                        im=independent_turret_frame(faction,unit,turret_facing,recoil_frame)
                        add(f"units/{faction}/{unit}/turret/facing_{turret_facing}/recoil_{recoil_frame}",
                            "turret_animation",im,faction=faction,unit=unit,turret_facing=turret_facing,
                            recoil_frame=recoil_frame,frame_count=3,loop=False,pivot=[16,16],
                            semantics="transparent turret overlay; rotate/aim independently of hull-facing sprite")
        for kind, count in (("explosion", 6), ("smoke", 6), ("muzzle_flash", 3), ("destruction", 8)):
            for frame in range(count):
                add(f"effects/{kind}/frame_{frame:02d}", "effect_animation", effect_frame(kind, frame),
                    effect=kind, frame=frame, frame_count=count, duration_ms=90 if kind != "smoke" else 140,
                    loop=(kind == "smoke"), pivot=[16, 16])
        # Atlas groups are deterministic and keep every atlas cell at exactly 32x32.
        atlas_dir = stage / "atlases"; atlas_dir.mkdir(parents=True)
        atlas_records = []
        groups: dict[str, list[dict[str, Any]]] = {}
        for item in sprites:
            groups.setdefault(item["category"], []).append(item)
        for category, group in sorted(groups.items()):
            group.sort(key=lambda x: x["id"])
            images = []
            for item in group:
                with Image.open(stage / item["file"]) as opened: images.append({"id": item["id"], "image": opened.convert("RGBA")})
            atlas = make_atlas(images, 16); atlas_name = f"{category}.png"; atlas.save(atlas_dir / atlas_name, optimize=False, compress_level=9)
            for i, item in enumerate(group): item["atlas"] = atlas_name; item["atlas_rect"] = [(i % 16) * TILE, (i // 16) * TILE, TILE, TILE]
            atlas_records.append({"file": f"atlases/{atlas_name}", "category": category, "columns": 16,
                                 "rows": math.ceil(len(group) / 16), "cell_size": [TILE, TILE], "sprite_count": len(group),
                                 "sha256_file": hashlib.sha256((atlas_dir / atlas_name).read_bytes()).hexdigest()})
        # Visual QA contact sheets: quick review surfaces for terrain, connections and animation.
        preview_dir = stage / "previews"; preview_dir.mkdir(parents=True, exist_ok=True)
        def save_contact_sheet(items_for_sheet: list[dict[str, Any]], filename: str, cols: int = 8, scale: int = 4) -> None:
            rows = math.ceil(len(items_for_sheet) / cols) or 1
            sheet = Image.new("RGBA", (cols*TILE*scale, rows*TILE*scale), (28,28,30,255))
            draw = ImageDraw.Draw(sheet)
            for idx, item in enumerate(items_for_sheet):
                x=(idx%cols)*TILE*scale; y=(idx//cols)*TILE*scale
                with Image.open(stage/item["file"]) as source: tile=source.convert("RGBA").resize((TILE*scale,TILE*scale),Image.Resampling.NEAREST)
                sheet.alpha_composite(tile,(x,y))
                draw.rectangle((x,y,x+TILE*scale-1,y+TILE*scale-1),outline=(80,80,84,255),width=1)
            sheet.convert("RGB").save(preview_dir/filename,optimize=False)

        def save_unit_condition_review(items_for_sheet: list[dict[str, Any]], filename: str, scale: int = 5) -> None:
            # Arrange each unit/faction on one row and the four health states in
            # adjacent columns; this makes damage-stage progression visually auditable.
            order = ("intact", "damaged", "critical", "destroyed")
            lookup = {(item.get("faction"), item.get("unit"), item.get("facing"), item.get("condition")): item for item in items_for_sheet}
            unit_order = ("tank", "harvester", "artillery", "infantry", "engineer", "rocket_soldier", "buggy")
            row_keys = [(faction, unit) for faction in ("gdi", "nod") for unit in unit_order]
            label_width = 112; header_height = 24; cell = TILE * scale
            sheet = Image.new("RGBA", (label_width + len(order)*cell, header_height + len(row_keys)*cell), (28, 28, 30, 255))
            draw = ImageDraw.Draw(sheet)
            for col, condition in enumerate(order):
                draw.text((label_width + col*cell + 5, 5), condition.upper(), fill=(235, 235, 235, 255))
            for row_index, (faction, unit) in enumerate(row_keys):
                y = header_height + row_index*cell
                draw.text((5, y + cell//2 - 4), f"{faction.upper()} / {unit}", fill=(235, 235, 235, 255))
                for col, condition in enumerate(order):
                    item = lookup.get((faction, unit, 2, condition))
                    x = label_width + col*cell
                    if item is not None:
                        with Image.open(stage/item["file"]) as source:
                            tile = source.convert("RGBA").resize((cell, cell), Image.Resampling.NEAREST)
                        sheet.alpha_composite(tile, (x, y))
                    draw.rectangle((x, y, x+cell-1, y+cell-1), outline=(80, 80, 84, 255), width=1)
            sheet.convert("RGB").save(preview_dir/filename,optimize=False)
        terrain_samples=[x for x in sprites if x["category"]=="terrain_transition"]
        terrain_samples=terrain_samples[:min(96,len(terrain_samples))]
        save_contact_sheet(terrain_samples,"terrain_transitions_review.png",cols=12,scale=3)
        # The legacy overview is intentionally compact; additionally emit a review
        # sheet for every directed terrain pair so all 940 masks are inspectable.
        pair_preview_dir = preview_dir / "terrain_pairs"
        pair_preview_dir.mkdir(parents=True, exist_ok=True)
        terrain_pair_preview_paths = []
        for terrain in ("grass", "dirt", "rock", "water", "sand"):
            for other in ("grass", "dirt", "rock", "water", "sand"):
                if terrain == other:
                    continue
                pair_items = [x for x in sprites if x["category"] == "terrain_transition"
                              and x.get("terrain") == terrain and x.get("alternate_terrain") == other]
                pair_name = f"{terrain}_to_{other}.png"
                save_contact_sheet(pair_items, str(Path("terrain_pairs") / pair_name), cols=7, scale=3)
                terrain_pair_preview_paths.append(f"previews/terrain_pairs/{pair_name}")
        road_samples=[x for x in sprites if x["category"]=="road_connection"]
        save_contact_sheet(road_samples,"road_masks_review.png",cols=4,scale=6)
        shore_samples=[x for x in sprites if x["category"]=="shoreline"]
        save_contact_sheet(shore_samples,"shoreline_masks_review.png",cols=4,scale=6)
        unit_samples=[x for x in sprites if x["category"]=="unit_animation" and x.get("faction")=="gdi" and x.get("unit")=="tank" and x.get("state")=="move" and x.get("facing")==0]
        save_contact_sheet(unit_samples,"tank_animation_review.png",cols=4,scale=6)
        building_samples=[x for x in sprites if x["category"]=="building"]
        save_contact_sheet(building_samples,"building_faction_damage_review.png",cols=8,scale=4)
        unit_condition_samples=[x for x in sprites if x["category"]=="unit_condition"]
        save_unit_condition_review(unit_condition_samples,"unit_damage_state_review.png",scale=5)
        # Render a real 12x8 multi-tile test map from base terrain and connectivity masks.
        # It is a visual QA artifact, not a claim that every terrain transition is seam-free.
        base_lookup={x.get("terrain"):x for x in sprites if x["category"]=="terrain_base" and x.get("variant")==0}
        road_lookup={x.get("mask"):x for x in sprites if x["category"]=="road_connection"}
        shore_lookup={tuple(x.get("land_edges",[])):x for x in sprites if x["category"]=="shoreline"}
        test_map=Image.new("RGBA",(12*TILE,8*TILE),(0,0,0,255))
        for ty in range(8):
            for tx in range(12):
                terrain_name="water" if tx>=9 and ty>=2 and ty<=6 else ("dirt" if ty in (3,4) and tx<9 else "grass")
                base_item=base_lookup[terrain_name]
                with Image.open(stage/base_item["file"]) as src: tile=src.convert("RGBA")
                if terrain_name != "water" and (ty in (3,4) or tx in (4,5) and ty in (2,5)):
                    mask=0
                    if ty>0 and (ty-1 in (3,4) or (tx in (4,5) and ty-1 in (2,5))): mask|=1
                    if tx<11 and (ty in (3,4) or (tx+1 in (4,5) and ty in (2,5))): mask|=2
                    if ty<7 and (ty+1 in (3,4) or (tx in (4,5) and ty+1 in (2,5))): mask|=4
                    if tx>0 and (ty in (3,4) or (tx-1 in (4,5) and ty in (2,5))): mask|=8
                    with Image.open(stage/road_lookup[mask]["file"]) as road:
                        road_overlay=road.convert("RGBA")
                        # The road mask sprite carries a dirt base for standalone use;
                        # isolate the actual road material here so a composed map does
                        # not accidentally replace surrounding grass with square dirt tiles.
                        pixels=road_overlay.load()
                        road_colors={PALETTE["road"],PALETTE["road_dark"],PALETTE["road_light"]}
                        for py in range(TILE):
                            for px in range(TILE):
                                if pixels[px,py][:3] not in road_colors:
                                    pixels[px,py]=(0,0,0,0)
                        tile.alpha_composite(road_overlay)
                if terrain_name=="water" and tx==8:
                    with Image.open(stage/shore_lookup[("E",)]["file"]) as shore: tile.alpha_composite(shore.convert("RGBA"))
                test_map.alpha_composite(tile,(tx*TILE,ty*TILE))
        test_map.convert("RGB").resize((12*TILE*3,8*TILE*3),Image.Resampling.NEAREST).save(preview_dir/"composed_map_continuity_review.png", optimize=False)
        # Animation uniqueness: every adjacent frame must differ in actual pixel data.
        animation_groups: dict[str, list[dict[str, Any]]] = {}
        for s in sprites:
            if s["category"] == "unit_animation": key = f"{s['faction']}/{s['unit']}/{s['state']}/facing_{s['facing']}"
            elif s["category"] == "effect_animation": key = s["effect"]
            else: continue
            animation_groups.setdefault(key, []).append(s)
        duplicate_frames = []
        for key, group in animation_groups.items():
            group.sort(key=lambda x: x["frame"])
            for a, b in zip(group, group[1:]):
                if a["pixel_sha256"] == b["pixel_sha256"]: duplicate_frames.append({"animation": key, "frames": [a["frame"], b["frame"]]})
        road_audit = audit_road_continuity(f"{seed}:road-continuity")
        audit = contrast_audit()
        audit["threshold"] = contrast_threshold
        for pair in audit["pairs"]: pair["pass"] = pair["ratio"] >= contrast_threshold
        audit["failures"] = [p for p in audit["pairs"] if p["ratio"] < contrast_threshold]
        report = {"generator_version": VERSION, "tile_size": TILE, "sprite_count": len(sprites), "atlas_count": len(atlas_records),
                  "animation_duplicate_frames": duplicate_frames, "contrast": audit,
                  "road_continuity": road_audit,
                  "terrain_adjacency_validation": {"directed_pairs_expected": 20,
                      "directed_pairs_found": len({(x.get("terrain"),x.get("alternate_terrain")) for x in sprites if x["category"]=="terrain_transition"}),
                      "masks_per_pair_expected": len(BLOB_MASKS),
                      "masks_per_pair_min": min((sum(1 for x in sprites if x["category"]=="terrain_transition" and x.get("terrain")==a and x.get("alternate_terrain")==b) for a in terrain_kinds for b in terrain_kinds if a!=b), default=0),
                      "pass": len({(x.get("terrain"),x.get("alternate_terrain")) for x in sprites if x["category"]=="terrain_transition"}) == 20 and all(sum(1 for x in sprites if x["category"]=="terrain_transition" and x.get("terrain")==a and x.get("alternate_terrain")==b) == len(BLOB_MASKS) for a in terrain_kinds for b in terrain_kinds if a!=b)},
                  "pass": not duplicate_frames and not audit["failures"] and road_audit["pass"] and len({(x.get("terrain"),x.get("alternate_terrain")) for x in sprites if x["category"]=="terrain_transition"}) == 20,
                  "warnings": ["Procedural C&C-inspired placeholders; not original Westwood artwork.",
                               "Contact sheets are provided under previews/ for visual QA.",
                               "Visual review in the target engine is still required for artistic quality and gameplay readability."],
                  "visual_qa_previews": ["previews/terrain_transitions_review.png", "previews/road_masks_review.png", "previews/shoreline_masks_review.png", "previews/tank_animation_review.png", "previews/building_faction_damage_review.png", "previews/unit_damage_state_review.png", "previews/composed_map_continuity_review.png", *terrain_pair_preview_paths],
                  "terrain_pair_preview_count": len(terrain_pair_preview_paths),
                  "terrain_pair_masks_previewed": sum(1 for x in sprites if x["category"] == "terrain_transition"),
                  "terrain_blob_masks_per_family": len(BLOB_MASKS), "terrain_blob_mask_count": len(BLOB_MASKS)*20,
                  "road_masks": 16, "shoreline_masks": 16, "independent_turret_frames": 144,
                  "unit_state_definitions": {k:list(v) for k,v in UNIT_STATES.items()},
                  "turret_overlay_count": sum(x["category"] == "turret_animation" for x in sprites),
                  "unit_condition_count": sum(x["category"] == "unit_condition" for x in sprites),
                  "unit_condition_coverage": {condition: sum(x["category"] == "unit_condition" and x.get("condition") == condition for x in sprites) for condition in ("intact", "damaged", "critical", "destroyed")},
                  "contrast_failures": len(audit["failures"])}
        manifest = {"schema_version": "1.2.0", "generator_version": VERSION, "seed": seed, "tile_size": TILE,
                    "mask_convention": {"bit_order": DIRECTION_NAMES, "blob_bit_order": BLOB_DIR_NAMES,
                                        "roads": "set bit = connected road edge", "shoreline": "set bit = land edge",
                                        "terrain_transition": "set bit = same-terrain neighbor; 47 valid blob masks per terrain"},
                    "sprites": sprites, "atlases": atlas_records, "contrast_report": "validation_report.json"}
        (stage / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
        (stage / "validation_report.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        (stage / "README.md").write_text(f"""# CNC32 generated asset pack\n\nGenerator: {VERSION}\nTile size: {TILE}x{TILE}\nSprites: {len(sprites)}\n\n## Mask conventions\n- Terrain transition: canonical 47-mask eight-neighbour blob set; bit order N, NE, E, SE, S, SW, W, NW; diagonals are constrained by adjacent cardinal bits.\n- Road: a set N/E/S/W bit means the road connects through that edge.\n- Shoreline: a set N/E/S/W bit means land/sand is on that edge.\n\n## Animation\nUnit poses are drawn direction-aware for eight facings and each unit's supported states (idle, move, turn, attack, harvest, unload, deploy, repair, death). Frame timings, loop flags, pivots and state names are in `manifest.json`. Inspect `validation_report.json` for duplicate-frame and palette contrast failures.\n\n## Files\n- `sprites/`: individual RGBA PNGs\n- `atlases/`: category atlases, each cell 32x32\n- `manifest.json`: sprite paths, hashes, frame metadata, pivots, and atlas rectangles\n- `validation_report.json`: contrast audit and animation validation\n\nThis is a procedural C&C-inspired asset pack, not original game art. Version 5.3 adds independently aimable turret overlays, three-step firing recoil, road-edge continuity checks, directed terrain-pair/mask coverage audits, and an eight-stage destruction effect (blast, debris, smoke dissipation). Version 5.5 adds full-frame intact/damaged/critical/destroyed conditions for every faction, unit type and facing, including a dedicated condition review sheet. Run `qa_v55.py` for symmetric multi-tile road seam checks, damage-stage differentiation and adjacent-frame motion metrics. The engine test harness is under `engine_tests/godot/`; it must be run in Godot and is not represented as passed by offline QA. Inspect `previews/` before integrating; a visual preview is a QA aid, not proof every possible runtime scenario is seamless.\n""", encoding="utf-8")
        # Strict pack validation before promotion: source PNGs, atlas hashes, every
        # atlas cell's exact pixel content, path safety, IDs, and required coverage.
        ids = [item["id"] for item in sprites]
        if len(ids) != len(set(ids)):
            raise ValueError("Duplicate sprite IDs detected")
        by_atlas = {record["file"]: record for record in atlas_records}
        atlas_images: dict[str, Image.Image] = {}
        try:
            for item in sprites:
                rel = Path(item["file"])
                if rel.is_absolute() or ".." in rel.parts:
                    raise ValueError(f"Unsafe sprite path: {item['file']}")
                p = stage / rel
                with Image.open(p) as im:
                    rgba = im.convert("RGBA")
                    if rgba.size != (TILE, TILE):
                        raise ValueError(f"Invalid dimensions: {item['id']} {rgba.size}")
                    if image_hash(rgba) != item["pixel_sha256"]:
                        raise ValueError(f"Pixel hash mismatch: {item['id']}")
                if hashlib.sha256(p.read_bytes()).hexdigest() != item["sha256_file"]:
                    raise ValueError(f"File hash mismatch: {item['id']}")
                atlas_rel = f"atlases/{item['atlas']}"
                if atlas_rel not in by_atlas:
                    raise ValueError(f"Missing atlas record for {item['id']}: {atlas_rel}")
                if atlas_rel not in atlas_images:
                    atlas_path = stage / atlas_rel
                    if hashlib.sha256(atlas_path.read_bytes()).hexdigest() != by_atlas[atlas_rel]["sha256_file"]:
                        raise ValueError(f"Atlas file hash mismatch: {atlas_rel}")
                    with Image.open(atlas_path) as opened:
                        atlas_images[atlas_rel] = opened.convert("RGBA")
                x, y, w, h = item["atlas_rect"]
                atlas = atlas_images[atlas_rel]
                if (w, h) != (TILE, TILE) or x < 0 or y < 0 or x + w > atlas.width or y + h > atlas.height:
                    raise ValueError(f"Invalid atlas rectangle: {item['id']} -> {item['atlas_rect']}")
                with Image.open(p) as opened:
                    if atlas.crop((x, y, x + w, y + h)).tobytes() != opened.convert("RGBA").tobytes():
                        raise ValueError(f"Atlas cell differs from source sprite: {item['id']}")
        finally:
            for atlas in atlas_images.values():
                atlas.close()
        required = {"terrain_transition": 940, "road_connection": 16, "shoreline": 16,
                    "cliff_connection": 16, "wall_connection": 16, "bridge": 6}
        category_counts = {category: sum(item["category"] == category for item in sprites)
                           for category in {item["category"] for item in sprites}}
        for category, expected in required.items():
            if category_counts.get(category, 0) != expected:
                raise ValueError(f"Coverage failure for {category}: expected {expected}, got {category_counts.get(category, 0)}")
        if len(atlas_records) != len(category_counts):
            raise ValueError("Atlas/category coverage mismatch")
        expected_unit_conditions = 2 * len(UNIT_STATES) * 8 * 4
        if category_counts.get("unit_condition", 0) != expected_unit_conditions:
            raise ValueError(f"Unit condition coverage failure: expected {expected_unit_conditions}, got {category_counts.get('unit_condition', 0)}")
        report["integrity_validation"] = {
            "unique_sprite_ids": len(ids), "source_files_verified": len(sprites),
            "atlas_files_verified": len(atlas_records), "atlas_cells_pixel_verified": len(sprites),
            "required_category_counts": required, "category_counts": category_counts,
            "pass": True,
        }
        (stage / "validation_report.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        backup = out.with_name(out.name + ".backup")
        if backup.exists():
            if backup.is_dir(): shutil.rmtree(backup)
            else: backup.unlink()
        if out.exists(): os.replace(out, backup)
        try: os.replace(stage, out)
        except Exception:
            if backup.exists() and not out.exists(): os.replace(backup, out)
            raise
        if backup.exists():
            if backup.is_dir(): shutil.rmtree(backup)
            else: backup.unlink()
        return report
    except Exception:
        if stage.exists(): shutil.rmtree(stage, ignore_errors=True)
        raise

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, default=Path("./cnc32_assets"), help="output directory")
    ap.add_argument("--seed", default="cnc32", help="stable generation seed")
    ap.add_argument("--force", action="store_true", help="replace existing output directory")
    ap.add_argument("--dry-run", action="store_true", help="print planned asset counts without writing files")
    ap.add_argument("--contrast-threshold", type=float, default=1.5, help="minimum contrast ratio for semantic palette pairs")
    args = ap.parse_args()
    if args.contrast_threshold < 1.0: ap.error("--contrast-threshold must be >= 1.0")
    if args.dry_run:
        planned_unit_frames = sum(2*8*STATE_FRAMES[state] for states in UNIT_STATES.values() for state in states)
        planned_conditions = 2 * len(UNIT_STATES) * 8 * 4
        planned_other = 20 + 24 + 48 + 16 + 16 + 16 + 6 + 16 + 23 + 144 + 16
        planned_sprites = 20*len(BLOB_MASKS) + planned_unit_frames + planned_conditions + planned_other
        print(json.dumps({"generator_version": VERSION, "tile_size": TILE, "planned_sprite_count": planned_sprites, "planned_atlas_count": 14,
                          "unit_condition_sprites": planned_conditions, "terrain_transitions": 20*len(BLOB_MASKS),
                          "valid_blob_masks_per_terrain": len(BLOB_MASKS), "road_masks": 16, "shoreline_masks": 16, "independent_turret_frames": 144,
                          "unit_state_definitions": {k:list(v) for k,v in UNIT_STATES.items()},
                          "unit_frames": sum(2*8*STATE_FRAMES[state] for states in UNIT_STATES.values() for state in states),
                          "independent_turret_frames": 2*3*8*3,
                          "effect_frames": 23, "destruction_frames": 8, "output": str(args.out)}, indent=2)); return 0
    try:
        report = build(args.out, force=args.force, seed=args.seed, contrast_threshold=args.contrast_threshold)
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr); return 2
    print(json.dumps({"output": str(args.out), **report}, indent=2))
    return 0 if report["pass"] else 1

if __name__ == "__main__": raise SystemExit(main())
