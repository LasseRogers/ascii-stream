import subprocess
import sys
import time

import numpy as np

from still import grid, probe_size, render


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
    start = time.perf_counter()
    count = 0
    sys.stdout.write("\x1b[2J")
    for frame in frames(sys.argv[1], cols, rows):
        sys.stdout.write("\x1b[H" + render(frame))
        sys.stdout.flush()
        count += 1
    elapsed = time.perf_counter() - start
    print(f"\n{count} frames, {count / elapsed:.1f} fps")
