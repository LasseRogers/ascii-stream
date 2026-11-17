import numpy as np
from numba import njit

from kernel import RAMP


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
