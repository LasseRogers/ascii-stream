import { CanvasScreen } from "./canvas.ts";
import { apply, DELTA, HELLO, KEY, messages, parseHello } from "./protocol.ts";

// Seek the source video when it is this many seconds away from the stream.
const MAX_DRIFT = 0.1;

const canvas = document.querySelector<HTMLCanvasElement>("#screen");
const video = document.querySelector<HTMLVideoElement>("#source");
if (!canvas || !video) throw new Error("page elements not found");

const socket = new WebSocket(`ws://${location.hostname}:7879`);
socket.binaryType = "arraybuffer";

let cells = new Uint16Array(0);
let screen: CanvasScreen | undefined;
let fps = 0;
let frame = 0;

socket.addEventListener("message", (event) => {
  for (const [kind, payload] of messages(new Uint8Array(event.data as ArrayBuffer))) {
    if (kind === HELLO) {
      const hello = parseHello(payload);
      cells = new Uint16Array(hello.cols * hello.rows);
      screen = new CanvasScreen(canvas, hello.cols, hello.rows);
      fps = hello.fpsNum / hello.fpsDen;
      frame = hello.frame - 1;
    } else if (kind === KEY || kind === DELTA) {
      apply(cells, kind, payload);
      frame++;
    }
  }
  screen?.draw(cells);
  follow(video, frame / fps);
});

function follow(video: HTMLVideoElement, time: number): void {
  const duration = video.duration;
  if (!Number.isFinite(duration)) return;
  const target = time % duration;
  let drift = video.currentTime - target;
  if (drift > duration / 2) drift -= duration;
  if (drift < -duration / 2) drift += duration;
  if (Math.abs(drift) > MAX_DRIFT) video.currentTime = target;
  if (video.paused) void video.play();
}
