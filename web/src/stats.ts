export interface Summary {
  fps: number;
  kibPerSecond: number;
  drawMs: number;
  late: number;
}

interface Sample {
  time: number;
  bytes: number;
  drawMs: number;
}

// A frame counts as late when it arrives this many frame intervals after the previous one.
const LATE_FACTOR = 1.5;

export class Stats {
  readonly #samples: Sample[] = [];
  #late = 0;

  record(time: number, bytes: number, drawMs: number, frameMs: number): void {
    const prev = this.#samples.at(-1);
    if (prev && time - prev.time > frameMs * LATE_FACTOR) this.#late++;
    this.#samples.push({ time, bytes, drawMs });
    while (this.#samples[0].time <= time - 1000) this.#samples.shift();
  }

  summary(): Summary {
    const n = this.#samples.length;
    const sum = (key: "bytes" | "drawMs") => this.#samples.reduce((total, s) => total + s[key], 0);
    return { fps: n, kibPerSecond: sum("bytes") / 1024, drawMs: n ? sum("drawMs") / n : 0, late: this.#late };
  }
}
