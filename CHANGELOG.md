# Changelog

## Unreleased

- A live demo at ayers.repair/opensource/grayscale/, published from `ci.yml` after the checks pass on `main`.
- The kernel walks pixels as fixed four-byte chunks, as clippy now requires. Behaviour is unchanged.
- `rust-toolchain.toml` pins Rust 1.99.1 instead of following `stable`.
- The package description says PNG, the only format the page accepts.

## 2.0.0

Built on the current toolchain: upload a PNG, see it in grayscale.

- Rust edition 2024, with the toolchain pinned to an exact version in `rust-toolchain.toml`.
- The kernel works on raw RGBA pixels. The browser decodes and encodes the image, so the `image` and `base64` crates and the base64 round trip are gone.
- Invalid input throws a JavaScript error instead of panicking the module.
- Unit tests for the kernel, plus wasm-bindgen tests for the exported function.
- TypeScript front end on Vite, replacing JavaScript on webpack.
- Hand-written CSS replaces the Tailwind v2 CDN stylesheet. The heading font is bundled, so no third-party requests are made.
- The upload control works from the keyboard. Errors are announced in a live status line. Animation stops under reduced motion. Dark mode is supported.
- CI runs format, clippy, tests, build and lint. Dependabot checks monthly.
- Removed the S3 deploy and its long-lived AWS keys, and stopped committing build output.
