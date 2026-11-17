import struct

import numpy as np
from numba import njit

from kernel import RAMP

HELLO, KEY, DELTA = 1, 2, 3
VERSION = 1


@njit(cache=True)
def to_cells(frame):
    rows, cols, _ = frame.shape
    levels = len(RAMP)
    out = np.empty(rows * cols, np.uint16)
    for y in range(rows):
        for x in range(cols):
            r = np.int32(frame[y, x, 0])
            g = np.int32(frame[y, x, 1])
            b = np.int32(frame[y, x, 2])
            glyph = ((54 * r + 183 * g + 19 * b) >> 8) * levels >> 8
            out[y * cols + x] = (r >> 4) << 12 | (g >> 4) << 8 | (b >> 4) << 4 | glyph
    return out


def message(kind, payload):
    return struct.pack("<BI", kind, len(payload)) + payload


def hello(cols, rows, fps_num, fps_den):
    return message(HELLO, struct.pack("<BHHHH", VERSION, cols, rows, fps_num, fps_den))


def key(cells):
    return message(KEY, cells.astype("<u2").tobytes())
