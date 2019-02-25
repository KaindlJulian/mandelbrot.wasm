import * as wasm from "mandelbrot-wasm";
//import * as wasm from "@juliankaindl/mandelbrot-wasm";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

console.time('draw');
wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, 0.25001, 0.0);
console.timeEnd('draw');