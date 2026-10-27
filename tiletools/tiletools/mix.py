"""Classic Westwood MIX archive reader (Tiberian Dawn / Red Alert).

Format from ``original/TIBERIANDAWN/MIXFILE.H/.CPP``: header
``{short count; long payload_size}``, then ``count`` 12-byte records
``{long crc; long offset; long size}`` (offset relative to the payload
start), then payload back to back. Files are found by the Westwood CRC of
the uppercased filename (``MiscAsm.cpp`` ``Calculate_CRC``).
"""

from __future__ import annotations

import struct
from dataclasses import dataclass
from pathlib import Path


@dataclass
class MixEntry:
    name: str
    crc: int
    offset: int  # relative to payload start
    size: int


def westwood_crc(data: bytes) -> int:
    """Rol1+add CRC over 4-byte little-endian words (MiscAsm.cpp)."""
    crc = 0
    words = len(data) // 4
    rem = data[words * 4 :]
    for i in range(words):
        (w,) = struct.unpack_from("<I", data, i * 4)
        crc = ((crc << 1) | (crc >> 31)) & 0xFFFFFFFF
        crc = (crc + w) & 0xFFFFFFFF
    if rem:
        k = len(rem)
        # Bytes fill a scratch word from the top, then ror by (4-k)*8.
        eax = 0
        for b in rem:
            eax = (eax | b) & 0xFFFFFFFF
            eax = ((eax >> 8) | (eax << 24)) & 0xFFFFFFFF
        eax = ((eax >> ((4 - k) * 8)) | (eax << (8 * (4 - k)))) & 0xFFFFFFFF
        crc = ((crc << 1) | (crc >> 31)) & 0xFFFFFFFF
        crc = (crc + eax) & 0xFFFFFFFF
    return crc


class MixArchive:
    def __init__(self, data: bytes):
        if len(data) < 6:
            raise ValueError("MIX too small")
        count, payload_size = struct.unpack_from("<Hi", data, 0)
        if count < 0 or 6 + count * 12 + payload_size > len(data) or payload_size < 0:
            raise ValueError("MIX header out of bounds")
        payload_start = 6 + count * 12
        crc_to_name: dict[int, str] = {}
        self.entries: dict[str, MixEntry] = {}
        for i in range(count):
            crc, offset, size = struct.unpack_from("<Iii", data, 6 + i * 12)
            self.entries[f"__crc{i}"] = MixEntry("", crc, offset, size)
        self._by_crc: dict[int, MixEntry] = {
            e.crc: e for e in self.entries.values()
        }
        self.data = data
        self.payload_start = payload_start
        self.payload_size = payload_size

    def register(self, name: str) -> None:
        """Bind a plaintext filename to its CRC entry (lookup by name)."""
        crc = westwood_crc(name.upper().encode("ascii"))
        entry = self._by_crc.get(crc)
        if entry is not None and entry.name == "":
            entry.name = name
            self.entries[name] = entry

    def get(self, name: str) -> bytes:
        entry = self.entries.get(name)
        if entry is None:
            entry = self._by_crc.get(westwood_crc(name.upper().encode("ascii")))
        if entry is None:
            raise KeyError(name)
        raw = self.data[
            self.payload_start + entry.offset : self.payload_start + entry.offset + entry.size
        ]
        return raw

    def has(self, name: str) -> bool:
        try:
            self.get(name)
            return True
        except KeyError:
            return False

    @classmethod
    def open(cls, path: Path) -> "MixArchive":
        return cls(path.read_bytes())
