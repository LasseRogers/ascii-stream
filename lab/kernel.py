import numpy as np
from numba import njit


@njit(cache=True)
def glyph_indices(frame, levels):
    rows, cols, _ = frame.shape
    out = np.empty((rows, cols), np.uint8)
    for y in range(rows):
        for x in range(cols):
            r, g, b = frame[y, x, 0], frame[y, x, 1], frame[y, x, 2]
            luma = (54 * r + 183 * g + 19 * b) >> 8
            out[y, x] = luma * levels >> 8
    return out


@njit(cache=True)
def put_decimal(out, n, value):
    if value >= 100:
        out[n] = 48 + value // 100
        n += 1
    if value >= 10:
        out[n] = 48 + value // 10 % 10
        n += 1
    out[n] = 48 + value % 10
    return n + 1
