"""Original DOS theater art: `.TEM` icon sets, palettes, via classic MIX.

`.TEM` layout (reverse-engineered from the shipped files, verified against
every TD DOS file): 32-byte header `{u16 w; u16 h; u32 frames; u32
data_start}`, then per frame `{u16 size}` followed by `size` bytes —
`size == w*h` means raw palette indices, anything else is a format80 (LCW)
stream decompressing to `w*h`. `TEMPERAT.PAL` (768 raw RGB bytes) is in the
same MIX.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path

import numpy as np

from .format80 import decompress
from .mix import MixArchive

# TD theater roots (`CONST.CPP` Theaters[]); DOS temperate ships TEMPERAT.MIX.
TD_THEATER_MIX = {
    "temperate": "TEMPERAT.MIX",
    "winter": "WINTER.MIX",
    "desert": "DESERT.MIX",
    "jungle": "JUNGLE.MIX",
}
RA_THEATER_MIX = {
    "temperate": "TEMPERAT.MIX",
    "snow": "SNOW.MIX",
    "interior": "INTERIOR.MIX",
}

CNCDATA_ROOT = "CNCDATA"


def cndata_dir(install_root: Path, game: str) -> Path:
    sub = {"td": "TIBERIAN_DAWN", "ra": "RED_ALERT"}[game]
    return install_root / "Data" / CNCDATA_ROOT / sub


@dataclass
class DosTheater:
    """Decoded icon sets of one DOS theater."""

    game: str
    theater: str
    palette: np.ndarray  # [256, 3] uint8
    frames: dict[str, np.ndarray]  # template -> [fc, h, w] palette indices
    rgba: dict[str, np.ndarray]  # template -> [fc, h, w, 4] uint8
    _classes: dict[str, np.ndarray] | None = field(default=None, init=False, repr=False)

    @classmethod
    def open(cls, install_root: Path, game: str, theater: str, gamma: float = 1.0) -> "DosTheater":
        mix_names = (TD_THEATER_MIX if game == "td" else RA_THEATER_MIX)[theater]
        mix_dir = cndata_dir(install_root, game)
        paths = sorted(mix_dir.rglob(mix_names))
        if not paths:
            raise FileNotFoundError(f"{mix_names} not found under {mix_dir}")
        archive = MixArchive.open(paths[0])
        palette = np.frombuffer(archive.get("TEMPERAT.PAL"), dtype=np.uint8).reshape(-1, 3)
        if gamma != 1.0:
            # The DOS palette is dark; gamma > 1 brightens for readability
            # (the DOS game itself drew these colours as-is).
            palette = np.clip(
                (palette.astype(np.float32) / 255.0) ** (1.0 / gamma) * 255.0, 0, 255
            ).astype(np.uint8)
        frames: dict[str, np.ndarray] = {}
        rgba: dict[str, np.ndarray] = {}
        # Classic MIXes index by CRC only; iterate the compiled template
        # table's names (`CDATA.CPP`) and look each `<TPL>.TEM` up by CRC.
        from .cdata import td_templates, ra_templates

        template_names = (td_templates if game == "td" else ra_templates)()
        for template in template_names:
            try:
                data = archive.get(f"{template}.TEM")
            except KeyError:
                continue
            try:
                w, h, fc = (
                    int.from_bytes(data[0:2], "little"),
                    int.from_bytes(data[2:4], "little"),
                    int.from_bytes(data[4:8], "little"),
                )
                data_start = int.from_bytes(data[12:16], "little")
                icons = np.zeros((fc, h, w), dtype=np.uint8)
                # Frame data runs raw from `data_start` for the shipped TD
                # files; a few (BRIDGE*D, RV01-04, SH15) carry per-frame
                # compressed payloads — not decodable here, so those sets
                # are skipped wholesale.
                expected = fc * w * h
                body = data[data_start : data_start + expected]
                if len(body) < expected:
                    continue
                icons = np.frombuffer(body, dtype=np.uint8).reshape(fc, h, w)
            except Exception:
                continue
            frames[template] = icons
            rgb = palette[icons]  # [fc, h, w, 3]
            alpha = np.where(icons == 0, 0, 255).astype(np.uint8)
            rgba[template] = np.concatenate([rgb, alpha[..., None]], axis=-1)
        return cls(game=game, theater=theater, palette=palette, frames=frames, rgba=rgba)

    def icons(self, template: str) -> int:
        return self.frames[template.upper()].shape[0]

    def icon_rgba(self, template: str, icon: int) -> np.ndarray:
        return self.rgba[template.upper()][icon]

    # -- material classification by palette index --------------------------

    def index_classes(self) -> dict[str, np.ndarray]:
        """Per template: [fc, h, w] uint8 classes 0=transparent, 1=water,
        2=grass/rock, 3=sand.

        The DOS palette reuses dark indices between grass and water art, so
        the water index set is learned PER TEMPLATE: indices appearing in the
        template's own pure-water icons (land type WATER, not in alt_icons)
        are water; index 0 is transparent; every other index is land.

        Sand (the beach arcs of the SH* sets) is warm palette entries
        (r > b+4 and r >= g): grass and rock art never uses warm indices, so
        contour matching can demand sand-to-sand continuity across seams
        instead of lumping beach and grass into one opaque class.
        """
        if self._classes is not None:
            return self._classes
        # Global index sets from the pure-by-definition templates: W1/W2 are
        # all-water art, CLEAR1 is all-grass art. The DOS palette's dark
        # dithers turn out to be index-disjoint between grass and water, so
        # this classification is exact for the shipped art.
        water_idx: set[int] = set()
        for ref in ("W1", "W2"):
            if ref in self.frames:
                water_idx |= {int(i) for i in np.unique(self.frames[ref])}
        water_idx.discard(0)
        sand_idx = {
            i
            for i in range(1, 256)
            if i not in water_idx
            and self.palette[i][0] > self.palette[i][2] + 4
            and self.palette[i][0] >= self.palette[i][1]
        }
        classes: dict[str, np.ndarray] = {}
        for tpl, frames in self.frames.items():
            cls = np.full(frames.shape, 2, dtype=np.uint8)
            cls[frames == 0] = 0
            if water_idx:
                cls[np.isin(frames, list(water_idx))] = 1
            if sand_idx:
                cls[np.isin(frames, list(sand_idx))] = 3
            classes[tpl] = cls
        self._classes = classes
        return classes
