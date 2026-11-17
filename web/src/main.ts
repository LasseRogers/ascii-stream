const canvas = document.querySelector<HTMLCanvasElement>("#screen");
const ctx = canvas?.getContext("2d");
if (!canvas || !ctx) throw new Error("canvas not available");

canvas.width = 320;
canvas.height = 40;
ctx.fillStyle = "#000";
ctx.fillRect(0, 0, canvas.width, canvas.height);
ctx.fillStyle = "#0f0";
ctx.font = "16px monospace";
ctx.fillText("hello from main.ts", 8, 24);
