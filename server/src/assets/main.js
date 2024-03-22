// This file imports and boots the JS wrapper around our wasm code.
// If we exported specific functions (like `greet`) from our wasm code,
// we could import them like this:
//
// import init, { greet } from './browser.js'
//
// The `from` part of this import gets replaced with the actual filename
// when we include this file in the HTML.
import wasmMain from './{browser_js_filename}'

async function main () {
  // Run our wasm code's main function.
  await wasmMain()
}

main()
