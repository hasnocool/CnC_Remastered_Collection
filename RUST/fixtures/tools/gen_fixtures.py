#!/usr/bin/env python3
"""Generate the license-clean synthetic original-format fixtures.

Nothing here is derived from Westwood data: every byte is computed from the
constants in this file. Run from anywhere:

    python3 RUST/fixtures/tools/gen_fixtures.py

The script rewrites the fixture files plus CHECKSUMS.sha256 (paths relative
to RUST/fixtures). The LCW payloads are plain format80 streams produced by
a small literal/RLE/absolute-copy encoder; the Rust test suite independently
decompresses them with cnc_asset::format80 and compares the result against
the hand-written expectations.
"""

from __future__ import annotations

import base64
import hashlib
from pathlib import Path

FIXTURES = Path(__file__).resolve().parent.parent
SIDE = 16
UU_LINE_WIDTH = 70  # matches cnc_asset::base64::uu_lines / Put_UUBlock


def le16(value: int) -> bytes:
    return bytes((value & 0xFF, (value >> 8) & 0xFF))


def lcw_encode(data: bytes) -> bytes:
    """Encode data as a valid Westwood format80 (LCW) stream.

    Only three opcodes are emitted: RLE fill (0xFE), absolute back-reference
    (0xFF) and literal runs (0x80 | count). That is deliberately a tiny encoder
    rather than a port of the retail optimiser, but every opcode it writes is
    decoded by the same table the engine uses.
    """
    out = bytearray()
    pos = 0
    size = len(data)
    while pos < size:
        # Longest run of one repeated byte.
        run = 1
        while pos + run < size and data[pos + run] == data[pos] and run < 0xFFFF:
            run += 1

        # Longest absolute back-reference into already-emitted output.
        best_len = 0
        best_off = 0
        for start in range(max(0, pos - 0xFFFF), pos):
            length = 0
            while (
                pos + length < size
                and data[start + length] == data[pos + length]
                and length < 0xFFFF
            ):
                length += 1
            if length > best_len:
                best_len, best_off = length, start

        if run >= 3 and run >= best_len:
            out.append(0xFE)
            out += le16(run)
            out.append(data[pos])
            pos += run
        elif best_len >= 3:
            out.append(0xFF)
            out += le16(best_len)
            out += le16(best_off)
            pos += best_len
        else:
            literal = min(63, size - pos)
            out.append(0x80 | literal)
            out += data[pos : pos + literal]
            pos += literal

    out.append(0x80)  # end of stream
    return bytes(out)


def uu_lines(data: bytes) -> list[str]:
    encoded = base64.b64encode(data).decode("ascii")
    return [encoded[i : i + UU_LINE_WIDTH] for i in range(0, len(encoded), UU_LINE_WIDTH)]


def ini_block(name: str, data: bytes) -> str:
    lines = [f"[{name}]"]
    for index, value in enumerate(uu_lines(data), start=1):
        lines.append(f"{index}={value}")
    return "\n".join(lines) + "\n"


def td_cell(x: int, y: int) -> tuple[int, int]:
    """Template type and icon byte for one TD cell (hand-written rule)."""
    if x == y:
        template = 12
    elif x in (0, SIDE - 1) or y in (0, SIDE - 1):
        template = 5
    else:
        template = 7
    icon = (x * 3 + y) % 8
    return template, icon


def td_bin_raw() -> bytes:
    raw = bytearray()
    for index in range(SIDE * SIDE):
        raw += bytes(td_cell(index % SIDE, index // SIDE))
    return bytes(raw)


def ra_cell_id(x: int, y: int) -> int:
    if x == y:
        return 11
    if (x == 0 or x == SIDE - 1) and (y == 0 or y == SIDE - 1):
        return 22
    return 0


def ra_pack_plain() -> bytes:
    """256 three-byte (u16 LE index, u8 id) records, row-major."""
    plain = bytearray()
    for index in range(SIDE * SIDE):
        plain += le16(index)
        plain.append(ra_cell_id(index % SIDE, index // SIDE))
    return bytes(plain)


def ra_overlay_plain() -> bytes:
    """Two overlay records: a road tile at cell 0 and at cell 255."""
    plain = bytearray()
    for index in (0, 255):
        plain += le16(index)
        plain.append(3)
    return bytes(plain)


TD_INI = """[Basic]
Name=Synthetic TD Golden Fixture
Player=0
Credits=5000
TechLevel=7
Theater=Temperate
Briefing=Secure the ridge and hold the pass.
Briefing1=Second briefing line proves continuation parsing.
[Objectives]
1=Establish a base at the ridge
2=Destroy all Nod units
[Houses]
0=GDI
1=Nod
[GDI]
Credits=5000
MaxUnit=120
MaxBuilding=90
Edge=West
[Nod]
Credits=2500
MaxUnit=100
MaxBuilding=80
Edge=East
[Wunits]
Unit1=UNIT_GDI_E1
Unit2=UNIT_GDI_MTNK
Unit3=UNIT_NOD_BGGY
[UNIT_GDI_E1]
House=0
Type=E1
X=5
Y=6
Facing=32
Health=255
[UNIT_GDI_MTNK]
House=0
Type=MTNK
X=10
Y=10
Facing=64
Health=200
[UNIT_NOD_BGGY]
House=1
Type=BGGY
X=12
Y=3
Facing=96
Health=180
[Structures]
Structure1=STRUCT_GDI_FACT
Structure2=STRUCT_NOD_OBLI
[STRUCT_GDI_FACT]
House=0
Type=FACT
X=4
Y=4
Facing=0
Health=255
[STRUCT_NOD_OBLI]
House=1
Type=OBLI
X=14
Y=14
Facing=128
Health=240
[Trigs]
Trigger1=TRIG_ENTRY
Trigger2=TRIG_CLOCK
[TRIG_ENTRY]
Name=Entry
Condition1=Player Enters
Action1=Win
Enabled=yes
Repeat=no
[TRIG_CLOCK]
Name=Clock
Condition1=Time
Action1=Win
Enabled=yes
Repeat=yes
[TRIGGER]
T_ENTRY=Player Enters,Win,17,GDI,,0
T_CLOCK=Time,Win,90,0,NOD_TEAM,1
[Waypoints]
0=55
1=170
2=255
[Map]
Width=16
Height=16
"""

RA_HEADER = """[Basic]
Name=Synthetic RA Golden Fixture
Player=Greece
Credits=5000
TechLevel=7
Theater=Temperate
Briefing=Defend the coast road.
"""

RA_TAIL = """[Houses]
0=Greece
1=USSR
[Greece]
Credits=40
TechLevel=7
MaxInfantry=60
MaxVessel=3
[USSR]
Credits=20
[Units]
1=Greece,HTNK,256,17,0,Guard,TRIGA
2=USSR,MTNK,128,240,64,Guard,
[Infantry]
1=USSR,E1,256,200,0,Guard,0,
[Structures]
1=Greece,POWR,256,16,0,,1,1
2=USSR,BARR,255,255,0,,
[Waypoints]
0=17
1=240
2=100
[Trigs]
1=TRIGA
2=TRIGB
[TRIGA]
Name=Alpha
Event=Time
Data=300
Action=Win
[TRIGB]
Name=Beta
Event=All Destr.
Data=0
Action=Lose
"""


def write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)


def main() -> None:
    td_dir = FIXTURES / "td"
    ra_dir = FIXTURES / "ra"

    raw = td_bin_raw()
    write(td_dir / "FIXTURE_RAW.bin", raw)
    write(td_dir / "FIXTURE.bin", lcw_encode(raw))
    write(td_dir / "FIXTURE.INI", TD_INI.encode("utf-8"))

    map_plain = ra_pack_plain()
    overlay_plain = ra_overlay_plain()
    map_lcw = lcw_encode(map_plain)
    overlay_lcw = lcw_encode(overlay_plain)
    mpr = (
        RA_HEADER
        + ini_block("MapPack", map_lcw)
        + ini_block("OverlayPack", overlay_lcw)
        + RA_TAIL
    )
    write(ra_dir / "FIXTURE.MPR", mpr.encode("utf-8"))

    # Pin every committed fixture byte, including this generator and the README.
    files = sorted(
        p
        for p in FIXTURES.rglob("*")
        if p.is_file() and p.name != "CHECKSUMS.sha256"
    )
    lines = []
    for path in files:
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        lines.append(f"{digest}  {path.relative_to(FIXTURES).as_posix()}")
    write(FIXTURES / "CHECKSUMS.sha256", ("\n".join(lines) + "\n").encode("ascii"))

    print(f"TD raw={len(raw)}B lcw={len(lcw_encode(raw))}B")
    print(f"RA map plain={len(map_plain)}B lcw={len(map_lcw)}B")
    print(f"RA overlay plain={len(overlay_plain)}B lcw={len(overlay_lcw)}B")
    print(f"pinned {len(lines)} files")


if __name__ == "__main__":
    main()
