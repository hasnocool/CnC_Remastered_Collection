"""DXT1 (BC1) and DXT5 (BC3) decoding with numpy.

The shipped terrain textures are DXT5, some families DXT1, both with the
NVTT 4-byte marker after the 124-byte header. Pillow's BCn decoder pads
differently from the shipped layout, so decode the blocks directly — the
formats are fully specified and exact.
"""

from __future__ import annotations

import struct

import numpy as np


def decode_dds_payload(
    payload: bytes, width: int, height: int, fourcc: bytes
) -> np.ndarray:
    """Decode raw BC payload to an [H, W, 4] uint8 RGBA array."""
    if fourcc == b"DXT5":
        return decode_bc3(payload, width, height)
    if fourcc == b"DXT1":
        return decode_bc1(payload, width, height)
    if fourcc == b"DXT3":
        return decode_bc2(payload, width, height)
    raise NotImplementedError(f"fourcc {fourcc!r}")


def _unpack565(c0: int, c1: int) -> np.ndarray:
    """Four RGB565-derived palette colors (as in BC1 without color-key)."""

    def expand(c: int) -> np.ndarray:
        r = ((c >> 11) & 0x1F) << 3
        g = ((c >> 5) & 0x3F) << 2
        b = (c & 0x1F) << 3
        return np.array((r | (r >> 5), g | (g >> 6), b | (b >> 5)), dtype=np.uint8)

    a, b = expand(c0).astype(np.int16), expand(c1).astype(np.int16)
    pal = np.empty((4, 3), dtype=np.uint8)
    pal[0] = expand(c0)
    pal[1] = expand(c1)
    if c0 > c1:
        pal[2] = ((2 * a + b) // 3).astype(np.uint8)
        pal[3] = ((a + 2 * b) // 3).astype(np.uint8)
    else:
        pal[2] = ((a + b) // 2).astype(np.uint8)
        pal[3] = np.array((0, 0, 0), dtype=np.uint8)  # BC1 color-key transparent
    return pal


def _decode_colors(block8: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """BC1-style color half-block -> (palette [4,3], indices [16] uint8)."""
    c0, c1 = struct.unpack_from("<HH", block8, 0)
    (bits,) = struct.unpack_from("<I", block8, 4)
    pal = _unpack565(c0, c1)
    idx = np.array([(bits >> (2 * i)) & 3 for i in range(16)], dtype=np.uint8)
    return pal, idx


def decode_bc1(data: bytes, width: int, height: int) -> np.ndarray:
    out = np.zeros((height, width, 4), dtype=np.uint8)
    blocks_y = height // 4
    blocks_x = width // 4
    block_size = 8
    for by in range(blocks_y):
        for bx in range(blocks_x):
            off = (by * blocks_x + bx) * block_size
            pal, idx = _decode_colors(data[off : off + 8])
            rgb = pal[idx]
            alpha = np.where(idx == 3, 0, 255).astype(np.uint8)
            out[by * 4 : by * 4 + 4, bx * 4 : bx * 4 + 4, :3] = rgb.reshape(4, 4, 3)
            out[by * 4 : by * 4 + 4, bx * 4 : bx * 4 + 4, 3] = alpha.reshape(4, 4)
    return out


def decode_bc2(data: bytes, width: int, height: int) -> np.ndarray:
    out = np.zeros((height, width, 4), dtype=np.uint8)
    blocks_y = height // 4
    blocks_x = width // 4
    for by in range(blocks_y):
        for bx in range(blocks_x):
            off = (by * blocks_x + bx) * 16
            alpha_bits = int.from_bytes(data[off : off + 8], "little")
            pal, idx = _decode_colors(data[off + 8 : off + 16])
            out[by * 4 : by * 4 + 4, bx * 4 : bx * 4 + 4, :3] = pal[idx].reshape(4, 4, 3)
            alpha = np.array([(alpha_bits >> (4 * i)) & 0xF for i in range(16)], dtype=np.uint8)
            out[by * 4 : by * 4 + 4, bx * 4 : bx * 4 + 4, 3] = alpha * 17
    return out


def decode_bc3(data: bytes, width: int, height: int) -> np.ndarray:
    out = np.zeros((height, width, 4), dtype=np.uint8)
    blocks_y = height // 4
    blocks_x = width // 4
    for by in range(blocks_y):
        for bx in range(blocks_x):
            off = (by * blocks_x + bx) * 16
            block = data[off : off + 16]
            a0, a1 = block[0], block[1]
            alpha_bits = int.from_bytes(block[2:8], "little")
            alut = np.empty(8, dtype=np.uint8)
            alut[0], alut[1] = a0, a1
            if a0 > a1:
                for i in range(6):
                    alut[2 + i] = (a0 * (5 - i) + a1 * (i + 1)) // 6
            else:
                for i in range(4):
                    alut[2 + i] = (a0 * (3 - i) + a1 * (i + 1)) // 4
                alut[6] = 0
                alut[7] = 255
            alpha_idx = np.array(
                [(alpha_bits >> (3 * i)) & 7 for i in range(16)], dtype=np.uint8
            )
            pal, idx = _decode_colors(block[8:16])
            out[by * 4 : by * 4 + 4, bx * 4 : bx * 4 + 4, 3] = alut[alpha_idx].reshape(4, 4)
            out[by * 4 : by * 4 + 4, bx * 4 : bx * 4 + 4, :3] = pal[idx].reshape(4, 4, 3)
    return out


def decode_shipped_dds(data: bytes) -> tuple[np.ndarray, int, int]:
    """Decode a shipped (124-byte header + 4-byte marker) DDS member."""
    if data[:4] != b"DDS ":
        raise ValueError("not a DDS member")
    height, width = struct.unpack_from("<II", data, 12)
    pfflags, fourcc = struct.unpack_from("<I4s", data, 80)
    data_start = max(124, struct.unpack_from("<I", data, 4)[0]) + 4
    payload = data[data_start:]
    if fourcc == b"\x00\x00\x00\x00" and pfflags & 0x40:
        # DDS_PIXELFORMAT: bitcount@88, R@92, G@96, B@100, A@104
        rmask, gmask, bmask, amask = struct.unpack_from("<IIII", data, 92)
        px = np.frombuffer(payload, dtype=np.uint8)[: width * height * 4].reshape(
            height, width, 4
        )
        if rmask == 0x000000FF:  # RGBA byte order
            return px.copy(), width, height
        if rmask == 0x00FF0000:  # BGRA byte order
            out = px[:, :, [2, 1, 0, 3]].copy()
            return out, width, height
        raise NotImplementedError(f"unusual masks {rmask:#x} {gmask:#x} {bmask:#x}")
    rgba = decode_dds_payload(payload, width, height, fourcc)
    return rgba, width, height
