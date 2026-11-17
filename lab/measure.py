import sys

from kernel import render
from play import frames, probe_fps
from still import CELL_ASPECT, probe_size

COLS = 160


if __name__ == "__main__":
    path = sys.argv[1]
    w, h = probe_size(path)
    rows = round(COLS * h / w * CELL_ASPECT)
    sizes = [len(render(f)) for f in frames(path, COLS, rows)]
    mean = sum(sizes) / len(sizes)
    print(f"{COLS}x{rows}, {len(sizes)} frames")
    print(f"{mean / 1024:.1f} KiB per frame, {mean / (COLS * rows):.1f} B per cell")
    print(f"{mean * probe_fps(path) / 1024**2:.2f} MiB/s")
