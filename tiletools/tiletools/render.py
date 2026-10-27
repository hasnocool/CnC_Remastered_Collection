"""PNG compositing from decoded original art.

Each map cell is its own 128x128 icon in the remaster's texture archive. The
compositor draws a CLEAR1 base under every cell (the engine's "no template"
fallback) and then alpha-composites the picked template icon on top, which
matches the engine's semantics for overlay/sliver tiles while remaining
visually identical for fully opaque picks.
"""

from __future__ import annotations

from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

from .join import Material
from .matcher import CellPick

FALLBACK_TEMPLATE = "CLEAR1"  # the engine's "no template" ground


def _to_pil(tile) -> Image.Image:
    if isinstance(tile, Image.Image):
        return tile
    return Image.fromarray(np.asarray(tile), "RGBA")


class Renderer:
    def __init__(self, art):
        self.art = art
        self._base: Image.Image | None = None
        self._cell: int | None = None

    def base_tile(self) -> Image.Image:
        if self._base is None:
            self._base = _to_pil(self.art.icon_rgba(FALLBACK_TEMPLATE, 0))
            self._cell = self._base.width
        return self._base

    def cell(self) -> int:
        if self._cell is None:
            self.base_tile()
        return self._cell or 24

    def compose(self, picks: np.ndarray, scale: float = 1.0) -> Image.Image:
        height, width = picks.shape[:2]
        cell = self.cell()
        canvas = Image.new("RGBA", (width * cell, height * cell))
        base = self.base_tile()
        for y in range(height):
            for x in range(width):
                tile = _to_pil(self.art.icon_rgba(picks[y, x].template, picks[y, x].icon))
                px = x * cell
                py = y * cell
                canvas.alpha_composite(base, (px, py))
                canvas.alpha_composite(tile, (px, py))
        if scale != 1.0:
            out = (max(1, round(canvas.width * scale)), max(1, round(canvas.height * scale)))
            canvas = canvas.resize(out, Image.LANCZOS)
        return canvas


def to_preview(img: Image.Image, max_side: int = 1600) -> Image.Image:
    factor = min(1.0, max_side / max(img.size))
    if factor < 1.0:
        out = (max(1, round(img.width * factor)), max(1, round(img.height * factor)))
        img = img.resize(out, Image.LANCZOS)
    return img


def label(img: Image.Image, text: str) -> Image.Image:
    banner = 34
    out = Image.new("RGBA", (img.width, img.height + banner), (12, 12, 12, 255))
    out.paste(img.convert("RGBA"), (0, banner))
    draw = ImageDraw.Draw(out)
    try:
        font = ImageFont.load_default(20)
    except TypeError:  # Pillow < 9.2 has no size arg
        font = ImageFont.load_default()
    draw.text((8, 6), text, fill=(240, 240, 240, 255), font=font)
    return out


def comparison_sheet(
    panels: list[tuple[str, Image.Image]], columns: int = 2, gap: int = 24
) -> Image.Image:
    """Grid of labelled panels, normalized to the first panel's size."""
    first = panels[0][1]
    size = (first.width, first.height)
    banner = 34
    rows = (len(panels) + columns - 1) // columns
    sheet = Image.new(
        "RGBA",
        (size[0] * columns + gap * (columns - 1), (size[1] + banner) * rows + gap * (rows - 1)),
        (60, 60, 80, 255),
    )
    for i, (name, img) in enumerate(panels):
        x = (i % columns) * (size[0] + gap)
        y = (i // columns) * (size[1] + banner + gap)
        sheet.paste(label(img.resize(size, Image.LANCZOS), name), (x, y))
    return sheet


def save(img: Image.Image, path: Path) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    img.convert("RGB").save(path, format="PNG")
    return path
