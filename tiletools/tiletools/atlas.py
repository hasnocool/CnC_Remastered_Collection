"""Visual atlas of decoded DOS tileset for human inspection."""

from __future__ import annotations

from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

from .dos import DosTheater


def _classify_edge_run(run: np.ndarray) -> str:
    """Classify an edge run: 'w' water, 'l' land, 'm' mixed, '.' transparent."""
    if len(run) == 0:
        return "."
    water = int((run == 1).sum())
    land = int((run == 2).sum())
    trans = int((run == 0).sum())
    total = water + land + trans
    if total == 0:
        return "."
    if trans / total > 0.98:
        return "."
    if water / total > 0.98:
        return "w"
    if land / total > 0.98:
        return "l"
    return "m"


def _fit_legend(classes: np.ndarray) -> str:
    """Return 4-letter fit legend NESW from class array [h, w]."""
    h, w = classes.shape
    # North: top row
    n = _classify_edge_run(classes[0, :])
    # South: bottom row
    s = _classify_edge_run(classes[h - 1, :])
    # West: left column
    w_run = _classify_edge_run(classes[:, 0])
    # East: right column
    e = _classify_edge_run(classes[:, w - 1])
    return f"{n}{e}{s}{w}"


def render_atlas(
    theater: DosTheater,
    templates: set[str],
    out_path: Path,
    scale: int = 4,
) -> Path:
    """Render a visual atlas of all decoded templates and icons.

    Args:
        theater: Decoded DOS theater with frames and rgba.
        templates: Set of template names to include (from CDATA).
        out_path: Output PNG path.
        scale: Upscale factor for icons (default 4).

    Returns:
        The output path.
    """
    classes = theater.index_classes()
    template_names = sorted(
        [t for t in theater.frames.keys() if t in templates and theater.frames[t].shape[0] > 0]
    )

    if not template_names:
        raise ValueError("No decodable templates found")

    # Layout parameters
    icon_w = theater.frames[template_names[0]].shape[2]
    icon_h = theater.frames[template_names[0]].shape[1]
    cell_w = icon_w * scale
    cell_h = icon_h * scale
    label_h = 18  # space for template name
    legend_h = 14  # space for fit legend
    gap = 1  # 1px grid between icons
    max_icons_per_row = 40  # wrap to keep width ~4000px

    # Calculate layout
    total_icons = sum(theater.frames[t].shape[0] for t in template_names)
    icons_per_row = min(max_icons_per_row, max(theater.frames[t].shape[0] for t in template_names))
    icons_per_row = max(1, icons_per_row)

    # Build rows: each template is a row of icons
    rows_data = []  # list of (template_name, list of (icon_idx, icon_rgba, fit_legend))
    for tpl in template_names:
        frames = theater.frames[tpl]
        cls = classes[tpl]
        n_icons = frames.shape[0]
        if n_icons == 0:
            continue
        row_icons = []
        for i in range(n_icons):
            rgba = theater.rgba[tpl][i]  # [h, w, 4]
            fit = _fit_legend(cls[i])
            row_icons.append((i, rgba, fit))
        rows_data.append((tpl, row_icons))

    # Calculate total grid dimensions with wrapping
    # We'll lay out in blocks: each block is up to max_icons_per_row icons wide
    block_rows = []
    current_block = []
    current_width = 0

    for tpl, icons in rows_data:
        for icon_idx, rgba, fit in icons:
            if current_width + cell_w + gap > max_icons_per_row * (cell_w + gap) and current_block:
                block_rows.append(current_block)
                current_block = []
                current_width = 0
            current_block.append((tpl, icon_idx, rgba, fit))
            current_width += cell_w + gap
        # Add a small separator between templates within a block
        current_width += gap * 2

    if current_block:
        block_rows.append(current_block)

    # Actually, let's use a simpler layout: fixed columns per row, templates stacked
    # Each template gets its own row(s), wrapping at max_icons_per_row
    max_cols = max_icons_per_row
    cell_w_gap = cell_w + gap
    cell_h_total = cell_h + label_h + legend_h + gap

    # Count total cells needed
    all_cells = []
    for tpl, icons in rows_data:
        for icon_idx, rgba, fit in icons:
            all_cells.append((tpl, icon_idx, rgba, fit))

    n_cells = len(all_cells)
    n_cols = min(max_cols, n_cells)
    n_rows = (n_cells + n_cols - 1) // n_cols

    # Canvas size
    canvas_w = n_cols * cell_w + (n_cols + 1) * gap
    canvas_h = n_rows * cell_h_total + (n_rows + 1) * gap

    # Create canvas
    canvas = Image.new("RGBA", (canvas_w, canvas_h), (20, 20, 30, 255))
    draw = ImageDraw.Draw(canvas)

    # Try to load a font
    try:
        font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 10)
        font_small = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 8)
    except Exception:
        font = ImageFont.load_default()
        font_small = ImageFont.load_default()

    # Draw grid and icons
    for idx, (tpl, icon_idx, rgba, fit) in enumerate(all_cells):
        row = idx // n_cols
        col = idx % n_cols

        x = gap + col * cell_w_gap
        y = gap + row * cell_h_total

        # Draw template name label (only on first icon of each template)
        # Find if this is first icon of template
        is_first = True
        if idx > 0:
            prev_tpl = all_cells[idx - 1][0]
            if prev_tpl == tpl:
                is_first = False

        if is_first:
            # Draw template name above the icon
            text_x = x + cell_w // 2
            text_y = y + label_h // 2
            bbox = draw.textbbox((0, 0), tpl, font=font)
            text_w = bbox[2] - bbox[0]
            draw.text((text_x - text_w // 2, text_y - (bbox[3] - bbox[1]) // 2), tpl, fill=(200, 200, 220), font=font)

        # Draw icon
        icon_y = y + label_h
        pil_icon = Image.fromarray(rgba, "RGBA")
        pil_icon = pil_icon.resize((cell_w, cell_h), Image.NEAREST)
        canvas.paste(pil_icon, (x, icon_y), pil_icon)

        # Draw fit legend under icon
        legend_y = icon_y + cell_h + 2
        bbox = draw.textbbox((0, 0), fit, font=font_small)
        text_w = bbox[2] - bbox[0]
        draw.text((x + cell_w // 2 - text_w // 2, legend_y), fit, fill=(180, 180, 200), font=font_small)

        # Draw icon index label
        idx_text = f"{icon_idx}"
        bbox = draw.textbbox((0, 0), idx_text, font=font_small)
        text_w = bbox[2] - bbox[0]
    # Draw white 1px grid lines AFTER icons so they appear on top
    # Vertical lines (at left edge of each cell + rightmost edge)
    for col in range(n_cols + 1):
        x = gap + col * cell_w_gap
        if x < canvas_w:
            for y in range(canvas_h):
                canvas.putpixel((x, y), (255, 255, 255, 255))
    # Horizontal lines (at top edge of each row + bottommost edge)
    for row in range(n_rows + 1):
        y = gap + row * cell_h_total
        if y < canvas_h:
            for x in range(canvas_w):
                canvas.putpixel((x, y), (255, 255, 255, 255))
    # Save

    # Save
    out_path.parent.mkdir(parents=True, exist_ok=True)
    canvas.convert("RGB").save(out_path, format="PNG")

    print(f"Atlas: {len(template_names)} templates, {n_cells} icons -> {out_path}")
    print(f"  Canvas: {canvas_w}x{canvas_h}, scale={scale}x")
    return out_path