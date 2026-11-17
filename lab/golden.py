from pathlib import Path

import numpy as np

from protocol import decode, encode

OUT = Path(__file__).parent.parent / "testdata"
COLS, ROWS = 8, 4


def golden_frames():
    f0 = np.zeros((ROWS, COLS, 3), np.uint8)
    f0[..., 0] = np.arange(COLS) * 32
    f0[..., 1] = (np.arange(ROWS) * 64)[:, None]
    f0[..., 2] = 128
    f1 = f0.copy()
    f1.reshape(-1, 3)[[7, 8, 20]] = [255, 0, 0]
    return np.stack([f0, f1, f1, np.full_like(f0, 255)])


if __name__ == "__main__":
    frames = golden_frames()
    stream = b"".join(encode(frames, COLS, ROWS, 30, 1))
    cells = np.stack(list(decode(stream)))
    OUT.mkdir(exist_ok=True)
    (OUT / "input.rgb").write_bytes(frames.tobytes())
    (OUT / "stream.bin").write_bytes(stream)
    (OUT / "cells.bin").write_bytes(cells.astype("<u2").tobytes())
