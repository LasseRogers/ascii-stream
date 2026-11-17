import numpy as np
from numba import njit


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


RAMP = np.frombuffer(b" .:-=+*#%@", np.uint8)
# Widest cell: "\x1b[38;2;255;255;255m" plus the glyph.
CELL_MAX = 20


@njit(cache=True)
def render_into(out, frame, ramp, shift):
    rows, cols, _ = frame.shape
    levels = len(ramp)
    half = (1 << shift) >> 1
    n = 0
    pr = pg = pb = -1
    for y in range(rows):
        for x in range(cols):
            r = np.int32(frame[y, x, 0])
            g = np.int32(frame[y, x, 1])
            b = np.int32(frame[y, x, 2])
            qr = r >> shift << shift | half
            qg = g >> shift << shift | half
            qb = b >> shift << shift | half
            if qr != pr or qg != pg or qb != pb:
                out[n:n + 7] = np.array([27, 91, 51, 56, 59, 50, 59], np.uint8)
                n = put_decimal(out, n + 7, qr)
                out[n] = 59
                n = put_decimal(out, n + 1, qg)
                out[n] = 59
                n = put_decimal(out, n + 1, qb)
                out[n] = 109
                n += 1
                pr, pg, pb = qr, qg, qb
            out[n] = ramp[((54 * r + 183 * g + 19 * b) >> 8) * levels >> 8]
            n += 1
        out[n] = 10
        n += 1
    return n


def render(frame, shift=0):
    rows, cols, _ = frame.shape
    out = np.empty(rows * (cols * CELL_MAX + 1), np.uint8)
    n = render_into(out, frame, RAMP, shift)
    return out[:n].tobytes() + b"\x1b[0m"


@njit(cache=True)
def put_move(out, n, y, x):
    out[n] = 27
    out[n + 1] = 91
    n = put_decimal(out, n + 2, y + 1)
    out[n] = 59
    n = put_decimal(out, n + 1, x + 1)
    out[n] = 72
    return n + 1
