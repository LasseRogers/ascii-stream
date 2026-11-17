import struct

import numpy as np
from numba import njit

from kernel import RAMP

HELLO, KEY, DELTA = 1, 2, 3
VERSION = 2


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


def hello(cols, rows, fps_num, fps_den, frame=0):
    payload = struct.pack("<BHHHHI", VERSION, cols, rows, fps_num, fps_den, frame)
    return message(HELLO, payload)


def key(cells):
    return message(KEY, cells.astype("<u2").tobytes())


@njit(cache=True)
def delta_runs(prev, cells):
    out = np.empty(cells.size * 3, np.uint16)
    n = 0
    i = 0
    while i < cells.size:
        if cells[i] == prev[i]:
            i += 1
            continue
        start = i
        while i < cells.size and cells[i] != prev[i]:
            i += 1
        out[n] = start
        out[n + 1] = i - start
        out[n + 2 : n + 2 + i - start] = cells[start:i]
        n += 2 + i - start
    return out[:n]


def delta(prev, cells):
    return message(DELTA, delta_runs(prev, cells).astype("<u2").tobytes())


def encode(frames, cols, rows, fps_num, fps_den):
    yield hello(cols, rows, fps_num, fps_den)
    prev = None
    for frame in frames:
        cells = to_cells(frame)
        yield key(cells) if prev is None else delta(prev, cells)
        prev = cells


@njit(cache=True)
def apply_runs(cells, runs):
    i = 0
    while i < runs.size:
        start, count = runs[i], runs[i + 1]
        cells[start : start + count] = runs[i + 2 : i + 2 + count]
        i += 2 + count


def decode(data):
    pos = 0
    cells = None
    while pos < len(data):
        kind, length = struct.unpack_from("<BI", data, pos)
        payload = data[pos + 5 : pos + 5 + length]
        pos += 5 + length
        if kind == HELLO:
            version, cols, rows, _, _, _ = struct.unpack("<BHHHHI", payload)
            if version != VERSION:
                raise ValueError(f"unsupported protocol version {version}")
            cells = np.zeros(cols * rows, np.uint16)
        elif kind == KEY:
            cells[:] = np.frombuffer(payload, "<u2")
            yield cells.copy()
        elif kind == DELTA:
            apply_runs(cells, np.frombuffer(payload, "<u2"))
            yield cells.copy()
