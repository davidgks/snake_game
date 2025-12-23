# Snake Game 🐍

**A simple Snake game built in Rust and compiled to WebAssembly, with a small TypeScript/Webpack frontend.**

---

## Features ✅

- Core game logic implemented in Rust (WASM-friendly).
- Browser UI and game loop wired up with TypeScript/JavaScript.
- Fast native-ish performance through WebAssembly.

---

## Repository layout 🔧

- `Cargo.toml`, `src/` — Rust crate for the game logic (compiled to WASM).
- `www/` — Frontend app (TypeScript + Webpack). Key files:
  - `www/bootstrap.js` — entry point
  - `www/index.ts` — TypeScript app code
  - `www/index.html` — HTML page served to the browser
  - `www/webpack.config.js` — build config (outputs to `www/public`)
- `pkg/` — generated wasm package (created by `wasm-pack`)

---

## Prerequisites ⚙️

- Rust (stable toolchain) — https://www.rust-lang.org/tools/install
- wasm-pack — https://rustwasm.github.io/wasm-pack/installer/
  - macOS: `brew install wasm-pack` (or use the installer script linked above)
- Node.js and npm (or yarn) — https://nodejs.org/

---

## Build & Run (development) ▶️

1. Build the WASM package from the project root (where `Cargo.toml` is):

```bash
wasm-pack build --target web
```

This command generates the `pkg/` folder which the frontend depends on (`www/package.json` references `file:../pkg`).

2. Install frontend dependencies and start dev server:

```bash
cd www
npm install
npm run dev
```

3. Open the dev server in your browser (webpack-dev-server default):

- http://localhost:8080 (or the URL shown in the terminal)

---

## Production build 🏁

From `www/` run:

```bash
npm run build
```

The built files are emitted to `www/public`. Serve that directory with any static file server (e.g., `serve`, `http-server`, or your own static hosting).

---

## Development notes & workflow 💡

- Edit game logic in `src/lib.rs` (Rust). Re-run `wasm-pack build --target web` to update `pkg/`.
- Edit UI in `www/` (TypeScript / JS / HTML). `npm run dev` runs `webpack-dev-server`.
- If you change Rust code while dev server is running, re-run `wasm-pack build` and refresh the browser.

---

## Contributing

Contributions are welcome — open an issue or a PR with a short description and any relevant details.

---

## License

This project does not include a license file. Add a `LICENSE` file to clarify terms if you plan to distribute.

---

If you want, I can add a short section describing the game controls and polishing steps (score, sound, mobile support). 💬
