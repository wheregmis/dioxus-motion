# Dioxus Motion Docs

This crate contains the Dioxus Motion documentation site that lives under `docs/` in the repository. It is a standalone Dioxus app that uses the local `dioxus-motion` crate, Tailwind CSS, and the transitions feature to demonstrate the current store-backed motion API.

## Prerequisites

- Rust toolchain with `cargo`
- Dioxus CLI (`cargo install dioxus-cli` if you do not already have `dx`)
- Node.js + npm for the Tailwind build step

## Install frontend dependencies

```bash
cd docs
npm install
```

## Build CSS

Tailwind compiles `input.css` into `assets/main.css`.

```bash
cd docs
npm run css
```

For iterative work you can keep the CSS watcher running:

```bash
cd docs
just css
```

## Run the docs app

```bash
cd docs
dx serve --platform web
```

The checked-in `Dioxus.toml` sets `base_path = "dioxus-motion"`, matching the GitHub Pages deployment path.

## Verification

Run the docs crate against the local workspace dependency:

```bash
cargo check -p docs --features web
```

If you only changed prose or examples, you should still re-run the command above so the docs app stays aligned with the current library API.
