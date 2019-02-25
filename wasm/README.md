__What's working__
- Initial Render of the Set on Re = [-2; 2]
- Coloring based on iterations
- Rendering to a given HTML5 Canvas Element

__How to use__

> npm install @juliankaindl/mandelbrot-wasm

Assuming all calls are async
```js
import * as wasm from "@juliankaindl/mandelbrot-wasm";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, 0.0, 0.0);
```

__WIP__
- Zooming
- Rendering around a specific point on the plane (the last 2 params dont have a meaning atm)
- Render with WebGL instead of HTML5 Canvas
- Add more input Options for
   - color palettes (predefined)
   - max iterations
- Update the dimensions