"""Parse the original template table from `original/TIBERIANDAWN/CDATA.CPP`.

The TD tileset metadata is compiled into the game: one
`TemplateTypeClass const Name(TEMPLATE_X, THEATERF_* mask, "INI name",
TXT_*, LAND_*, width, height, ALT_LAND, alt_icons|NULL);` ctor per template
(216 entries). This module parses them straight from the source so the
generator's metadata is the original's, not a transcription.

`original/REDALERT/CDATA.CPP` follows the same pattern for RA.
"""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
CDATA_TD = REPO_ROOT / "original" / "TIBERIANDAWN" / "CDATA.CPP"
CDATA_RA = REPO_ROOT / "original" / "REDALERT" / "CDATA.CPP"


@dataclass
class TemplateDef:
    enum: str
    ini_name: str
    theater_mask: int
    land: str
    width: int
    height: int
    alt_land: str
    alt_icons: tuple[int, ...]  # empty = none

    @property
    def icons(self) -> int:
        return self.width * self.height

    def icon_land(self, icon: int) -> str:
        return self.alt_land if icon in self.alt_icons else self.land

    def supports_theater(self, bit: int) -> bool:
        return bool(self.theater_mask & bit)


_CTOR = re.compile(
    r"TemplateTypeClass\s+(?:const\s+)?(\w+)\s*\(\s*"
    r"(TEMPLATE_\w+)\s*,\s*"
    r"((?:THEATERF_\w+\|?)+)\s*,\s*"
    r'"([^"]+)"\s*,\s*'
    r"(TXT_\w+|NULL)\s*,\s*"
    r"LAND_(\w+)\s*,\s*"
    r"(\d+)\s*,\s*(\d+)\s*,\s*"
    r"LAND_(\w+)\s*,\s*"
    r"(.*?)\)\s*;",
    re.S,
)

_SLOPE_ARRAY = re.compile(
    r"static\s+char\s+const\s+(_slope\w+)\[\]\s*=\s*\{([^}]*)\};"
)


def parse_cdata(path: Path) -> dict[str, TemplateDef]:
    text = path.read_text(errors="replace")
    # `_slope<maskbits>` arrays: name encodes a land bitmask, body lists the
    # icon indices that carry the alternate (beach/clear) land type.
    slopes: dict[str, tuple[int, ...]] = {}
    for m in _SLOPE_ARRAY.finditer(text):
        nums = [int(n) for n in re.findall(r"-?\d+", m.group(2))]
        slopes[m.group(1)] = tuple(n for n in nums if n >= 0)

    out: dict[str, TemplateDef] = {}
    for m in _CTOR.finditer(text):
        mask = 0
        for t in re.findall(r"THEATERF_(\w+)", m.group(3)):
            mask |= 1 << THEATER_BIT[t]
        icons = m.group(10).strip()
        if icons.startswith("NULL") or icons in ("", "0"):
            alt: tuple[int, ...] = ()
        else:
            ref = re.search(r"(_slope\w+)", icons)
            if ref and ref.group(1) in slopes:
                alt = slopes[ref.group(1)]
            else:
                nums = re.findall(r"-?\d+", icons)
                alt = tuple(int(n) for n in nums if int(n) >= 0)
        name = m.group(4)
        out[name] = TemplateDef(
            enum=m.group(2),
            ini_name=name,
            theater_mask=mask,
            land=m.group(6),
            width=int(m.group(7)),
            height=int(m.group(8)),
            alt_land=m.group(9),
            alt_icons=alt,
        )
    return out


THEATER_BIT = {"WINTER": 0, "DESERT": 1, "TEMPERATE": 2, "JUNGLE": 3, "SNOW": 4, "INTERIOR": 5}


def td_templates() -> dict[str, TemplateDef]:
    return parse_cdata(CDATA_TD)


def ra_templates() -> dict[str, TemplateDef]:
    return parse_cdata(CDATA_RA)


if __name__ == "__main__":
    t = td_templates()
    print(len(t), "templates")
    for name in ("CLEAR1", "W1", "W2", "SH11", "D01"):
        d = t.get(name)
        print(name, d)
