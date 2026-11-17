import subprocess
import sys
import time

import numpy as np

from kernel import render
from still import grid, probe_size


def frames(path, cols, rows):
    proc = subprocess.Popen(
        [
            "ffmpeg",
            "-v",
            "error",
            "-i",
            path,
            "-vf",
            f"scale={cols}:{rows}",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-",
        ],
        stdout=subprocess.PIPE,
    )
    assert proc.stdout
    size = rows * cols * 3
    while len(raw := proc.stdout.read(size)) == size:
        yield np.frombuffer(raw, np.uint8).reshape(rows, cols, 3)
    proc.wait()


if __name__ == "__main__":
    cols, rows = grid(*probe_size(sys.argv[1]))
    out = sys.stdout.buffer
    start = time.perf_counter()
    count = 0
    out.write(b"\x1b[2J")
    for frame in frames(sys.argv[1], cols, rows):
        out.write(b"\x1b[H" + render(frame))
        out.flush()
        count += 1
    elapsed = time.perf_counter() - start
    print(f"\n{count} frames, {count / elapsed:.1f} fps")
