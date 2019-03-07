import * as wasm from "@juliankaindl/mandelbrot-wasm";

// initial drawing
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
let currentZoom = 1.0;
wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, 0.0, 0.0, currentZoom);

// on apply
document.getElementById('apply-btn').addEventListener('click', _ => {
    const real = parseFloat(document.getElementById('real').value);
    const imaginary = parseFloat(document.getElementById('imaginary').value);
    wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, real, imaginary, currentZoom);
});

// on reset
document.getElementById('reset-btn').addEventListener('click', _ => {
    currentZoom = 1.0;
    wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, 0.0, 0.0, currentZoom);
});

// on zoom in 
document.getElementById('zoom-in').addEventListener('click', _ => {
    currentZoom++;
    const real = parseFloat(document.getElementById('real').value);
    const imaginary = parseFloat(document.getElementById('imaginary').value);
    wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, real, imaginary, currentZoom);
});

// on zoom out
document.getElementById('zoom-out').addEventListener('click', _ => {
    if (currentZoom >= 2) {
        currentZoom--;
        const real = parseFloat(document.getElementById('real').value);
        const imaginary = parseFloat(document.getElementById('imaginary').value);
        wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, real, imaginary, currentZoom);
    }
});

