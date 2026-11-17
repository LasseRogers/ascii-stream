import shutil
import subprocess
import sys

import numpy as np


def probe_size(path):
    out = subprocess.run(
        [
            "ffprobe",
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-of",
            "csv=p=0",
            path,
        ],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    w, h = out.strip().split(",")[:2]
    return int(w), int(h)


# Terminal cells are about twice as tall as they are wide.
CELL_ASPECT = 0.5


def grid(w, h):
    term = shutil.get_terminal_size()
    cols = term.columns
    rows = round(cols * h / w * CELL_ASPECT)
    if rows > term.lines - 1:
        rows = term.lines - 1
        cols = round(rows * w / h / CELL_ASPECT)
    return cols, rows


def read_frame(path, at, cols, rows):
    raw = subprocess.run(
        [
            "ffmpeg",
            "-v",
            "error",
            "-ss",
            str(at),
            "-i",
            path,
            "-frames:v",
            "1",
            "-vf",
            f"scale={cols}:{rows}",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-",
        ],
        check=True,
        capture_output=True,
    ).stdout
    return np.frombuffer(raw, np.uint8).reshape(rows, cols, 3)


RAMP = " .:-=+*#%@"


def render(frame):
    luma = frame @ np.array([0.2126, 0.7152, 0.0722])
    glyph = (luma * (len(RAMP) - 1) / 255).round().astype(int)
    lines = []
    for y in range(frame.shape[0]):
        cells = []
        for x in range(frame.shape[1]):
            r, g, b = frame[y, x]
            cells.append(f"\x1b[38;2;{r};{g};{b}m{RAMP[glyph[y, x]]}")
        lines.append("".join(cells))
    return "\n".join(lines) + "\x1b[0m"


if __name__ == "__main__":
    cols, rows = grid(*probe_size(sys.argv[1]))
    frame = read_frame(sys.argv[1], 3, cols, rows)
    print(render(frame))
