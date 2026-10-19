# Grayscale

Converts an image to grayscale in the browser. The browser decodes the file; a Rust kernel compiled to WebAssembly rewrites the RGBA pixels; nothing is uploaded.

## Commands

- Setup: `npm ci` (installs wasm-pack too). Rust comes from `rust-toolchain.toml`.
- Rust checks: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
- Boundary test: `npx wasm-pack test --node --test wasm`
- Build: `npm run build` (wasm-pack, then `tsc`, then Vite into `dist/`)
- Lint: `npm run lint`. It needs `pkg/` from `npm run wasm` or `npm run build` first, because typed lint reads the generated bindings.
- Dev server: `npm run dev`. It runs until stopped; do not start it unless asked.

## Layout

- `src/kernel.rs`: pure pixel kernels and their unit tests. No wasm-bindgen types here.
- `src/lib.rs`: the `#[wasm_bindgen]` exports only, each a thin wrapper over a kernel.
- `tests/wasm.rs`: the exported API tested across the JS boundary (wasm32 only).
- `web/`: the page. `index.html`, `src/main.ts`, `src/styles.css`.
- `pkg/` and `dist/` are generated and git-ignored. Never edit or commit them.

See `docs/ARCHITECTURE.md` for the data flow.

## Conventions

- Kernel behaviour changes start with a failing test in `src/kernel.rs`.
- Exports return `Result<_, JsError>`; never `unwrap` or panic on input from JS.
- No inline styles or scripts in HTML. CSS colours are custom properties on `:root`.
- TypeScript is strict; `typescript-eslint` caps TypeScript below 6.1, so do not bump it past that.
- Conventional commits, lower-case after the type (`fix: ...`, `refactor(kernel): ...`).

## Done means

Every command in the Rust checks, the boundary test, build and lint passes, and `CHANGELOG.md` has an entry for any user-visible change.
