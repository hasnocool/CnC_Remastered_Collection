"""Westwood format80 (LCW) decompression — Python port of the Rust port.

Opcodes (n = control byte, operands little-endian):

- `0xxxxxxx`: short copy from dest: count = (n >> 4) + 3,
  back-offset = next + ((n & 0x0F) << 8)
- `0x80`: end of stream
- `10xxxxxx`: literal run of `n & 0x3F` bytes
- `11xxxxxx` (not FE/FF): copy `(n & 0x3F) + 3` bytes from dest offset (LE u16)
- `0xFE`: RLE fill: `w1` bytes of `b1`
- `0xFF`: long copy: `w1` bytes from dest offset `w2`
"""

from __future__ import annotations


def decompress(input: bytes, expected_len: int) -> bytearray:
    out = bytearray()
    i = 0
    relative = input[:1] == b"\x00"
    if relative:
        i = 1

    def need(n: int) -> None:
        nonlocal i
        if i + n > len(input):
            raise ValueError(f"unexpected end of input at {i}")

    while True:
        need(1)
        n = input[i]
        i += 1
        if n & 0x80 == 0:  # short copy from dest
            need(1)
            back = input[i] + ((n & 0x0F) << 8)
            i += 1
            count = (n >> 4) + 3
            _copy_forward(out, len(out) - back, count)
        elif n == 0x80:
            break
        elif n & 0xC0 == 0x80:  # literal run
            count = n & 0x3F
            need(count)
            out += input[i : i + count]
            i += count
        elif n in (0xFE, 0xFF):
            need(2)
            w1 = struct_unpack_u16(input, i)
            i += 2
            if n == 0xFF:
                need(2)
                w2 = struct_unpack_u16(input, i)
                i += 2
                _copy_forward(out, w2, w1)
            else:
                need(1)
                b = input[i]
                i += 1
                out += bytes([b]) * w1
        else:  # 11xxxxxx: long copy from dest
            need(2)
            (off,) = struct_unpack_u16(input, i)
            i += 2
            count = (n & 0x3F) + 3
            _copy_forward(out, off, count)
        if len(out) > expected_len:
            raise ValueError(f"output overrun: {len(out)} > {expected_len}")
    if len(out) != expected_len:
        raise ValueError(f"length mismatch: {len(out)} != {expected_len}")
    return out


def struct_unpack_u16(data: bytes, off: int) -> tuple[int]:
    return (data[off] | (data[off + 1] << 8),)


def _copy_forward(out: bytearray, from_: int, count: int) -> None:
    if from_ < 0:
        raise ValueError("back-reference before start of output")
    for step in range(count):
        out.append(out[from_ + step])
