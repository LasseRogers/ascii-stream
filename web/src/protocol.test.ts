import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import { apply, DELTA, HELLO, KEY, messages, parseHello } from "./protocol.ts";

const golden = (name: string) => new Uint8Array(readFileSync(new URL(`../../testdata/${name}`, import.meta.url)));

test("decodes golden cells", () => {
  let cells = new Uint16Array(0);
  const screens: number[] = [];
  for (const [kind, payload] of messages(golden("stream.bin"))) {
    if (kind === HELLO) {
      const hello = parseHello(payload);
      assert.deepEqual(hello, { cols: 8, rows: 4, fpsNum: 30, fpsDen: 1, frame: 0 });
      cells = new Uint16Array(hello.cols * hello.rows);
    } else if (kind === KEY || kind === DELTA) {
      apply(cells, kind, payload);
      screens.push(...cells);
    }
  }
  const expected = golden("cells.bin");
  const view = new DataView(expected.buffer, expected.byteOffset, expected.byteLength);
  const want = Array.from({ length: expected.length / 2 }, (_, i) => view.getUint16(i * 2, true));
  assert.deepEqual(screens, want);
});
