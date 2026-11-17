import { CanvasScreen } from "./canvas.ts";
import { apply, DELTA, HELLO, KEY, messages, parseHello } from "./protocol.ts";

const canvas = document.querySelector<HTMLCanvasElement>("#screen");
if (!canvas) throw new Error("canvas not found");

const socket = new WebSocket(`ws://${location.hostname}:7879`);
socket.binaryType = "arraybuffer";

let cells = new Uint16Array(0);
let screen: CanvasScreen | undefined;

socket.addEventListener("message", (event) => {
  for (const [kind, payload] of messages(new Uint8Array(event.data as ArrayBuffer))) {
    if (kind === HELLO) {
      const hello = parseHello(payload);
      cells = new Uint16Array(hello.cols * hello.rows);
      screen = new CanvasScreen(canvas, hello.cols, hello.rows);
    } else if (kind === KEY || kind === DELTA) {
      apply(cells, kind, payload);
    }
  }
  screen?.draw(cells);
});
