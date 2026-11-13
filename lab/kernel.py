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
