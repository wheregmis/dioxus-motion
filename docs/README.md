# Dioxus Motion documentation

A Dioxus 0.7.10 app using the local motion library. The landing page and guides share a live spring playground, accessible tabs from [Dioxus Components](https://github.com/DioxusLabs/dioxus-components), and [dioxus-code](https://github.com/DioxusLabs/dioxus-code). Rust snippets are highlighted at compile time; syntax parsers do not run in the browser.

## Run locally

Use Rust 1.89 or newer, Node.js 22, and Dioxus CLI 0.7.10:

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked
cd docs
npm ci
npm run css
dx serve --platform web
```

For CSS edits, run `npm run css -- --watch` in a second terminal. Edit `input.css`; `assets/main.css` is generated. Theme tokens live only in `input.css`.

## Check and bundle

From the repository root:

```sh
cargo check -p docs --locked
cargo test --workspace --locked
```

From `docs/`:

```sh
npm run css -- --minify
dx bundle --release --platform web --locked
```

The bundle lives at `../target/dx/docs/release/web/public`. `Dioxus.toml` sets the project base path to `/dioxus-motion/`. Serve that directory at this base path when inspecting the release bundle.

## Publishing

The GitHub Pages workflow builds pull requests without deploying them. Pushes to `main`, or a manual run on a selected branch, build locked Rust dependencies, regenerate CSS, upload the bundle root, and deploy through GitHub’s official Pages actions. Set **Settings → Pages → Source → GitHub Actions** once for this repository. No `gh-pages` branch or nested `docs/` deployment directory is required.

The workflow copies `index.html` to `404.html`, allowing direct navigation to guide routes. GitHub Pages responds with HTTP 404 for these SPA fallbacks; the app still renders the requested route.

## Updating examples

`quick_start.rs` is both a compiled live example and the source shown by `code!`. Keep other displayed examples synchronized with their live components, and use `code_str!` for static snippets. The docs describe the development branch until the new API is published; do not label it as the published 0.3.6 API.

The primitives dependency is pinned to an upstream revision. Review and verify keyboard behavior before updating it. The highlighter is pinned to a published version.

## Maintaining lessons

Keep each runnable example in `src/pages/lessons/`. Guide code panels load that file with `dioxus_code::code!`, so displayed code cannot diverge from the preview. Use `Lesson` for a goal, preview, concrete exercise, and complete source. Keep examples finite or provide a Stop control. Compile and browser-check the guide after changes.
