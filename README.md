# Grayscale

Convert a PNG to grayscale in your browser. The image is processed on your device by Rust compiled to WebAssembly, and it is never uploaded anywhere.

By [Zachariah Ayers](https://ayers.repair) · [GitHub](https://github.com/ZachariahAyers)

## Why

Converting an image should not need a server, an account or an upload. The browser already decodes and encodes images well. The one step that is pure computation, turning every pixel into its luma, is a small Rust function compiled to WebAssembly and tested on its own.

## Stack

- Rust (edition 2024) and wasm-bindgen, built with wasm-pack
- TypeScript on Vite
- Canvas APIs for decoding and encoding: `createImageBitmap`, `OffscreenCanvas`
- rustfmt, clippy, ESLint with typescript-eslint, and GitHub Actions CI

## Run it

You need Node.js 22.13 or later and [rustup](https://rustup.rs). The toolchain and the WebAssembly target come from `rust-toolchain.toml`. wasm-pack is installed by `npm ci`.

```bash
git clone https://github.com/ZachariahAyers/grayscale.git
cd grayscale
npm ci
npm run dev        # dev server with a debug build of the kernel
npm run build      # production build in dist/
```

Checks:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test                              # kernel unit tests
npx wasm-pack test --node --test wasm   # the exported API, in WebAssembly
npm run lint                            # after a build, which generates pkg/
```

## Structure

```
src/kernel.rs        pixel kernels and their unit tests
src/lib.rs           the WebAssembly export
tests/wasm.rs        tests across the JavaScript boundary
web/                 the page: index.html, src/main.ts, src/styles.css
docs/ARCHITECTURE.md how a file becomes a grayscale image
```

## License

Apache License 2.0. See [LICENSE](LICENSE).
