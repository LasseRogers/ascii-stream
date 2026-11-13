import subprocess
import sys

import numpy as np

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
    print(sum(1 for _ in frames(sys.argv[1], cols, rows)))
