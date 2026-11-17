import numpy as np

from golden import COLS, OUT, ROWS, golden_frames
from protocol import decode, encode


def test_encoder_matches_golden_stream():
    stream = b"".join(encode(golden_frames(), COLS, ROWS, 30, 1))
    assert stream == (OUT / "stream.bin").read_bytes()


def test_decoder_matches_golden_cells():
    cells = np.stack(list(decode((OUT / "stream.bin").read_bytes())))
    assert cells.astype("<u2").tobytes() == (OUT / "cells.bin").read_bytes()
