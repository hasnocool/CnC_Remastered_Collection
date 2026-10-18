#!/usr/bin/env python3
"""AI sprite atlas for the Rust remaster.

Two steps, both driven by the local AI image-generator project at
/mnt/aux/projects/image-generator (override with --generator-root):

  generate  Render one flat-background concept image per sprite class with SDXL.
  assemble  Chroma-key, crop, despill, quantize and pack the renders into the
            same atlas.png + atlas.json format the game already loads.

Run with the image generator's virtualenv, e.g.:

  /mnt/aux/projects/image-generator/.venv/bin/python scripts/ai_sprites.py generate
  /mnt/aux/projects/image-generator/.venv/bin/python scripts/ai_sprites.py assemble

The generated atlas replaces the procedural one in assets/ (both are ignored by
git and can be rebuilt at any time).
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path
from typing import Iterable

import numpy as np
from PIL import Image

CELL = 64          # atlas cell size in pixels
COLS = 8           # cells per atlas row
PIXEL = 32         # logical pixel grid the render is reduced to
DEFAULT_GENERATOR = "/mnt/aux/projects/image-generator"

# key, subject prompt, category
CLASSES: list[tuple[str, str, str]] = [
    ("InfantryE1", "one single infantry soldier standing and aiming a rifle", "unit"),
    ("VehicleE1", "a small fast recon buggy with a mounted machine gun", "vehicle"),
    ("VehicleE2", "a main battle tank with a long cannon", "vehicle"),
    ("VehicleE4", "a mobile rocket artillery truck with a missile rack", "vehicle"),
    ("VehicleE6", "a heavy siege tank with twin cannons", "vehicle"),
    ("AircraftE1", "a transport helicopter with rotor blades", "aircraft"),
    ("AircraftE2", "an attack helicopter with rocket pods and rotor blades", "aircraft"),
    ("ConstructionYard", "a large military construction yard with a crane", "building"),
    ("PowerPlant", "a military power plant with cooling towers", "building"),
    ("Refinery", "a military ore refinery with storage tanks and pipes", "building"),
    ("Barracks", "a military barracks with a flag and entrance", "building"),
    ("WarFactory", "a military vehicle factory with a large roll-up door", "building"),
    ("Helipad", "a military helipad with an H marking and a control tower", "building"),
    ("Silo", "a small military storage silo with steel tanks", "building"),
    ("Radar", "a military radar station with a large dish antenna", "building"),
    ("MissileSilo", "a military missile silo with a raised launcher and blast doors", "building"),
]

SUFFIX = (
    ", video game RTS sprite, high three-quarter top-down view, single object, centered, "
    "flat solid magenta background, bold readable silhouette, thick black outline, "
    "limited palette, no text, no scenery"
)
NEGATIVE = (
    "character sheet, multiple objects, multiple views, front and back, two figures, text, "
    "watermark, scenery, landscape, background, cropped, blurry, people, photograph"
)


def prompt_for(desc: str, kind: str) -> str:
    if kind == "building":
        return "a military base structure, " + desc + SUFFIX
    return desc + SUFFIX


def chroma_key(rgb: np.ndarray) -> np.ndarray:
    """Alpha mask: background-coloured pixels connected to the image border."""
    h, w, _ = rgb.shape
    corners = np.array(
        [rgb[0, 0], rgb[0, w - 1], rgb[h - 1, 0], rgb[h - 1, w - 1]], dtype=np.int32
    )
    bg = np.median(corners, axis=0)
    near_bg = np.linalg.norm(rgb.astype(np.int32) - bg, axis=2) < 115

    visited = np.zeros((h, w), dtype=bool)
    stack = [(0, x) for x in range(w)] + [(h - 1, x) for x in range(w)]
    stack += [(y, 0) for y in range(h)] + [(y, w - 1) for y in range(h)]
    while stack:
        y, x = stack.pop()
        if visited[y, x] or not near_bg[y, x]:
            continue
        visited[y, x] = True
        if y > 0:
            stack.append((y - 1, x))
        if y + 1 < h:
            stack.append((y + 1, x))
        if x > 0:
            stack.append((y, x - 1))
        if x + 1 < w:
            stack.append((y, x + 1))
    return np.where(visited, 0, 255).astype(np.uint8)


def largest_component(mask: np.ndarray) -> tuple[int, int, int, int] | None:
    """Bounding box of the largest 4-connected opaque blob (row-run union-find)."""
    h, w = mask.shape
    parent: dict[int, int] = {}
    runs: list[tuple[int, int, int, int]] = []

    def find(a: int) -> int:
        while parent[a] != a:
            parent[a] = parent[parent[a]]
            a = parent[a]
        return a

    def union(a: int, b: int) -> None:
        ra, rb = find(a), find(b)
        if ra != rb:
            parent[rb] = ra

    prev: list[tuple[int, int, int, int]] = []
    for y in range(h):
        row = mask[y]
        cur: list[tuple[int, int, int, int]] = []
        x = 0
        while x < w:
            if row[x]:
                start = x
                while x < w and row[x]:
                    x += 1
                node = len(runs)
                parent[node] = node
                runs.append((y, start, x - 1, node))
                cur.append(runs[-1])
            else:
                x += 1
        for _, cs, ce, cn in cur:
            for _, ps, pe, pn in prev:
                if cs <= pe + 1 and ps <= ce + 1:
                    union(cn, pn)
        prev = cur

    areas: dict[int, int] = {}
    for _, s, e, node in runs:
        root = find(node)
        areas[root] = areas.get(root, 0) + (e - s + 1)
    if not areas:
        return None
    best = max(areas, key=areas.get)
    xs0 = min(r[1] for r in runs if find(r[3]) == best)
    xs1 = max(r[2] for r in runs if find(r[3]) == best)
    ys0 = min(r[0] for r in runs if find(r[3]) == best)
    ys1 = max(r[0] for r in runs if find(r[3]) == best)
    return xs0, ys0, xs1, ys1


def despill(rgb: np.ndarray, alpha: np.ndarray) -> np.ndarray:
    """Remove magenta fringes left by the chroma background."""
    out = rgb.astype(np.float32)
    r, g, b = out[..., 0], out[..., 1], out[..., 2]
    spill = np.maximum(0.0, (r + b) * 0.5 - g)
    spill = np.where(spill > 12.0, spill, 0.0)
    out[..., 0] = np.clip(r - spill * 0.85, 0, 255)
    out[..., 2] = np.clip(b - spill * 0.85, 0, 255)
    return out.astype(np.uint8)


def render_cell(raw_path: Path, pixel: int = PIXEL) -> Image.Image:
    """Turn one flat-background render into a square RGBA sprite cell."""
    rgb = np.asarray(Image.open(raw_path).convert("RGB"))
    alpha = chroma_key(rgb)
    box = largest_component(alpha > 0)
    if box is None:
        raise SystemExit(f"{raw_path}: no opaque pixels left after chroma keying")
    x0, y0, x1, y1 = box

    rgb = despill(rgb, alpha)
    sprite = Image.fromarray(np.dstack([rgb, alpha]), "RGBA").crop(
        (x0, y0, x1 + 1, y1 + 1)
    )
    cw, ch = sprite.size
    side = int(max(cw, ch) * 1.20)
    canvas = Image.new("RGBA", (side, side), (0, 0, 0, 0))
    canvas.paste(sprite, ((side - cw) // 2, (side - ch) // 2), sprite)

    # premultiplied downscale avoids dark fringes around the silhouette
    arr = np.asarray(canvas).astype(np.float32)
    a = arr[..., 3:4] / 255.0
    premultiplied = np.dstack([arr[..., :3] * a, arr[..., 3:4]]).astype(np.uint8)
    small = Image.fromarray(premultiplied, "RGBA").resize((pixel, pixel), Image.LANCZOS)
    s = np.asarray(small).astype(np.float32)
    a = s[..., 3:4] / 255.0
    colour = np.where(a > 0.0, s[..., :3] / np.maximum(a, 1e-3), 0.0)
    colour = np.clip(colour, 0, 255).astype(np.uint8)
    alpha_s = np.where(s[..., 3] > 90, 255, 0).astype(np.uint8)

    # quantize the visible colours only, so transparency does not skew the palette
    visible = alpha_s > 0
    if visible.any():
        flat = Image.fromarray(colour.copy(), "RGB")
        palette = flat.quantize(colors=28, method=Image.MEDIANCUT, dither=Image.Dither.NONE)
        colour = np.array(palette.convert("RGB"), copy=True)
        colour[~visible] = 0

    # gentle contrast/saturation lift for readability at game zoom
    mean = colour.reshape(-1, 3).mean(axis=0)
    colour = np.clip((colour.astype(np.float32) - mean) * 1.15 + mean, 0, 255).astype(np.uint8)
    colour[~visible] = 0
    out = np.dstack([colour, alpha_s])

    # 1px dark outline on the low-res grid, then nearest upscale to the cell size
    solid = alpha_s > 0
    dil = np.zeros_like(solid)
    dil[1:, :] |= solid[:-1, :]
    dil[:-1, :] |= solid[1:, :]
    dil[:, 1:] |= solid[:, :-1]
    dil[:, :-1] |= solid[:, 1:]
    out[dil & ~solid] = (14, 14, 16, 255)

    return Image.fromarray(out, "RGBA").resize((CELL, CELL), Image.NEAREST)


def cmd_generate(args: argparse.Namespace) -> int:
    gen_root = Path(args.generator_root).expanduser().resolve()
    if not (gen_root / "src" / "generator.py").exists():
        raise SystemExit(f"image generator not found at {gen_root}")
    out_dir = Path(args.raw_dir).expanduser().resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    sys.path.insert(0, str(gen_root))
    os.chdir(gen_root)
    from src.generator import SDXLGenerator, GenerationConfig  # noqa: E402

    todo = [
        (i, key, desc, kind)
        for i, (key, desc, kind) in enumerate(CLASSES)
        if args.force or not (out_dir / f"{key}.png").exists()
    ]
    if not todo:
        print(f"all {len(CLASSES)} renders already in {out_dir} (use --force to redo)")
        return 0

    generator = SDXLGenerator()
    generator.load_model()
    try:
        for i, key, desc, kind in todo:
            cfg = GenerationConfig(
                width=args.width,
                height=args.height,
                num_inference_steps=args.steps,
                guidance_scale=args.guidance,
                seed=args.seed + i,
                negative_prompt=NEGATIVE,
            )
            print(f"[{i + 1}/{len(CLASSES)}] {key}", flush=True)
            result = generator.generate(prompt_for(desc, kind), cfg)
            result.image.save(out_dir / f"{key}.png")
    finally:
        generator.unload()
    return 0


def cmd_assemble(args: argparse.Namespace) -> int:
    raw_dir = Path(args.raw_dir).expanduser().resolve()
    out_dir = Path(args.out).expanduser().resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    frames_dir = Path(args.frames_dir).expanduser().resolve()
    frames_dir.mkdir(parents=True, exist_ok=True)

    rows = (len(CLASSES) + COLS - 1) // COLS
    atlas = Image.new("RGBA", (COLS * CELL, rows * CELL), (0, 0, 0, 0))
    frames: dict[str, dict[str, int]] = {}
    for i, (key, _, _) in enumerate(CLASSES):
        raw = raw_dir / f"{key}.png"
        if not raw.exists():
            raise SystemExit(f"missing render {raw}; run the generate step first")
        col, row = i % COLS, i // COLS
        x, y = col * CELL, row * CELL
        cell = render_cell(raw)
        cell.save(frames_dir / f"{key}.png")
        atlas.paste(cell, (x, y))
        frames[key] = {"x": x, "y": y, "w": CELL, "h": CELL}
        print(f"  {key:16s} -> ({x:3d},{y:3d})")

    atlas.save(out_dir / "atlas.png")
    manifest = {"width": atlas.width, "height": atlas.height, "frames": frames}
    (out_dir / "atlas.json").write_text(json.dumps(manifest, indent=2) + "\n")
    preview = out_dir / "ai_preview.png"
    atlas.resize((atlas.width * 2, atlas.height * 2), Image.NEAREST).save(preview)
    print(
        f"wrote {out_dir / 'atlas.png'} ({atlas.width}x{atlas.height}), atlas.json, "
        f"{preview} and {len(CLASSES)} frames in {frames_dir}"
    )
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    gen = sub.add_parser("generate", help="render concept art with SDXL")
    gen.add_argument("--raw-dir", default="assets/ai_raw", help="raw render directory")
    gen.add_argument("--generator-root", default=DEFAULT_GENERATOR)
    gen.add_argument("--width", type=int, default=768)
    gen.add_argument("--height", type=int, default=768)
    gen.add_argument("--steps", type=int, default=24)
    gen.add_argument("--guidance", type=float, default=7.5)
    gen.add_argument("--seed", type=int, default=1000)
    gen.add_argument("--force", action="store_true")
    gen.set_defaults(func=cmd_generate)

    asm = sub.add_parser("assemble", help="pack renders into atlas.png + atlas.json")
    asm.add_argument("--raw-dir", default="assets/ai_raw", help="raw render directory")
    asm.add_argument("--out", default="assets")
    asm.add_argument(
        "--frames-dir",
        default="assets/sprites",
        help="also write each processed 64x64 cell here for hand editing",
    )
    asm.set_defaults(func=cmd_assemble)

    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
