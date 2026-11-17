const RAMP = " .:-=+*#%@";

export class CanvasScreen {
  readonly #ctx: CanvasRenderingContext2D;
  readonly #cols: number;
  readonly #cellW: number;
  readonly #cellH: number;
  readonly #shown: Uint16Array;

  constructor(canvas: HTMLCanvasElement, cols: number, rows: number, fontSize = 12) {
    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("canvas not available");
    const font = `${fontSize}px monospace`;
    ctx.font = font;
    this.#cellW = Math.ceil(ctx.measureText("M").width);
    this.#cellH = this.#cellW * 2;
    canvas.width = cols * this.#cellW;
    canvas.height = rows * this.#cellH;
    ctx.font = font;
    ctx.textBaseline = "middle";
    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    this.#ctx = ctx;
    this.#cols = cols;
    this.#shown = new Uint16Array(cols * rows).fill(0xffff);
  }

  draw(cells: Uint16Array): void {
    const ctx = this.#ctx;
    for (let i = 0; i < cells.length; i++) {
      const cell = cells[i];
      if (cell === this.#shown[i]) continue;
      this.#shown[i] = cell;
      const x = (i % this.#cols) * this.#cellW;
      const y = Math.floor(i / this.#cols) * this.#cellH;
      ctx.fillStyle = "#000";
      ctx.fillRect(x, y, this.#cellW, this.#cellH);
      const [r, g, b] = [cell >> 12, (cell >> 8) & 15, (cell >> 4) & 15].map((v) => v * 16 + 8);
      ctx.fillStyle = `rgb(${r} ${g} ${b})`;
      ctx.fillText(RAMP[cell & 15], x, y + this.#cellH / 2);
    }
  }
}
