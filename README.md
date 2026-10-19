# Grayscale

[![CI](https://github.com/ZachariahAyers/grayscale/actions/workflows/ci.yml/badge.svg)](https://github.com/ZachariahAyers/grayscale/actions/workflows/ci.yml)

A web app that converts a PNG to grayscale in your browser, with no upload.

Live demo: [ayers.repair/opensource/grayscale](https://ayers.repair/opensource/grayscale/).

## Features

- Converts a PNG image to grayscale
- Runs entirely on your device; the image is never uploaded
- Needs no account and no server

## Tech stack

Rust, WebAssembly (wasm-bindgen, wasm-pack), TypeScript, Vite.

## Getting started

Requires Node.js 22.13 or newer and [rustup](https://rustup.rs). The Rust toolchain and WebAssembly target are set by `rust-toolchain.toml`.

```bash
git clone https://github.com/ZachariahAyers/grayscale.git
cd grayscale
npm ci
npm run dev
```

## Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
npx wasm-pack test --node --test wasm
npm run build
npm run lint
```

How it is put together: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## License

Apache License 2.0. See [LICENSE](LICENSE).

---

Built by [Zachariah Ayers](https://ayers.repair).
