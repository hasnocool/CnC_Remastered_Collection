"""Original template art: enumerate, decode, measure.

MEG member layout (verified against the shipped archives 2026-10-26):

- Path: ``DATA\\ART\\TEXTURES\\SRGB\\<GAME_ROOT>\\TERRAIN\\<THEATER>\\<FAM>\\``
  ``<TPL>.<EXT>-<icon>-<frame>.DDS`` with the family dir spelled ``<TPL>``
  in TD (``SH1``) and ``<TPL>.<EXT>`` in RA (``SH01.SNO``).
- ``icon`` is the flat cell index inside the template's grid (4 digits);
  ``frame`` is the animation frame (2 digits) — frame ``00`` is the
  canonical still the map's ``TIcon`` refers to. Some members are neither
  (ZIP/BMP padding slots shipped inside the archive); they decode-fail and
  are skipped.
- Flat members ``<TPL>.<EXT>-<icon>.DDS`` (no frame) are single-frame
  families, e.g. ``B1``.
- RA zero-pads two-digit families in map data (``SH1`` -> ``SH01``); this
  module exposes archive spelling.

STATUS: experimental. The MEG member naming could not be tied to template
semantics without the closed GlyphX client (the C# editor's TILESETS.XML
base file is not shipped in this install), so the working pipeline decodes
DOS art instead (``dos.py``). This module remains the record of the MEG
investigation.
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path

from PIL import Image

from .dds import decode_shipped_dds
from .meg import MegArchive

MEMBER_ROOT = {"td": "TIBERIAN_DAWN", "ra": "RED_ALERT"}

THEATER_DIR_EXT = {
    "temperate": ("TEMPERATE", "TEM"),
    "winter": ("WINTER", "WIN"),
    "desert": ("DESERT", "DES"),
    "snow": ("SNOW", "SNO"),
    "interior": ("INTERIOR", "INT"),
    "jungle": ("JUNGLE", "JUN"),
}

_MEMBER = re.compile(
    r"^(?P<base>[^\\]+?\.[A-Z]{3})-(?P<icon>\d{4})(?:-(?P<frame>\d{2,4}))?\.DDS$"
)


@dataclass
class OriginalArt:
    """Per-theater handle over one game's texture MEG."""

    archive: MegArchive
    game: str
    theater: str

    _icons: dict[str, dict[int, list[str]]] = field(default_factory=dict, init=False)
    _rgba_cache: dict[tuple[str, int], Image.Image] = field(default_factory=dict, init=False)

    @classmethod
    def open(cls, install_root: Path, game: str, theater: str) -> "OriginalArt":
        archive = MegArchive.open(install_root, game)
        return cls(archive=archive, game=game, theater=theater)

    # -- enumeration ------------------------------------------------------

    def template_names(self) -> list[str]:
        """All template family names this theater ships, archive spelling."""
        if not self._icons:
            self._enumerate()
        return sorted(self._icons)

    def icons(self, template: str) -> list[int]:
        if not self._icons:
            self._enumerate()
        return sorted(self._icons.get(template.upper(), {}))

    def _enumerate(self) -> None:
        dir_ext, file_ext = THEATER_DIR_EXT[self.theater]
        marker = f"TERRAIN\\{dir_ext}\\"
        per_family: dict[str, dict[int, list[str]]] = {}
        for name in self.archive.members:
            idx = name.find(marker)
            if idx < 0:
                continue
            rel = name[idx + len(marker) :]
            file_part = rel.split("\\", 1)[1] if "\\" in rel else rel
            m = _MEMBER.match(file_part)
            if not m:
                continue
            base = m["base"]
            if not base.endswith("." + file_ext):
                continue
            template = base[: -(len(file_ext) + 1)]
            icon = int(m["icon"])
            frame = int(m["frame"]) if m["frame"] is not None else 0
            fam = per_family.setdefault(template, {})
            fam.setdefault(icon, []).append((frame, name))
        # Order candidate names by frame so frame 00 wins.
        self._icons = {
            t: {i: [n for _f, n in sorted(entries)] for i, entries in fam.items()}
            for t, fam in per_family.items()
        }

    # -- decoding ---------------------------------------------------------

    def icon_rgba(self, template: str, icon: int) -> Image.Image:
        """RGBA Pillow image at frame 0 (128x128 for terrain families)."""
        key = (template.upper(), icon)
        hit = self._rgba_cache.get(key)
        if hit is not None:
            return hit
        if not self._icons:
            self._enumerate()
        names = self._icons.get(template.upper(), {}).get(icon, [])
        for name in names:
            try:
                rgba, w, h = decode_shipped_dds(self.archive.member_bytes(name))
            except Exception:
                continue  # ZIP/BMP padding slots shipped inside the archive
            if w != h:
                continue  # non-square members are not cell icons
            img = Image.fromarray(rgba, "RGBA")
            self._rgba_cache[key] = img
            return img
        raise KeyError(f"{template} icon {icon} not decodable in {self.archive.path.name}")
