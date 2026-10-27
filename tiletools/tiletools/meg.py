"""C&C Remastered texture MEG archive reader.

Port of ``RUST/cnc-asset/src/original_assets.rs`` (index_shipped_archive,
normalize_dds) to Python. Layout: header, u16-prefixed name table, 14-byte
gap, one 20-byte record per member (offset, stored, unpacked, 1-based index,
full size), then raw payloads in name order. The final name is a terminator
whose payload runs from the previous member's end to EOF.
"""

from __future__ import annotations

import struct
from dataclasses import dataclass
from pathlib import Path

MAGIC_0 = 0xFFFF_FFFF
MAGIC_1 = 0x3F7D_70A4
MAX_MIP_LEVELS = 20


@dataclass
class MegArchive:
    path: Path
    data: bytes
    members: dict[str, tuple[int, int]]

    @classmethod
    def open(cls, install_root: Path, game: str) -> "MegArchive":
        name = {"td": "TEXTURES_TD_SRGB.MEG", "ra": "TEXTURES_RA_SRGB.MEG"}[game]
        path = install_root / "Data" / name
        data = path.read_bytes()
        return cls(path=path, data=data, members=index_shipped_archive(data))

    def member_bytes(self, name: str) -> bytes:
        start, end = self.members[name]
        return self.data[start:end]

    def has(self, name: str) -> bool:
        return name in self.members


def index_shipped_archive(data: bytes) -> dict[str, tuple[int, int]]:
    if len(data) < 0x18:
        raise ValueError("texture archive too small")
    if _u32(data, 0) != MAGIC_0 or _u32(data, 4) != MAGIC_1:
        raise ValueError("not a shipped texture archive (bad magic)")
    count = _u32(data, 0x0C)
    names_size = _u32(data, 0x14)
    names_end = 0x18 + names_size
    if names_end > len(data):
        raise ValueError("name table exceeds archive bounds")

    names: list[str] = []
    pos = 0x18
    while pos + 2 <= names_end:
        (nlen,) = struct.unpack_from("<H", data, pos)
        pos += 2
        if nlen == 0 or pos + nlen > names_end:
            raise ValueError(f"corrupt name table at offset {pos:#x}")
        nm = data[pos : pos + nlen]
        if not all(0x21 <= c <= 0x7E for c in nm):
            raise ValueError(f"non-ASCII member name in table at offset {pos:#x}")
        names.append(nm.decode("ascii"))
        pos += nlen
    if len(names) != count:
        raise ValueError(f"name table holds {len(names)} names, header declares {count}")

    records = names_end + 14
    offsets: list[int] = []
    for i in range(count):
        rec = records + i * 20
        if rec + 20 > len(data):
            break
        (idx,) = struct.unpack_from("<I", data, rec + 12)
        if idx != i + 1:
            break
        (off,) = struct.unpack_from("<I", data, rec)
        offsets.append(off)
    offsets.append(len(data))

    members: dict[str, tuple[int, int]] = {}
    for name, (start, end) in zip(names, zip(offsets, offsets[1:])):
        if start >= end or end > len(data):
            raise ValueError(f"member {name} has empty or out-of-bounds payload")
        members[name] = (start, end)
    return members


def _u32(data: bytes, o: int) -> int:
    return struct.unpack_from("<I", data, o)[0]


def steam_install_root() -> Path | None:
    home = Path.home()
    candidates = [
        home / ".steam/steam/steamapps/common/CnCRemastered",
        home / ".local/share/Steam/steamapps/common/CnCRemastered",
    ]
    vdf = home / ".steam/steam/steamapps/libraryfolders.vdf"
    if vdf.is_file():
        for line in vdf.read_text().splitlines():
            line = line.strip()
            if line.startswith('"path"'):
                inner = line[line.find('"', 6) + 1 :]
                end = inner.find('"')
                if end > 0:
                    candidates.append(
                        Path(inner[:end].replace("\\\\", "/")) / "steamapps/common/CnCRemastered"
                    )
    for c in candidates:
        if (c / "Data").is_dir():
            return c
    return None
