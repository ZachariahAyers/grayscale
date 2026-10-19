# Changelog

## 2.0.0

A rebuild on the current toolchain. Same purpose: upload a PNG, see it in grayscale.

- Rust edition 2024 on the stable toolchain, pinned with `rust-toolchain.toml`.
- The kernel works on raw RGBA pixels. The browser decodes and encodes the image, so the `image` and `base64` crates and the base64 round trip are gone.
- Invalid input throws a JavaScript error instead of panicking the module.
- Unit tests for the kernel, plus wasm-bindgen tests for the exported function.
- TypeScript front end on Vite, replacing JavaScript on webpack.
- Hand-written CSS replaces the Tailwind v2 CDN stylesheet. The heading font is bundled, so no third-party requests are made.
- The upload control works from the keyboard. Errors are announced in a live status line. Animation stops under reduced motion. Dark mode is supported.
- CI runs format, clippy, tests, build and lint. Dependabot checks monthly.
- Removed the S3 deploy and its long-lived AWS keys, and stopped committing build output.

## 1.0.0

The first version: a webpack page that sent a base64 PNG to Rust, which decoded it with the `image` crate, converted it to grayscale and returned a PNG data URL.
