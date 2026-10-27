"""Extract paired (INI, BIN) campaign scenarios from the SC-000/SC-001 mixes.

The remaster ships the TD campaign scenarios in ``SC-000.MIX``/``SC-001.MIX``
(plain-text INIs + raw 64x64 .BIN map images side by side in the same MIX).
The CRC/name table doesn't match the DOS filename hash, so pairing is done by
content: an INI's ``Scenario N for house H`` headline plus its ``[MAP]`` dims
must agree with the BIN's used-cell rectangle at that window.
"""

from __future__ import annotations

import re
from pathlib import Path

import numpy as np

from .mix import MixArchive

_HEADLINE = re.compile(r"^;\s*Scenario\s+(\d+)\s+control\s+for\s+house\s+(\w+)", re.M | re.I)
_HOUSE_CODE = {"GoodGuy": "G", "BadGuy": "B", "Neutral": "C", "Jurgen": "J"}


def _map_section(text: str) -> dict[str, int]:
    m = re.search(r"\[MAP\]([^\[]*)", text, re.I)
    if not m:
        return {}
    out = {}
    for key, value in re.findall(r"^(\w+)\s*=\s*([-\w]+)", m.group(1), re.M | re.I):
        if key.lower() in ("x", "y", "width", "height"):
            try:
                out[key.lower()] = int(value)
            except ValueError:
                pass
    return out


def extract_scenarios(base: Path, out_dir: Path, repeats: tuple[str, ...] = ("SC-000.MIX", "SC-001.MIX")) -> int:
    """Copy paired scenarios to ``out_dir`` as ``SC<code>NN<dir>.INI/BIN``."""
    out_dir.mkdir(parents=True, exist_ok=True)
    written = 0
    used_names: set[str] = set()
    for mix_name in repeats:
        mix_path = base / mix_name
        if not mix_path.exists():
            continue
        archive = MixArchive.open(mix_path)
        # The mixes store INI/BIN pairs interleaved in entry order.
        payloads: list[tuple[int, bytes]] = []
        for e in archive.entries.values():
            raw = archive.data[archive.payload_start + e.offset : archive.payload_start + e.offset + e.size]
            if raw[:1] == b";" or len(raw) == 64 * 64 * 2:
                payloads.append((e.offset, raw))
        payloads.sort()
        i = 0
        while i < len(payloads):
            offset, raw = payloads[i]
            if raw[:1] != b";":
                i += 1
                continue
            text = raw.decode("latin-1")
            head = _HEADLINE.search(text)
            dims = _map_section(text)
            w, h = dims.get("width", 0), dims.get("height", 0)
            x, y = dims.get("x", 0), dims.get("y", 0)
            binraw = None
            if i + 1 < len(payloads) and len(payloads[i + 1][1]) == 64 * 64 * 2:
                # sanity: the partner BIN's used window should match [MAP]
                cand = payloads[i + 1][1]
                g = np.frombuffer(cand, dtype=np.uint8).reshape(64, 64, 2)
                if w and h:
                    window_used = (g[:, :, 0][y : y + h, x : x + w] != 255).mean()
                    if window_used > 0.3:
                        binraw = cand
                else:
                    binraw = cand
            if head and w and h and binraw is not None:
                num, house = head.group(1), head.group(2)
                code = _HOUSE_CODE.get(house, house[:1].upper())
                base_name = None
                for dv in ("E", "W"):
                    for var in "ABCD":
                        nm = f"SC{code}{int(num):02d}{dv}{var}"
                        if nm not in used_names:
                            base_name = nm
                            break
                    if base_name:
                        break
                if not base_name:
                    base_name = f"SC{code}{int(num):02d}X{len(used_names):02d}"
                used_names.add(base_name)
                (out_dir / f"{base_name}.INI").write_bytes(raw)
                (out_dir / f"{base_name}.BIN").write_bytes(binraw)
                written += 1
                i += 2
                continue
            i += 1
    return written
