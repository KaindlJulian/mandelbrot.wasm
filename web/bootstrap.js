// for consistency
import ("normalize.css")
// for components
import("spectre.css");

// To make all wasm calls async
import("./index.js")
  .catch(e => console.error("Error importing `index.js`:", e));
