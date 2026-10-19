# Architecture

Grayscale is a static page. All work happens in the visitor's browser.

## Data flow

```
File input (PNG)
  -> createImageBitmap          browser decodes the file
  -> OffscreenCanvas 2D         drawImage, then getImageData: RGBA bytes
  -> grayscale(rgba)            Rust, compiled to WebAssembly
  -> putImageData               grayscale pixels back on the canvas
  -> convertToBlob(image/png)   browser encodes the result
  -> <img src=blob:...>         shown on the page
```

The browser does the decoding and encoding it already does well. Rust does only the pixel math. That keeps the WebAssembly module small, with no image codecs in it, and keeps the kernel a pure function that is easy to test.

## Rust crate

| File | Role |
| --- | --- |
| `src/kernel.rs` | `luma` and `grayscale_rgba`: pure functions over `&mut [u8]`, with unit tests that run natively under `cargo test`. |
| `src/lib.rs` | `grayscale(&[u8]) -> Result<Vec<u8>, JsError>`, the only export. It copies the input, runs the kernel, and turns a kernel error into a thrown JS `Error`. |
| `tests/wasm.rs` | `wasm-bindgen-test` tests of the export, run under Node by `wasm-pack test --node`. |

Luma uses the Rec. 709 weights (0.2126, 0.7152, 0.0722) in integer form, the same weights v1 used through the `image` crate. Alpha is kept as is.

`wasm-pack build --target web` writes `pkg/`: an ES module whose default export, `init()`, fetches the `.wasm` file next to it. Vite bundles that file as a static asset.

## Web

| File | Role |
| --- | --- |
| `web/index.html` | Markup only. The file input starts disabled until the module has loaded. |
| `web/src/main.ts` | Loads the module, runs the pipeline above, and reports errors in a live status line. Only the newest selection is shown. |
| `web/src/styles.css` | All styling, with colour tokens on `:root`, a dark scheme and reduced motion support. |

## Tooling

- Rust: toolchain 1.99.1 and the `wasm32-unknown-unknown` target, pinned in `rust-toolchain.toml`; edition 2024; clippy pedantic; `unsafe_code` forbidden.
- Web: Vite, TypeScript in strict mode, ESLint with `typescript-eslint` strict type-checked rules.
- CI (`.github/workflows/ci.yml`): format, clippy for host and wasm, Rust tests, wasm tests, build, lint.
- Dependabot: monthly, grouped, for cargo, npm and GitHub Actions.
