import * as wasm from "mandelbrot-wasm";
//import * as wasm from "@juliankaindl/mandelbrot-wasm";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
wasm.draw(ctx, 600, 600, -0.15, 0.65);