import assert from "node:assert/strict";
import { test } from "node:test";

import { Stats } from "./stats.ts";

const FRAME_MS = 1000 / 30;

test("summarises the last second", () => {
  const stats = new Stats();
  for (let i = 0; i < 30; i++) stats.record(i * FRAME_MS, 1024, 2, FRAME_MS);
  assert.deepEqual(stats.summary(), { fps: 30, kibPerSecond: 30, drawMs: 2, late: 0 });
  stats.record(2000, 1024, 2, FRAME_MS);
  assert.deepEqual(stats.summary(), { fps: 1, kibPerSecond: 1, drawMs: 2, late: 1 });
});
