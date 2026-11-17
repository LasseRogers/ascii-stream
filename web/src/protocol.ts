export const HELLO = 1;
export const KEY = 2;
export const DELTA = 3;
export const VERSION = 1;

export interface Hello {
  cols: number;
  rows: number;
  fpsNum: number;
  fpsDen: number;
}

export function parseHello(payload: Uint8Array): Hello {
  const view = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
  if (view.getUint8(0) !== VERSION) throw new Error(`unsupported protocol version ${view.getUint8(0)}`);
  return {
    cols: view.getUint16(1, true),
    rows: view.getUint16(3, true),
    fpsNum: view.getUint16(5, true),
    fpsDen: view.getUint16(7, true),
  };
}

export function* messages(data: Uint8Array): Generator<[number, Uint8Array]> {
  const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  let pos = 0;
  while (pos + 5 <= data.length) {
    const length = view.getUint32(pos + 1, true);
    yield [data[pos], data.subarray(pos + 5, pos + 5 + length)];
    pos += 5 + length;
  }
}

export function apply(cells: Uint16Array, kind: number, payload: Uint8Array): void {
  const view = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
  const value = (i: number) => view.getUint16(i * 2, true);
  const count = payload.byteLength / 2;
  if (kind === KEY) {
    for (let i = 0; i < cells.length; i++) cells[i] = value(i);
  } else if (kind === DELTA) {
    let i = 0;
    while (i + 2 <= count) {
      const start = value(i);
      const n = value(i + 1);
      for (let j = 0; j < n; j++) cells[start + j] = value(i + 2 + j);
      i += 2 + n;
    }
  }
}
