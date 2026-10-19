# Changelog

## Unreleased

- A live demo at ayers.repair/opensource/grayscale/, published from `ci.yml` after the checks pass on `main`.
- The kernel walks pixels as fixed four-byte chunks, as clippy now requires. Behaviour is unchanged.
- `rust-toolchain.toml` pins Rust 1.99.1 instead of following `stable`.
- The package description says PNG, the only format the page accepts.

## 2.0.0

Converts a PNG to grayscale in the browser with a tested Rust kernel compiled to WebAssembly; nothing leaves the device.

- Rust edition 2024, with the toolchain pinned to an exact version in `rust-toolchain.toml`.
- The kernel works on raw RGBA pixels; the browser decodes and encodes the image.
- Invalid input throws a JavaScript error; the module never panics on it.
- Unit tests for the kernel, plus wasm-bindgen tests for the exported function.
- TypeScript front end on Vite.
- Hand-written CSS with a bundled heading font, so no third-party requests are made.
- The file control works from the keyboard. Errors are announced in a live status line. Animation stops under reduced motion. Dark mode is supported.
- CI runs format, clippy, tests, build and lint. Dependabot checks monthly.
