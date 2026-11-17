import subprocess
import sys
import time

import numpy as np

from kernel import Delta
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


def probe_fps(path):
    out = subprocess.run(
        ["ffprobe", "-v", "error", "-select_streams", "v:0",
         "-show_entries", "stream=r_frame_rate", "-of", "csv=p=0", path],
        check=True, capture_output=True, text=True,
    ).stdout
    num, den = out.strip().split("/")
    return int(num) / int(den)


if __name__ == "__main__":
    path = sys.argv[1]
    shift = int(sys.argv[2]) if len(sys.argv) > 2 else 4
    cols, rows = grid(*probe_size(path))
    fps = probe_fps(path)
    delta = Delta(rows, cols, shift)
    out = sys.stdout.buffer
    out.write(b"\x1b[2J")
    start = time.perf_counter()
    count = 0
    for frame in frames(path, cols, rows):
        out.write(delta(frame))
        out.flush()
        count += 1
        delay = start + count / fps - time.perf_counter()
        if delay > 0:
            time.sleep(delay)
    out.write(f"\x1b[0m\x1b[{rows + 1};1H".encode())
    elapsed = time.perf_counter() - start
    print(f"\n{count} frames in {elapsed:.2f} s, {count / elapsed:.1f} fps")
