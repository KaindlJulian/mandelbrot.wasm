__What's working__
- Initial Render of the Set on Re = [-2; 2]
- Coloring based on iterations
- Rendering to a given HTML5 Canvas Element
- Render around a given point on the plane
- Simple zooming
   - At a given zoomLvl: Will render [-2/zoomLvl;2/zoomLvl] around the point in both axis
   - zoomLvl > 0

__How to use__

> npm install @juliankaindl/mandelbrot-wasm

Assuming all calls are async
```js
import * as wasm from "@juliankaindl/mandelbrot-wasm";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

// draw(ctx, width, height, realPart, imaginaryPart, zoomLvl)
wasm.draw(ctx, canvas.scrollWidth, canvas.scrollHeight, 0.0, 0.0, 1.0);
```

__WIP__
- Proper Zooming
- Render with WebGL instead of HTML5 Canvas
- Add more input Options for
   - color palettes (predefined)
   - max iterations
- Update the dimensions